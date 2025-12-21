//! Newbee OCR CLI
//!
//! A powerful and easy-to-use command-line OCR tool based on rust-paddle-ocr.
//!
//! Features:
//! - Single image OCR
//! - Batch processing with pipeline optimization
//! - Multiple language models support
//! - Embedded models support via features
//! - JSON output format
//! - Progress display for batch processing

mod models;
mod pipeline;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use ocr_rs::{Backend, OcrEngine, OcrEngineConfig};
use serde::{Deserialize, Serialize};

use crate::models::{
    print_models_table, DetectionModel, EmbeddedModels, ModelResolver, RecognitionModel,
};
use crate::pipeline::{OcrPipeline, PipelineConfig, PipelineStats};

/// Newbee OCR - A powerful OCR command-line tool
#[derive(Parser)]
#[command(
    name = "newbee-ocr",
    author = "ChenZibo <qw.54@163.com>",
    version,
    about = "A powerful and easy-to-use OCR command-line tool based on PaddleOCR",
    long_about = "Newbee OCR provides high-performance text recognition with support for \
                  multiple languages, batch processing, and flexible configuration options."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform OCR on a single image
    #[command(visible_alias = "r")]
    Recognize {
        /// Path to the input image
        #[arg(value_name = "IMAGE")]
        input: PathBuf,

        /// Recognition model/language to use
        #[arg(short = 'l', long, default_value = "chinese")]
        language: String,

        /// Detection model version
        #[arg(short = 'd', long, default_value = "v5")]
        det_model: String,

        /// Path to models directory
        #[arg(short = 'm', long, value_name = "DIR")]
        models_dir: Option<PathBuf>,

        /// Output format
        #[arg(short = 'f', long, default_value = "text", value_enum)]
        format: OutputFormat,

        /// Output file (default: stdout)
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Engine precision mode
        #[arg(long, default_value = "balanced", value_enum)]
        precision: PrecisionModeArg,

        /// Number of threads
        #[arg(short = 't', long, default_value_t = 4)]
        threads: i32,

        /// GPU backend to use
        #[arg(long, value_enum)]
        gpu: Option<GpuBackend>,

        /// Show timing information
        #[arg(long)]
        timing: bool,

        /// Verbose output
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Batch process images in a directory
    #[command(visible_alias = "b")]
    Batch {
        /// Path to the input directory
        #[arg(value_name = "DIR")]
        input: PathBuf,

        /// Recognition model/language to use
        #[arg(short = 'l', long, default_value = "chinese")]
        language: String,

        /// Detection model version
        #[arg(short = 'd', long, default_value = "v5")]
        det_model: String,

        /// Path to models directory
        #[arg(short = 'm', long, value_name = "DIR")]
        models_dir: Option<PathBuf>,

        /// Output format
        #[arg(short = 'f', long, default_value = "text", value_enum)]
        format: OutputFormat,

        /// Output directory (default: print to stdout)
        #[arg(short = 'o', long, value_name = "DIR")]
        output: Option<PathBuf>,

        /// Process subdirectories recursively
        #[arg(short = 'r', long)]
        recursive: bool,

        /// Engine precision mode
        #[arg(long, default_value = "fast", value_enum)]
        precision: PrecisionModeArg,

        /// Number of threads per engine
        #[arg(short = 't', long, default_value_t = 4)]
        threads: i32,

        /// Number of image loader threads
        #[arg(long)]
        loader_threads: Option<usize>,

        /// GPU backend to use
        #[arg(long, value_enum)]
        gpu: Option<GpuBackend>,

        /// Show progress bar
        #[arg(long, default_value_t = true)]
        progress: bool,

        /// Show statistics after processing
        #[arg(long, default_value_t = true)]
        stats: bool,

        /// Verbose output
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// List available models and supported languages
    #[command(visible_alias = "ls")]
    List {
        /// Show detailed information
        #[arg(short = 'd', long)]
        detailed: bool,
    },

    /// Show information about a specific model
    Info {
        /// Model name (e.g., chinese, korean, latin)
        model: String,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    /// Plain text output
    Text,
    /// JSON output
    Json,
    /// JSON Lines (one JSON object per line)
    Jsonl,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum PrecisionModeArg {
    /// Fast mode - optimized for speed
    Fast,
}

impl PrecisionModeArg {
    fn to_engine_config(&self) -> OcrEngineConfig {
        match self {
            PrecisionModeArg::Fast => OcrEngineConfig::fast(),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum GpuBackend {
    /// Metal (macOS/iOS)
    Metal,
    /// OpenCL (cross-platform)
    Opencl,
    /// Vulkan (cross-platform)
    Vulkan,
    /// CUDA (NVIDIA)
    Cuda,
}

impl GpuBackend {
    fn to_backend(&self) -> Backend {
        match self {
            GpuBackend::Metal => Backend::Metal,
            GpuBackend::Opencl => Backend::OpenCL,
            GpuBackend::Vulkan => Backend::Vulkan,
            GpuBackend::Cuda => Backend::CUDA,
        }
    }
}

/// OCR result for JSON output
#[derive(Debug, Serialize, Deserialize)]
struct OcrOutput {
    /// Source image path
    file: String,
    /// Recognition results
    results: Vec<TextRegion>,
    /// Processing time in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    time_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TextRegion {
    /// Recognized text
    text: String,
    /// Confidence score (0-1)
    confidence: f32,
    /// Bounding box
    bbox: BoundingBox,
}

#[derive(Debug, Serialize, Deserialize)]
struct BoundingBox {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

fn main() -> Result<()> {
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Recognize {
            input,
            language,
            det_model,
            models_dir,
            format,
            output,
            precision,
            threads,
            gpu,
            timing,
            verbose,
        } => cmd_recognize(
            &input,
            &language,
            &det_model,
            models_dir.as_deref(),
            format,
            output.as_deref(),
            precision,
            threads,
            gpu,
            timing,
            verbose,
        ),
        Commands::Batch {
            input,
            language,
            det_model,
            models_dir,
            format,
            output,
            recursive,
            precision,
            threads,
            loader_threads,
            gpu,
            progress,
            stats,
            verbose,
        } => cmd_batch(
            &input,
            &language,
            &det_model,
            models_dir.as_deref(),
            format,
            output.as_deref(),
            recursive,
            precision,
            threads,
            loader_threads,
            gpu,
            progress,
            stats,
            verbose,
        ),
        Commands::List { detailed } => cmd_list(detailed),
        Commands::Info { model } => cmd_info(&model),
    }
}

/// 创建 OCR 引擎
fn create_engine(
    rec_model: RecognitionModel,
    det_model: DetectionModel,
    models_dir: Option<&Path>,
    precision: PrecisionModeArg,
    threads: i32,
    gpu: Option<GpuBackend>,
    verbose: bool,
) -> Result<OcrEngine> {
    let mut config = precision.to_engine_config();
    config.thread_count = threads;

    if let Some(gpu_backend) = gpu {
        config.backend = gpu_backend.to_backend();
    }

    // 首先尝试使用内嵌模型
    if let (Some(det_bytes), Some(rec_bytes), Some(charset_bytes)) = (
        EmbeddedModels::get_det_model(det_model),
        EmbeddedModels::get_rec_model(rec_model),
        EmbeddedModels::get_charset(rec_model),
    ) {
        if verbose {
            println!(
                "{} Using embedded models for {} detection and {} recognition",
                "ℹ".blue(),
                det_model,
                rec_model
            );
        }

        return OcrEngine::from_bytes(det_bytes, rec_bytes, charset_bytes, Some(config))
            .map_err(|e| anyhow::anyhow!("Failed to create OCR engine: {}", e));
    }

    // 否则从文件加载
    let resolver = ModelResolver::new(models_dir);

    let det_path = resolver
        .resolve_det_model(det_model)
        .or_else(|| {
            // 尝试直接路径
            models_dir.map(|d| d.join(det_model.model_filename()))
        })
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Detection model not found: {}. Please specify --models-dir or use embedded models.",
                det_model.model_filename()
            )
        })?;

    let rec_path = resolver
        .resolve_rec_model(rec_model)
        .or_else(|| models_dir.map(|d| d.join(rec_model.model_filename())))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Recognition model not found: {}. Please specify --models-dir or use embedded models.",
                rec_model.model_filename()
            )
        })?;

    let charset_path = resolver
        .resolve_charset(rec_model)
        .or_else(|| models_dir.map(|d| d.join(rec_model.charset_filename())))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Charset file not found: {}. Please specify --models-dir.",
                rec_model.charset_filename()
            )
        })?;

    if verbose {
        println!(
            "{} Loading detection model: {}",
            "ℹ".blue(),
            det_path.display()
        );
        println!(
            "{} Loading recognition model: {}",
            "ℹ".blue(),
            rec_path.display()
        );
        println!("{} Loading charset: {}", "ℹ".blue(), charset_path.display());
    }

    OcrEngine::new(&det_path, &rec_path, &charset_path, Some(config))
        .map_err(|e| anyhow::anyhow!("Failed to create OCR engine: {}", e))
}

/// 单图识别命令
fn cmd_recognize(
    input: &Path,
    language: &str,
    det_model_str: &str,
    models_dir: Option<&Path>,
    format: OutputFormat,
    output: Option<&Path>,
    precision: PrecisionModeArg,
    threads: i32,
    gpu: Option<GpuBackend>,
    timing: bool,
    verbose: bool,
) -> Result<()> {
    // 解析模型类型
    let rec_model = RecognitionModel::from_str(language)
        .ok_or_else(|| anyhow::anyhow!("Unknown language/model: {}", language))?;

    let det_model = DetectionModel::from_str(det_model_str)
        .ok_or_else(|| anyhow::anyhow!("Unknown detection model: {}", det_model_str))?;

    // 检查输入文件
    if !input.exists() {
        anyhow::bail!("Input file not found: {}", input.display());
    }

    if verbose {
        println!("{} Input: {}", "ℹ".blue(), input.display());
        println!("{} Language: {} ({})", "ℹ".blue(), language, rec_model);
        println!("{} Detection model: {}", "ℹ".blue(), det_model);
        println!("{} Precision: {:?}", "ℹ".blue(), precision);
        println!("{} Threads: {}", "ℹ".blue(), threads);
        if let Some(gpu) = gpu {
            println!("{} GPU backend: {:?}", "ℹ".blue(), gpu);
        }
        println!();
    }

    // 创建引擎
    let start = Instant::now();
    let engine = create_engine(
        rec_model, det_model, models_dir, precision, threads, gpu, verbose,
    )?;
    let engine_time = start.elapsed();

    if verbose {
        println!("{} Engine loaded in {:?}", "✓".green(), engine_time);
    }

    // 执行 OCR
    let start = Instant::now();
    let image =
        image::open(input).with_context(|| format!("Failed to open image: {}", input.display()))?;

    let results = engine
        .recognize(&image)
        .map_err(|e| anyhow::anyhow!("OCR failed: {}", e))?;
    let ocr_time = start.elapsed();

    // 格式化输出
    let ocr_output = OcrOutput {
        file: input.display().to_string(),
        results: results
            .iter()
            .map(|r| TextRegion {
                text: r.text.clone(),
                confidence: r.confidence,
                bbox: BoundingBox {
                    x: r.bbox.rect.left(),
                    y: r.bbox.rect.top(),
                    width: r.bbox.rect.width(),
                    height: r.bbox.rect.height(),
                },
            })
            .collect(),
        time_ms: if timing {
            Some(ocr_time.as_millis() as u64)
        } else {
            None
        },
    };

    let output_str = format_output(&ocr_output, format)?;

    // 写入输出
    if let Some(output_path) = output {
        fs::write(output_path, &output_str)
            .with_context(|| format!("Failed to write output: {}", output_path.display()))?;

        if verbose {
            println!(
                "{} Output written to: {}",
                "✓".green(),
                output_path.display()
            );
        }
    } else {
        println!("{}", output_str);
    }

    if timing && !matches!(format, OutputFormat::Json | OutputFormat::Jsonl) {
        println!();
        println!("{} OCR completed in {:?}", "⏱".cyan(), ocr_time);
    }

    Ok(())
}

/// 批量处理命令
fn cmd_batch(
    input: &Path,
    language: &str,
    det_model_str: &str,
    models_dir: Option<&Path>,
    format: OutputFormat,
    output: Option<&Path>,
    recursive: bool,
    precision: PrecisionModeArg,
    threads: i32,
    loader_threads: Option<usize>,
    gpu: Option<GpuBackend>,
    progress: bool,
    stats: bool,
    verbose: bool,
) -> Result<()> {
    // 解析模型类型
    let rec_model = RecognitionModel::from_str(language)
        .ok_or_else(|| anyhow::anyhow!("Unknown language/model: {}", language))?;

    let det_model = DetectionModel::from_str(det_model_str)
        .ok_or_else(|| anyhow::anyhow!("Unknown detection model: {}", det_model_str))?;

    // 检查输入目录
    if !input.is_dir() {
        anyhow::bail!("Input path is not a directory: {}", input.display());
    }

    // 收集图片
    let images = pipeline::collect_images(input, recursive)?;

    if images.is_empty() {
        println!("{} No images found in: {}", "⚠".yellow(), input.display());
        return Ok(());
    }

    if verbose {
        println!("{} Found {} images", "ℹ".blue(), images.len());
        println!("{} Language: {} ({})", "ℹ".blue(), language, rec_model);
        println!("{} Detection model: {}", "ℹ".blue(), det_model);
        println!("{} Recursive: {}", "ℹ".blue(), recursive);
        println!();
    }

    // 创建输出目录
    if let Some(output_dir) = output {
        fs::create_dir_all(output_dir).with_context(|| {
            format!(
                "Failed to create output directory: {}",
                output_dir.display()
            )
        })?;
    }

    // 创建引擎
    let start = Instant::now();
    let engine = create_engine(
        rec_model, det_model, models_dir, precision, threads, gpu, verbose,
    )?;
    let engine_time = start.elapsed();

    if verbose {
        println!("{} Engine loaded in {:?}", "✓".green(), engine_time);
        println!();
    }

    // 配置流水线
    let mut pipeline_config = PipelineConfig::new().with_progress(progress);

    if let Some(loaders) = loader_threads {
        pipeline_config = pipeline_config.with_loader_threads(loaders);
    }

    // 创建流水线处理器
    let pipeline = OcrPipeline::new(engine, pipeline_config);

    // 处理图片
    let total_start = Instant::now();
    let results = pipeline.process_batch(images);
    let total_time = total_start.elapsed();

    // 输出结果
    match format {
        OutputFormat::Jsonl => {
            // JSON Lines 格式：每个结果一行
            for result in &results {
                let ocr_output = OcrOutput {
                    file: result.task.path.display().to_string(),
                    results: match &result.results {
                        Ok(rs) => rs
                            .iter()
                            .map(|r| TextRegion {
                                text: r.text.clone(),
                                confidence: r.confidence,
                                bbox: BoundingBox {
                                    x: r.bbox.rect.left(),
                                    y: r.bbox.rect.top(),
                                    width: r.bbox.rect.width(),
                                    height: r.bbox.rect.height(),
                                },
                            })
                            .collect(),
                        Err(_) => Vec::new(),
                    },
                    time_ms: Some(result.duration.as_millis() as u64),
                };

                let json = serde_json::to_string(&ocr_output)?;

                if let Some(output_dir) = output {
                    let filename = result
                        .task
                        .path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let output_path = output_dir.join(format!("{}.json", filename));
                    fs::write(&output_path, &json)?;
                } else {
                    println!("{}", json);
                }
            }
        }
        OutputFormat::Json => {
            // 完整 JSON 数组
            let outputs: Vec<OcrOutput> = results
                .iter()
                .map(|result| OcrOutput {
                    file: result.task.path.display().to_string(),
                    results: match &result.results {
                        Ok(rs) => rs
                            .iter()
                            .map(|r| TextRegion {
                                text: r.text.clone(),
                                confidence: r.confidence,
                                bbox: BoundingBox {
                                    x: r.bbox.rect.left(),
                                    y: r.bbox.rect.top(),
                                    width: r.bbox.rect.width(),
                                    height: r.bbox.rect.height(),
                                },
                            })
                            .collect(),
                        Err(_) => Vec::new(),
                    },
                    time_ms: Some(result.duration.as_millis() as u64),
                })
                .collect();

            let json = serde_json::to_string_pretty(&outputs)?;

            if let Some(output_dir) = output {
                let output_path = output_dir.join("results.json");
                fs::write(&output_path, &json)?;
                println!(
                    "{} Results written to: {}",
                    "✓".green(),
                    output_path.display()
                );
            } else {
                println!("{}", json);
            }
        }
        OutputFormat::Text => {
            for result in &results {
                let filename = result
                    .task
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                if let Some(output_dir) = output {
                    let output_path = output_dir.join(format!("{}.txt", filename));
                    let mut file = fs::File::create(&output_path)?;

                    match &result.results {
                        Ok(rs) => {
                            for r in rs {
                                writeln!(file, "{}", r.text)?;
                            }
                        }
                        Err(e) => {
                            writeln!(file, "Error: {}", e)?;
                        }
                    }
                } else {
                    println!("{}", "─".repeat(60).dimmed());
                    println!("{} {}", "File:".bright_cyan(), filename);
                    println!("{}", "─".repeat(60).dimmed());

                    match &result.results {
                        Ok(rs) => {
                            if rs.is_empty() {
                                println!("{}", "(No text detected)".dimmed());
                            } else {
                                for (i, r) in rs.iter().enumerate() {
                                    println!(
                                        "[{}] {} {}",
                                        i + 1,
                                        r.text,
                                        format!("({:.0}%)", r.confidence * 100.0).dimmed()
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            println!("{} {}", "Error:".red(), e);
                        }
                    }
                    println!();
                }
            }

            if let Some(output_dir) = output {
                println!(
                    "{} Results written to: {}",
                    "✓".green(),
                    output_dir.display()
                );
            }
        }
    }

    // 显示统计信息
    if stats {
        let pipeline_stats = PipelineStats::from_results(&results);
        pipeline_stats.print();

        println!("  {} {:?}", "Total wall time:".bright_cyan(), total_time);
        println!();
    }

    Ok(())
}

/// 列出模型命令
fn cmd_list(detailed: bool) -> Result<()> {
    print_models_table();

    if detailed {
        println!();
        println!("{}", "Embedded Models Status:".bright_white().bold());
        println!();

        let det_models = EmbeddedModels::embedded_det_models();
        let rec_models = EmbeddedModels::embedded_rec_models();

        if det_models.is_empty() && rec_models.is_empty() {
            println!(
                "  {}",
                "No embedded models. Use --models-dir to specify external models.".dimmed()
            );
        } else {
            if !det_models.is_empty() {
                println!(
                    "  {} Detection: {}",
                    "✓".green(),
                    det_models
                        .iter()
                        .map(|m| m.name())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            if !rec_models.is_empty() {
                println!(
                    "  {} Recognition: {}",
                    "✓".green(),
                    rec_models
                        .iter()
                        .map(|m| m.name())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }

        println!();
        println!("{}", "To embed models, compile with features:".dimmed());
        println!(
            "  {}",
            "cargo build --features embed-det-v5,embed-rec-chinese".dimmed()
        );
    }

    Ok(())
}

/// 模型信息命令
fn cmd_info(model_name: &str) -> Result<()> {
    if let Some(rec_model) = RecognitionModel::from_str(model_name) {
        println!();
        println!(
            "{} {}",
            "Recognition Model:".bright_white().bold(),
            rec_model.name().bright_cyan()
        );
        println!();

        println!(
            "  {} {}",
            "Model file:".bright_cyan(),
            rec_model.model_filename()
        );
        println!(
            "  {} {}",
            "Charset file:".bright_cyan(),
            rec_model.charset_filename()
        );

        let embedded = EmbeddedModels::get_rec_model(rec_model).is_some();
        println!(
            "  {} {}",
            "Embedded:".bright_cyan(),
            if embedded {
                "Yes".green()
            } else {
                "No".yellow()
            }
        );

        println!();
        println!("  {}", "Supported Languages:".bright_cyan());

        for (i, lang) in rec_model.supported_languages().iter().enumerate() {
            if i > 0 && i % 5 == 0 {
                println!();
            }
            print!("    • {}", lang);
            if (i + 1) % 5 != 0 && i < rec_model.supported_languages().len() - 1 {
                print!(",");
            }
        }
        println!();
        println!();

        return Ok(());
    }

    if let Some(det_model) = DetectionModel::from_str(model_name) {
        println!();
        println!(
            "{} {}",
            "Detection Model:".bright_white().bold(),
            det_model.name().bright_cyan()
        );
        println!();

        println!(
            "  {} {}",
            "Model file:".bright_cyan(),
            det_model.model_filename()
        );

        let embedded = EmbeddedModels::get_det_model(det_model).is_some();
        println!(
            "  {} {}",
            "Embedded:".bright_cyan(),
            if embedded {
                "Yes".green()
            } else {
                "No".yellow()
            }
        );

        let desc = match det_model {
            DetectionModel::V5 => "PP-OCRv5 detection model - Recommended for most use cases",
            DetectionModel::V5Fp16 => {
                "PP-OCRv5 FP16 detection model - Faster inference, lower memory"
            }
            DetectionModel::V4 => "PP-OCRv4 detection model - Legacy version, good compatibility",
        };
        println!("  {} {}", "Description:".bright_cyan(), desc);
        println!();

        return Ok(());
    }

    anyhow::bail!(
        "Unknown model: {}. Use 'newbee-ocr list' to see available models.",
        model_name
    );
}

/// 格式化输出
fn format_output(output: &OcrOutput, format: OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(output)?),
        OutputFormat::Jsonl => Ok(serde_json::to_string(output)?),
        OutputFormat::Text => {
            let mut lines = Vec::new();

            if output.results.is_empty() {
                lines.push("(No text detected)".to_string());
            } else {
                for (i, region) in output.results.iter().enumerate() {
                    lines.push(format!(
                        "[{}] {} ({:.0}%)",
                        i + 1,
                        region.text,
                        region.confidence * 100.0
                    ));
                }
            }

            Ok(lines.join("\n"))
        }
    }
}

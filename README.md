# Newbee OCR CLI

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](../LICENSE)

A powerful and easy-to-use command-line OCR tool based on [rust-paddle-ocr](https://github.com/zibo-chen/rust-paddle-ocr), providing high-performance text recognition with support for multiple languages, batch processing, and flexible configuration options.

## ✨ Features

- 🚀 **High Performance**: Built on MNN inference engine for fast text recognition
- 🌍 **Multi-Language Support**: 12+ recognition models covering major languages worldwide
- 📦 **Batch Processing**: Efficient pipeline architecture for processing multiple images
- 🎯 **Single Image OCR**: Quick recognition for individual images
- 💾 **Multiple Output Formats**: Text, JSON, and JSON Lines
- 🔧 **Flexible Configuration**: Customizable precision modes, thread count, and GPU acceleration
- 📊 **Statistics**: Detailed processing statistics for batch operations
- 🎨 **Embedded Models**: Optional feature to embed models into binary for portable deployment

## 📦 Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/zibo-chen/rust-paddle-ocr.git
cd rust-paddle-ocr/newbee_ocr_cli

# Build the project
cargo build --release

# The binary will be available at target/release/nbocr
```

### With Embedded Models (Optional)

You can embed specific models into the binary for portable deployment:

```bash
# Build with Chinese recognition model embedded
cargo build --release --features embed-rec-chinese

# Build with multiple models
cargo build --release --features "embed-det-v5,embed-rec-chinese,embed-rec-english"
```

Available features:
- Detection models: `embed-det-v5`, `embed-det-v5-fp16`, `embed-det-v4`
- Recognition models: `embed-rec-chinese`, `embed-rec-korean`, `embed-rec-latin`, `embed-rec-eslav`, `embed-rec-thai`, `embed-rec-greek`, `embed-rec-english`, `embed-rec-cyrillic`, `embed-rec-arabic`, `embed-rec-devanagari`, `embed-rec-tamil`, `embed-rec-telugu`

## 🚀 Quick Start

### Single Image Recognition

```bash
# Basic usage
nbocr recognize image.png --models-dir ./models

# With timing information
nbocr r image.png -m ./models --timing

# JSON output
nbocr r image.png -m ./models -f json -o output.json
```

### Batch Processing

```bash
# Process all images in a directory
nbocr batch ./images --models-dir ./models

# With statistics and progress bar
nbocr b ./images -m ./models --stats --progress

# Recursive processing with JSON output
nbocr b ./images -m ./models --recursive -f json -o results.jsonl
```

### List Available Models

```bash
# Show all supported models
nbocr list

# Show detailed information
nbocr ls --detailed
```

### Model Information

```bash
# Get detailed info about a specific model
nbocr info latin
```

## 📖 Commands

### `recognize` (alias: `r`)

Perform OCR on a single image.

```bash
nbocr recognize <IMAGE> [OPTIONS]
```

**Options:**
- `-l, --language <LANGUAGE>`: Recognition model/language to use (default: chinese)
- `-d, --det-model <DET_MODEL>`: Detection model version (default: v5)
- `-m, --models-dir <DIR>`: Path to models directory
- `-f, --format <FORMAT>`: Output format (text, json, jsonl)
- `-o, --output <FILE>`: Output file (default: stdout)
- `--precision <PRECISION>`: Engine precision mode (fast, balanced, high)
- `-t, --threads <THREADS>`: Number of threads (default: 4)
- `--gpu <GPU>`: GPU backend to use (metal, opencl, vulkan, cuda)
- `--timing`: Show timing information
- `-v, --verbose`: Verbose output

**Examples:**

```bash
# Basic recognition
nbocr r image.png -m ./models

# Chinese text recognition
nbocr r document.jpg -m ./models -l chinese

# English text with GPU acceleration (macOS)
nbocr r page.png -m ./models -l english --gpu metal

# Output as JSON
nbocr r image.png -m ./models -f json -o result.json
```

### `batch` (alias: `b`)

Batch process images in a directory with pipeline optimization.

```bash
nbocr batch <DIRECTORY> [OPTIONS]
```

**Options:**
- `-l, --language <LANGUAGE>`: Recognition model/language to use (default: chinese)
- `-d, --det-model <DET_MODEL>`: Detection model version (default: v5)
- `-m, --models-dir <DIR>`: Path to models directory
- `-f, --format <FORMAT>`: Output format (text, json, jsonl)
- `-o, --output <FILE>`: Output file (default: stdout)
- `--recursive`: Process subdirectories recursively
- `--precision <PRECISION>`: Engine precision mode (fast)
- `-t, --threads <THREADS>`: Number of threads (default: 4)
- `--loader-threads <N>`: Number of image loader threads (default: auto)
- `--gpu <GPU>`: GPU backend to use
- `--progress`: Show progress bar
- `--stats`: Display processing statistics
- `-v, --verbose`: Verbose output

**Examples:**

```bash
# Process all images in a directory
nbocr b ./photos -m ./models --stats

# Recursive processing with progress
nbocr b ./documents -m ./models --recursive --progress

# High performance batch processing
nbocr b ./images -m ./models -t 8 --loader-threads 4 --progress

# Save results as JSON Lines
nbocr b ./scans -m ./models -f jsonl -o results.jsonl
```

### `list` (alias: `ls`)

List available models and supported languages.

```bash
nbocr list [OPTIONS]
```

**Options:**
- `--detailed`: Show detailed information for each model

**Example:**

```bash
nbocr list
nbocr ls --detailed
```

### `info`

Show information about a specific model.

```bash
nbocr info <MODEL>
```

**Example:**

```bash
nbocr info latin
nbocr info korean
```

## 🌍 Supported Languages

### Recognition Models

| Model | Model File | Supported Languages | Total Languages |
|-------|------------|-------------------|----------------|
| **chinese** | PP-OCRv5_mobile_rec.mnn | Chinese (Simplified), Chinese (Traditional), English | 3 |
| **korean** | korean_PP-OCRv5_mobile_rec_infer.mnn | Korean, English | 2 |
| **latin** | latin_PP-OCRv5_mobile_rec_infer.mnn | French, German, Afrikaans, Italian, Spanish, Bosnian, Portuguese, Czech, Welsh, Danish, Estonian, Irish, Croatian, Uzbek, Hungarian, Serbian (Latin), Indonesian, Occitan, Icelandic, Lithuanian, Maori, Malay, Dutch, Norwegian, Polish, Slovak, Slovenian, Albanian, Swedish, Swahili, Tagalog, Turkish, Latin, Azerbaijani, Kurdish, Latvian, Maltese, Pali, Romanian, Vietnamese, Finnish, Basque, Galician, Luxembourgish, Romansh, Catalan, Quechua | 47 |
| **eslav** | eslav_PP-OCRv5_mobile_rec_infer.mnn | Russian, Belarusian, Ukrainian, English | 4 |
| **thai** | th_PP-OCRv5_mobile_rec_infer.mnn | Thai, English | 2 |
| **greek** | el_PP-OCRv5_mobile_rec_infer.mnn | Greek, English | 2 |
| **english** | en_PP-OCRv5_mobile_rec_infer.mnn | English | 1 |
| **cyrillic** | cyrillic_PP-OCRv5_mobile_rec_infer.mnn | Russian, Belarusian, Ukrainian, Serbian (Cyrillic), Bulgarian, Mongolian, Abkhazian, Adyghe, Kabardian, Avar, Dargin, Ingush, Chechen, Lak, Lezgin, Tabasaran, Kazakh, Kyrgyz, Tajik, Macedonian, Tatar, Chuvash, Bashkir, Malian, Moldovan, Udmurt, Komi, Ossetian, Buryat, Kalmyk, Tuvan, Sakha, Karakalpak, English | 34 |
| **arabic** | arabic_PP-OCRv5_mobile_rec_infer.mnn | Arabic, Persian, Uyghur, Urdu, Pashto, Kurdish, Sindhi, Balochi, English | 9 |
| **devanagari** | devanagari_PP-OCRv5_mobile_rec_infer.mnn | Hindi, Marathi, Nepali, Bihari, Maithili, Angika, Bhojpuri, Magahi, Santali, Newari, Konkani, Sanskrit, Haryanvi, English | 14 |
| **tamil** | ta_PP-OCRv5_mobile_rec_infer.mnn | Tamil, English | 2 |
| **telugu** | te_PP-OCRv5_mobile_rec_infer.mnn | Telugu, English | 2 |

### Detection Models

| Model | Model File | Description |
|-------|------------|-------------|
| **v5** | PP-OCRv5_mobile_det.mnn | PP-OCRv5 detection model (recommended) |
| **v5-fp16** | PP-OCRv5_mobile_det_fp16.mnn | PP-OCRv5 FP16 detection model (faster) |
| **v4** | ch_PP-OCRv4_det_infer.mnn | PP-OCRv4 detection model |

## 🎯 Precision Modes

The tool uses fast mode optimized for speed:

| Mode | Speed | Accuracy | Use Case |
|------|-------|----------|----------|
| **fast** | ⚡⚡⚡ | ⭐⭐⭐ | General purpose (default) |

## 🔧 Pipeline Architecture

The batch processing uses an efficient pipeline architecture:

```
┌─────────────┐    ┌──────────────┐    ┌──────────────┐
│  Task       │───▶│  Image       │───▶│  OCR         │
│  Dispatcher │    │  Loader Pool │    │  Inference   │
└─────────────┘    └──────────────┘    └──────────────┘
                         (4-8              (1 engine
                        threads)          multi-threaded)
```

**Benefits:**
- Parallel image loading while inference is running
- Automatic buffering to balance load
- Optimal CPU/GPU utilization
- Typically 2-3x faster than sequential processing

## 📊 Output Formats

### Text Format (Default)

```
[1] The dominant sequence transduction models are based on complex recurrent or (99%)
[2] convolutional neural networks that include an encoder and a decoder. The best (98%)
```

### JSON Format

```json
{
  "file": "image.png",
  "results": [
    {
      "text": "Sample text",
      "confidence": 0.99,
      "bbox": {
        "x": 10,
        "y": 20,
        "width": 300,
        "height": 50
      }
    }
  ]
}
```

### JSON Lines Format

```jsonl
{"file":"image1.png","results":[{"text":"Text 1","confidence":0.99,"bbox":{"x":10,"y":20,"width":300,"height":50}}]}
{"file":"image2.png","results":[{"text":"Text 2","confidence":0.98,"bbox":{"x":15,"y":25,"width":320,"height":55}}]}
```

## 🔥 Performance Tips

1. **Use GPU acceleration** on supported platforms:
   ```bash
   nbocr b ./images -m ./models --gpu metal  # macOS
   nbocr b ./images -m ./models --gpu opencl # Cross-platform
   ```

2. **Adjust thread count** based on your CPU:
   ```bash
   nbocr b ./images -m ./models -t 8  # 8 threads
   ```

3. **Tune loader threads** for batch processing:
   ```bash
   nbocr b ./images -m ./models --loader-threads 6
   ```

4. **Use fast mode** for quick preview:
   ```bash
   nbocr r image.png -m ./models --precision fast
   ```

5. **Use embedded models** for faster startup (no model loading):
   ```bash
   cargo build --release --features "embed-det-v5,embed-rec-chinese"
   ```

## 📝 Examples

### Example 1: Process Screenshots with Chinese Text

```bash
nbocr batch ~/Screenshots \
  --models-dir ~/models \
  --language chinese \
  --recursive \
  --progress \
  --stats \
  -f jsonl \
  -o results.jsonl
```

### Example 2: Extract English Text from PDFs (converted to images)

```bash
# Convert PDF to images first
convert -density 300 document.pdf page-%03d.png

# Process with high precision
nbocr batch ./pages \
  --models-dir ~/models \
  --language english \
  --precision high \
  --stats
```

### Example 3: Real-time OCR with Camera

```bash
# Capture image with camera and process
fswebcam capture.jpg
nbocr r capture.jpg -m ~/models --precision fast
```

### Example 4: Multilingual Document

```bash
# Process document with multiple languages
nbocr r document.png -m ~/models -l latin -v
```

## 🛠️ Development

### Project Structure

```
newbee_ocr_cli/
├── src/
│   ├── main.rs          # CLI entry point and command handlers
│   ├── models.rs        # Model definitions and embedded models
│   └── pipeline.rs      # Batch processing pipeline
├── Cargo.toml           # Dependencies and features
└── README.md           # This file
```

### Building with Custom Features

```bash
# Debug build
cargo build --features "embed-rec-chinese"

# Release build with all embedded models
cargo build --release --features "embed-det-v5,embed-rec-chinese,embed-rec-english,embed-rec-latin"

# Check code
cargo check

# Run tests
cargo test
```

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](../LICENSE) file for details.

## 🙏 Acknowledgments

- Built on top of [rust-paddle-ocr](https://github.com/zibo-chen/rust-paddle-ocr)
- Models from [PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR)
- Powered by [MNN](https://github.com/alibaba/MNN) inference engine

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📮 Contact

- GitHub: [zibo-chen/rust-paddle-ocr](https://github.com/zibo-chen/rust-paddle-ocr)
- Issues: [GitHub Issues](https://github.com/zibo-chen/rust-paddle-ocr/issues)

---

Made with ❤️ by the rust-paddle-ocr team

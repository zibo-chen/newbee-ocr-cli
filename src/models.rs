//! 模型模块
//!
//! 定义内嵌模型和语言支持

use std::fmt;
use std::path::Path;

const PP_OCRV6_SUPPORTED_LANGUAGES: &[&str] = &[
    "Chinese (Simplified)",
    "Chinese (Traditional)",
    "English",
    "Japanese",
    "Afrikaans",
    "Azerbaijani",
    "Bosnian",
    "Catalan",
    "Czech",
    "Welsh",
    "Danish",
    "German",
    "Spanish",
    "Estonian",
    "Basque",
    "Finnish",
    "French",
    "Irish",
    "Galician",
    "Croatian",
    "Hungarian",
    "Indonesian",
    "Icelandic",
    "Italian",
    "Kurdish",
    "Latin",
    "Luxembourgish",
    "Lithuanian",
    "Latvian",
    "Maori",
    "Malay",
    "Maltese",
    "Dutch",
    "Norwegian",
    "Occitan",
    "Polish",
    "Portuguese",
    "Quechua",
    "Romansh",
    "Romanian",
    "Serbian (Latin)",
    "Slovak",
    "Slovenian",
    "Albanian",
    "Swedish",
    "Swahili",
    "Tagalog",
    "Turkish",
    "Uzbek",
    "Vietnamese",
];

const PP_OCRV6_LANGUAGE_ALIASES: &[&str] = &[
    "ch",
    "zh",
    "cn",
    "chinese",
    "chinese_simplified",
    "simplified_chinese",
    "chinese_cht",
    "cht",
    "traditional_chinese",
    "chinese_traditional",
    "en",
    "english",
    "japan",
    "japanese",
    "ja",
    "jp",
    "af",
    "afrikaans",
    "az",
    "azerbaijani",
    "bs",
    "bosnian",
    "ca",
    "catalan",
    "cs",
    "czech",
    "cy",
    "welsh",
    "da",
    "danish",
    "de",
    "german",
    "es",
    "spanish",
    "et",
    "estonian",
    "eu",
    "basque",
    "fi",
    "finnish",
    "fr",
    "french",
    "ga",
    "irish",
    "gl",
    "galician",
    "hr",
    "croatian",
    "hu",
    "hungarian",
    "id",
    "indonesian",
    "is",
    "icelandic",
    "it",
    "italian",
    "ku",
    "kurdish",
    "la",
    "latin",
    "lb",
    "luxembourgish",
    "lt",
    "lithuanian",
    "lv",
    "latvian",
    "mi",
    "maori",
    "ms",
    "malay",
    "mt",
    "maltese",
    "nl",
    "dutch",
    "no",
    "norwegian",
    "oc",
    "occitan",
    "pl",
    "polish",
    "pt",
    "portuguese",
    "qu",
    "quechua",
    "rm",
    "romansh",
    "ro",
    "romanian",
    "rs_latin",
    "serbian_latin",
    "sr_latn",
    "sk",
    "slovak",
    "sl",
    "slovenian",
    "sq",
    "albanian",
    "sv",
    "swedish",
    "sw",
    "swahili",
    "tl",
    "tagalog",
    "tr",
    "turkish",
    "uz",
    "uzbek",
    "vi",
    "vietnamese",
];

fn normalize_model_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace([' ', '-'], "_")
        .replace("pp_ocr", "ppocr")
        .replace("ppocr_", "ppocr")
}

/// 支持的语言/模型类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecognitionModel {
    /// 中文 (PP-OCRv5, 简体中文、繁体中文、英文)
    Chinese,
    /// 韩语 (Korean, English)
    Korean,
    /// 拉丁语系 (French, German, Italian, Spanish, Portuguese, etc.)
    Latin,
    /// 东斯拉夫语 (Russian, Belarusian, Ukrainian, English)
    EastSlavic,
    /// 泰语 (Thai, English)
    Thai,
    /// 希腊语 (Greek, English)
    Greek,
    /// 英语 (English only)
    English,
    /// 西里尔字母 (Russian, Bulgarian, Mongolian, etc.)
    Cyrillic,
    /// 阿拉伯语系 (Arabic, Persian, Uyghur, Urdu, etc.)
    Arabic,
    /// 天城文 (Hindi, Marathi, Nepali, Sanskrit, etc.)
    Devanagari,
    /// 泰米尔语 (Tamil, English)
    Tamil,
    /// 泰卢固语 (Telugu, English)
    Telugu,
    /// PP-OCRv6 tiny 统一多语言识别模型
    V6Tiny,
    /// PP-OCRv6 small 统一多语言识别模型
    V6Small,
    /// PP-OCRv6 medium 统一多语言识别模型
    V6Medium,
}

impl RecognitionModel {
    /// 获取模型名称
    pub fn name(&self) -> &'static str {
        match self {
            RecognitionModel::Chinese => "chinese",
            RecognitionModel::Korean => "korean",
            RecognitionModel::Latin => "latin",
            RecognitionModel::EastSlavic => "eslav",
            RecognitionModel::Thai => "thai",
            RecognitionModel::Greek => "greek",
            RecognitionModel::English => "english",
            RecognitionModel::Cyrillic => "cyrillic",
            RecognitionModel::Arabic => "arabic",
            RecognitionModel::Devanagari => "devanagari",
            RecognitionModel::Tamil => "tamil",
            RecognitionModel::Telugu => "telugu",
            RecognitionModel::V6Tiny => "v6-tiny",
            RecognitionModel::V6Small => "v6-small",
            RecognitionModel::V6Medium => "v6-medium",
        }
    }

    /// 获取模型文件名
    pub fn model_filename(&self) -> &'static str {
        match self {
            RecognitionModel::Chinese => "PP-OCRv5_mobile_rec.mnn",
            RecognitionModel::Korean => "korean_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Latin => "latin_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::EastSlavic => "eslav_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Thai => "th_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Greek => "el_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::English => "en_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Cyrillic => "cyrillic_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Arabic => "arabic_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Devanagari => "devanagari_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Tamil => "ta_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::Telugu => "te_PP-OCRv5_mobile_rec_infer.mnn",
            RecognitionModel::V6Tiny => "PP-OCRv6_tiny_rec.mnn",
            RecognitionModel::V6Small => "PP-OCRv6_small_rec.mnn",
            RecognitionModel::V6Medium => "PP-OCRv6_medium_rec.mnn",
        }
    }

    /// 获取字符集文件名
    pub fn charset_filename(&self) -> &'static str {
        match self {
            RecognitionModel::Chinese => "ppocr_keys_v5.txt",
            RecognitionModel::Korean => "ppocr_keys_korean.txt",
            RecognitionModel::Latin => "ppocr_keys_latin.txt",
            RecognitionModel::EastSlavic => "ppocr_keys_eslav.txt",
            RecognitionModel::Thai => "ppocr_keys_th.txt",
            RecognitionModel::Greek => "ppocr_keys_el.txt",
            RecognitionModel::English => "ppocr_keys_en.txt",
            RecognitionModel::Cyrillic => "ppocr_keys_cyrillic.txt",
            RecognitionModel::Arabic => "ppocr_keys_arabic.txt",
            RecognitionModel::Devanagari => "ppocr_keys_devanagari.txt",
            RecognitionModel::Tamil => "ppocr_keys_ta.txt",
            RecognitionModel::Telugu => "ppocr_keys_te.txt",
            RecognitionModel::V6Tiny => "ppocr_keys_v6_tiny.txt",
            RecognitionModel::V6Small => "ppocr_keys_v6_small.txt",
            RecognitionModel::V6Medium => "ppocr_keys_v6_medium.txt",
        }
    }

    /// 获取支持的语言列表
    pub fn supported_languages(&self) -> &'static [&'static str] {
        match self {
            RecognitionModel::Chinese => &[
                "Chinese (Simplified)",
                "Chinese (Traditional)",
                "English",
                "Japanese",
            ],
            RecognitionModel::Korean => &["Korean", "English"],
            RecognitionModel::Latin => &[
                "French",
                "German",
                "Afrikaans",
                "Italian",
                "Spanish",
                "Bosnian",
                "Portuguese",
                "Czech",
                "Welsh",
                "Danish",
                "Estonian",
                "Irish",
                "Croatian",
                "Uzbek",
                "Hungarian",
                "Serbian (Latin)",
                "Indonesian",
                "Occitan",
                "Icelandic",
                "Lithuanian",
                "Maori",
                "Malay",
                "Dutch",
                "Norwegian",
                "Polish",
                "Slovak",
                "Slovenian",
                "Albanian",
                "Swedish",
                "Swahili",
                "Tagalog",
                "Turkish",
                "Latin",
                "Azerbaijani",
                "Kurdish",
                "Latvian",
                "Maltese",
                "Pali",
                "Romanian",
                "Vietnamese",
                "Finnish",
                "Basque",
                "Galician",
                "Luxembourgish",
                "Romansh",
                "Catalan",
                "Quechua",
            ],
            RecognitionModel::EastSlavic => &["Russian", "Belarusian", "Ukrainian", "English"],
            RecognitionModel::Thai => &["Thai", "English"],
            RecognitionModel::Greek => &["Greek", "English"],
            RecognitionModel::English => &["English"],
            RecognitionModel::Cyrillic => &[
                "Russian",
                "Belarusian",
                "Ukrainian",
                "Serbian (Cyrillic)",
                "Bulgarian",
                "Mongolian",
                "Abkhazian",
                "Adyghe",
                "Kabardian",
                "Avar",
                "Dargin",
                "Ingush",
                "Chechen",
                "Lak",
                "Lezgin",
                "Tabasaran",
                "Kazakh",
                "Kyrgyz",
                "Tajik",
                "Macedonian",
                "Tatar",
                "Chuvash",
                "Bashkir",
                "Malian",
                "Moldovan",
                "Udmurt",
                "Komi",
                "Ossetian",
                "Buryat",
                "Kalmyk",
                "Tuvan",
                "Sakha",
                "Karakalpak",
                "English",
            ],
            RecognitionModel::Arabic => &[
                "Arabic", "Persian", "Uyghur", "Urdu", "Pashto", "Kurdish", "Sindhi", "Balochi",
                "English",
            ],
            RecognitionModel::Devanagari => &[
                "Hindi", "Marathi", "Nepali", "Bihari", "Maithili", "Angika", "Bhojpuri", "Magahi",
                "Santali", "Newari", "Konkani", "Sanskrit", "Haryanvi", "English",
            ],
            RecognitionModel::Tamil => &["Tamil", "English"],
            RecognitionModel::Telugu => &["Telugu", "English"],
            RecognitionModel::V6Tiny | RecognitionModel::V6Small | RecognitionModel::V6Medium => {
                PP_OCRV6_SUPPORTED_LANGUAGES
            }
        }
    }

    /// 从字符串解析模型类型
    pub fn from_str(s: &str) -> Option<Self> {
        match normalize_model_key(s).as_str() {
            "v6" | "ppocrv6" | "v6_tiny" | "ppocrv6_tiny" => Some(RecognitionModel::V6Tiny),
            "v6_small" | "ppocrv6_small" => Some(RecognitionModel::V6Small),
            "v6_medium" | "ppocrv6_medium" => Some(RecognitionModel::V6Medium),
            "chinese" | "ch" | "cn" | "zh" | "ja" | "jp" | "japanese" => {
                Some(RecognitionModel::Chinese)
            }
            "korean" | "ko" | "kr" => Some(RecognitionModel::Korean),
            "latin" | "la" | "french" | "fr" | "german" | "de" | "afrikaans" | "af" | "italian"
            | "it" | "spanish" | "es" | "bosnian" | "bs" | "portuguese" | "pt" | "czech" | "cs"
            | "welsh" | "cy" | "danish" | "da" | "estonian" | "et" | "irish" | "ga"
            | "croatian" | "hr" | "uzbek" | "uz" | "hungarian" | "hu" | "serbian_latin"
            | "sr_latn" | "indonesian" | "id" | "occitan" | "oc" | "icelandic" | "is"
            | "lithuanian" | "lt" | "maori" | "mi" | "malay" | "ms" | "dutch" | "nl"
            | "norwegian" | "no" | "polish" | "pl" | "slovak" | "sk" | "slovenian" | "sl"
            | "albanian" | "sq" | "swedish" | "sv" | "swahili" | "sw" | "tagalog" | "tl"
            | "turkish" | "tr" | "azerbaijani" | "az" | "kurdish" | "ku" | "latvian" | "lv"
            | "maltese" | "mt" | "pali" | "pi" | "romanian" | "ro" | "vietnamese" | "vi"
            | "finnish" | "fi" | "basque" | "eu" | "galician" | "gl" | "luxembourgish" | "lb"
            | "romansh" | "rm" | "catalan" | "ca" | "quechua" | "qu" => {
                Some(RecognitionModel::Latin)
            }
            "eslav" | "eastslav" | "east_slavic" | "russian" | "ru" | "belarusian" | "be"
            | "ukrainian" | "uk" => Some(RecognitionModel::EastSlavic),
            "thai" | "th" => Some(RecognitionModel::Thai),
            "greek" | "el" => Some(RecognitionModel::Greek),
            "english" | "en" => Some(RecognitionModel::English),
            "cyrillic" | "serbian_cyrillic" | "sr_cyrl" | "bulgarian" | "bg" | "mongolian"
            | "mn" | "abkhazian" | "ab" | "adyghe" | "ady" | "kabardian" | "kbd" | "avar"
            | "av" | "dargin" | "dar" | "ingush" | "inh" | "chechen" | "ce" | "lak" | "lezgin"
            | "lez" | "tabasaran" | "tab" | "kazakh" | "kk" | "kyrgyz" | "ky" | "tajik" | "tg"
            | "macedonian" | "mk" | "tatar" | "tt" | "chuvash" | "cv" | "bashkir" | "ba"
            | "malian" | "mari" | "moldovan" | "mo" | "udmurt" | "udm" | "komi" | "kv"
            | "ossetian" | "os" | "buryat" | "bua" | "kalmyk" | "xal" | "tuvan" | "tyv"
            | "sakha" | "sah" | "karakalpak" | "kaa" => Some(RecognitionModel::Cyrillic),
            "arabic" | "ar" | "persian" | "fa" | "uyghur" | "ug" | "urdu" | "ur" | "pashto"
            | "ps" | "sindhi" | "sd" | "balochi" | "bal" => Some(RecognitionModel::Arabic),
            "devanagari" | "deva" | "hindi" | "hi" | "marathi" | "mr" | "nepali" | "ne"
            | "bihari" | "bh" | "maithili" | "mai" | "angika" | "anp" | "bhojpuri" | "bho"
            | "magahi" | "mag" | "santali" | "sat" | "newari" | "new" | "konkani" | "kok"
            | "sanskrit" | "sa" | "haryanvi" | "bgc" => Some(RecognitionModel::Devanagari),
            "tamil" | "ta" => Some(RecognitionModel::Tamil),
            "telugu" | "te" => Some(RecognitionModel::Telugu),
            _ => None,
        }
    }

    /// 获取所有可用模型
    pub fn all() -> &'static [RecognitionModel] {
        &[
            RecognitionModel::Chinese,
            RecognitionModel::Korean,
            RecognitionModel::Latin,
            RecognitionModel::EastSlavic,
            RecognitionModel::Thai,
            RecognitionModel::Greek,
            RecognitionModel::English,
            RecognitionModel::Cyrillic,
            RecognitionModel::Arabic,
            RecognitionModel::Devanagari,
            RecognitionModel::Tamil,
            RecognitionModel::Telugu,
            RecognitionModel::V6Tiny,
            RecognitionModel::V6Small,
            RecognitionModel::V6Medium,
        ]
    }
}

impl fmt::Display for RecognitionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// 检测模型类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DetectionModel {
    /// PP-OCRv5 检测模型
    #[default]
    V5,
    /// PP-OCRv5 FP16 检测模型 (更快)
    V5Fp16,
    /// PP-OCRv4 检测模型
    V4,
    /// PP-OCRv6 tiny 检测模型
    V6Tiny,
    /// PP-OCRv6 small 检测模型
    V6Small,
    /// PP-OCRv6 medium 检测模型
    V6Medium,
}

impl DetectionModel {
    /// 获取模型名称
    pub fn name(&self) -> &'static str {
        match self {
            DetectionModel::V5 => "v5",
            DetectionModel::V5Fp16 => "v5-fp16",
            DetectionModel::V4 => "v4",
            DetectionModel::V6Tiny => "v6-tiny",
            DetectionModel::V6Small => "v6-small",
            DetectionModel::V6Medium => "v6-medium",
        }
    }

    /// 获取模型文件名
    pub fn model_filename(&self) -> &'static str {
        match self {
            DetectionModel::V5 => "PP-OCRv5_mobile_det.mnn",
            DetectionModel::V5Fp16 => "PP-OCRv5_mobile_det_fp16.mnn",
            DetectionModel::V4 => "ch_PP-OCRv4_det_infer.mnn",
            DetectionModel::V6Tiny => "PP-OCRv6_tiny_det.mnn",
            DetectionModel::V6Small => "PP-OCRv6_small_det.mnn",
            DetectionModel::V6Medium => "PP-OCRv6_medium_det.mnn",
        }
    }

    /// 获取同档 v6 识别模型；v4/v5 保持由 language 参数决定。
    pub fn paired_recognition_model(&self) -> Option<RecognitionModel> {
        match self {
            DetectionModel::V6Tiny => Some(RecognitionModel::V6Tiny),
            DetectionModel::V6Small => Some(RecognitionModel::V6Small),
            DetectionModel::V6Medium => Some(RecognitionModel::V6Medium),
            DetectionModel::V5 | DetectionModel::V5Fp16 | DetectionModel::V4 => None,
        }
    }

    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match normalize_model_key(s).as_str() {
            "v5" | "ppocrv5" => Some(DetectionModel::V5),
            "v5_fp16" | "v5fp16" | "ppocrv5_fp16" => Some(DetectionModel::V5Fp16),
            "v4" | "ppocrv4" => Some(DetectionModel::V4),
            "v6" | "ppocrv6" | "v6_tiny" | "ppocrv6_tiny" => Some(DetectionModel::V6Tiny),
            "v6_small" | "ppocrv6_small" => Some(DetectionModel::V6Small),
            "v6_medium" | "ppocrv6_medium" => Some(DetectionModel::V6Medium),
            _ => None,
        }
    }

    /// 获取所有可用模型
    pub fn all() -> &'static [DetectionModel] {
        &[
            DetectionModel::V5,
            DetectionModel::V5Fp16,
            DetectionModel::V4,
            DetectionModel::V6Tiny,
            DetectionModel::V6Small,
            DetectionModel::V6Medium,
        ]
    }
}

impl fmt::Display for DetectionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn is_ppocrv6_language_alias(language: &str) -> bool {
    let key = normalize_model_key(language);
    PP_OCRV6_LANGUAGE_ALIASES.contains(&key.as_str())
}

fn is_ppocrv6_japanese_alias(language: &str) -> bool {
    matches!(
        normalize_model_key(language).as_str(),
        "japan" | "japanese" | "ja" | "jp"
    )
}

pub fn is_language_supported_by_detection_model(language: &str, det_model: DetectionModel) -> bool {
    match det_model {
        DetectionModel::V6Tiny => {
            is_ppocrv6_language_alias(language) && !is_ppocrv6_japanese_alias(language)
        }
        DetectionModel::V6Small | DetectionModel::V6Medium => is_ppocrv6_language_alias(language),
        DetectionModel::V5 | DetectionModel::V5Fp16 | DetectionModel::V4 => {
            RecognitionModel::from_str(language).is_some()
        }
    }
}

pub fn unsupported_language_message(language: &str, det_model: DetectionModel) -> String {
    match det_model {
        DetectionModel::V6Tiny => format!(
            "Language '{}' is not officially supported by {}. PP-OCRv6 tiny supports Chinese, English, and the official Latin-script language set, but not Japanese. Use v6-small/v6-medium for Japanese or v5 script-specific models for non-v6 scripts.",
            language, det_model
        ),
        DetectionModel::V6Small | DetectionModel::V6Medium => format!(
            "Language '{}' is not officially supported by {}. PP-OCRv6 small/medium support Chinese, Traditional Chinese, English, Japanese, and the official 46 Latin-script languages. Use v5 script-specific models for Korean, Cyrillic, Arabic, Devanagari, Thai, Greek, Tamil, or Telugu.",
            language, det_model
        ),
        DetectionModel::V5 | DetectionModel::V5Fp16 | DetectionModel::V4 => {
            format!("Unknown language/model: {}", language)
        }
    }
}

/// 内嵌模型数据
pub struct EmbeddedModels;

impl EmbeddedModels {
    /// 获取内嵌的检测模型字节 (如果有)
    #[allow(unused_variables)]
    pub fn get_det_model(model: DetectionModel) -> Option<&'static [u8]> {
        match model {
            #[cfg(feature = "embed-det-v5")]
            DetectionModel::V5 => Some(include_bytes!("../models/PP-OCRv5_mobile_det.mnn")),

            #[cfg(feature = "embed-det-v5-fp16")]
            DetectionModel::V5Fp16 => {
                Some(include_bytes!("../models/PP-OCRv5_mobile_det_fp16.mnn"))
            }

            #[cfg(feature = "embed-det-v4")]
            DetectionModel::V4 => Some(include_bytes!("../models/ch_PP-OCRv4_det_infer.mnn")),

            #[cfg(feature = "embed-det-v6-tiny")]
            DetectionModel::V6Tiny => Some(include_bytes!("../models/PP-OCRv6_tiny_det.mnn")),

            #[cfg(feature = "embed-det-v6-small")]
            DetectionModel::V6Small => Some(include_bytes!("../models/PP-OCRv6_small_det.mnn")),

            #[cfg(feature = "embed-det-v6-medium")]
            DetectionModel::V6Medium => Some(include_bytes!("../models/PP-OCRv6_medium_det.mnn")),

            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// 获取内嵌的识别模型字节 (如果有)
    #[allow(unused_variables)]
    pub fn get_rec_model(model: RecognitionModel) -> Option<&'static [u8]> {
        match model {
            #[cfg(feature = "embed-rec-chinese")]
            RecognitionModel::Chinese => Some(include_bytes!("../models/PP-OCRv5_mobile_rec.mnn")),

            #[cfg(feature = "embed-rec-korean")]
            RecognitionModel::Korean => Some(include_bytes!(
                "../models/korean_PP-OCRv5_mobile_rec_infer.mnn"
            )),

            #[cfg(feature = "embed-rec-latin")]
            RecognitionModel::Latin => Some(include_bytes!(
                "../models/latin_PP-OCRv5_mobile_rec_infer.mnn"
            )),

            #[cfg(feature = "embed-rec-eslav")]
            RecognitionModel::EastSlavic => Some(include_bytes!(
                "../models/eslav_PP-OCRv5_mobile_rec_infer.mnn"
            )),

            #[cfg(feature = "embed-rec-thai")]
            RecognitionModel::Thai => {
                Some(include_bytes!("../models/th_PP-OCRv5_mobile_rec_infer.mnn"))
            }

            #[cfg(feature = "embed-rec-greek")]
            RecognitionModel::Greek => {
                Some(include_bytes!("../models/el_PP-OCRv5_mobile_rec_infer.mnn"))
            }

            #[cfg(feature = "embed-rec-english")]
            RecognitionModel::English => {
                Some(include_bytes!("../models/en_PP-OCRv5_mobile_rec_infer.mnn"))
            }

            #[cfg(feature = "embed-rec-cyrillic")]
            RecognitionModel::Cyrillic => Some(include_bytes!(
                "../models/cyrillic_PP-OCRv5_mobile_rec_infer.mnn"
            )),

            #[cfg(feature = "embed-rec-arabic")]
            RecognitionModel::Arabic => Some(include_bytes!(
                "../models/arabic_PP-OCRv5_mobile_rec_infer.mnn"
            )),

            #[cfg(feature = "embed-rec-devanagari")]
            RecognitionModel::Devanagari => Some(include_bytes!(
                "../models/devanagari_PP-OCRv5_mobile_rec_infer.mnn"
            )),

            #[cfg(feature = "embed-rec-tamil")]
            RecognitionModel::Tamil => {
                Some(include_bytes!("../models/ta_PP-OCRv5_mobile_rec_infer.mnn"))
            }

            #[cfg(feature = "embed-rec-telugu")]
            RecognitionModel::Telugu => {
                Some(include_bytes!("../models/te_PP-OCRv5_mobile_rec_infer.mnn"))
            }

            #[cfg(feature = "embed-rec-v6-tiny")]
            RecognitionModel::V6Tiny => Some(include_bytes!("../models/PP-OCRv6_tiny_rec.mnn")),

            #[cfg(feature = "embed-rec-v6-small")]
            RecognitionModel::V6Small => Some(include_bytes!("../models/PP-OCRv6_small_rec.mnn")),

            #[cfg(feature = "embed-rec-v6-medium")]
            RecognitionModel::V6Medium => Some(include_bytes!("../models/PP-OCRv6_medium_rec.mnn")),

            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// 获取内嵌的字符集字节 (如果有)
    #[allow(unused_variables)]
    pub fn get_charset(model: RecognitionModel) -> Option<&'static [u8]> {
        match model {
            #[cfg(feature = "embed-rec-chinese")]
            RecognitionModel::Chinese => Some(include_bytes!("../models/ppocr_keys_v5.txt")),

            #[cfg(feature = "embed-rec-korean")]
            RecognitionModel::Korean => Some(include_bytes!("../models/ppocr_keys_korean.txt")),

            #[cfg(feature = "embed-rec-latin")]
            RecognitionModel::Latin => Some(include_bytes!("../models/ppocr_keys_latin.txt")),

            #[cfg(feature = "embed-rec-eslav")]
            RecognitionModel::EastSlavic => Some(include_bytes!("../models/ppocr_keys_eslav.txt")),

            #[cfg(feature = "embed-rec-thai")]
            RecognitionModel::Thai => Some(include_bytes!("../models/ppocr_keys_th.txt")),

            #[cfg(feature = "embed-rec-greek")]
            RecognitionModel::Greek => Some(include_bytes!("../models/ppocr_keys_el.txt")),

            #[cfg(feature = "embed-rec-english")]
            RecognitionModel::English => Some(include_bytes!("../models/ppocr_keys_en.txt")),

            #[cfg(feature = "embed-rec-cyrillic")]
            RecognitionModel::Cyrillic => Some(include_bytes!("../models/ppocr_keys_cyrillic.txt")),

            #[cfg(feature = "embed-rec-arabic")]
            RecognitionModel::Arabic => Some(include_bytes!("../models/ppocr_keys_arabic.txt")),

            #[cfg(feature = "embed-rec-devanagari")]
            RecognitionModel::Devanagari => {
                Some(include_bytes!("../models/ppocr_keys_devanagari.txt"))
            }

            #[cfg(feature = "embed-rec-tamil")]
            RecognitionModel::Tamil => Some(include_bytes!("../models/ppocr_keys_ta.txt")),

            #[cfg(feature = "embed-rec-telugu")]
            RecognitionModel::Telugu => Some(include_bytes!("../models/ppocr_keys_te.txt")),

            #[cfg(feature = "embed-rec-v6-tiny")]
            RecognitionModel::V6Tiny => Some(include_bytes!("../models/ppocr_keys_v6_tiny.txt")),

            #[cfg(feature = "embed-rec-v6-small")]
            RecognitionModel::V6Small => Some(include_bytes!("../models/ppocr_keys_v6_small.txt")),

            #[cfg(feature = "embed-rec-v6-medium")]
            RecognitionModel::V6Medium => {
                Some(include_bytes!("../models/ppocr_keys_v6_medium.txt"))
            }

            #[allow(unreachable_patterns)]
            _ => None,
        }
    }

    /// 检查是否有内嵌的检测模型
    #[allow(dead_code)]
    pub fn has_embedded_det() -> bool {
        cfg!(feature = "embed-det-v5")
            || cfg!(feature = "embed-det-v5-fp16")
            || cfg!(feature = "embed-det-v4")
            || cfg!(feature = "embed-det-v6-tiny")
            || cfg!(feature = "embed-det-v6-small")
            || cfg!(feature = "embed-det-v6-medium")
    }

    /// 检查是否有内嵌的识别模型
    #[allow(dead_code)]
    pub fn has_embedded_rec() -> bool {
        cfg!(feature = "embed-rec-chinese")
            || cfg!(feature = "embed-rec-korean")
            || cfg!(feature = "embed-rec-latin")
            || cfg!(feature = "embed-rec-eslav")
            || cfg!(feature = "embed-rec-thai")
            || cfg!(feature = "embed-rec-greek")
            || cfg!(feature = "embed-rec-english")
            || cfg!(feature = "embed-rec-cyrillic")
            || cfg!(feature = "embed-rec-arabic")
            || cfg!(feature = "embed-rec-devanagari")
            || cfg!(feature = "embed-rec-tamil")
            || cfg!(feature = "embed-rec-telugu")
            || cfg!(feature = "embed-rec-v6-tiny")
            || cfg!(feature = "embed-rec-v6-small")
            || cfg!(feature = "embed-rec-v6-medium")
    }

    /// 获取所有内嵌的识别模型
    pub fn embedded_rec_models() -> Vec<RecognitionModel> {
        let mut models = Vec::new();

        #[cfg(feature = "embed-rec-chinese")]
        models.push(RecognitionModel::Chinese);

        #[cfg(feature = "embed-rec-korean")]
        models.push(RecognitionModel::Korean);

        #[cfg(feature = "embed-rec-latin")]
        models.push(RecognitionModel::Latin);

        #[cfg(feature = "embed-rec-eslav")]
        models.push(RecognitionModel::EastSlavic);

        #[cfg(feature = "embed-rec-thai")]
        models.push(RecognitionModel::Thai);

        #[cfg(feature = "embed-rec-greek")]
        models.push(RecognitionModel::Greek);

        #[cfg(feature = "embed-rec-english")]
        models.push(RecognitionModel::English);

        #[cfg(feature = "embed-rec-cyrillic")]
        models.push(RecognitionModel::Cyrillic);

        #[cfg(feature = "embed-rec-arabic")]
        models.push(RecognitionModel::Arabic);

        #[cfg(feature = "embed-rec-devanagari")]
        models.push(RecognitionModel::Devanagari);

        #[cfg(feature = "embed-rec-tamil")]
        models.push(RecognitionModel::Tamil);

        #[cfg(feature = "embed-rec-telugu")]
        models.push(RecognitionModel::Telugu);

        #[cfg(feature = "embed-rec-v6-tiny")]
        models.push(RecognitionModel::V6Tiny);

        #[cfg(feature = "embed-rec-v6-small")]
        models.push(RecognitionModel::V6Small);

        #[cfg(feature = "embed-rec-v6-medium")]
        models.push(RecognitionModel::V6Medium);

        models
    }

    /// 获取所有内嵌的检测模型
    pub fn embedded_det_models() -> Vec<DetectionModel> {
        let mut models = Vec::new();

        #[cfg(feature = "embed-det-v5")]
        models.push(DetectionModel::V5);

        #[cfg(feature = "embed-det-v5-fp16")]
        models.push(DetectionModel::V5Fp16);

        #[cfg(feature = "embed-det-v4")]
        models.push(DetectionModel::V4);

        #[cfg(feature = "embed-det-v6-tiny")]
        models.push(DetectionModel::V6Tiny);

        #[cfg(feature = "embed-det-v6-small")]
        models.push(DetectionModel::V6Small);

        #[cfg(feature = "embed-det-v6-medium")]
        models.push(DetectionModel::V6Medium);

        models
    }
}

/// 模型路径解析器
pub struct ModelResolver {
    models_dir: Option<std::path::PathBuf>,
}

impl ModelResolver {
    /// 创建新的解析器
    pub fn new(models_dir: Option<&Path>) -> Self {
        Self {
            models_dir: models_dir.map(|p| p.to_path_buf()),
        }
    }

    /// 解析检测模型路径
    pub fn resolve_det_model(&self, model: DetectionModel) -> Option<std::path::PathBuf> {
        if let Some(ref dir) = self.models_dir {
            let path = dir.join(model.model_filename());
            if path.exists() {
                return Some(path);
            }
        }

        // 尝试当前目录下的 models 文件夹
        let local_path = std::path::PathBuf::from("models").join(model.model_filename());
        if local_path.exists() {
            return Some(local_path);
        }

        None
    }

    /// 解析识别模型路径
    pub fn resolve_rec_model(&self, model: RecognitionModel) -> Option<std::path::PathBuf> {
        if let Some(ref dir) = self.models_dir {
            let path = dir.join(model.model_filename());
            if path.exists() {
                return Some(path);
            }
        }

        let local_path = std::path::PathBuf::from("models").join(model.model_filename());
        if local_path.exists() {
            return Some(local_path);
        }

        None
    }

    /// 解析字符集路径
    pub fn resolve_charset(&self, model: RecognitionModel) -> Option<std::path::PathBuf> {
        if let Some(ref dir) = self.models_dir {
            let path = dir.join(model.charset_filename());
            if path.exists() {
                return Some(path);
            }
        }

        let local_path = std::path::PathBuf::from("models").join(model.charset_filename());
        if local_path.exists() {
            return Some(local_path);
        }

        None
    }
}

/// 打印模型信息表格
pub fn print_models_table() {
    use colored::*;

    println!();
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════════════════"
            .bright_blue()
    );
    println!(
        "{}",
        "                        Available Recognition Models"
            .bright_white()
            .bold()
    );
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════════════════"
            .bright_blue()
    );
    println!();

    for model in RecognitionModel::all() {
        let embedded = if EmbeddedModels::get_rec_model(*model).is_some() {
            "[embedded]".green().to_string()
        } else {
            "[external]".yellow().to_string()
        };

        println!(
            "{} {} {}",
            format!("{:<12}", model.name()).bright_cyan().bold(),
            embedded,
            format!("- {}", model.model_filename()).dimmed()
        );

        let languages = model.supported_languages();
        let lang_str = if languages.len() > 5 {
            format!(
                "{}, ... ({} languages total)",
                languages[..5].join(", "),
                languages.len()
            )
        } else {
            languages.join(", ")
        };
        println!("             {}", lang_str.dimmed());
        println!();
    }

    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════════════════"
            .bright_blue()
    );
    println!(
        "{}",
        "                        Available Detection Models"
            .bright_white()
            .bold()
    );
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════════════════"
            .bright_blue()
    );
    println!();

    for model in DetectionModel::all() {
        let embedded = if EmbeddedModels::get_det_model(*model).is_some() {
            "[embedded]".green().to_string()
        } else {
            "[external]".yellow().to_string()
        };

        let desc = match model {
            DetectionModel::V5 => "PP-OCRv5 detection model",
            DetectionModel::V5Fp16 => "PP-OCRv5 FP16 detection model (faster)",
            DetectionModel::V4 => "PP-OCRv4 detection model",
            DetectionModel::V6Tiny => "PP-OCRv6 tiny detection model (pairs with v6 tiny rec)",
            DetectionModel::V6Small => "PP-OCRv6 small detection model (balanced)",
            DetectionModel::V6Medium => "PP-OCRv6 medium detection model (accuracy first)",
        };

        println!(
            "{} {} {} - {}",
            format!("{:<10}", model.name()).bright_cyan().bold(),
            embedded,
            format!("- {}", model.model_filename()).dimmed(),
            desc.dimmed()
        );
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ppocrv6_detection_tiers() {
        assert_eq!(DetectionModel::from_str("v6"), Some(DetectionModel::V6Tiny));
        assert_eq!(
            DetectionModel::from_str("v6-small"),
            Some(DetectionModel::V6Small)
        );
        assert_eq!(
            DetectionModel::from_str("ppocr-v6-medium"),
            Some(DetectionModel::V6Medium)
        );
    }

    #[test]
    fn ppocrv6_detection_tiers_select_matching_recognition_models() {
        assert_eq!(
            DetectionModel::V6Tiny.paired_recognition_model(),
            Some(RecognitionModel::V6Tiny)
        );
        assert_eq!(
            DetectionModel::V6Small.paired_recognition_model(),
            Some(RecognitionModel::V6Small)
        );
        assert_eq!(
            DetectionModel::V6Medium.paired_recognition_model(),
            Some(RecognitionModel::V6Medium)
        );
        assert_eq!(DetectionModel::V5.paired_recognition_model(), None);
    }

    #[test]
    fn parses_language_aliases_for_existing_official_groups() {
        assert_eq!(
            RecognitionModel::from_str("fr"),
            Some(RecognitionModel::Latin)
        );
        assert_eq!(
            RecognitionModel::from_str("japanese"),
            Some(RecognitionModel::Chinese)
        );
        assert_eq!(
            RecognitionModel::from_str("sr-cyrl"),
            Some(RecognitionModel::Cyrillic)
        );
        assert_eq!(
            RecognitionModel::from_str("hindi"),
            Some(RecognitionModel::Devanagari)
        );
    }

    #[test]
    fn ppocrv6_recognition_models_use_unified_charset() {
        assert_eq!(
            RecognitionModel::V6Small.model_filename(),
            "PP-OCRv6_small_rec.mnn"
        );
        assert_eq!(
            RecognitionModel::V6Small.charset_filename(),
            "ppocr_keys_v6_small.txt"
        );
        assert_eq!(
            RecognitionModel::V6Medium.model_filename(),
            "PP-OCRv6_medium_rec.mnn"
        );
        assert!(RecognitionModel::V6Medium
            .supported_languages()
            .contains(&"Japanese"));
    }

    #[test]
    fn ppocrv6_language_validation_uses_official_language_set() {
        assert!(is_language_supported_by_detection_model(
            "fr",
            DetectionModel::V6Small
        ));
        assert!(is_language_supported_by_detection_model(
            "japanese",
            DetectionModel::V6Medium
        ));
        assert!(!is_language_supported_by_detection_model(
            "japanese",
            DetectionModel::V6Tiny
        ));
        assert!(!is_language_supported_by_detection_model(
            "ar",
            DetectionModel::V6Small
        ));
        assert!(!is_language_supported_by_detection_model(
            "hindi",
            DetectionModel::V6Medium
        ));
    }
}

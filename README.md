# Newbee OCR CLI

A command-line OCR tool built on [rust-paddle-ocr](https://github.com/zibo-chen/rust-paddle-ocr) / `ocr-rs` 2.3.1. The default binary embeds PP-OCRv6 tiny, so basic OCR works without a separate model download. PP-OCRv6 small, medium, and legacy/script-specific models are downloaded from GitHub on demand when local files are missing.

## Install

### Homebrew (macOS and Linux)

```bash
brew install zibo-chen/tap/nbocr
```

### WinGet (Windows)

```powershell
winget install --id ChenZibo.NewbeeOCRCLI --exact
```

### Debian and Ubuntu

```bash
sudo install -d -m 0755 /etc/apt/keyrings
sudo curl -fsSL https://zibo-chen.github.io/nbocr-packages/keys/nbocr.asc \
  -o /etc/apt/keyrings/nbocr.asc
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/nbocr.asc] https://zibo-chen.github.io/nbocr-packages/apt stable main" \
  | sudo tee /etc/apt/sources.list.d/nbocr.list >/dev/null
sudo apt update
sudo apt install nbocr
```

### Fedora, RHEL, Rocky Linux, and compatible distributions

```bash
sudo curl -fsSL https://zibo-chen.github.io/nbocr-packages/nbocr.repo \
  -o /etc/yum.repos.d/nbocr.repo
sudo dnf install nbocr
```

### Installer scripts

macOS and Linux:

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/zibo-chen/newbee-ocr-cli/releases/latest/download/newbee_ocr_cli-installer.sh | sh
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/zibo-chen/newbee-ocr-cli/releases/latest/download/newbee_ocr_cli-installer.ps1 | iex"
```

## Build

```bash
cd newbee_ocr_cli
cargo build --release
```

The binary is written to `target/release/nbocr`.

## Quick Use

Default embedded PP-OCRv6 tiny:

```bash
nbocr r image.png
```

Use another v6 tier. Missing files are downloaded into `--models-dir` or `./models`:

```bash
nbocr r document.jpg -d v6-small -l ja
nbocr r document.jpg -d v6-medium -l fr -f json
```

Batch:

```bash
nbocr b ./images --recursive -f jsonl -o results.jsonl
```

## PP-OCRv6 Support

Use `-d v6-tiny`, `-d v6-small`, or `-d v6-medium`. The CLI automatically pairs detection with the matching v6 recognition model.

PP-OCRv6 is not one model per language. Each v6 tier uses one multilingual recognition model plus its tier-specific charset:

| Tier | Detection | Recognition | Charset | Language scope |
|---|---|---|---|---|
| `v6-tiny` | `PP-OCRv6_tiny_det.mnn` | `PP-OCRv6_tiny_rec.mnn` | `ppocr_keys_v6_tiny.txt` | Chinese, English, and the official Latin-script set; Japanese excluded |
| `v6-small` | `PP-OCRv6_small_det.mnn` | `PP-OCRv6_small_rec.mnn` | `ppocr_keys_v6_small.txt` | Official 50 v6 recognition languages |
| `v6-medium` | `PP-OCRv6_medium_det.mnn` | `PP-OCRv6_medium_rec.mnn` | `ppocr_keys_v6_medium.txt` | Official 50 v6 recognition languages |

`-l, --language` validates the requested language and keeps output/config explicit; for v6 it does not select a separate language-specific recognition file.

Korean, Cyrillic, Arabic, Devanagari, Thai, Greek, Tamil, and Telugu should use PP-OCRv5 script-specific recognition models.

## Model Files

The default v6 tiny files are embedded by default. Other missing files are downloaded from:

```text
https://raw.githubusercontent.com/zibo-chen/rust-paddle-ocr/v2.3.1/models/
```

Download target:
- `--models-dir <DIR>` when provided.
- `./models` otherwise.

Already-downloaded files are reused.

To install converted Paddle models manually:

```bash
python ../rust-paddle-ocr/script/convert_paddle_to_mnn.py \
  --ocr-dir /path/to/paddle/inference/models \
  --install-dir ./models
```

## Commands

Inspect models:

```bash
nbocr list
nbocr list --detailed
nbocr info v6-small
```

Important options:

| Option | Description |
|---|---|
| `-m, --models-dir <DIR>` | Model directory and auto-download target |
| `-d, --det-model <MODEL>` | `v4`, `v5`, `v5-fp16`, `v6-tiny`, `v6-small`, `v6-medium` |
| `-l, --language <LANG>` | Language/model alias, for example `zh`, `en`, `ja`, `fr`, `arabic` |
| `-f, --format <FMT>` | `text`, `json`, or `jsonl` |
| `--gpu <BACKEND>` | `metal`, `opencl`, `vulkan`, or `cuda` |
| `--timing` | Print timing information |

## Embedded Models

The default feature set embeds PP-OCRv6 tiny. Optional features can embed additional models into the binary.

```bash
cargo build --release --features "embed-det-v6-small,embed-rec-v6-small"
```

Available v6 embed features:

```text
embed-det-v6-tiny
embed-det-v6-small
embed-det-v6-medium
embed-rec-v6-tiny
embed-rec-v6-small
embed-rec-v6-medium
```

## License

Apache-2.0

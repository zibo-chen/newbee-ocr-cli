# Newbee OCR CLI

A command-line OCR tool built on [rust-paddle-ocr](../rust-paddle-ocr). It supports single-image OCR, batch OCR, JSON/JSONL output, GPU backends, and optional embedded models.

## Build

```bash
cd newbee_ocr_cli
cargo build --release
```

The binary is written to `target/release/nbocr`.

## Model Files

Put all runtime MNN models under `models/`.

PP-OCRv6 files:

```text
models/PP-OCRv6_tiny_det.mnn
models/PP-OCRv6_tiny_rec.mnn
models/ppocr_keys_v6_tiny.txt

models/PP-OCRv6_small_det.mnn
models/PP-OCRv6_small_rec.mnn
models/ppocr_keys_v6_small.txt

models/PP-OCRv6_medium_det.mnn
models/PP-OCRv6_medium_rec.mnn
models/ppocr_keys_v6_medium.txt
```

The converter defaults to FP16 and can install converted files directly:

```bash
python ../rust-paddle-ocr/script/convert_paddle_to_mnn.py \
  --ocr-dir /path/to/paddle/inference/models \
  --install-dir ./models
```

## PP-OCRv6 Support

Use `-d v6-tiny`, `-d v6-small`, or `-d v6-medium`. The CLI automatically pairs the matching v6 detection and recognition model.

```bash
nbocr r document.jpg -m ./models -d v6-small -l ja
nbocr r document.jpg -m ./models -d v6-medium -l fr -f json
```

Language boundary:
- `v6-small` and `v6-medium`: official 50 v6 recognition languages, including Simplified Chinese, Traditional Chinese, English, Japanese, and 46 Latin-script languages.
- `v6-tiny`: same v6 set except Japanese.
- Korean, Cyrillic, Arabic, Devanagari, Thai, Greek, Tamil, and Telugu should use PP-OCRv5 script-specific models.

## Common Commands

Single image:

```bash
nbocr recognize image.png -m ./models
nbocr r image.png -m ./models -l english --gpu metal
nbocr r image.png -m ./models -d v6-small -l japanese --timing
```

Batch:

```bash
nbocr batch ./images -m ./models --stats --progress
nbocr b ./images -m ./models --recursive -f jsonl -o results.jsonl
```

Inspect models:

```bash
nbocr list
nbocr list --detailed
nbocr info v6-small
```

Important options:

| Option | Description |
|---|---|
| `-m, --models-dir <DIR>` | Model directory |
| `-d, --det-model <MODEL>` | `v4`, `v5`, `v5-fp16`, `v6-tiny`, `v6-small`, `v6-medium` |
| `-l, --language <LANG>` | Language/model alias, for example `zh`, `en`, `ja`, `fr`, `arabic` |
| `-f, --format <FMT>` | `text`, `json`, or `jsonl` |
| `--gpu <BACKEND>` | `metal`, `opencl`, `vulkan`, or `cuda` |
| `--timing` | Print timing information |

## Embedded Models

Optional features can embed selected models into the binary.

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

# srttl

## Overview

`srttl` is a CLI tool written in rust that allows you to translate
SRT files from one language to another using DeepL's API

You must have provide your own DeepL API Key

To build it simply run `cargo build`

NOTE: DeepL is not perfect. It sometimes randomly does things like
duplicating lines. I suggest manually cleaining the SRT file after
using this tool (and maybe before too).

## Usage

```
Translates SRT files using DeepL

Usage: srttl --input-lang <INPUT_LANG> --output-lang <OUTPUT_LANG> --input-file <INPUT_FILE> --output-file <OUTPUT_FILE> --api-key <API_KEY>

Options:
      --input-lang <INPUT_LANG>    [possible values: bg, cs, da, de, el, en, es, et, fi, fr, hu, id, it, ja, lt, lv, nl, pl, pt, ro, ru, sk, sl, sv, tr, uk, zh]
      --output-lang <OUTPUT_LANG>  [possible values: bg, cs, da, de, el, en-gb, en-us, es, et, fi, fr, hu, id, it, ja, lt, lv, nl, pl, pt-br, pt-pt, ro, ru, sk, sl, sv, tr, uk, zh]
  -f, --input-file <INPUT_FILE>
  -F, --output-file <OUTPUT_FILE>
  -k, --api-key <API_KEY>
  -h, --help                       Print help
  -V, --version                    Print version
```

## Examples

Basic usage is as follows:

`srttl --input-lang ja --output-lang en-us --input-file jpsubs.srt --output-file ensubs.srt --api-key $MYAPIKEY`

Or through cargo:

`cargo run -- --input-lang ja --output-lang en-us --input-file jpsubs.srt --output-file ensubs.srt --api-key $MYAPIKEY`

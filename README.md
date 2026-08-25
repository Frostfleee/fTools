# fTools

A lightweight desktop utility for Windows, built with [Tauri](https://tauri.app) and Rust. fTools bundles a handful of small, focused tools into a single compact window: a Unicode text styler, a color format converter with a visual picker, and an image/audio file converter.

## Features

### Font Styler
Type text and instantly convert it into Unicode based styled variants: italic, bold, fraktur, double struck, small caps, monospace, sans serif, circled, squared, upside down, superscript, script, and more. One click copies the result to the clipboard.

### Color Format Converter
Convert a color between HEX, HEX8 (with shorthand variants), RGB, RGBA (with percentage variants), HSL, HSLA, HSV, CMYK, CSS named colors, platform specific formats (Swift's `UIColor`, Android's `Color.parseColor`, Flutter's `Color(0x...)`), and OKLCH/OKLab/LCH. A built in visual picker (hue/saturation box plus darkness and transparency sliders) lets you pick a color by eye instead of typing values.

### Media File Converter
Drag and drop a file, or browse for one, and convert it to another format.

**Images**: JPG, JPEG, PNG, WEBP, GIF, BMP, TIFF, AVIF, HEIC, ICO

**Audio**
- Convert to: MP3, WAV, FLAC, AAC, OGG, M4A, OPUS, AIFF
- AAC and M4A sources also decode natively as input, alongside all of the formats above

**Video**: MP4 and MOV only, and only as a lossless container remux, not a re-encode (see notes below). MKV, WEBM, AVI, WMV, FLV, animated GIF, and MPEG are listed in the UI but currently disabled and marked "soon".

Conversion options:
- Keep metadata
- Preserve the original file's creation and modification dates
- Overwrite the original file instead of writing a new one

### Settings
- App theme: Light, Dark, or Transparent
- High contrast mode
- Always on top
- Remember last used tool
- Close on focus loss
- Hide warning notifications

## Format support notes

- **WMA input is not possible.** There is no open source WMA decoder available anywhere in the Rust ecosystem, so this is a hard limitation rather than a bug.
- **WMA output is not available yet.** The encoder successfully writes compressed audio, but Windows' own ASF muxer currently fails while finalizing the file. A proper fix means querying the WMA encoder for its actual supported profiles instead of assuming one, which is planned for a future release rather than shipped half working.
- **Video conversion only covers MP4 and MOV, and only remuxes the container.** Streams are copied through Media Foundation as is (`MF_READWRITE_DISABLE_CONVERTERS` plus pinning each stream's current type to its native type), so no video or audio codec is ever decoded or re-encoded. This works because MP4 and MOV share the same underlying ISO Base Media File Format lineage. Since Windows ships no native `.mov` sink, MOV output is written through the MP4 sink to a temp file and renamed afterward. This path has been reasoned through against the actual Media Foundation bindings but, unlike the audio paths, has not yet been verified on a real Windows machine, so treat it as experimental for now. All other video formats are disabled in the UI until real transcoding is implemented.
- AAC and raw M4A both use AAC audio; AAC is written as a bare `.aac` (ADTS) stream, while M4A wraps the same audio in an MP4 container.

## Built with

- [Tauri 2](https://tauri.app) for the desktop application shell, with the `dialog` plugin for file picking and the `single-instance` plugin to focus the existing window instead of opening a second one
- Rust for the backend, vanilla HTML/CSS/JS for the frontend (no framework, no build step)
- [image](https://crates.io/crates/image), [img-parts](https://crates.io/crates/img-parts), [libheif-rs](https://crates.io/crates/libheif-rs), [ico](https://crates.io/crates/ico) for image encoding and decoding
- [symphonia](https://crates.io/crates/symphonia) for audio decoding
- [hound](https://crates.io/crates/hound), [flacenc](https://crates.io/crates/flacenc), [mp3lame-encoder](https://crates.io/crates/mp3lame-encoder), [vorbis_rs](https://crates.io/crates/vorbis_rs), [opus-rs](https://crates.io/crates/opus-rs), [ogg](https://crates.io/crates/ogg), and [rubato](https://crates.io/crates/rubato) for audio encoding and resampling
- [windows-rs](https://crates.io/crates/windows) (Media Foundation bindings) for AAC, M4A, and WMA encoding, and for the MP4/MOV remux path

## Installation

Download the latest installer from the [Releases](../../releases) page and run it. fTools is Windows only (x64) since it relies on Windows Media Foundation for some audio formats.

## Building from source

Prerequisites:
- [Rust](https://rustup.rs), a recent stable toolchain (some dependencies require 1.85 or newer, for edition 2024 support)
- [Tauri CLI](https://tauri.app/start/): `cargo install tauri-cli`
- The usual Tauri prerequisites on Windows: Microsoft C++ Build Tools (or Visual Studio) and the WebView2 runtime

```
git clone <this-repo>
cd fTools
cargo tauri build
```

The installer is produced at `src-tauri/target/release/bundle/nsis/`.

For local development without producing an installer:

```
cargo tauri dev
```

## Project structure

```
main/
├── index.html                     Main window UI: every tool's markup
├── app.js                         Frontend logic for every tool
├── styles.css                     App styling; imports themes.css and the bundled icon fonts
├── themes.css                     CSS variables for the themes and high contrast variations
└── src-tauri/
    ├── src/
    │   └── main.rs                Rust backend: image, audio, and video conversion commands
    ├── Cargo.toml                 Rust dependencies
    ├── tauri.conf.json            App, window, and bundle configuration
    ├── capabilities/              
    │   └── default.json           Core window/webview/dialog permissions for the main window
    └── permissions/
        └── convert-image.toml     Custom permission exposing convert_image, convert_audio, convert_video
```

## Roadmap

- Real video transcoding and support for MKV, WEBM, AVI, WMV, FLV, animated GIF, and MPEG. The current MP4/MOV support is a lossless container remux only, with no codec conversion, and is still unverified on real hardware
- WMA output, once the encoder's supported profiles can be queried safely rather than assumed
- Auto Clicker tool (the UI panel exists with its toggle, interval, mode, and hold options, but it is not yet wired up to any backend logic)

## License

MIT. See [LICENSE](LICENSE).

Note: this project links against LAME (`mp3lame-encoder`) and libheif (`libheif-rs`), both licensed under LGPL-3.0. The MIT license above covers this project's own code; those two libraries remain under their own LGPL-3.0 terms regardless.
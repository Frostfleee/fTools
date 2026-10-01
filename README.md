# fTools

A lightweight desktop utility for Windows, built with [Tauri](https://tauri.app) and [Rust](https://rust-lang.org/). fTools bundles a handful of small, focused tools into a single compact window: a Unicode text styler, a color format converter with a visual picker, a media file converter and so much more.

## Built with
- [Tauri v2](https://tauri.app) for the desktop shell, with plugins such as `single-instance` (prevents more than one window from opening), `dialog`, `fs` and `global-shortcut`.
- [Rust](https://rust-lang.org/) for the backend, vanilla HTML/CSS/JS for the frontend (no framework, no build step).
- [image](https://crates.io/crates/image), [img-parts](https://crates.io/crates/img-parts), [libheif-rs](https://crates.io/crates/libheif-rs), [ico](https://crates.io/crates/ico) for image encoding and decoding.
- [symphonia](https://crates.io/crates/symphonia) for audio decoding.
- [hound](https://crates.io/crates/hound), [flacenc](https://crates.io/crates/flacenc), [mp3lame-encoder](https://crates.io/crates/mp3lame-encoder), [vorbis_rs](https://crates.io/crates/vorbis_rs), [opus-rs](https://crates.io/crates/opus-rs), [ogg](https://crates.io/crates/ogg), and [rubato](https://crates.io/crates/rubato) for audio encoding and resampling.
- [windows-rs](https://crates.io/crates/windows) (Media Foundation bindings) for AAC, M4A, WMA and WMV.
- [openh264](https://crates.io/crates/openh264) for H.264 decoding and encoding, and libvpx for VP8/VP9, when a video has to be re-encoded (for example MP4 to WebM).
- Hand-written readers and writers for MP4/MOV, MKV/WebM, AVI and FLV, plus an MPEG-4 Part 2 (Xvid/DivX) decoder for old AVI files.
- [reqwest](https://crates.io/crates/reqwest) and [sha2](https://crates.io/crates/sha2) for the in-app updater.
- [window-vibrancy](https://crates.io/crates/window-vibrancy) for the acrylic window background.

## Installation
Download the latest version in [Releases](../../releases) page and run it. fTools is Windows only (x64) since it relies on Windows Media Foundation for some audio and video formats. Once installed, it updates itself from inside the app.

## Building from source
Prerequisites:
- [Rust](https://rustup.rs), a recent stable toolchain (some dependencies require 1.85 or newer, for edition 2024 support)
- [Tauri CLI](https://tauri.app/start/): `cargo install tauri-cli`
- The usual Tauri prerequisites on Windows: Microsoft C++ Build Tools (or Visual Studio) and the WebView2 runtime
- [LLVM](https://releases.llvm.org) (libclang, used to generate the libvpx bindings)
- libvpx and libheif, for example via [vcpkg](https://vcpkg.io): `vcpkg install libvpx:x64-windows-static libheif:x64-windows-static`
- A `main/src-tauri/.cargo/config.toml` that points `VPX_LIB_DIR`, `VPX_INCLUDE_DIR` and `VPX_VERSION` at your libvpx install

```Shell
git clone https://github.com/Frostfleee/fTools/
cd fTools
cargo tauri build
```

For local development without producing an installer:

```Shell
cargo tauri dev
```

## Project structure

```
main/
├── dist/
│   ├── fonts/
│   │   ├── codicon.css        Icon font CSS classes
│   │   └── codicon.ttf        Codicon icon font glyphs
│   ├── icons/
│   │   ├── icon.png           Regular version of the app icon
│   │   └── icon_hc.png        High contrast version of the app icon
│   ├── qrcode/
│   │   └── generator.js       QR code creation logic
│   ├── index.html             Main window UI: every tool's markup
│   ├── app.js                 Frontend logic for every tool
│   ├── styles.css             Styling for the main window and all tools
│   └── themes.css             CSS variables for the themes and high contrast variations
└── src-tauri/
    ├── .cargo/
    │   └── config.toml        Sets vpx env vars before build scripts run
    ├── src/
    │   ├── main.rs            Tauri commands: conversion, auto clicker, etc
    │   ├── update.rs          In-app updater
    │   ├── demux.rs           Reads MKV, WebM, MP4, MOV...
    │   ├── ebml.rs            Matroska element read/write
    │   ├── matroska.rs        MKV/WebM writer
    │   ├── mp4.rs             MP4/MOV writer
    │   ├── avi.rs             AVI reader and writer
    │   ├── flv.rs             FLV reader and writer
    │   ├── wmv.rs             WMV/WMA via Media Foundation
    │   ├── transcode.rs       Copies or re-encodes tracks
    │   ├── h264.rs            H.264 parsing helpers
    │   ├── mpeg4.rs           Xvid/DivX decoder
    │   ├── opus_decode.rs     Opus decoder
    │   ├── animation.rs       Video to GIF and GIF to video
    │   └── vpx.rs             libvpx bindings (VP8/VP9)
    ├── capabilities/
    │   └── default.json       Core window/webview/dialog permissions for the main window
    ├── permissions/
    │   └── app-commands.toml  Permissions (convert_image, convert_audio, convert_video...)
    ├── icons/
    │   └── Icon.ico           App and installer icon
    ├── gen/schemas/           Tauri-generated permission schemas
    ├── build.rs               Tauri build script
    ├── Cargo.toml             Rust dependencies
    ├── Cargo.lock             Locked dependency versions
    └── tauri.conf.json        App, window, and bundle configuration
```

## Requirements
| Requirement | Details |
|:---|:---|
| OS | Windows 10 (1809+) or Windows 11 |
| Architecture | x64 |
| RAM | 2 GB min, 4 GB recommended (app uses ~10 MB) |
| Storage | ~40 MB free |
| Runtime | None required |
| Permissions | No admin rights. [SmartScreen](https://learn.microsoft.com/en-us/windows/security/operating-system-security/virus-and-threat-protection/microsoft-defender-smartscreen/) may warn once on first launch (app isn't code signed) |

## License
MIT. See [LICENSE](LICENSE).

Note: this project links against LAME (`mp3lame-encoder`) and libheif (`libheif-rs`), both licensed under LGPL-3.0. The MIT license above covers this project's own code; those two libraries remain under their own LGPL-3.0 terms regardless. libvpx and OpenH264 are BSD-licensed.
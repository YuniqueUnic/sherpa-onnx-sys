# Syzygy native integration

This fork keeps the 1.13.8 Rust FFI API and the upstream release archives.
Archive extraction is isolated by release name. Static desktop linking uses
the workspace's `ort-sys` runtime instead of linking a second ONNX Runtime.

## Windows CRT selection

Native library linkage and C/C++ runtime linkage are independent. The upstream
build script always selected `MT` archives, which produced `LNK2038` and
`LNK2005` when linked with a default Rust MSVC consumer using `MD`.

The archive selector now reads the target's `CARGO_CFG_TARGET_FEATURE`:
an exact `crt-static` feature selects `MT`; otherwise it selects `MD`.
This applies to both static and shared archives on x64 and arm64. Other
platform filenames and explicit `SHERPA_ONNX_LIB_DIR` overrides are unchanged.
The archive name also separates the CRT variants in the existing cache.
No linker errors are suppressed and static Sherpa linking remains enabled.

The pure archive matrix is covered in `src/tests/prebuilt_archive.rs`.
The existing `src/tests/onnxruntime.rs` test checks that Sherpa and Rust share
the same native ORT API and version pointer. Consumer verification must link
the actual Windows desktop executable, not just run `cargo check`.

On 2026-10-07, all nine crate tests passed on Windows x64 MSVC, including the
native ORT test. The standalone lock file also records the previously missing
`cc` build-dependency edge without changing any dependency versions.

From the parent checkout, with the MSVC tools available:

```text
cargo test --locked --manifest-path crates/patches/sherpa-onnx-sys/Cargo.toml --target x86_64-pc-windows-msvc --lib
```

Official assets: [sherpa-onnx v1.13.8](https://github.com/k2-fsa/sherpa-onnx/releases/tag/v1.13.8).
MSVC contract: [runtime library options](https://learn.microsoft.com/en-us/cpp/build/reference/md-mt-ld-use-run-time-library).

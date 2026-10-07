#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkMode {
    Static,
    Shared,
}

pub fn archive_name(
    link_mode: LinkMode,
    target_os: &str,
    target_arch: &str,
    target_features: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let version = env!("CARGO_PKG_VERSION");
    let crt = if target_features
        .split(',')
        .any(|feature| feature == "crt-static")
    {
        "MT"
    } else {
        "MD"
    };
    let name = match (link_mode, target_os, target_arch) {
        (LinkMode::Static, "linux", "x86_64") => {
            format!("sherpa-onnx-v{version}-linux-x64-static-lib.tar.bz2")
        }
        (LinkMode::Static, "linux", "aarch64") => {
            format!("sherpa-onnx-v{version}-linux-aarch64-static-lib.tar.bz2")
        }
        (LinkMode::Static, "macos", "x86_64") => {
            format!("sherpa-onnx-v{version}-osx-x64-static-lib.tar.bz2")
        }
        (LinkMode::Static, "macos", "aarch64") => {
            format!("sherpa-onnx-v{version}-osx-arm64-static-lib.tar.bz2")
        }
        (LinkMode::Static, "windows", "x86_64") => {
            format!("sherpa-onnx-v{version}-win-x64-static-{crt}-Release-lib.tar.bz2")
        }
        (LinkMode::Static, "windows", "aarch64") => {
            format!("sherpa-onnx-v{version}-win-arm64-static-{crt}-Release-lib.tar.bz2")
        }
        (LinkMode::Shared, "linux", "x86_64") => {
            format!("sherpa-onnx-v{version}-linux-x64-shared-lib.tar.bz2")
        }
        (LinkMode::Shared, "linux", "aarch64") => {
            format!("sherpa-onnx-v{version}-linux-aarch64-shared-cpu-lib.tar.bz2")
        }
        (LinkMode::Shared, "macos", "x86_64") => {
            format!("sherpa-onnx-v{version}-osx-x64-shared-lib.tar.bz2")
        }
        (LinkMode::Shared, "macos", "aarch64") => {
            format!("sherpa-onnx-v{version}-osx-arm64-shared-lib.tar.bz2")
        }
        (LinkMode::Shared, "windows", "x86_64") => {
            format!("sherpa-onnx-v{version}-win-x64-shared-{crt}-Release-lib.tar.bz2")
        }
        (LinkMode::Shared, "windows", "aarch64") => {
            format!("sherpa-onnx-v{version}-win-arm64-shared-{crt}-Release-lib.tar.bz2")
        }
        (LinkMode::Shared, "android", "aarch64" | "arm" | "x86" | "x86_64") => {
            format!("sherpa-onnx-v{version}-android.tar.bz2")
        }
        (LinkMode::Shared, "ios", "aarch64" | "x86_64") => {
            format!("sherpa-onnx-v{version}-ios-shared-onnxruntime-static.xcframework.zip")
        }
        _ => {
            return Err(format!(
                "Unsupported target for sherpa-onnx prebuilt libs: os={target_os}, arch={target_arch}"
            )
            .into());
        }
    };

    Ok(name)
}

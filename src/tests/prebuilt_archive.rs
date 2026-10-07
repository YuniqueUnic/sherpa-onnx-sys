use crate::prebuilt_archive::{archive_name, LinkMode};

#[test]
fn windows_archives_match_target_crt_independently_of_link_mode() {
    for (arch, archive_arch) in [("x86_64", "x64"), ("aarch64", "arm64")] {
        for (mode, archive_mode) in [(LinkMode::Static, "static"), (LinkMode::Shared, "shared")] {
            for (features, crt) in [
                ("", "MD"),
                ("fxsr,sse,sse2", "MD"),
                ("crt-static", "MT"),
                ("fxsr,crt-static,sse2", "MT"),
                ("sse2,crt-static", "MT"),
                ("not-crt-static,crt-static-disabled", "MD"),
            ] {
                let name = archive_name(mode, "windows", arch, features).unwrap();
                let version = env!("CARGO_PKG_VERSION");
                assert_eq!(
                    name,
                    format!("sherpa-onnx-v{version}-win-{archive_arch}-{archive_mode}-{crt}-Release-lib.tar.bz2")
                );
            }
        }
    }
}

#[test]
fn crt_does_not_change_non_windows_archives() {
    for (mode, os, arch, suffix) in [
        (
            LinkMode::Static,
            "linux",
            "x86_64",
            "linux-x64-static-lib.tar.bz2",
        ),
        (
            LinkMode::Shared,
            "linux",
            "aarch64",
            "linux-aarch64-shared-cpu-lib.tar.bz2",
        ),
        (
            LinkMode::Static,
            "macos",
            "aarch64",
            "osx-arm64-static-lib.tar.bz2",
        ),
        (LinkMode::Shared, "android", "arm", "android.tar.bz2"),
        (
            LinkMode::Shared,
            "ios",
            "x86_64",
            "ios-shared-onnxruntime-static.xcframework.zip",
        ),
    ] {
        for features in ["", "crt-static"] {
            let version = env!("CARGO_PKG_VERSION");
            assert_eq!(
                archive_name(mode, os, arch, features).unwrap(),
                format!("sherpa-onnx-v{version}-{suffix}")
            );
        }
    }
}

#[test]
fn unsupported_targets_still_fail() {
    assert!(archive_name(LinkMode::Static, "windows", "x86", "").is_err());
    assert!(archive_name(LinkMode::Static, "android", "aarch64", "").is_err());
    assert!(archive_name(LinkMode::Shared, "unknown", "x86_64", "").is_err());
}

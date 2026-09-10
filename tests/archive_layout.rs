#[path = "../archive_layout.rs"]
mod archive_layout;

use std::fs;

use tempfile::tempdir;

#[test]
fn finds_desktop_lib_under_the_archive_top_level_directory() {
    let temp = tempdir().expect("temporary cache root");
    let stem = "sherpa-onnx-v1.13.7-osx-arm64-static-lib";
    let expected = temp.path().join(stem).join("lib");
    fs::create_dir_all(&expected).expect("nested desktop library directory");

    assert_eq!(
        archive_layout::find_prebuilt_lib_dir(temp.path(), stem, "lib", "arm64-v8a"),
        Some(expected)
    );
}

#[test]
fn finds_android_jni_libs_at_the_extraction_root() {
    let temp = tempdir().expect("temporary cache root");
    let expected = temp.path().join("jniLibs").join("arm64-v8a");
    fs::create_dir_all(&expected).expect("android library directory");

    assert_eq!(
        archive_layout::find_prebuilt_lib_dir(
            temp.path(),
            "sherpa-onnx-v1.13.7-android",
            "lib",
            "arm64-v8a",
        ),
        Some(expected)
    );
}

#[test]
fn does_not_accept_another_android_abi() {
    let temp = tempdir().expect("temporary cache root");
    fs::create_dir_all(temp.path().join("jniLibs").join("x86_64"))
        .expect("another android ABI directory");

    assert_eq!(
        archive_layout::find_prebuilt_lib_dir(
            temp.path(),
            "sherpa-onnx-v1.13.7-android",
            "lib",
            "arm64-v8a",
        ),
        None
    );
}

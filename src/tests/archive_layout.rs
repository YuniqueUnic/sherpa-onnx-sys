use crate::archive_layout;

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

#[test]
fn does_not_reuse_another_versions_android_libraries() {
    let cache = tempdir().expect("temporary cache root");
    fs::create_dir_all(cache.path().join("jniLibs/arm64-v8a"))
        .expect("old unversioned Android libraries");
    let extraction = cache.path().join("sherpa-onnx-v1.13.8-android");
    fs::create_dir(&extraction).expect("current version extraction directory");

    assert_eq!(
        archive_layout::find_prebuilt_lib_dir(
            &extraction,
            "sherpa-onnx-v1.13.8-android",
            "lib",
            "arm64-v8a",
        ),
        None
    );
}

#[test]
fn finds_simulator_libraries_without_selecting_device_libraries() {
    let extraction = tempdir().expect("temporary extraction root");
    let stem = "sherpa-onnx-v1.13.8-ios";
    let payload = extraction.path().join(stem);
    fs::create_dir_all(payload.join("lib")).expect("device libraries");
    let simulator = payload.join("lib-sim");
    fs::create_dir(&simulator).expect("simulator libraries");

    assert_eq!(
        archive_layout::find_prebuilt_lib_dir(extraction.path(), stem, "lib-sim", "arm64-v8a"),
        Some(simulator)
    );
}

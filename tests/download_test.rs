use proto_pdk_test_utils::*;

generate_download_install_tests!("zls-test", "0.13.0");

// mod canary {
//     use super::*;
//
//     generate_download_install_tests!("zls-test", "canary");
// }

#[tokio::test(flavor = "multi_thread")]
async fn supports_linux_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls-test", |config| {
            config.host(HostOS::Linux, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("zls-linux-aarch64-0.13.0".into()),
            checksum_url: Some(
                "https://builds.zigtools.org/zls-linux-aarch64-0.13.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into()
            ),
            download_name: Some("zls-linux-aarch64-0.13.0.tar.xz".into()),
            download_url: "https://builds.zigtools.org/zls-linux-aarch64-0.13.0.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_linux_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls-test", |config| {
            config.host(HostOS::Linux, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("zls-linux-x86_64-0.13.0".into()),
            checksum_url: Some(
                "https://builds.zigtools.org/zls-linux-x86_64-0.13.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into()
            ),
            download_name: Some("zls-linux-x86_64-0.13.0.tar.xz".into()),
            download_url: "https://builds.zigtools.org/zls-linux-x86_64-0.13.0.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_linux_x86() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls-test", |config| {
            config.host(HostOS::Linux, HostArch::X86);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("zls-linux-x86-0.13.0".into()),
            checksum_url: Some(
                "https://builds.zigtools.org/zls-linux-x86-0.13.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into()
            ),
            download_name: Some("zls-linux-x86-0.13.0.tar.xz".into()),
            download_url: "https://builds.zigtools.org/zls-linux-x86-0.13.0.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_macos_arm64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls-test", |config| {
            config.host(HostOS::MacOS, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("zls-macos-aarch64-0.13.0".into()),
            checksum_url: Some(
                "https://builds.zigtools.org/zls-macos-aarch64-0.13.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into()
            ),
            download_name: Some("zls-macos-aarch64-0.13.0.tar.xz".into()),
            download_url: "https://builds.zigtools.org/zls-macos-aarch64-0.13.0.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_macos_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls-test", |config| {
            config.host(HostOS::MacOS, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("zls-macos-x86_64-0.13.0".into()),
            checksum_url: Some(
                "https://builds.zigtools.org/zls-macos-x86_64-0.13.0.tar.xz.minisig".into()
            ),
            checksum_public_key: Some(
                "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into()
            ),
            download_name: Some("zls-macos-x86_64-0.13.0.tar.xz".into()),
            download_url: "https://builds.zigtools.org/zls-macos-x86_64-0.13.0.tar.xz".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_windows_x64() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls-test", |config| {
            config.host(HostOS::Windows, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("zls-windows-x86_64-0.13.0".into()),
            checksum_url: Some(
                "https://builds.zigtools.org/zls-windows-x86_64-0.13.0.zip.minisig".into()
            ),
            checksum_public_key: Some(
                "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into()
            ),
            download_name: Some("zls-windows-x86_64-0.13.0.zip".into()),
            download_url: "https://builds.zigtools.org/zls-windows-x86_64-0.13.0.zip".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn supports_windows_x86() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls-test", |config| {
            config.host(HostOS::Windows, HostArch::X86);
        })
        .await;

    assert_eq!(
        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await,
        DownloadPrebuiltOutput {
            archive_prefix: Some("zls-windows-x86-0.13.0".into()),
            checksum_url: Some(
                "https://builds.zigtools.org/zls-windows-x86-0.13.0.zip.minisig".into()
            ),
            checksum_public_key: Some(
                "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into()
            ),
            download_name: Some("zls-windows-x86-0.13.0.zip".into()),
            download_url: "https://builds.zigtools.org/zls-windows-x86-0.13.0.zip".into(),
            ..Default::default()
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn locates_unix_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls", |config| {
            config.host(HostOS::Linux, HostArch::Arm64);
        })
        .await;

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .exes
            .get("zls")
            .unwrap()
            .exe_path,
        Some("zls".into())
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn locates_windows_bin() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox
        .create_plugin_with_config("zls", |config| {
            config.host(HostOS::Windows, HostArch::X64);
        })
        .await;

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.13.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .exes
            .get("zls")
            .unwrap()
            .exe_path,
        Some("zls.exe".into())
    );
}

use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_download_install_tests!("zls-test", "0.11.0");

mod canary {
    use super::*;

    generate_download_install_tests!("zls-test", "canary");
}

#[test]
fn supports_linux_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_url:
                "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/0.11.0/aarch64-linux/zls"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_url:
                "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/0.11.0/x86_64-linux/zls"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x86() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X86,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_url:
                "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/0.11.0/x86-linux/zls"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::MacOS,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_url:
                "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/0.11.0/aarch64-macos/zls"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::MacOS,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_url:
                "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/0.11.0/x86_64-macos/zls"
                    .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_url: "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/0.11.0/x86_64-windows/zls.exe"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x86() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls-test", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X86,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("0.11.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_url: "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/0.11.0/x86-windows/zls.exe".into(),
            ..Default::default()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::Arm64,
        os: HostOS::Linux,
        ..Default::default()
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.11.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("zls".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_sandbox();
    let mut plugin = create_plugin("zls", sandbox.path());

    plugin.set_environment(HostEnvironment {
        arch: HostArch::X64,
        os: HostOS::Windows,
        ..Default::default()
    });

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("0.11.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("zls.exe".into())
    );
}

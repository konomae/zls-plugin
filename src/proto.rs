use crate::zls_dist::ZlsDist;
use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "ZLS";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine,
        minimum_proto_version: Some(Version::new(0, 47, 4)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations![
            HostOS::Linux => [HostArch::X86, HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X86, HostArch::X64],
        ],
    )?;

    let version = input.context.version;

    let arch = match env.arch {
        HostArch::X86 => "x86",
        HostArch::X64 => "x86_64",
        HostArch::Arm64 => "aarch64",
        _ => unreachable!(),
    };

    let os = env.os;

    let prefix = match os {
        HostOS::Linux => format!("zls-linux-{arch}-{version}"),
        HostOS::MacOS => format!("zls-macos-{arch}-{version}"),
        HostOS::Windows => format!("zls-windows-{arch}-{version}"),
        _ => unreachable!(),
    };

    let filename = if os.is_windows() {
        format!("{prefix}.zip")
    } else {
        format!("{prefix}.tar.xz")
    };

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some(prefix),
        checksum_url: Some(format!("https://builds.zigtools.org/{filename}.minisig")),
        checksum_public_key: Some(
            "RWR+9B91GBZ0zOjh6Lr17+zKf5BoSuFvrx2xSeDE57uIYvnKBGmMjOex".into(),
        ),
        download_url: format!("https://builds.zigtools.org/{filename}"),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "zls".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("zls")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let response: ZlsDist = fetch_json("https://releases.zigtools.org/v1/zls/index.json")?;
    let versions = response.versions.keys().map(|t| t.to_owned()).collect();
    let output = LoadVersionsOutput::from(versions)?;

    Ok(Json(output))
}

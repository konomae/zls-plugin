use crate::zls_dist::ZlsDist;
use extism_pdk::*;
use proto_pdk::*;

static NAME: &str = "ZLS";
static BIN: &str = "zls";

#[plugin_fn]
pub fn register_tool(_: ()) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::CLI,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_proto_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X86, HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X86, HostArch::X64],
        ],
    )?;

    let mut version = input.context.version;
    if version.is_canary() {
        let response: ZlsDist =
            fetch_url("https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/index.json")?;
        version = VersionSpec::parse(response.latest)?;
    }

    let arch = match env.arch {
        HostArch::X86 => "x86",
        HostArch::X64 => "x86_64",
        HostArch::Arm64 => "aarch64",
        _ => unreachable!(),
    };

    let os = env.os;

    let prefix = match os {
        HostOS::Linux => format!("{arch}-linux"),
        HostOS::MacOS => format!("{arch}-macos"),
        HostOS::Windows => format!("{arch}-windows"),
        _ => unreachable!(),
    };

    let filename = os.get_file_name(BIN, "exe");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/{version}/{prefix}/{filename}"
        ),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_proto_environment()?;

    Ok(Json(LocateExecutablesOutput {
        primary: Some(ExecutableConfig::new(
            // proto renames the executable to the tool id.
            // https://github.com/moonrepo/proto/blob/0441fba931060122b55dd972573e126e607b306e/crates/core/src/tool.rs#L933
            env.os.get_file_name(get_tool_id()?, "exe"),
        )),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let response: ZlsDist =
        fetch_url("https://zigtools-releases.nyc3.digitaloceanspaces.com/zls/index.json")?;
    let versions = response.versions.keys().map(|t| t.to_owned()).collect();

    let mut output = LoadVersionsOutput::from(versions)?;
    output
        .aliases
        .insert("master".into(), Version::parse(&response.latest)?);

    Ok(Json(output))
}

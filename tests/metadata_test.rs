use proto_pdk_test_utils::*;

#[tokio::test(flavor = "multi_thread")]
async fn registers_metadata() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("zls-test").await;

    let metadata = plugin.register_tool(RegisterToolInput::default()).await;

    assert_eq!(metadata.name, "ZLS");
    assert_eq!(metadata.type_of, PluginType::CommandLine);
    assert_eq!(
        metadata.plugin_version.unwrap().to_string(),
        env!("CARGO_PKG_VERSION")
    );
}

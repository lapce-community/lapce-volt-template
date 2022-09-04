use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    // Check for user specified LSP server path
    // ```
    // [lapce-plugin-name.lsp]
    // serverPath = "[path or filename]"
    // ```
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        PLUGIN_RPC.start_lsp(
                            Url::parse(&format!("urn:{}", server_path))?,
                            "[language_id]",
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    // Architecture check
    let _ = match VoltEnvironment::architecture().as_deref() {
        Ok("x86_64") => "x86_64",
        Ok("aarch64") => "aarch64",
        _ => return Ok(()),
    };

    // OS check
    let _ = match VoltEnvironment::operating_system().as_deref() {
        Ok("macos") => "macos",
        Ok("linux") => "linux",
        Ok("windows") => "windows",
        _ => return Ok(()),
    };

    // Download URL
    // let _ = format!("https://github.com/<name>/<project>/releases/download/<version>/{filename}");

    // see lapce_plugin::Http for available API to download files

    let _ = match VoltEnvironment::operating_system().as_deref() {
        Ok("windows") => {
            format!("{}.exe", "[filename]")
        }
        _ => "[filename]".to_string(),
    };

    // Plugin working directory
    let volt_uri = VoltEnvironment::uri()?;
    let exec_path = Url::parse(&volt_uri)?.join("[filename]")?;

    // Available language IDs
    // https://github.com/lapce/lapce/blob/HEAD/lapce-proxy/src/buffer.rs#L173
    PLUGIN_RPC.start_lsp(exec_path, "[language_id]", params.initialization_options);

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}

use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "webflow-mcp-server";
const SERVER_PATH: &str = "node_modules/webflow-mcp-server/dist/index.js";

#[derive(Debug, Deserialize)]
struct WebflowContextServerSettings {
    webflow_token: String,
}

struct WebflowModelContextExtension;

impl zed::Extension for WebflowModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-webflow", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `webflow_token` setting".into());
        };
        let settings: WebflowContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: vec![("WEBFLOW_TOKEN".into(), settings.webflow_token)],
        })
    }
}

zed::register_extension!(WebflowModelContextExtension);

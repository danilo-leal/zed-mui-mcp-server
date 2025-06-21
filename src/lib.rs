use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

const PACKAGE_JSON_CONTENT: &str = r#"{
  "name": "mcp-mui-server-wrapper",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "@mui/mcp": "^0.1.0"
  }
}"#;

#[derive(Debug, Deserialize, JsonSchema)]
struct MuiContextServerSettings {
    #[serde(default)]
    preferred_theme: Option<String>,
    #[serde(default)]
    component_filter: Option<Vec<String>>,
}

struct MuiModelContextExtension;

impl zed::Extension for MuiModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Get settings, but make them optional since the MUI MCP might not require configuration
        let settings = ContextServerSettings::for_project("mcp-server-mui", project)?;
        let settings: MuiContextServerSettings = if let Some(settings) = settings.settings {
            serde_json::from_value(settings).map_err(|e| e.to_string())?
        } else {
            MuiContextServerSettings {
                preferred_theme: None,
                component_filter: None,
            }
        };

        let node_modules_dir = std::env::current_dir().unwrap().join("node_modules");
        let mui_mcp_dir = node_modules_dir.join("@mui").join("mcp");
        let package_binary = mui_mcp_dir.join("dist").join("stdio.cjs.js");

        if !package_binary.exists() {
            let package_json_path = std::env::current_dir().unwrap().join("package.json");
            if !package_json_path.exists() {
                fs::write(&package_json_path, PACKAGE_JSON_CONTENT).map_err(|e| e.to_string())?;
            }

            zed::npm_install_package("@mui/mcp", "^0.1.0")?;

            if !package_binary.exists() {
                return Err(format!(
                    "Failed to install @mui/mcp package or binary not found at {}",
                    package_binary.display()
                ));
            }
        }

        let mut env = vec![];

        if let Some(theme) = settings.preferred_theme {
            env.push(("MUI_PREFERRED_THEME".to_string(), theme));
        }

        if let Some(components) = settings.component_filter {
            let components_str = components.join(",");
            env.push(("MUI_COMPONENT_FILTER".to_string(), components_str));
        }

        let args = vec![package_binary.to_string_lossy().to_string()];

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();

        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(MuiContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(MuiModelContextExtension);

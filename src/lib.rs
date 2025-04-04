use zed_extension_api as zed;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Structure to store function information
#[derive(Clone)]
struct FunctionInfo {
    name: String,
    line_number: usize,
    signature: String,
}

// Main extension struct
struct FunctionExtractor {
    functions: Arc<Mutex<HashMap<String, Vec<FunctionInfo>>>>,
}

impl zed::Extension for FunctionExtractor {
    fn new() -> Self {
        FunctionExtractor {
            functions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Override language_server_command to specify language server commands for different languages
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::process::Command> {
        // Return appropriate language server command based on language
        match language_server_id.as_ref() {
            "python" => {
                // Use Python language server (pyright)
                Ok(zed::process::Command::new("pyright-langserver").arg("--stdio"))
            }
            "typescript" | "javascript" => {
                // Use TypeScript language server
                Ok(zed::process::Command::new("typescript-language-server").arg("--stdio"))
            }
            "cpp" => {
                // Use clangd for C++
                Ok(zed::process::Command::new("clangd").arg("--background-index"))
            }
            "rust" => {
                // Use rust-analyzer for Rust
                Ok(zed::process::Command::new("rust-analyzer"))
            }
            _ => {
                // For unsupported languages, delegate to default implementation
                Err(format!("No language server specified for {}", language_server_id).into())
            }
        }
    }

    // Provide initialization options for language servers if needed
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            "python" => {
                // Configuration for pyright
                Ok(Some(serde_json::json!({
                    "python": {
                        "analysis": {
                            "autoSearchPaths": true,
                            "useLibraryCodeForTypes": true,
                            "diagnosticMode": "workspace"
                        }
                    }
                })))
            }
            "typescript" | "javascript" => {
                // Configuration for TypeScript language server
                Ok(Some(serde_json::json!({
                    "typescript": {
                        "suggest": {
                            "completeFunctionCalls": true
                        }
                    }
                })))
            }
            _ => Ok(None),
        }
    }
}

// Register the extension
zed::register_extension!(FunctionExtractor);

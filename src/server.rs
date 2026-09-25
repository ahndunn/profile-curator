use schemars::schema_for;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info};

use crate::export::{export_profile_for_cv_writer, CvWriterProfile};
use crate::mcp::{JsonRpcRequest, JsonRpcResponse, McpCallToolResult, McpContentItem, McpTool};
use crate::patch::{apply_patch, ProfilePatch};
use crate::schema::{sample_curated_profile, CuratedProfile};
use crate::validation::{generate_next_questions, validate_profile};

#[derive(Default)]
pub struct Server;

impl Server {
    pub fn new() -> Self {
        Self
    }

    /// Run the MCP server over standard I/O (stdin/stdout) in a completely stateless loop
    pub async fn run_stdio(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        info!("Profile Curator MCP Server started on stdio");

        while reader.read_line(&mut line).await? > 0 {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                line.clear();
                continue;
            }

            debug!("Received MCP message: {}", trimmed);
            let req: Result<JsonRpcRequest, _> = serde_json::from_str(trimmed);
            match req {
                Ok(request) => {
                    let maybe_response = self.handle_request(request).await;
                    if let Some(resp) = maybe_response {
                        let serialized = serde_json::to_string(&resp)? + "\n";
                        stdout.write_all(serialized.as_bytes()).await?;
                        stdout.flush().await?;
                    }
                }
                Err(e) => {
                    error!("Invalid JSON-RPC request: {}", e);
                    let resp = JsonRpcResponse::error(
                        None,
                        -32700,
                        format!("Parse error: {}", e),
                        None,
                    );
                    let serialized = serde_json::to_string(&resp)? + "\n";
                    stdout.write_all(serialized.as_bytes()).await?;
                    stdout.flush().await?;
                }
            }
            line.clear();
        }

        info!("Profile Curator MCP Server stdin closed. Exiting.");
        Ok(())
    }

    pub async fn handle_request(&self, req: JsonRpcRequest) -> Option<JsonRpcResponse> {
        let id = req.id.clone();
        match req.method.as_str() {
            "initialize" => {
                let init_result = json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "profile-curator-mcp",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                });
                Some(JsonRpcResponse::success(id, init_result))
            }
            "notifications/initialized" => None,
            "ping" => Some(JsonRpcResponse::success(id, json!({}))),
            "tools/list" => {
                let tools = self.list_tools();
                Some(JsonRpcResponse::success(id, json!({ "tools": tools })))
            }
            "tools/call" => {
                let response = self.call_tool(req.params).await;
                Some(JsonRpcResponse::success(id, response))
            }
            unknown => Some(JsonRpcResponse::error(
                id,
                -32601,
                format!("Method '{}' not found", unknown),
                None,
            )),
        }
    }

    fn list_tools(&self) -> Vec<McpTool> {
        let profile_schema = schema_for!(CuratedProfile);
        let profile_schema_json = serde_json::to_value(profile_schema).unwrap_or(json!({}));

        let patch_schema = schema_for!(ProfilePatch);
        let patch_schema_json = serde_json::to_value(patch_schema).unwrap_or(json!({}));

        vec![
            McpTool {
                name: "get_profile_schema".to_string(),
                description: "Returns the complete JSON Schema of CuratedProfile (including professional background and career orientation) and ProfilePatch. AI Agents use this to understand the data model.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
            },
            McpTool {
                name: "create_empty_profile".to_string(),
                description: "Returns an empty scaffolded CuratedProfile ready to be iteratively updated during candidate interviews.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
            },
            McpTool {
                name: "get_sample_profile".to_string(),
                description: "Returns a rich, realistic example of a curated profile with background (education, experience, projects, skills, awards) and career orientation (target roles, locations, domains, seniorities).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }),
            },
            McpTool {
                name: "patch_profile".to_string(),
                description: "Incrementally applies a patch to a candidate's profile. Stateless: caller passes the current profile and the patch; receives the updated profile and a human-readable list of applied changes.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "profile": profile_schema_json.clone(),
                        "patch": patch_schema_json
                    },
                    "required": ["profile", "patch"]
                }),
            },
            McpTool {
                name: "validate_profile".to_string(),
                description: "Analyzes a curated profile for completeness, critical missing elements (e.g. missing contact, missing target roles, missing skills), and returns readiness scores and recommendations.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "profile": profile_schema_json.clone()
                    },
                    "required": ["profile"]
                }),
            },
            McpTool {
                name: "recommend_next_questions".to_string(),
                description: "Inspects the candidate profile and suggests targeted conversational questions for the AI agent to ask next to fill in profile gaps or explore career orientation.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "profile": profile_schema_json.clone()
                    },
                    "required": ["profile"]
                }),
            },
            McpTool {
                name: "export_to_cv_writer".to_string(),
                description: "Converts a curated profile into the exact JSON format required by cv-writer's render_cv tool. Enables seamless resume generation without manual data reshaping.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "profile": profile_schema_json
                    },
                    "required": ["profile"]
                }),
            },
        ]
    }

    async fn call_tool(&self, params: Option<serde_json::Value>) -> serde_json::Value {
        let params = match params {
            Some(p) => p,
            None => {
                return serde_json::to_value(McpCallToolResult {
                    is_error: Some(true),
                    content: vec![McpContentItem::Text {
                        text: "Missing params object in tools/call".to_string(),
                    }],
                })
                .unwrap();
            }
        };

        let tool_name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let arguments = params.get("arguments").cloned().unwrap_or(json!({}));

        match tool_name {
            "get_profile_schema" => {
                let profile_schema = schema_for!(CuratedProfile);
                let patch_schema = schema_for!(ProfilePatch);
                let cv_writer_schema = schema_for!(CvWriterProfile);
                let text = serde_json::to_string_pretty(&json!({
                    "curated_profile_schema": profile_schema,
                    "profile_patch_schema": patch_schema,
                    "cv_writer_target_schema": cv_writer_schema
                }))
                .unwrap_or_default();

                serde_json::to_value(McpCallToolResult {
                    is_error: Some(false),
                    content: vec![McpContentItem::Text { text }],
                })
                .unwrap()
            }
            "create_empty_profile" => {
                let empty = CuratedProfile::default();
                let text = serde_json::to_string_pretty(&empty).unwrap_or_default();
                serde_json::to_value(McpCallToolResult {
                    is_error: Some(false),
                    content: vec![McpContentItem::Text { text }],
                })
                .unwrap()
            }
            "get_sample_profile" => {
                let sample = sample_curated_profile();
                let text = serde_json::to_string_pretty(&sample).unwrap_or_default();
                serde_json::to_value(McpCallToolResult {
                    is_error: Some(false),
                    content: vec![McpContentItem::Text { text }],
                })
                .unwrap()
            }
            "patch_profile" => {
                let profile_val = match arguments.get("profile") {
                    Some(v) => v,
                    None => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: "Missing required argument 'profile'".to_string(),
                            }],
                        })
                        .unwrap();
                    }
                };

                let patch_val = match arguments.get("patch") {
                    Some(v) => v,
                    None => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: "Missing required argument 'patch'".to_string(),
                            }],
                        })
                        .unwrap();
                    }
                };

                let profile: CuratedProfile = match serde_json::from_value(profile_val.clone()) {
                    Ok(p) => p,
                    Err(e) => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: format!("Invalid CuratedProfile format: {}", e),
                            }],
                        })
                        .unwrap();
                    }
                };

                let patch: ProfilePatch = match serde_json::from_value(patch_val.clone()) {
                    Ok(p) => p,
                    Err(e) => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: format!("Invalid ProfilePatch format: {}", e),
                            }],
                        })
                        .unwrap();
                    }
                };

                let (updated_profile, diff) = apply_patch(profile, patch);
                let result = json!({
                    "updated_profile": updated_profile,
                    "applied_changes": diff
                });

                serde_json::to_value(McpCallToolResult {
                    is_error: Some(false),
                    content: vec![McpContentItem::Text {
                        text: serde_json::to_string_pretty(&result).unwrap_or_default(),
                    }],
                })
                .unwrap()
            }
            "validate_profile" => {
                let profile_val = match arguments.get("profile") {
                    Some(v) => v,
                    None => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: "Missing required argument 'profile'".to_string(),
                            }],
                        })
                        .unwrap();
                    }
                };

                let profile: CuratedProfile = match serde_json::from_value(profile_val.clone()) {
                    Ok(p) => p,
                    Err(e) => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: format!("Invalid CuratedProfile format: {}", e),
                            }],
                        })
                        .unwrap();
                    }
                };

                let report = validate_profile(&profile);
                serde_json::to_value(McpCallToolResult {
                    is_error: Some(false),
                    content: vec![McpContentItem::Text {
                        text: serde_json::to_string_pretty(&report).unwrap_or_default(),
                    }],
                })
                .unwrap()
            }
            "recommend_next_questions" => {
                let profile_val = match arguments.get("profile") {
                    Some(v) => v,
                    None => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: "Missing required argument 'profile'".to_string(),
                            }],
                        })
                        .unwrap();
                    }
                };

                let profile: CuratedProfile = match serde_json::from_value(profile_val.clone()) {
                    Ok(p) => p,
                    Err(e) => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: format!("Invalid CuratedProfile format: {}", e),
                            }],
                        })
                        .unwrap();
                    }
                };

                let questions = generate_next_questions(&profile);
                serde_json::to_value(McpCallToolResult {
                    is_error: Some(false),
                    content: vec![McpContentItem::Text {
                        text: serde_json::to_string_pretty(&json!({ "suggested_questions": questions }))
                            .unwrap_or_default(),
                    }],
                })
                .unwrap()
            }
            "export_to_cv_writer" => {
                let profile_val = match arguments.get("profile") {
                    Some(v) => v,
                    None => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: "Missing required argument 'profile'".to_string(),
                            }],
                        })
                        .unwrap();
                    }
                };

                let profile: CuratedProfile = match serde_json::from_value(profile_val.clone()) {
                    Ok(p) => p,
                    Err(e) => {
                        return serde_json::to_value(McpCallToolResult {
                            is_error: Some(true),
                            content: vec![McpContentItem::Text {
                                text: format!("Invalid CuratedProfile format: {}", e),
                            }],
                        })
                        .unwrap();
                    }
                };

                let cv_writer_input = export_profile_for_cv_writer(&profile);
                serde_json::to_value(McpCallToolResult {
                    is_error: Some(false),
                    content: vec![McpContentItem::Text {
                        text: serde_json::to_string_pretty(&json!({
                            "cv_profile": cv_writer_input,
                            "cv_writer_tool_call_sample": {
                                "name": "render_cv",
                                "arguments": {
                                    "profile": cv_writer_input
                                }
                            }
                        }))
                        .unwrap_or_default(),
                    }],
                })
                .unwrap()
            }
            unknown => serde_json::to_value(McpCallToolResult {
                is_error: Some(true),
                content: vec![McpContentItem::Text {
                    text: format!("Unknown tool '{}'", unknown),
                }],
            })
            .unwrap(),
        }
    }
}

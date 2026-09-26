use profile_curator::mcp::{JsonRpcRequest, McpCallToolResult, McpContentItem};
use profile_curator::schema::CuratedProfile;
use profile_curator::Server;
use serde_json::json;

#[tokio::test]
async fn test_mcp_initialize_and_tools_list() {
    let server = Server::new();

    // 1. Initialize
    let init_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: None,
    };
    let resp = server.handle_request(init_req).await.expect("Expected init response");
    assert_eq!(resp.id, Some(json!(1)));
    let result = resp.result.expect("Expected result");
    assert_eq!(result.get("serverInfo").unwrap().get("name").unwrap(), "profile-curator-mcp");

    // 2. Tools list
    let list_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/list".to_string(),
        params: None,
    };
    let resp = server.handle_request(list_req).await.expect("Expected list response");
    let result = resp.result.expect("Expected result");
    let tools = result.get("tools").unwrap().as_array().unwrap();
    let tool_names: Vec<&str> = tools.iter().map(|t| t.get("name").unwrap().as_str().unwrap()).collect();
    
    assert!(tool_names.contains(&"get_profile_schema"));
    assert!(tool_names.contains(&"create_empty_profile"));
    assert!(tool_names.contains(&"patch_profile"));
    assert!(tool_names.contains(&"validate_profile"));
    assert!(tool_names.contains(&"recommend_next_questions"));
    assert!(tool_names.contains(&"export_to_cv_writer"));
}

#[tokio::test]
async fn test_conversational_incremental_patch_and_export_flow() {
    let server = Server::new();

    // Step 1: Create empty profile
    let create_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(10)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "create_empty_profile",
            "arguments": {}
        })),
    };
    let resp = server.handle_request(create_req).await.unwrap();
    let call_res: McpCallToolResult = serde_json::from_value(resp.result.unwrap()).unwrap();
    let text = match &call_res.content[0] {
        McpContentItem::Text { text } => text,
    };
    let mut current_profile: CuratedProfile = serde_json::from_str(text).unwrap();

    // Step 2: Agent chats with user, discovers name and target roles
    let patch_1 = json!({
        "contact": {
            "name": "Minh Nguyen",
            "email": "minh.nguyen@example.com",
            "current_location": "Ho Chi Minh City, Vietnam"
        },
        "career_orientation": {
            "add_target_roles": ["AI Engineer", "Backend Engineer"],
            "add_preferred_locations": ["Ho Chi Minh City", "Singapore", "Remote"],
            "add_target_seniorities": ["Senior"],
            "add_target_domains": ["LLM Infrastructure", "FinTech"]
        }
    });

    let patch_req_1 = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(11)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "patch_profile",
            "arguments": {
                "profile": current_profile,
                "patch": patch_1
            }
        })),
    };
    let resp_1 = server.handle_request(patch_req_1).await.unwrap();
    let call_res_1: McpCallToolResult = serde_json::from_value(resp_1.result.unwrap()).unwrap();
    let text_1 = match &call_res_1.content[0] {
        McpContentItem::Text { text } => text,
    };
    let parsed_1: serde_json::Value = serde_json::from_str(text_1).unwrap();
    current_profile = serde_json::from_value(parsed_1.get("updated_profile").unwrap().clone()).unwrap();

    assert_eq!(current_profile.contact.name.as_deref(), Some("Minh Nguyen"));
    assert_eq!(current_profile.career_orientation.target_roles, vec!["AI Engineer", "Backend Engineer"]);

    // Step 3: Agent asks for recommendations to know what to ask next
    let questions_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(12)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "recommend_next_questions",
            "arguments": {
                "profile": current_profile
            }
        })),
    };
    let resp_q = server.handle_request(questions_req).await.unwrap();
    let call_res_q: McpCallToolResult = serde_json::from_value(resp_q.result.unwrap()).unwrap();
    let text_q = match &call_res_q.content[0] {
        McpContentItem::Text { text } => text,
    };
    assert!(text_q.contains("experience") || text_q.contains("skills"));

    // Step 4: Add experience, projects, skills via patch 2
    let patch_2 = json!({
        "add_experience": [
            {
                "company": "VNG Corporation",
                "location": "Ho Chi Minh City",
                "roles": [
                    {
                        "title": "Senior AI / Backend Engineer",
                        "dates": "2021 -- Present",
                        "highlights": [
                            "Deployed high throughput LLM inference service reducing GPU costs by 45%.",
                            "Designed stateless gRPC services in Rust."
                        ]
                    }
                ]
            }
        ],
        "upsert_skills": [
            {
                "category": "Core Engineering",
                "items": ["Rust", "Python", "Kubernetes", "PyTorch"]
            }
        ],
        "upsert_languages": [
            {
                "language": "Vietnamese",
                "proficiency": "Native"
            },
            {
                "language": "English",
                "proficiency": "Professional Working"
            }
        ]
    });

    let patch_req_2 = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(13)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "patch_profile",
            "arguments": {
                "profile": current_profile,
                "patch": patch_2
            }
        })),
    };
    let resp_2 = server.handle_request(patch_req_2).await.unwrap();
    let call_res_2: McpCallToolResult = serde_json::from_value(resp_2.result.unwrap()).unwrap();
    let text_2 = match &call_res_2.content[0] {
        McpContentItem::Text { text } => text,
    };
    let parsed_2: serde_json::Value = serde_json::from_str(text_2).unwrap();
    current_profile = serde_json::from_value(parsed_2.get("updated_profile").unwrap().clone()).unwrap();

    // Step 5: Export to cv-writer
    let export_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(14)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "export_to_cv_writer",
            "arguments": {
                "profile": current_profile
            }
        })),
    };
    let resp_exp = server.handle_request(export_req).await.unwrap();
    let call_res_exp: McpCallToolResult = serde_json::from_value(resp_exp.result.unwrap()).unwrap();
    let text_exp = match &call_res_exp.content[0] {
        McpContentItem::Text { text } => text,
    };
    let parsed_exp: serde_json::Value = serde_json::from_str(text_exp).unwrap();
    let cv_profile = parsed_exp.get("cv_profile").expect("Expected cv_profile in export output");

    // Verify cv_writer format compatibility
    assert_eq!(cv_profile.get("contact").unwrap().get("name").unwrap(), "Minh Nguyen");
    let exp_arr = cv_profile.get("experience").unwrap().as_array().unwrap();
    assert_eq!(exp_arr[0].get("company").unwrap(), "VNG Corporation");
    let skills_arr = cv_profile.get("skills").unwrap().as_array().unwrap();
    assert!(skills_arr.iter().any(|s| s.get("category").unwrap() == "Languages"));
}

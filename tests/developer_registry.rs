use markdown_studio::domain::developer::ToolId;

#[test]
fn every_registered_tool_has_route_and_title() {
    let tools = ToolId::all().collect::<Vec<_>>();
    assert_eq!(tools.len(), 34);
    assert!(tools
        .iter()
        .all(|tool| !tool.route().is_empty() && !tool.title().is_empty()));
}

#[test]
fn registered_tool_routes_are_unique() {
    let tools = ToolId::all().collect::<Vec<_>>();
    for (index, tool) in tools.iter().enumerate() {
        assert!(tools[..index]
            .iter()
            .all(|other| other.route() != tool.route()));
    }
}

#[test]
fn registered_tool_routes_resolve_to_the_same_tool() {
    assert_eq!(ToolId::from_route("regex"), Some(ToolId::Regex));
}

#[test]
fn network_tool_adapters_accept_secondary_input() {
    let result = ToolId::HttpStatus
        .execute("404", "")
        .expect("HTTP status adapter should execute");
    assert!(result.contains("404 Not Found"));
}

#[test]
fn subnet_tool_adapter_accepts_secondary_input() {
    let result = ToolId::Subnet
        .execute("192.168.1.0/24", "")
        .expect("subnet adapter should execute");
    assert!(result.contains("Network: 192.168.1.0"));
}

#[test]
fn timestamp_tool_adapter_accepts_secondary_input() {
    let result = ToolId::Timestamp
        .execute("0", "")
        .expect("timestamp adapter should execute");
    assert!(result.contains("1970-01-01"));
}

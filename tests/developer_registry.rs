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

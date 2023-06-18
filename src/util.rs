pub fn route_for_id_prefix(id_prefix: &str) -> &'static str {
    match id_prefix {
        "acct" => "/accounts",
        "award" => "/awards",
        "claim" => "/claims",
        "ecterm" => "/ec_terms",
        "media" => "/media",
        "person" => "/people",
        "season" => "/seasons",
        "show" => "/shows",
        "tag" => "/tags",
        "work" => "/worked_on",
        _ => panic!("Unknown id prefix {id_prefix}s"),
    }
}
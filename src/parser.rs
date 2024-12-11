use minidom::Element;

pub const NS_D: &str = "DAV:";
pub const NS_C: &str = "urn:ietf:params:xml:ns:caldav";
pub const NS_CS: &str = "http://calendarserver.org/ns/";
pub const NS_I: &str = "http://apple.com/ns/ical/";

pub fn format_ns_attrs() -> String {
    format!("xmlns:d=\"{NS_D}\" xmlns:c=\"{NS_C}\" xmlns:cs=\"{NS_CS}\" xmlns:i=\"{NS_I}\"")
}

pub fn follow_tree(el: &Element, tree: &str, namespace: &str) -> Option<Element> {
    let parts = tree.split(".");
    let mut cur_el = el;
    for part in parts {
        cur_el = cur_el.get_child(part, namespace)?;
    }
    Some(cur_el.clone())
}

pub fn add_path(url: &str, path: &str) -> String {
    let base = url.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    format!("{}/{}", base, path)
}

pub fn go_back(url: &str) -> String {
    // TODO fixme url??
    let url = url.trim_end_matches('/');
    let mut parts: Vec<&str> = url.split_inclusive('/').collect();
    parts.pop();
    parts.concat().trim_end_matches('/').to_string()
}

#[cfg(test)]
mod tests {
    use crate::parser::go_back;

    #[test]
    fn test_go_back() {
        assert_eq!(go_back("example/one/two"), "example/one".to_string());
    }
}

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Url {
    raw: String,
    host: Vec<String>,
    path: Vec<String>,
}

impl Url {
    pub fn new(base: String, route_path: String) -> Self {

        let spl = if route_path.starts_with('/') { "" } else { "/" };

        let raw = format!("{base}{spl}{route_path}");
        let host = vec![base];
        let path = vec![route_path];

        Self { raw, host, path }
    }
}

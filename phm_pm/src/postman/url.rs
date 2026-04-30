use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
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
        let path = route_path
            .split('/')
            .filter_map(|s| {
                let s = s.trim();

                if s.is_empty() {
                    None
                } else {
                    Some(s.replace('/', "").trim().to_string())
                }
            })
            .collect();

        Self { raw, host, path }
    }
}

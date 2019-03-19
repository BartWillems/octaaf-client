use serde::Serialize;

static API_URL: &'static str = "https://api.octaafbot.xyz/api/v1";

fn format_uri(endpoint: &str) -> String {
    if endpoint.starts_with("/") {
        format!("{}{}", API_URL, endpoint)
    } else {
        format!("{}/{}", API_URL, endpoint)
    }
}

pub fn get(endpoint: &str) -> reqwest::Result<reqwest::Response> {
    let uri = &format_uri(endpoint);
    reqwest::Client::new().get(uri).send()
}

pub fn get_parameterized<T: Serialize + ?Sized>(
    endpoint: &str,
    params: &T,
) -> reqwest::Result<reqwest::Response> {
    let uri = &format_uri(endpoint);
    reqwest::Client::new().get(uri).query(params).send()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_uri() {
        let uri: &str = &format!("{}/{}", API_URL, "test");
        assert_eq!(format_uri("test"), uri);
        assert_eq!(format_uri("/test"), uri);
    }
}

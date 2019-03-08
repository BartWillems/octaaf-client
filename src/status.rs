static API_URL: &'static str = "http://188.166.33.109:8080/api/v1/health";

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub healthy: bool,
    pub response: String,
    pub status_code: u16,
}

pub fn get_status() -> Status {
    let req = reqwest::Client::new().get(API_URL).send();

    let mut res: reqwest::Response;
    match req {
        Ok(req) => {
            res = req;
        }
        Err(e) => {
            debug!("Unable to do a request to the server.");
            return Status {
                healthy: false,
                response: e.to_string(),
                status_code: 0,
            };
        }
    }

    Status {
        healthy: res.status().is_success(),
        response: res.text().unwrap_or_default(),
        status_code: res.status().as_u16(),
    }
}

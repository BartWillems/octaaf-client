use std::fmt;

static API_URL: &'static str = "https://api.octaafbot.xyz/api/v1/health";

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub healthy: bool,
    pub response: String,
    pub status_code: u16,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl Status {
    pub fn get() -> Result<Status, Status> {
        let req = reqwest::Client::new().get(API_URL).send();

        let mut res: reqwest::Response;
        match req {
            Ok(req) => {
                res = req;
            }
            Err(e) => {
                debug!("Unable to do a request to the server.");
                return Err(Status {
                    healthy: false,
                    response: e.to_string(),
                    status_code: 0,
                });
            }
        }

        Ok(Status {
            healthy: res.status().is_success(),
            response: res.text().unwrap_or_default(),
            status_code: res.status().as_u16(),
        })
    }
}

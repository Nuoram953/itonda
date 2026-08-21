use std::collections::HashMap;

use reqwest::Client;

use super::AuthError;

pub const OPENID_NS: &str = "http://specs.openid.net/auth/2.0";
pub const OPENID_IDENTIFIER_SELECT: &str = "http://specs.openid.net/auth/2.0/identifier_select";

pub struct OpenId2Client {
    endpoint: String,
    http_client: Client,
}

impl OpenId2Client {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            http_client: Client::new(),
        }
    }

    pub fn build_auth_url(&self, return_to: &str, realm: &str) -> String {
        let params = [
            ("openid.ns", OPENID_NS),
            ("openid.mode", "checkid_setup"),
            ("openid.return_to", return_to),
            ("openid.realm", realm),
            ("openid.identity", OPENID_IDENTIFIER_SELECT),
            ("openid.claimed_id", OPENID_IDENTIFIER_SELECT),
        ];

        let mut serializer = url::form_urlencoded::Serializer::new(String::new());
        for (k, v) in params {
            serializer.append_pair(k, v);
        }
        let query = serializer.finish();

        format!("{}?{}", self.endpoint, query)
    }

    pub async fn verify_callback(
        &self,
        query_params: &[(String, String)],
    ) -> Result<String, AuthError> {
        let mut validation_params: HashMap<String, String> = query_params.iter().cloned().collect();

        validation_params.insert(
            "openid.mode".to_string(),
            "check_authentication".to_string(),
        );

        let response = self
            .http_client
            .post(&self.endpoint)
            .form(&validation_params)
            .send()
            .await?;

        let body = response.text().await?;

        let is_valid = body.lines().any(|line| line.trim() == "is_valid:true");

        if !is_valid {
            return Err(AuthError::OpenIdValidation(
                "OpenID signature verification failed (is_valid:false)".into(),
            ));
        }

        let claimed_id = validation_params
            .get("openid.claimed_id")
            .ok_or(AuthError::MissingParameter("openid.claimed_id"))?;

        Ok(claimed_id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_auth_url() {
        let client = OpenId2Client::new("https://steamcommunity.com/openid/login");
        let return_to = "http://localhost:3005/api/v1/auth/steam/callback";
        let realm = "http://localhost:3005/";

        let url = client.build_auth_url(return_to, realm);

        assert!(url.starts_with("https://steamcommunity.com/openid/login?"));
        assert!(url.contains("openid.ns=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0"));
        assert!(url.contains("openid.mode=checkid_setup"));
        assert!(url.contains(
            "openid.return_to=http%3A%2F%2Flocalhost%3A3005%2Fapi%2Fv1%2Fauth%2Fsteam%2Fcallback"
        ));
        assert!(url.contains("openid.realm=http%3A%2F%2Flocalhost%3A3005%2F"));
        assert!(url.contains(
            "openid.identity=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select"
        ));
        assert!(url.contains(
            "openid.claimed_id=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select"
        ));
    }
}

use async_trait::async_trait;

use super::{AuthError, AuthProfile, StorefrontAuthenticator, openid::OpenId2Client};
use crate::storefronts::models::StorefrontId;

pub const STEAM_OPENID_ENDPOINT: &str = "https://steamcommunity.com/openid/login";
pub const STEAM_CLAIMED_ID_PREFIX: &str = "https://steamcommunity.com/openid/id/";

pub struct SteamAuthenticator {
    openid_client: OpenId2Client,
}

impl Default for SteamAuthenticator {
    fn default() -> Self {
        Self::new()
    }
}

impl SteamAuthenticator {
    pub fn new() -> Self {
        Self {
            openid_client: OpenId2Client::new(STEAM_OPENID_ENDPOINT),
        }
    }

    pub fn parse_steam_id(claimed_id: &str) -> Result<u64, AuthError> {
        let trimmed = claimed_id.trim();
        if let Some(id_str) = trimmed.strip_prefix(STEAM_CLAIMED_ID_PREFIX) {
            let id_str = id_str.trim_end_matches('/');
            id_str.parse::<u64>().map_err(|_| {
                AuthError::InvalidClaimedId(format!("Could not parse SteamID as u64: '{id_str}'"))
            })
        } else {
            Err(AuthError::InvalidClaimedId(format!(
                "Claimed ID does not start with Steam prefix '{STEAM_CLAIMED_ID_PREFIX}': '{trimmed}'"
            )))
        }
    }
}

#[async_trait]
impl StorefrontAuthenticator for SteamAuthenticator {
    fn storefront_id(&self) -> StorefrontId {
        StorefrontId::Steam
    }

    fn generate_auth_url(&self, return_to: &str, realm: &str) -> String {
        self.openid_client.build_auth_url(return_to, realm)
    }

    async fn verify_callback(&self, params: &[(String, String)]) -> Result<AuthProfile, AuthError> {
        let claimed_id = self.openid_client.verify_callback(params).await?;
        let steam_id = Self::parse_steam_id(&claimed_id)?;

        Ok(AuthProfile {
            storefront: StorefrontId::Steam,
            external_id: steam_id.to_string(),
            display_name: None,
            avatar_url: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_steam_id() {
        let claimed_id = "https://steamcommunity.com/openid/id/76561198012345678";
        let steam_id = SteamAuthenticator::parse_steam_id(claimed_id).unwrap();
        assert_eq!(steam_id, 76561198012345678);

        let claimed_id_slash = "https://steamcommunity.com/openid/id/76561198012345678/";
        let steam_id_slash = SteamAuthenticator::parse_steam_id(claimed_id_slash).unwrap();
        assert_eq!(steam_id_slash, 76561198012345678);
    }

    #[test]
    fn test_parse_invalid_steam_id() {
        let wrong_prefix = "https://evil.com/openid/id/76561198012345678";
        assert!(SteamAuthenticator::parse_steam_id(wrong_prefix).is_err());

        let non_numeric = "https://steamcommunity.com/openid/id/notanumber";
        assert!(SteamAuthenticator::parse_steam_id(non_numeric).is_err());

        assert!(SteamAuthenticator::parse_steam_id("").is_err());
    }
}

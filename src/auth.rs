use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, TokenResponse, TokenUrl, AuthorizationCode,
};
use url::Url;

pub struct OAuth2Host {
    client: BasicClient,
}

impl OAuth2Host {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        auth_url: &str,
        token_url: &str,
    ) -> Result<Self, String> {
        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(auth_url.to_string()).map_err(|e| e.to_string())?,
            Some(TokenUrl::new(token_url.to_string()).map_err(|e| e.to_string())?),
        );
        Ok(OAuth2Host { client })
    }

    pub fn get_auth_url(&self) -> String {
        let (auth_url, _csrf_token) = self.client.authorize_url(oauth2::CsrfToken::new_random).generate_url();
        auth_url.to_string()
    }

    pub async fn exchange_code(&self, code: String) -> Result<String, String> {
        let token_result = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| e.to_string())?;

        Ok(token_result.access_token().secret().to_string())
    }
}

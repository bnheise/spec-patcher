use clap::Args;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Args, Deserialize)]
pub struct ConnectionOpt {
    #[arg(long = "url")]
    pub base_url: Option<Url>,
    #[command(flatten)]
    pub basic_auth: Option<BasicAuth>,
    #[command(flatten)]
    pub secret: Option<TokenAuth>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    pub base_url: Url,
    pub auth: Auth,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Auth {
    Basic(BasicAuth),
    OAuth(TokenAuth),
}

#[derive(Debug, Clone, Deserialize, Args)]
#[group(id = "basic_auth", required = false, multiple = true)]
pub struct BasicAuth {
    #[arg(short = 'u', long = "user", conflicts_with = "oauth")]
    pub username: String,
    #[arg(short = 'p', long = "password", conflicts_with = "oauth")]
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Args)]
#[group(id = "oauth", required = false, multiple = true)]
pub struct TokenAuth {
    #[arg(short = 'i', long = "client_id", conflicts_with = "basic_auth")]
    pub client_id: String,
    #[arg(long = "secret", conflicts_with = "basic_auth")]
    pub client_secret: String,
}

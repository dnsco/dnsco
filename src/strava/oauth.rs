use oauth2::{Config, Token, TokenError};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;

#[derive(Debug, Clone)]
pub struct OauthToken(pub String);

const AUTH_URL: &str = "https://www.strava.com/oauth/authorize";
const TOKEN_URL: &str = "https://www.strava.com/oauth/token";

pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

pub fn oauth_dance(oauth_config: ClientConfig) -> Result<Token, TokenError> {
    // Set up the config for the Github OAuth2 process.
    let mut config = Config::new(
        oauth_config.client_id,
        oauth_config.client_secret,
        AUTH_URL,
        TOKEN_URL,
    );

    // This example is requesting access to the user's public repos and email.
    config = config.add_scope("activity:read_all");

    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    config = config.set_redirect_url(oauth_config.redirect_url);

    // Set the state parameter (optional)
    // Please upgrade to 2.0, this is deprecated because it reuses the same state for every request
    #[allow(deprecated)]
    let config = config.set_state("1234");

    // Generate the authorization URL to which we'll redirect the user.
    let authorize_url = config.authorize_url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // These variables will store the code & state retrieved during the authorization process.
    let mut code = String::new();
    let mut state = String::new();

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                {
                    let mut reader = BufReader::new(&stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    let code_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "code"
                        })
                        .unwrap();

                    let (_, value) = code_pair;
                    code = value.into_owned();

                    let state_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "state"
                        })
                        .unwrap();

                    let (_, value) = state_pair;
                    state = value.into_owned();
                }

                let message = "Go back to your terminal :)";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).unwrap();

                // The server will terminate itself after collecting the first code.
                break;
            }
            Err(_) => {}
        }
    }

    println!("Github returned the following code:\n{}\n", code);
    println!("Github returned the following state:\n{}\n", state);

    config.exchange_code(code)
}

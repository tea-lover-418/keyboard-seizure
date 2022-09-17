use google_youtube3::{YouTube, oauth2, hyper, hyper_rustls};

pub struct ChatConnector {
  hub: YouTube<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
  chat_level: i16
}

impl ChatConnector {
  pub async fn new(key: &str) -> Self {
    let secret: oauth2::ApplicationSecret = Default::default();

    let hyper_client = hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build());
    let auth = oauth2::InstalledFlowAuthenticator::builder(
      secret,
      oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    ).build().await.unwrap();

    return Self {
      hub: YouTube::new(hyper_client, auth),
      chat_level: 0,
    };
  }

  /** Returns current chat level */
  pub fn read_chat() -> i16 {
    return 1
  }
}
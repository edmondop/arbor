use {
    futures::FutureExt,
    gpui_http_client::{AsyncBody, HttpClient, Response, Url, http},
    std::sync::Arc,
};

/// A minimal HTTP client that performs blocking requests on a background thread.
/// Only used for loading remote images (GitHub avatars).
pub struct SimpleHttpClient;

impl HttpClient for SimpleHttpClient {
    fn type_name(&self) -> &'static str {
        "SimpleHttpClient"
    }

    fn user_agent(&self) -> Option<&http::HeaderValue> {
        None
    }

    fn send(
        &self,
        req: http::Request<AsyncBody>,
    ) -> futures::future::BoxFuture<'static, anyhow::Result<Response<AsyncBody>>> {
        let uri = req.uri().to_string();
        let method = req.method().clone();

        async move {
            let (status, body_bytes) =
                smol::unblock(move || blocking_request(&method, &uri)).await?;

            let response = Response::builder()
                .status(status)
                .body(AsyncBody::from(body_bytes))?;
            Ok(response)
        }
        .boxed()
    }

    fn proxy(&self) -> Option<&Url> {
        None
    }
}

fn blocking_request(
    method: &http::Method,
    uri: &str,
) -> anyhow::Result<(http::StatusCode, Vec<u8>)> {
    let config = ureq::config::Config::builder()
        .http_status_as_error(false)
        .build();
    let agent = ureq::Agent::new_with_config(config);

    let response = match *method {
        http::Method::GET => agent.get(uri).call()?,
        _ => anyhow::bail!("unsupported HTTP method: {method}"),
    };

    let status_code = response.status();
    let status = http::StatusCode::from_u16(status_code.into())
        .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);

    let body = response.into_body().read_to_vec()?;

    Ok((status, body))
}

pub fn create_http_client() -> Arc<dyn HttpClient> {
    Arc::new(SimpleHttpClient)
}

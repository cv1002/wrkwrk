// Standard Mods
use std::{collections::HashMap, sync::Arc, time::Duration};
// External Mods
use reqwest::{
    header::{HeaderName, HeaderValue},
    Request, Version,
};
use tokio::{runtime::Runtime, task::JoinHandle, time::Instant};
// Internal Mods
pub mod httprequest;
use crate::{lua::WrkLuaVM, util::transform::Transformation, CommandLineArgs};

pub struct Client {
    id: (usize, usize),
    lua: Arc<WrkLuaVM>,
    client: reqwest::Client,
}
unsafe impl Send for Client {}
unsafe impl Sync for Client {}

impl Client {
    pub fn new(id: (usize, usize), lua: Arc<WrkLuaVM>) -> Result<Self, mlua::Error> {
        let client = reqwest::Client::new();
        Ok(Self { id, lua, client })
    }
    pub fn client_loop(
        mut self,
        runtime: &Runtime,
        args: Arc<CommandLineArgs>,
        end_time: Instant,
    ) -> JoinHandle<()> {
        runtime.spawn(async move {
            loop {
                // Request and response
                let request = self.make_request(args.as_ref()).unwrap();
                self.handle_response(request).await;
                // Release delay
                let _ = self.lua.delay();
                // At end time we end the procedure
                if Instant::now() >= end_time {
                    break;
                }
            }
        })
    }
}
// Private methods
impl Client {
    async fn handle_response(&mut self, request: Request) {
        let response = self.client.execute(request).await;
        match response {
            Ok(resp) => {
                // Get some response information
                let status = resp.status().as_u16();
                let headers = resp
                    .headers()
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
                    .collect::<HashMap<String, String>>();
                let body = resp.bytes().await.into_iter().fold(Vec::new(), |x, y| {
                    x.into_iter().chain(y.into_iter()).collect()
                });
                // Handle response function in script, if response is nil, skip
                let _ = self.lua.response(status, headers, body);
            }
            Err(_) => {}
        }
    }

    fn make_request(&mut self, args: &CommandLineArgs) -> Result<Request, mlua::Error> {
        let request = httprequest::HttpRequest::get_request(self.lua.get_vm()).unwrap();

        let method = match request.method.as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "DELETE" => reqwest::Method::DELETE,
            "PUT" => reqwest::Method::PUT,
            _ => unimplemented!(),
        };
        let url = format!(
            "{}://{}:{}/{}",
            request.scheme, request.host, request.port, request.url
        );
        let headers = {
            request
                .headers
                .iter()
                .map(|(k, v)| {
                    (
                        HeaderName::try_from(k).unwrap(),
                        HeaderValue::try_from(v).unwrap(),
                    )
                })
                .collect()
        };
        let timeout = Duration::from_millis(request.timeout.into());
        let version = match request.version.as_str() {
            // If give a version in commandline arguments
            _ if args.http10 => Version::HTTP_10,
            _ if args.http11 => Version::HTTP_11,
            _ if args.http2 => Version::HTTP_2,
            _ if args.http3 => Version::HTTP_3,
            // If don't give a version in commandline arguments
            "HTTP1.0" => Version::HTTP_10,
            "HTTP1.1" => Version::HTTP_11,
            "HTTP2" => Version::HTTP_2,
            "HTTP3" => Version::HTTP_3,
            // If nothing, use http1.1
            _ => Version::HTTP_11,
        };
        Ok(self
            .client
            .request(method, url)
            // Prepare body
            .transformation(|req| {
                if let Some(body) = request.body {
                    req.body(reqwest::Body::try_from(body).unwrap())
                } else {
                    req
                }
            })
            // Prepare other parameters
            .headers(headers)
            .timeout(timeout)
            .version(version)
            .build()
            .unwrap())
    }
}

use std::{sync::Arc, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Request, Version,
};
use tokio::time::Instant;

use crate::{
    lua::{Wrk, WrkLuaVM},
    util::transform::Transformation,
    CommandLineArgs,
};

pub struct Client {
    pub lua: Arc<WrkLuaVM>,
    pub wrk: Wrk,
    pub client: reqwest::Client,
}
unsafe impl Send for Client {}
unsafe impl Sync for Client {}

impl Client {
    pub fn new(lua: Arc<WrkLuaVM>) -> Result<Self, mlua::Error> {
        let wrk = lua.get_wrk()?;
        let client = reqwest::Client::new();
        Ok(Self { lua, wrk, client })
    }
    pub fn make_request(&mut self, args: &CommandLineArgs) -> Result<Request, mlua::Error> {
        let request = self.lua.get_request()?;

        let method = {
            let method = request.method.as_deref().unwrap();
            match method {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "DELETE" => reqwest::Method::DELETE,
                "PUT" => reqwest::Method::PUT,
                _ => unimplemented!(),
            }
        };
        let url =
            { request.host.unwrap() + &request.port.unwrap().to_string() + &request.url.unwrap() };

        let headers = {
            let headers = request.headers.unwrap();
            let mut headermap = HeaderMap::with_capacity(headers.len());
            for (k, v) in headers {
                headermap.append(
                    HeaderName::try_from(k).unwrap(),
                    HeaderValue::try_from(v).unwrap(),
                );
            }
            headermap
        };
        let timeout = { Duration::from_micros(request.timeout.unwrap().into()) };
        let version = {
            if args.http10 {
                Version::HTTP_10
            } else if args.http11 {
                Version::HTTP_11
            } else if args.http2 {
                Version::HTTP_2
            } else if args.http3 {
                Version::HTTP_3
            } else {
                match request.version.as_deref().unwrap() {
                    "HTTP1.0" => Version::HTTP_10,
                    "HTTP1.1" => Version::HTTP_11,
                    "HTTP2" => Version::HTTP_2,
                    "HTTP3" => Version::HTTP_3,
                    _ => Version::HTTP_2,
                }
            }
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
    pub async fn client_loop(mut self, args: Arc<CommandLineArgs>, end_time: Instant) {
        loop {
            let request = self.make_request(args.as_ref()).unwrap();
            self.client.execute(request).await.unwrap();
            // At end time we end the procedure
            if Instant::now() >= end_time {
                break;
            }
        }
    }
}

use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Request, Version,
};
use tokio::time::Instant;

use crate::{
    lua::{Wrk, WrkLuaVM},
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
            let method = request
                .method
                .as_deref()
                .or(self.wrk.method.as_deref())
                .unwrap_or("GET");
            match method {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "DELETE" => reqwest::Method::DELETE,
                "PUT" => reqwest::Method::PUT,
                _ => unimplemented!(),
            }
        };
        let url = {
            let url = request
                .url
                .as_deref()
                .or(self.wrk.path.as_deref())
                .unwrap_or("");
            let url = request
                .host
                .as_deref()
                .or(self.wrk.host.as_deref())
                .unwrap_or("127.0.0.1")
                .to_owned()
                + &request.port.or(self.wrk.port).unwrap_or(80).to_string()
                + url;
            url
        };
        let body = {
            reqwest::Body::try_from(
                request
                    .body
                    .clone()
                    .or(self.wrk.body.clone())
                    .unwrap_or(Vec::new()),
            )
            .unwrap()
        };
        let headers = {
            let headers = request
                .headers
                .and_then(|headers| headers.into())
                .unwrap_or(HashMap::new());
            let mut headermap = HeaderMap::with_capacity(headers.len());
            for (k, v) in headers {
                headermap.append(
                    HeaderName::try_from(k).unwrap(),
                    HeaderValue::try_from(v).unwrap(),
                );
            }
            headermap
        };
        let timeout =
            { Duration::from_micros(request.timeout.or(args.timeout).unwrap_or(30000).into()) };
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
                Version::HTTP_2
            }
        };

        Ok(self
            .client
            .request(method, url)
            .body(body)
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

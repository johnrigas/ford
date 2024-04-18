use std::fmt::Debug;

use reqwest::{IntoUrl,Method, Response, cookie::Jar};
use futures::prelude::*;
use serde::Serialize;
use std::sync::Arc;

pub async fn api<T: IntoUrl, J: Serialize + Sized + Debug>(url: T, method: Method, headers: reqwest::header::HeaderMap, query_params: Vec<(&str, &str)>, json: J, cookie_jar: Option<Jar>) -> Response {
    let client = reqwest::Client::builder()
    // .proxy(reqwest::Proxy::http("http://z011cyyytmyw37:mk8hnotgyjdtkltege949k5rkz@us-west-static-01.quotaguard.com:9293").unwrap())
    .build().unwrap();

    let url = reqwest::Url::parse_with_params(url.into_url().unwrap().as_str(), &query_params).unwrap();

    if method == Method::POST {

        client
        .request(method, url)
        .headers(headers)
        .json(&json)
        .send()
        .await.unwrap()
    } else if cookie_jar.is_some() {
        let client = reqwest::Client::builder().cookie_provider(Arc::new(cookie_jar.unwrap()))
        // .proxy(reqwest::Proxy::http("http://z011cyyytmyw37:mk8hnotgyjdtkltege949k5rkz@us-west-static-01.quotaguard.com:9293").unwrap())
        .build().unwrap();
    
        client
        .request(method, url)
        .headers(headers)
        .send()
        .await.unwrap()

    } else {
        client
        .request(method, url)
        .headers(headers)
        .send()
        .await.unwrap()
    }

}

pub async fn fetch_many(urls: Vec<String>) -> Vec<String> {
    let futures = urls.into_iter().map(|u| async move {
        api(u, Method::GET, reqwest::header::HeaderMap::new(), vec![], {}, Option::<Jar>::None).await.text().await.unwrap()
    });
    let stream = futures::stream::iter(futures).buffer_unordered(10);
    stream.collect::<Vec<_>>().await
}


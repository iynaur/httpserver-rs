#![allow(unused)]
//#![warn(unused_imports)]

mod simple_http;
use crate::simple_http::Http;

fn main() {
    let ip_port = String::from("0.0.0.0:7878");
    Http::new(ip_port);


}



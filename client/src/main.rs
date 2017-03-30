extern crate hyper;
extern crate hyper_openssl;

use hyper::{Client};
use hyper::client::{Request, Response};
use hyper::net::{HttpsConnector};
use hyper_openssl::OpensslClient;
use std::io::prelude::*;

fn https_client() -> Client {
  Client::with_connector(
    HttpsConnector::new(
      OpensslClient::new().expect("Can't initialize ssl")))
}

fn main() {
  let mut c = https_client();
  let url = "https://rustsslserver/";
  let mut req = c.get(url);
  let mut resp = req.send().expect("Sending request failed");
  println!("response status was {:?}", resp.status.to_u16());
  let mut buf = String::new();
  resp.read_to_string(&mut buf).expect("Can't read response string");
  println!("response body was:\n{}", buf);
}

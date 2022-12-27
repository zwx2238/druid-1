use lazy_static::lazy_static;
use reqwest::blocking::RequestBuilder;

lazy_static! {
    pub static ref CLIENT : reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub fn post(path: impl Into<String>) -> RequestBuilder {
    CLIENT.post(format!("http://127.0.0.1:10001/{}", path.into()))
}

pub fn get_druid(path: impl Into<String>) -> RequestBuilder {
    CLIENT.get(format!("http://127.0.0.1:12480/{}", path.into()))
}
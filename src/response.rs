use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub main: Main
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f32
}

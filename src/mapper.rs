use crate::dto::Response;

#[derive(Default, Clone, Copy)]
pub struct Mapper {}

impl Mapper {

    pub fn map_to_response(&self, source: Vec<String>) -> Response {
        let str = source.iter().map(|x| x.to_string()).collect::<String>();
        serde_json::from_str(str.as_str()).unwrap()
    }
}
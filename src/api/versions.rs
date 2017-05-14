use modifiers::Jsonify;
use iron::{IronResult,Handler,Request,Response,status};

#[derive(Debug,RustcEncodable)]
pub struct Versions {
    ver: Vec<&'static str>,
}

impl Versions {
    pub fn versions() -> Self {
        Versions {
            ver: vec![
                "v1.0"
            ],
        }
    }
}

impl Handler for Versions {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Jsonify(self))))
    }
}

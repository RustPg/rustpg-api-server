use iron::Response;
use iron::modifier::Modifier;
use iron::headers::ContentType;
use rustc_serialize::{json,Encodable};

pub struct Jsonify<T: Encodable>(pub T);

impl<T: Encodable> Modifier<Response> for Jsonify<T> {
    fn modify(self, resp: &mut Response) {
        resp.headers.set(ContentType::json());
        resp.body = Some(Box::new(json::encode(&self.0)
            .expect("Jsonify: invalid encode")));
    }
}


use super::{ Attendance, HttpsClientBuilder };
use rustc_serialize::json;
use hyper::header::{Headers, ContentType};

pub struct AttendanceApi;

impl AttendanceApi {

    pub fn register(&self, entity: &Attendance) {

        let url = "http://localhost:3000/attendances";
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        let json = json::encode(entity).unwrap();

        let _ = HttpsClientBuilder::build()
            .post(url)
            .headers(headers)
            .body(&*json)
            .send();
    }
}


use super::{ Attendance, HttpsClientBuilder };
use rustc_serialize::json;
use hyper::header::{Headers, ContentType};
use std::io::Read;

pub struct AttendanceApi;

impl AttendanceApi {

    pub fn register(&self, entity: &Attendance) {

        let url = "http://localhost:3000/attendances";
        let json = json::encode(entity).unwrap();
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        HttpsClientBuilder::build()
            .post(url)
            .headers(headers)
            .body(&*json)
            .send();
    }
}

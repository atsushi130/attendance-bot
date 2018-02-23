
use super::{ Attendance, HttpsClientBuilder };
use rustc_serialize::json;
use std::io::Read;

pub struct AttendanceApi;

impl AttendanceApi {

    pub fn register(&self, entity: &Attendance) {

        let url = "http://localhost:8000/attendances";
        let mut response = String::new();

        let json = json::encode(entity).unwrap();

        HttpsClientBuilder::build()
            .post(url)
            .body(&*json)
            .send();
    }
}

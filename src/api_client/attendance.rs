
#[derive(RustcEncodable)]
#[derive(Debug)]
pub struct Attendance {
    user: String,
    date: String,
    type_string: String
}

impl Attendance {
    pub fn from(user: &str, date: &str, type_string: &str) -> Self {
        Attendance {
            user: user.to_string(),
            date: date.to_string(),
            type_string: type_string.to_string()
        }
    }
}
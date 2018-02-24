
#[derive(RustcEncodable)]
#[derive(Debug)]
pub struct Attendance {
    user: String,
    check_at: String,
    attendance_type: i64
}

impl Attendance {
    pub fn from(user: &str, check_at: &str, attendance_type: &i64) -> Self {
        Attendance {
            user: user.to_string(),
            check_at: check_at.to_string(),
            attendance_type: *attendance_type
        }
    }
}
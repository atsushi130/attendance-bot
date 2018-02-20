
pub enum AttendanceType {
    CheckIn,
    CheckOut
}

impl AttendanceType {
    pub fn from(value: &str) -> Self {
        match value {
            "出勤" => AttendanceType::CheckIn,
            "退勤" => AttendanceType::CheckOut,
            _     => AttendanceType::CheckIn
        }
    }
}
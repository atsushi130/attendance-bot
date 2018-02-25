
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum AttendanceType {
    CheckIn,
    CheckOut,
    Unknown
}

impl AttendanceType {

    pub fn new(value: &str) -> Self {
        match value {
            "出勤" | "yo"  | "checkin"  => AttendanceType::CheckIn,
            "退勤" | "bye" | "checkout" => AttendanceType::CheckOut,
            _ => AttendanceType::Unknown
        }
    }

    pub fn from_id(id: i64) -> Self {
        match id {
            0 => AttendanceType::CheckIn,
            1 => AttendanceType::CheckOut,
            _ => AttendanceType::Unknown
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            AttendanceType::CheckIn  => "出勤".to_string(),
            AttendanceType::CheckOut => "退勤".to_string(),
            AttendanceType::Unknown  => "不正な打刻です".to_string()
        }
    }
}
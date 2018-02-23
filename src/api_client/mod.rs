
mod attendance_api;
pub use self::attendance_api::AttendanceApi;

mod attendance;
pub use self::attendance::Attendance;

mod https_client;
use self::https_client::HttpsClientBuilder;
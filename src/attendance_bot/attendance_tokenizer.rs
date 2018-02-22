
pub struct AttendanceTokenizer;

impl AttendanceTokenizer {
    pub fn tokenize(&self, string: &str) -> (String, String) {

        let splitted: Vec<&str> = string.split(": ").collect();

        if let Some(user) = splitted.first() {
            if let Some(token) = splitted.last() {
                return (user.to_string(), token.to_string())
            }
        }

        ("".to_string(), "".to_string())
    }
}

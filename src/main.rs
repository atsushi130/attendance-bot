
extern crate slack;
use slack::RtmClient;

extern crate chrono;

mod attendance_bot;
use attendance_bot::AttendanceBot;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let token = match args.len() {
        0 | 1 => panic!("token not found."),
        x => args[x - 1].clone(),
    };

    connect(&token);
}

fn connect(token: &str) {

    let mut handler = AttendanceBot::from();
    let r = RtmClient::login_and_run(&token, &mut handler);
    match r {
        Ok(_) => {}
        Err(_) => connect(token)
    }
}
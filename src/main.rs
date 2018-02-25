
extern crate slack;
use slack::RtmClient;

extern crate hyper;
extern crate hyper_native_tls;
extern crate chrono;
extern crate rustc_serialize;

mod attendance_bot;
use attendance_bot::AttendanceBot;

mod api_client;
mod extensions;

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let token = match args.len() {
        0 | 1 => panic!("token not found."),
        x => args[x - 1].clone(),
    };

    connect(&token);
}

fn connect(token: &str) {

    let mut handler = AttendanceBot::new();
    let r = RtmClient::login_and_run(&token, &mut handler);
    match r {
        Ok(_) => {}
        Err(_) => connect(token)
    }
}
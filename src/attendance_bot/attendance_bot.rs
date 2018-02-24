
use slack::{ Event, EventHandler, RtmClient };
use chrono::Local;
use std::marker::PhantomData;
use super::{ AttendanceType, AttendanceTokenizer };
use api_client::{ AttendanceApi, Attendance };
use extensions::DateTimeToString;

pub struct AttendanceBot<'a> {
    phantom: PhantomData<&'a str>
}


impl<'a> AttendanceBot<'a> {

    const NAME: &'a str = "@attendance-bot";

    pub fn from<'b>() -> AttendanceBot<'b> {
        AttendanceBot {
            phantom: PhantomData
        }
    }

    fn to_me(&self, text: &str) -> bool {
        text.contains(AttendanceBot::NAME)
    }

    fn analyze_event(&self, event: Event) -> (Option<String>, Option<String>) {

        if let Event::DesktopNotification { content, channel, .. } = event {
            return (content, channel)
        }

        (None, None)
    }

    fn send(&self, client: &RtmClient, channel: &str, message: &str) {
        let _ = client.sender().send_message(channel, message);
    }
}

impl<'a> EventHandler for AttendanceBot<'a> {

    fn on_event(&mut self, cli: &RtmClient, event: Event) {

        let (maybe_message, maybe_channel) = self.analyze_event(event);

        maybe_message.iter()
            .zip(maybe_channel.iter())
            .filter(|&(message, _)| self.to_me(message.as_str()))
            .map(|(message, channel)| (message.replace(format!("{} ", AttendanceBot::NAME).as_str(), ""), channel))
            .map(|(type_string, channel)| (AttendanceTokenizer.tokenize(&type_string), channel))
            .map(|((user, type_string), channel)| (user, AttendanceType::from(&type_string), channel))
            .filter(|&(_, ref attendance_type, _)| *attendance_type != AttendanceType::Unknown)
            .for_each(|(user, attendance_type, channel)| {

                // api request
                let attendance = Attendance::from(&user, &Local::now().to_format_string(), &(attendance_type as i64));
                AttendanceApi.register(&attendance);

                // slack send
                let message = format!("@{} さんが {} に *{}* しました！", user, Local::now().to_format_string(), attendance_type.to_string());
                self.send(cli, channel, message.as_str())
            })
    }

    fn on_close(&mut self, _: &RtmClient) {}
    fn on_connect(&mut self, _: &RtmClient) {}
}
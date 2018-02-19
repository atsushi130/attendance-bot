
use slack::{ Event, EventHandler, RtmClient };
use std::marker::PhantomData;
use super::AttendanceType;

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

impl<'a> EventHandler for TabelogBot<'a> {

    fn on_event(&mut self, cli: &RtmClient, event: Event) {

        let (maybe_message, maybe_channel) = self.analyze_event(event);

        maybe_message.iter()
            .zip(maybe_channel.iter())
            .filter(|&(message, _)| self.to_me(message.as_str()))
            .map(|(message, channel)| (message.replace(format!("{} ", TabelogBot::NAME).as_str(), ""), channel))
    }

    fn on_close(&mut self, _: &RtmClient) {}
    fn on_connect(&mut self, _: &RtmClient) {}
}
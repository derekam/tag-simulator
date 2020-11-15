use iced::{Element, Row, Button, Text, Align, button, Length};
use crate::iced_ui::Message;

#[derive(Default)]
pub(crate) struct Controls {
    toggle_button: button::State,
    next_button: button::State,
    reset_button: button::State,
}

impl Controls {

    pub fn view(&mut self,
            is_playing: bool) -> Element<Message> {
        let playback_controls = Row::new()
            .spacing(10)
            .push(
                Button::new(
                    &mut self.toggle_button,
                    Text::new(if is_playing { "Pause" } else { "Play" }),
                )
                    .on_press(Message::TogglePlayback)
            )
            .push(
                Button::new(&mut self.next_button, Text::new("Next"))
                    .on_press(Message::Next)
            );

        Row::new()
            .padding(10)
            .spacing(20)
            .align_items(Align::Center)
            .push(playback_controls)
            .push(
                Button::new(&mut self.reset_button, Text::new("Reset"))
                    .on_press(Message::Reset)
            )
            .height(Length::Units(50))
            .into()
    }

}
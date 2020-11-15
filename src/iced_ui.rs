use iced::{Application, Command, Element, executor, Column, Container, Length, Subscription, Canvas};
use crate::simulation::Simulation;
use crate::time;
use std::time::{Instant};
use crate::environment::Environment;
use crate::parameters::TagParams;
use crate::agents::agent::Agent;

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    TogglePlayback,
    Next,
    Reset,
}

impl<X: Agent> Application for Simulation<X> {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = TagParams;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Simulation::new(flags),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Tag Simulation")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        log::debug!("Update called: {:?}.", message);

        match message {
            Message::Tick(_) | Message::Next => {
                self.step();
            }
            Message::TogglePlayback => {
                self.is_running = !self.is_running;
            }
            Message::Reset => {
                self.environment.reset(self.parameters);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_running {
            // throttle to update less frequently under load/large numbers of agents
            time::every(std::time::Duration::from_millis(((self.parameters.num_players / 50) * 10) as u64))
                .map(|instant| Message::Tick(instant))
        } else {
            Subscription::none()
        }
    }

    fn view(&mut self) -> Element<Message> {
        let controls = self.controls.view(self.is_running);

        let content = Column::new()
            .push(Canvas::new()
                      .width(Length::Fill)
                      .height(Length::Fill)
                      .push(self.environment.clone()),
            )
            .push(controls);

        Container::new(content)
            .width(Length::Units(self.environment.width as u16))
            .height(Length::Units(self.environment.height as u16 + 50))
            .center_x()
            .center_y()
            .into()
    }

}
use iced::{Sandbox, widget::{button, row, text, container}, Length};

pub struct App {
    counter_value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementClicked,
    DecrementClicked,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            counter_value: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementClicked => self.counter_value += 1,
            Message::DecrementClicked => self.counter_value -= 1,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let increment_button = button("increment").on_press(Message::IncrementClicked);
        let decrement_button = button("decrement").on_press(Message::DecrementClicked);
        let text = text(self.counter_value);

        let content = row![
            decrement_button,
            text,
            increment_button,
        ]
        .spacing(20)
        .padding(100);

        container(content).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
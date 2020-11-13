/* 
    Usage: everything that needs rendering goes in here
*/
use iced::{button, Element, Button, Sandbox, Settings, Text, Column, Align};

pub fn draw_window() {
    Window::run(Settings::default()) 
}

#[derive(Default)]
struct Window {
    value: i32,
    increment_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
}

impl Sandbox for Window {
    type Message = Message;

    fn new() -> Window {
        Window::default()
    }

    fn title(&self) -> String {
        String::from("Rustcord GUI test")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
        Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(
            Text::new(self.value.to_string()).size(50)
            )
            .into()
    }
}
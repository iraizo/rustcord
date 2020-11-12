/* 
    Usage: everything that needs rendering goes in here
*/
use iced::{Element, Sandbox, Settings, Text};

pub fn draw_window() {
    Window::run(Settings::default())
}

struct Window;

impl Sandbox for Window {
    type Message = ();

    fn new() -> Window {
        Window
    }

    fn title(&self) -> String {
        String::from("test")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("test").into()
    }
}
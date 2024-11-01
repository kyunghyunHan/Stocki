use iced::{
    widget::{button, canvas::Canvas, container, pick_list, Column, Container},
    Element, Length,
};
use std::time::Instant;
use stocki::plot::{Candlestick, Chart};

#[derive(Debug, Clone)]
pub enum CustomEvent {
    ScrollRight,
}
#[derive(Debug, Clone)]
pub enum Message {
    AddCandlestick,
    RemoveCandlestick,
    FruitSelected(Fruit),
    Tick(Instant),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Orange,
    Strawberry,
    Tomato,
}

impl ToString for Fruit {
    fn to_string(&self) -> String {
        match self {
            Fruit::Apple => "Apple".to_string(),
            Fruit::Orange => "Orange".to_string(),
            Fruit::Strawberry => "Strawberry".to_string(),
            Fruit::Tomato => "Tomato".to_string(),
        }
    }
}

#[derive(Default)]
struct Stocki {
    candlesticks: Vec<Candlestick>,
    selected_option: Option<Fruit>,
    auto_scroll: bool,
    scroll_offset: f32,
}

impl Stocki {
    pub fn new() -> Self {
        Self {
            candlesticks: vec![Candlestick {
                open: 100.0,
                close: 110.0,
                high: 115.0,
                low: 95.0,
            }],
            selected_option: None,
            auto_scroll: true,
            scroll_offset: 0.0,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(Chart::new(self.candlesticks.clone()))
            .width(Length::Fill)
            .height(Length::from(300));

        let fruits = [
            Fruit::Apple,
            Fruit::Orange,
            Fruit::Strawberry,
            Fruit::Tomato,
        ];

        Column::new()
            .push(
                pick_list(fruits, self.selected_option, Message::FruitSelected)
                    .placeholder("Select your favorite fruit..."),
            )
            .push(button("Add Candlestick").on_press(Message::AddCandlestick))
            .push(button("Remove Candlestick").on_press(Message::RemoveCandlestick))
            .push(
                Container::new(canvas)
                    .width(Length::from(300))
                    .height(Length::from(300))
                    .padding(20)
                    .style(container::rounded_box),
            )
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddCandlestick => {
                let last_close = self.candlesticks.last().map(|c| c.close).unwrap_or(100.0);
                let open = last_close;
                let close = open + (rand::random::<f32>() - 0.5) * 20.0;
                let high = open.max(close) + rand::random::<f32>() * 10.0;
                let low = open.min(close) - rand::random::<f32>() * 10.0;

                self.candlesticks.push(Candlestick {
                    open,
                    close,
                    high,
                    low,
                });
                self.auto_scroll = true;
            }
            Message::RemoveCandlestick => {
                self.candlesticks.pop();
                self.auto_scroll = true;
            }
            Message::FruitSelected(fruit) => {
                self.selected_option = Some(fruit);
            }
            Message::Tick(_) => {
                if self.auto_scroll {
                    self.scroll_offset += 0.5;
                }
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("Candlestick Chart", Stocki::update, Stocki::view)
}

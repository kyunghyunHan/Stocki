// use iced::{
//     mouse,
//     widget::{
//         button, canvas,
//         canvas::{
//             event::{self, Event},
//             Canvas, Program,
//         },
//         column, container, pick_list, text, Column, Container, PickList,
//     },
//     Color, Element, Length, Point, Rectangle, Size, Subscription, time,
// };
// use std::time::Instant;

// #[derive(Debug, Clone)]
// pub enum Message {
//     AddCandlestick,
//     RemoveCandlestick,
//     FruitSelected(Fruit),
//     Tick(Instant),
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Fruit {
//     Apple,
//     Orange,
//     Strawberry,
//     Tomato,
// }

// impl ToString for Fruit {
//     fn to_string(&self) -> String {
//         match self {
//             Fruit::Apple => "Apple".to_string(),
//             Fruit::Orange => "Orange".to_string(),
//             Fruit::Strawberry => "Strawberry".to_string(),
//             Fruit::Tomato => "Tomato".to_string(),
//         }
//     }
// }

// #[derive(Default)]
// struct Counter {
//     candlesticks: Vec<Candlestick>,
//     selected_option: Option<Fruit>,
//     auto_scroll: bool,
//     scroll_offset: f32,
// }

// #[derive(Debug, Clone)]
// struct Candlestick {
//     open: f32,
//     close: f32,
//     high: f32,
//     low: f32,
// }

// #[derive(Default)]
// struct ChartState {
//     offset: f32,
//     dragging: bool,
//     drag_start: Point,
//     last_offset: f32,
//     auto_scroll: bool,
//     scroll_offset: f32,
// }

// struct Chart {
//     candlesticks: Vec<Candlestick>,
//     state: ChartState,
// }

// impl Chart {
//     fn new(candlesticks: Vec<Candlestick>) -> Self {
//         Self {
//             candlesticks,
//             state: ChartState {
//                 auto_scroll: true,
//                 scroll_offset: 0.0,
//                 ..ChartState::default()
//             },
//         }
//     }
// }

// impl Counter {
//     pub fn new() -> Self {
//         Self {
//             candlesticks: vec![
//                 Candlestick {
//                     open: 100.0,
//                     close: 110.0,
//                     high: 115.0,
//                     low: 95.0,
//                 },
//             ],
//             selected_option: None,
//             auto_scroll: true,
//             scroll_offset: 0.0,
//         }
//     }

//     pub fn subscription(&self) -> Subscription<Message> {
//         time::every(std::time::Duration::from_millis(16)).map(Message::Tick)
//     }

//     pub fn view(&self) -> Element<Message> {
//         let canvas = Canvas::new(Chart::new(self.candlesticks.clone()))
//             .width(Length::Fill)
//             .height(Length::from(500));

//         let fruits = [
//             Fruit::Apple,
//             Fruit::Orange,
//             Fruit::Strawberry,
//             Fruit::Tomato,
//         ];

//         Column::new()
//             .push(
//                 pick_list(fruits, self.selected_option, Message::FruitSelected)
//                     .placeholder("Select your favorite fruit..."),
//             )
//             .push(button("Add Candlestick").on_press(Message::AddCandlestick))
//             .push(button("Remove Candlestick").on_press(Message::RemoveCandlestick))
//             .push(
//                 Container::new(canvas)
//                     .width(Length::Fill)
//                     .height(Length::from(500))
//                     .padding(20),
//             )
//             .into()
//     }

//     pub fn update(&mut self, message: Message) {
//         match message {
//             Message::AddCandlestick => {
//                 let last_close = self.candlesticks.last().map(|c| c.close).unwrap_or(100.0);
//                 let open = last_close;
//                 let close = open + (rand::random::<f32>() - 0.5) * 20.0;
//                 let high = open.max(close) + rand::random::<f32>() * 10.0;
//                 let low = open.min(close) - rand::random::<f32>() * 10.0;

//                 self.candlesticks.push(Candlestick {
//                     open,
//                     close,
//                     high,
//                     low,
//                 });
//                 self.auto_scroll = true;
//             }
//             Message::RemoveCandlestick => {
//                 self.candlesticks.pop();
//                 self.auto_scroll = true;
//             }
//             Message::FruitSelected(fruit) => {
//                 self.selected_option = Some(fruit);
//             }
//             Message::Tick(_) => {
//                 if self.auto_scroll {
//                     self.scroll_offset += 0.5;
//                 }
//             }
//         }
//     }
// }

// impl<Message> Program<Message> for Chart {
//     type State = ChartState;

//     fn update(
//         &self,
//         state: &mut Self::State,
//         event: Event,
//         bounds: Rectangle,
//         cursor: mouse::Cursor,
//     ) -> (event::Status, Option<Message>) {
//         let cursor_position = if let Some(position) = cursor.position() {
//             position
//         } else {
//             return (event::Status::Ignored, None);
//         };
        
//         match event {
//             Event::Mouse(mouse_event) => match mouse_event {
//                 mouse::Event::ButtonPressed(mouse::Button::Left) => {
//                     state.dragging = true;
//                     state.drag_start = cursor_position;
//                     state.last_offset = state.offset;
//                     state.auto_scroll = false;
//                     (event::Status::Captured, None)
//                 }
//                 mouse::Event::ButtonReleased(mouse::Button::Left) => {
//                     state.dragging = false;
//                     (event::Status::Captured, None)
//                 }
//                 mouse::Event::CursorMoved { .. } => {
//                     if state.dragging {
//                         let delta_x = cursor_position.x - state.drag_start.x;
//                         state.offset = state.last_offset + delta_x;
//                         (event::Status::Captured, None)
//                     } else {
//                         (event::Status::Ignored, None)
//                     }
//                 }
//                 _ => (event::Status::Ignored, None),
//             },
//             _ => (event::Status::Ignored, None),
//         }
//     }

//     fn draw(
//         &self,
//         state: &Self::State,
//         renderer: &iced::Renderer,
//         _theme: &iced::Theme,
//         bounds: Rectangle,
//         _cursor: mouse::Cursor,
//     ) -> Vec<canvas::Geometry> {
//         let mut frame = canvas::Frame::new(renderer, bounds.size());
//         if self.candlesticks.is_empty() {
//             return vec![frame.into_geometry()];
//         }

//         let price_range = self
//             .candlesticks
//             .iter()
//             .fold((f32::MAX, f32::MIN), |acc, c| {
//                 (acc.0.min(c.low), acc.1.max(c.high))
//             });

//         let price_diff = price_range.1 - price_range.0;
//         let y_scale = (bounds.height - 40.0) / price_diff;

//         let fixed_candle_width = 20.0;
//         let body_width = fixed_candle_width * 0.8;
//         let total_width = fixed_candle_width * self.candlesticks.len() as f32;

//         let scroll_amount = if state.auto_scroll { state.scroll_offset } else { 0.0 };
//         let start_x = if total_width > bounds.width {
//             20.0 - scroll_amount + state.offset
//         } else {
//             20.0 + state.offset
//         };

//         for (i, candlestick) in self.candlesticks.iter().enumerate() {
//             let x = start_x + (i as f32 * fixed_candle_width);

//             if x < -fixed_candle_width || x > bounds.width {
//                 continue;
//             }

//             let scale_price =
//                 |price: f32| -> f32 { bounds.height - 20.0 - ((price - price_range.0) * y_scale) };
//             let open_y = scale_price(candlestick.open);
//             let close_y = scale_price(candlestick.close);
//             let high_y = scale_price(candlestick.high);
//             let low_y = scale_price(candlestick.low);
            
//             let color = if candlestick.close >= candlestick.open {
//                 Color::from_rgb(0.0, 0.8, 0.0)
//             } else {
//                 Color::from_rgb(0.8, 0.0, 0.0)
//             };

//             frame.stroke(
//                 &canvas::Path::new(|builder| {
//                     let center_x = x + (body_width / 2.0);
//                     builder.move_to(Point::new(center_x, high_y));
//                     builder.line_to(Point::new(center_x, low_y));
//                 }),
//                 canvas::Stroke::default().with_color(color).with_width(1.0),
//             );

//             let body_height = (close_y - open_y).abs();
//             let body_y = close_y.min(open_y);
//             frame.fill_rectangle(
//                 Point::new(x, body_y),
//                 Size::new(body_width, body_height),
//                 color,
//             );
//         }

//         vec![frame.into_geometry()]
//     }
// }

// fn main() -> iced::Result {
//     Counter::run(iced::Settings::default())
// }
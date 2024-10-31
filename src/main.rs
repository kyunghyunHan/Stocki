use iced::{
    widget::{
        button, canvas,
        canvas::{Canvas, Program},
        column, container, pick_list, text, Column, Container, PickList,
    },
    Color, Element, Length, Point, Rectangle, Size,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    AddCandlestick,
    RemoveCandlestick,
    FruitSelected(Fruit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Orange,
    Strawberry,
    Tomato,
}

#[derive(Default)]
struct Counter {
    candlesticks: Vec<Candlestick>,
    selected_option: Option<Fruit>,
}

#[derive(Debug, Clone)]
struct Candlestick {
    open: f32,
    close: f32,
    high: f32,
    low: f32,
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Apple => "Apple",
            Self::Orange => "Orange",
            Self::Strawberry => "Strawberry",
            Self::Tomato => "Tomato",
        })
    }
}

impl Counter {
    pub fn new() -> Self {
        Self {
            candlesticks: vec![
                Candlestick {
                    open: 100.0,
                    close: 110.0,
                    high: 115.0,
                    low: 95.0,
                },
                // 초기 데이터 추가
            ],
            selected_option: None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(Chart {
            candlesticks: self.candlesticks.clone(),
        })
        .width(Length::Fill)
        .height(Length::from(500));

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
                    .width(Length::Fill)
                    .height(Length::from(500))
                    .padding(20),
            )
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddCandlestick => {
                let last_close = self.candlesticks
                    .last()
                    .map(|c| c.close)
                    .unwrap_or(100.0);
                
                // 새로운 캔들스틱 생성 (랜덤한 변동 추가)
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
            }
            Message::RemoveCandlestick => {
                self.candlesticks.pop();
            }
            Message::FruitSelected(fruit) => {
                self.selected_option = Some(fruit);
            }
        }
    }
}

struct Chart {
    candlesticks: Vec<Candlestick>,
}

impl<Message> Program<Message> for Chart {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        if self.candlesticks.is_empty() {
            return vec![frame.into_geometry()];
        }

        // 차트의 스케일 계산
        let price_range = self.candlesticks.iter().fold((f32::MAX, f32::MIN), |acc, c| {
            (
                acc.0.min(c.low),
                acc.1.max(c.high),
            )
        });
        
        let price_diff = price_range.1 - price_range.0;
        let y_scale = (bounds.height - 40.0) / price_diff;
        let candle_width = (bounds.width - 40.0) / self.candlesticks.len() as f32;
        let body_width = candle_width * 0.8;

        // 각 캔들스틱 그리기
        for (i, candlestick) in self.candlesticks.iter().enumerate() {
            let x = 20.0 + (i as f32 * candle_width);
            
            // Y 좌표 계산 (차트 하단이 원점)
            let scale_price = |price: f32| -> f32 {
                bounds.height - 20.0 - ((price - price_range.0) * y_scale)
            };

            let open_y = scale_price(candlestick.open);
            let close_y = scale_price(candlestick.close);
            let high_y = scale_price(candlestick.high);
            let low_y = scale_price(candlestick.low);

            // 캔들 색상 결정
            let color = if candlestick.close >= candlestick.open {
                Color::from_rgb(0.0, 0.8, 0.0) // 상승봉
            } else {
                Color::from_rgb(0.8, 0.0, 0.0) // 하락봉
            };

            // 심지 그리기
            frame.stroke(
                &canvas::Path::new(|builder| {
                    let center_x = x + (body_width / 2.0);
                    builder.move_to(Point::new(center_x, high_y));
                    builder.line_to(Point::new(center_x, low_y));
                }),
                canvas::Stroke::default().with_color(color).with_width(1.0),
            );

            // 몸통 그리기
            let body_height = (close_y - open_y).abs();
            let body_y = close_y.min(open_y);
            
            frame.fill_rectangle(
                Point::new(x, body_y),
                Size::new(body_width, body_height),
                color,
            );
        }

        vec![frame.into_geometry()]
    }
}

fn main() -> iced::Result {
    let mut counter = Counter::new();
    iced::run("Candlestick Chart", Counter::update, Counter::view)
}
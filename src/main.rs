use iced::{
    widget::{
        button, canvas,
        canvas::{Canvas, Program},
        column, container, pick_list, text, Column, Container, PickList,
    },
    Color, Element, Length,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
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
    value: i32,
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
    pub fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(Chart {
            candlesticks: self.candlesticks.clone(),
        })
        .width(Length::Fill)
        .height(Length::from(500)); // 캔버스 높이 지정
        let dropdown_options = vec!["Option 1", "Option 2", "Option 3"];
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
            .push(button("+").on_press(Message::Increment))
            .push(text(self.value).size(50))
            .push(button("-").on_press(Message::Decrement))
            .push(
                Container::new(canvas)
                    .width(Length::Fill)
                    .center(800)
                    .height(Length::from(500))
                    .padding(20) // 패딩 추가
                    .style(container::rounded_box), // 사용자 정의 스타일 적용
            ) // 캔버스 높이 지정
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                let new_value = self.value + 1;
                self.value = new_value;

                // 새 캔들스틱 추가
                self.candlesticks.push(Candlestick {
                    open: new_value as f32,
                    close: new_value as f32,
                    high: new_value as f32 + 1.0,
                    low: new_value as f32 - 1.0,
                });
            }
            Message::Decrement => {
                let new_value = self.value - 1;
                self.value = new_value;

                // 새 캔들스틱 추가
                self.candlesticks.push(Candlestick {
                    open: new_value as f32,
                    close: new_value as f32,
                    high: new_value as f32 + 1.0,
                    low: new_value as f32 - 1.0,
                });
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
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        if self.candlesticks.is_empty() {
            return vec![frame.into_geometry()]; // 캔들스틱이 없을 경우 빈 프레임 반환
        }

        for (i, candlestick) in self.candlesticks.iter().enumerate() {
            let x = i as f32 * 20.0; // x 좌표 간격
            let open_y = bounds.height - candlestick.open * 10.0; // 스케일 조정
            let close_y = bounds.height - candlestick.close * 10.0; // 스케일 조정
            let high_y = bounds.height - candlestick.high * 10.0; // 스케일 조정
            let low_y = bounds.height - candlestick.low * 10.0; // 스케일 조정

            // 캔들 몸체 그리기
            let body_top = close_y.min(open_y);
            let body_bottom = close_y.max(open_y);
            let body_color = if candlestick.close >= candlestick.open {
                Color::from_rgb(0.0, 1.0, 0.0) // 상승은 초록색
            } else {
                Color::from_rgb(1.0, 0.0, 0.0) // 하락은 빨간색
            };

            // 몸체 그리기
            frame.fill_rectangle(
                iced::Point::new(150., 150.),
                iced::Size::new(15.0, 100.),
                Color::from_rgb(0.0, 255.0, 0.0),
            );

            // 고가와 저가 선 그리기
            frame.stroke(
                &canvas::Path::new(|builder| {
                    builder.move_to(iced::Point::new(x + 7.5, high_y));
                    builder.line_to(iced::Point::new(x + 7.5, low_y));
                }),
                canvas::Stroke {
                    style: canvas::Style::Solid(Color::from_rgb(0.0, 0.0, 255.0)),
                    width: 1.0,
                    line_cap: canvas::LineCap::Round,
                    line_join: canvas::LineJoin::Round,
                    line_dash: canvas::LineDash::default(),
                },
            );
        }

        vec![frame.into_geometry()]
    }
}

fn main() -> iced::Result {
    iced::run("A simple candlestick chart", Counter::update, Counter::view)
}

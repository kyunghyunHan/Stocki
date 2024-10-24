#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs;
use yahoo_finance_api as yahoo;
use rayon::prelude::*; // rayon을 가져옵니다.

use yahoo_finance_api::time::{macros::datetime, OffsetDateTime};
use yahoo_finance_api::Quote;

use eframe::egui::{self, Color32, Stroke};
use std::net::UdpSocket; // `egui_plot` crate를 import
use tokio_test;
use egui_plot::Bar;
//lib
use stocki::plot::candle;
// #[derive(Deserialize, Debug)]
// struct Candle {
//     #[serde(rename = "candle_date_time_utc")]
//     candle_date_time_utc: String,
//     #[serde(rename = "opening_price")]
//     opening_price: f64,
//     #[serde(rename = "high_price")]
//     high_price: f64,
//     #[serde(rename = "low_price")]
//     low_price: f64,
//     #[serde(rename = "trade_price")]
//     trade_price: f64,
// }
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([980.0, 900.0]), // 사이드바를 고려하여 크기를 조정합니다.
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    socket: UdpSocket,
    destination: String,
    current_screen: Screen,
    files: Vec<String>,
    button_text: String,       // 초기 텍스트
    stocks: Vec<&'static str>, // 주식 이름을 저장할 벡터
    stock_data: Vec<Bar>,
}
pub  fn fetch_quotes() -> Vec<Bar> {
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(2023-12-20 0:00:00.00 UTC);
    let end = datetime!(2024-1-1 23:59:59.99 UTC);

    let resp = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1d")).unwrap();
    let quotes = resp.quotes().unwrap();
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);

    quotes.into_par_iter().map(|quote| {
        let open = quote.open;
        let close = quote.close;
        let volume = quote.volume;
        let color = if close >= open { green } else { red };
        let timestamp = quote.timestamp as f64;

        Bar::from(Bar {
            name: "a".to_string(),
            orientation: egui_plot::Orientation::Vertical,
            argument: timestamp,
            value: volume as f64,
            base_offset: Some(0.),
            bar_width: 20.,
            stroke: Stroke::new(1.0, color),
            fill: color,
        })
    }).collect()
}

//  fn get_data() -> Vec<Quote> {
//     let provider = yahoo::YahooConnector::new().unwrap();
//     let start = datetime!(2023-12-20 0:00:00.00 UTC);
//     let end = datetime!(2024-1-1 23:59:59.99 UTC);
//     let resp = tokio_test::block_on(provider.get_quote_history("AAPL", start, end)).unwrap();

//     // let resp = provider
//     //     .get_quote_history("AAPL", start, end)
//     //     .await
//     //     .unwrap();
//     let quotes = resp.quotes().unwrap();
//     quotes
// }
impl Default for MyApp {
    fn default() -> Self {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("Couldn't bind to address");

        Self {
            name: "Arthur".to_owned(),
            age: 42,
            socket,
            destination: "127.0.0.1:8080".to_owned(),
            current_screen: Screen::Main,
            // stock_chart: StockChart::default(), // StockChart 초기화
            files: Vec::new(),
            button_text: "Stocks".to_string(), // 초기 텍스트
            stocks: vec!["GOOG", "GOOG2"],     // 초기 주식 이름
            stock_data: fetch_quotes(),            // 빈 벡터로 초기화
        }
    }
}

enum Screen {
    Main,
    Secondary,
    Tertiary,
}

struct StockChart {
    prices: Vec<f64>,
}

impl Default for StockChart {
    fn default() -> Self {
        // 예시로 랜덤한 주식 가격 데이터를 생성합니다.
        let prices = (0..100)
            .map(|i| (i as f64, (i as f64 * 0.1).sin() * 10.0 + 50.0))
            .map(|(_, y)| y)
            .collect();
        Self { prices }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("이거");
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        egui::Window::new("Stock Picker").show(ctx, |ui| {
            println!("{}", "힝");
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().text_edit_width = 50.;
                ui.label("Stock:");
                // ui.text_edit_singleline(&mut self.stock);
            });
            ui.horizontal(|ui| {
                if ui.button("PICK").clicked() {
                    // self.stocks_map.lock().unwrap().insert(self.stock.clone(),
                    //     Arc::new(Mutex::new(Stock::default(&self.stock))));
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.horizontal(|ui| {
            //     ui.heading("Main Screen");
            // });
            // ui.separator();

            ui.horizontal(|ui| {
                // 사이드바를 포함하는 좌측 영역
                ui.vertical(|ui| {
                    if ui.button("Main Screen").clicked() {
                        self.current_screen = Screen::Main;
                    }
                    if ui.button("Chart").clicked() {
                        self.current_screen = Screen::Secondary;
                    }
                    if ui.button("Tertiary Screen").clicked() {
                        self.current_screen = Screen::Tertiary;
                    }
                });

                ui.separator();

                // 메인 화면 내용 표시
                ui.vertical(|ui| match self.current_screen {
                    Screen::Main => {
                        // ui.heading("Main Screen");
                        ui.horizontal(|ui| {
                            let name_label = ui.label("Your name: ");
                            ui.text_edit_singleline(&mut self.name)
                                .labelled_by(name_label.id);
                        });
                        ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                        if ui.button("Increment").clicked() {
                            self.age += 1;
                        }

                        ui.label(format!("Hello '{}', age {}", self.name, self.age));

                        if ui.button("Send UDP Message").clicked() {
                            let message = format!("Name: {}, Age: {}", self.name, self.age);
                            let _ = self.socket.send_to(message.as_bytes(), &self.destination);
                        }
                    }
                    Screen::Secondary => {
                        ui.heading("Files in Folder");

                        if ui.button("Load Files").clicked() {
                            // 폴더 경로를 지정합니다. 필요에 따라 수정하세요.
                            let path = "./"; // 여기에 실제 폴더 경로를 넣으세요.
                            self.files = std::fs::read_dir(path)
                                .map(|dir| {
                                    dir.filter_map(|entry| {
                                        entry.ok().map(|e| {
                                            e.file_name().into_string().unwrap_or_default()
                                        })
                                    })
                                    .collect()
                                })
                                .unwrap_or_default(); // 오류 처리 추가 가능
                        }

                        ui.separator();
                        for file in &self.files {
                            ui.label(file);
                        }
                    }

                    Screen::Tertiary => {
                        // ui.heading("Stock Price Chart");

                        ui.group(|ui| {
                            // ui.label("Candle Chart Section");
                            // candle::candle_chart(ui);
                            let button_text = self.button_text.clone(); // 불변 참조의 복사본 생성

                            ui.menu_button(&button_text, |ui| {
                                for &stock in &self.stocks {
                                    if ui.button(stock).clicked() {
                                        println!("{} 클릭", stock);
                                        self.button_text = stock.to_string(); // 가변 참조를 수정
                                        ui.close_menu(); // 메뉴 닫기
                                    }
                                }
                            });
                        });
                        // candle::candle_chart(ui);

                        ui.group(|ui| {
                            ui.label("Candle Chart Section");
                            candle::candle_chart(ui,&self.stock_data);
                        });
                    }
                });
            });
        });
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;
use std::fs;

use eframe::egui::{self, Color32, Stroke};
use std::net::UdpSocket; // `egui_plot` crate를 import

//lib
use stocki::plot::candle;
#[derive(Deserialize, Debug)]
struct Candle {
    #[serde(rename = "candle_date_time_utc")]
    candle_date_time_utc: String,
    #[serde(rename = "opening_price")]
    opening_price: f64,
    #[serde(rename = "high_price")]
    high_price: f64,
    #[serde(rename = "low_price")]
    low_price: f64,
    #[serde(rename = "trade_price")]
    trade_price: f64,
}
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
    stock_chart: StockChart, // StockChart 필드 추가
    files: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        let socket = UdpSocket::bind("127.0.0.1:34254").expect("Couldn't bind to address");
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            socket,
            destination: "127.0.0.1:8080".to_owned(),
            current_screen: Screen::Main,
            stock_chart: StockChart::default(), // StockChart 초기화
            files: Vec::new(),                  // 빈 벡터로 초기화
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
            println!("{}","힝");
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
            ui.horizontal(|ui| {
                ui.heading("Main Screen");
            });
            ui.separator();

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
                        ui.heading("Main Screen");
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
                        ui.heading("Stock Price Chart");
                        ui.group(|ui| {
                            ui.label("Candle Chart Section");
                            candle::candle_chart(ui);
                        });
                    }
                });
            });
        });
    }
}

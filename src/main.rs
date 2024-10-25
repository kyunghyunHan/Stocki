#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use eframe::egui;
use egui_plot::{Bar,BoxElem};
use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};
//lib
use stocki::{plot::{plot}, utils::get_data,types::StockType};

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([980.0, 900.0]),
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
    button_text: Arc<Mutex<String>>, // 선택한 주식 이름을 공유하는 Arc<Mutex>
    stocks: Vec<&'static str>,       // 주식 이름을 저장할 벡터
    stock_types:  Vec<&'static str>,          // 주식 이름을 저장할 벡터

    stock_data: Vec<BoxElem>,            // 주식 데이터
    last_update: Instant,            // 마지막 업데이트 시간
    rx: Receiver<Vec<BoxElem>>,          // 수신자
    stock_type:Arc<Mutex<String>>,
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let selected_type = Arc::new(Mutex::new("day".to_string())); // 초기 주식 이름

        let selected_stock = Arc::new(Mutex::new("AAPL".to_string())); // 초기 주식 이름

        let selected_stock_clone = Arc::clone(&selected_stock);
        let selected_type_clone = Arc::clone(&selected_type);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1)); // 30초 대기
                let stock_name = selected_stock_clone.lock().unwrap().clone(); // 선택된 주식 이름 가져오기
                let stock_type = selected_type_clone.lock().unwrap().clone(); // 선택된 주식 이름 가져오기

                let new_data = get_data(&stock_name,&stock_type); // 주식 데이터를 가져옴
                if tx.send(new_data).is_err() {
                    break; // 메인 스레드가 더 이상 데이터를 수신하지 않으면 종료
                }
            }
        });

        Self {
            button_text: selected_stock,
            stocks: vec!["AAPL", "GOOGL"],
            stock_types: vec!["Day", "YEAR"],
            stock_data: get_data("AAPL","Day"), // 초기 주식 데이터
            last_update: Instant::now(),
            rx,
            stock_type: selected_type, // 여기에 필드 이름을 추가합니다.
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(new_data) = self.rx.try_recv() {
            println!("새 데이터 수신");
            self.stock_data = new_data;
            self.last_update = Instant::now(); // 마지막 업데이트 시간 갱신

            ctx.request_repaint();
        }
        ctx.request_repaint_after(Duration::from_secs(1));

        // 상단 바
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        let button_text = self.button_text.lock().unwrap().clone(); // 공유된 주식 이름 가져오기
                        let stock_type = self.stock_type.lock().unwrap().clone(); // 공유된 주식 이름 가져오기

                        ui.menu_button(&button_text, |ui| {
                            for &stock in &self.stocks {
                                if ui.button(stock).clicked() {
                                    println!("{} 클릭", stock);
                                    *self.button_text.lock().unwrap() = stock.to_string(); // 선택된 주식 이름 업데이트
                                    self.stock_data = get_data(stock,&stock_type); // 클릭 시 즉시 데이터 업데이트
                                    ui.close_menu(); // 메뉴 닫기
                                }
                            }
                        });
                    });
                    ui.group(|ui| {
                        ui.label("Candle Chart Section");
                        // plot::bar_chart(ui, &self.stock_data);
                    });
                    ui.group(|ui| {
                        ui.label("Candle Chart Section");
                        let button_text = self.button_text.lock().unwrap().clone(); // 공유된 주식 이름 가져오기
                        let stock_type = self.stock_type.lock().unwrap().clone(); // 공유된 주식 이름 가져오기
                        ui.horizontal(|ui| {
                           
                            ui.menu_button(&stock_type, |ui| {
                                for &stock in &self.stock_types {
                                    if ui.button(stock).clicked() {
                                        println!("{} 클릭", stock);
                                        *self.stock_type.lock().unwrap() = stock.to_string(); // 선택된 주식 이름 업데이트
                                        self.stock_data = get_data(&button_text,&stock_type); // 클릭 시 즉시 데이터 업데이트
                                        ui.close_menu(); // 메뉴 닫기
                                    }
                                }
                            });

                        });
                   
                        plot::bar_chart(ui, &self.stock_data);
                    });
                });
            });
        });
    }
}

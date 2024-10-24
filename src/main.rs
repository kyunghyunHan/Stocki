#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use std::{sync::{mpsc::{self, Receiver, Sender},Arc,Mutex},thread,time::{Duration,Instant}};
use eframe::egui;
use egui_plot::Bar;
//lib
use stocki::{plot::candle, utils::get_data};

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
    button_text: String,       // 초기 텍스트
    stocks: Vec<&'static str>, // 주식 이름을 저장할 벡터
    stock_data: Vec<Bar>,      // 공유 데이터를 위한 Arc<Mutex>
    last_update: Instant,      // 마지막 업데이트 시간
    rx: Receiver<Vec<Bar>>,    // 스레드에서 주식 데이터를 수신하는 채널
}

impl Default for MyApp {
    fn default() -> Self {
        // 스레드와 UI 간 통신을 위한 채널 생성
        let (tx, rx) = mpsc::channel();

        // 30초마다 데이터를 갱신하는 스레드 생성
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(3)); // 30초 대기
                let new_data = get_data("AAPL"); // 주식 데이터를 가져옴
                if tx.send(new_data).is_err() {
                    break; // 메인 스레드가 더 이상 데이터를 수신하지 않으면 종료
                }
            }
        });

        Self {
            button_text: "AAPL".to_string(),
            stocks: vec!["AAPL", "GOOGL"],
            stock_data: get_data("AAPL"), // 초기 주식 데이터
            last_update: Instant::now(),  // 현재 시간으로 초기화
            rx,                           // 채널의 수신자
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //수신
        if let Ok(new_data) = self.rx.try_recv() {
            println!("새 데이터 수신");
            self.stock_data = new_data;
            self.last_update = Instant::now(); // 마지막 업데이트 시간 갱신
            
            // 데이터가 갱신되면 UI를 다시 그리도록 요청
            ctx.request_repaint();
        }
        ctx.request_repaint_after(Duration::from_secs(1));
        //상단 바
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                //주식차트
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        let button_text = self.button_text.clone(); // 불변 참조의 복사본 생성

                        ui.menu_button(&button_text, |ui| {
                            for &stock in &self.stocks {
                                if ui.button(stock).clicked() {
                                    println!("{} 클릭", stock);
                                    self.button_text = stock.to_string(); // 가변 참조를 수정
                                    self.stock_data = get_data(stock);
                                    ui.close_menu(); // 메뉴 닫기
                                }
                            }
                        });
                    });
                    ui.group(|ui| {
                        ui.label("Candle Chart Section");
                        candle::candle_chart(ui, &self.stock_data);
                    });
                });
            });
        });
    }
}

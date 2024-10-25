use eframe::egui::{Color32, Stroke};
use egui_plot::{Bar, BoxElem, BoxSpread};
use rayon::prelude::*;
use tokio_test;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::time::macros::datetime;

pub fn get_data(stock_name: &str) -> Vec<BoxElem> {
    println!("{}", stock_name);
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(2000-1-1 0:00:00.00 UTC);
    let end = datetime!(2024-12-31 23:59:59.99 UTC);
    let resp = tokio_test::block_on(provider.get_quote_history(stock_name, start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);

    quotes
        .into_par_iter()
        .map(|quote| {
            let open = quote.open as f64; // open을 f64로 변환
            let close = quote.close as f64; // close을 f64로 변환
            let volume = quote.volume as f64; // volume을 f64로 변환
            let color = if close >= open { green } else { red };
            let timestamp = quote.timestamp as f64;

            // BoxSpread를 정의합니다.
            let lower_bound = open; // 아래 경계
            let lower_whisker = open; // 아래 수염
            let upper_whisker = close; // 위 수염
            let upper_bound = close; // 위 경계
            let median = (open + close) / 2.0; // 중앙값
            let spread = BoxSpread::new(
                lower_bound,
                lower_whisker,
                upper_whisker,
                upper_bound,
                median,
            );

            // (
            //     // Bar::from(Bar {
            //     //     name: "a".to_string(),
            //     //     orientation: egui_plot::Orientation::Vertical,
            //     //     argument: timestamp,
            //     //     value: volume,
            //     //     base_offset: Some(0.),
            //     //     bar_width: 20.,
            //     //     stroke: Stroke::new(1.0, color),
            //     //     fill: color,
            //     // }),
            //     // BoxElem::new(timestamp, spread), // argument는 timestamp, spread는 위에서 정의한 값을 사용
            // )
            BoxElem::new(timestamp, spread).stroke(Stroke::new(1.0, green))

            // argument는 timestamp, spread는 위에서 정의한 값을 사용
        })
        .collect()
}

use yahoo_finance_api as yahoo;

use eframe::egui::{Color32, Stroke};
use egui_plot::Bar;
use rayon::prelude::*; 
use tokio_test;

use yahoo_finance_api::time::macros::datetime;
pub fn get_data(stock_name: &str) -> Vec<Bar> {
    println!("{}",stock_name);
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(2023-12-20 0:00:00.00 UTC);
    let end = datetime!(2024-1-1 23:59:59.99 UTC);
    let resp = tokio_test::block_on(provider.get_quote_history(stock_name, start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);

    quotes
        .into_par_iter()
        .map(|quote| {
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
        })
        .collect()
}

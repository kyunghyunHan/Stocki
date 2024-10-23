use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use eframe::egui::{self, Color32, Stroke, Vec2};
use egui_plot::{Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Line, Orientation, Plot, PlotPoints};
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::time::{macros::datetime, OffsetDateTime};
use rayon::prelude::*; // rayon을 가져옵니다.

use reqwest::{header, Error};
use serde::Deserialize;
pub fn candle_chart(ui: &mut egui::Ui) {
    println!("호출대나");
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(1990-1-1 0:00:00.00 UTC);
    let end = datetime!(2024-1-1 23:59:59.99 UTC);
    let resp = tokio_test::block_on(provider.get_quote_history("AAPL", start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);

    // println!("주식 갯수:{}", quotes.len());

    let bars: Vec<Bar> = quotes.par_iter().map(|quote| {
        let open = quote.open; // 개장가
        let high = quote.high; // 고가
        let low = quote.low; // 저가
        let close = quote.close; // 종가
        let volume = quote.volume;
        // 종가와 개장가를 비교하여 색상을 결정
        let color = if close >= open {
            green // 종가가 개장가보다 높으면 녹색
        } else {
            red   // 종가가 개장가보다 낮으면 빨간색
        };
        let timestamp = quote.timestamp as f64;

        let bar = Bar::from(Bar {
            name: "a".to_string(),
            orientation: Orientation::Vertical, // 세로 막대그래프
            argument: timestamp,
            value: volume as f64,           // Y축 값 (종가를 막대 높이로 사용)
            base_offset: Some(0.), // 막대가 시작하는 기준 (저가를 기준으로 할 수 있음)
            bar_width: 20.,
            stroke: Stroke::new(1.0, color),
            fill: color,
        });

        bar
    }).collect(); // 바를 수집합니다.
    let data = BarChart::new(bars);
    let plot = Plot::new("candlestick chart").view_aspect(10.0);
    plot.show(ui, |plot_ui| {
        plot_ui.bar_chart(data);
    });
}

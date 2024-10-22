use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use eframe::egui::{self, Color32, Stroke, Vec2};
use egui_plot::{BoxElem, BoxPlot, BoxSpread, Line, Orientation, Plot, PlotPoints};
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::time::{macros::datetime, OffsetDateTime};

use reqwest::{header, Error};
use serde::Deserialize;
pub fn candle_chart(ui: &mut egui::Ui) {
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(2020-1-1 0:00:00.00 UTC);
    let end = datetime!(2020-1-31 23:59:59.99 UTC);
    let resp = tokio_test::block_on(provider.get_quote_history("AAPL", start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);
    let mut v = Vec::new();
    for quote in quotes {
        let open = quote.open; // 개장가
        let high = quote.high; // 고가
        let low = quote.low; // 저가
        let close = quote.close; // 종가
        let box_elem = BoxElem::new(
            close,                                         // 종가를 중심으로
            BoxSpread::new(open, low, high, close, close), // 값들을 사용하여 BoxSpread 생성
        )
        .whisker_width(0.0)
        .fill(green) // 색상 설정
        .stroke(Stroke::new(1.0, green)); // 선 두께 및 색상 설정
        v.push(box_elem); // BoxPlot에 추가
    }
    let data = BoxPlot::new(v);

   
    let plot = Plot::new("candlestick chart")
        .view_aspect(10.0)
        .min_size(Vec2::from([12., 12.]));

    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data);
    });
}

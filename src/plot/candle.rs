use eframe::egui::{self, Color32, Stroke, Vec2};
use egui_plot::{Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Line, Orientation, Plot, PlotPoints};
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api as yahoo;
use yahoo_finance_api::time::{macros::datetime, OffsetDateTime};
use rayon::prelude::*; // rayon을 가져옵니다.

use reqwest::{header, Error};
use serde::Deserialize;

pub fn candle_chart(ui: &mut egui::Ui,bars:& Vec<Bar>) {
    let data = BarChart::new(bars.clone());
    let plot = Plot::new("candlestick chart").view_aspect(10.0);
    plot.show(ui, |plot_ui| {
        plot_ui.bar_chart(data);
    });
}

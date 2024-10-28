use eframe::egui::{self, Color32, Stroke, Vec2};
use egui_plot::{Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Line, Orientation, Plot, PlotPoints};
use reqwest::{header, Error};
use std::time::{Duration, UNIX_EPOCH};
use yahoo_finance_api::time::{macros::datetime, OffsetDateTime};

pub fn bar_chart(ui: &mut egui::Ui, bars: &Vec<BoxElem>) {
    let start_timestamp = datetime!(2024-12-30 00:00:00.00 UTC).unix_timestamp() as f64;

    let end_timestamp = datetime!(2024-12-31 23:59:59.99 UTC).unix_timestamp() as f64;

    // let bar_data= BarChart::new(bars.iter().map(|(bar, _)| bar.clone()).collect());
    // let box_plot= BoxPlot::new(bars.iter().map(|(_, plot)| plot.clone()).collect());
    let box_plot = BoxPlot::new(bars.clone());
    let plot = Plot::new("candlestick chart")
        .view_aspect(3.0)
        .x_axis_position(egui_plot::VPlacement::Bottom)
        .width(600.)
        .height(500.)
        .include_x(start_timestamp)
        .include_x(end_timestamp)
        .allow_boxed_zoom(true)
        .x_axis_label("label")
        // .link_axis(group_id, link_x, link_y)
        .min_size(Vec2::from([12., 12.]));
    println!("{}", start_timestamp);
    plot.show(ui, |plot_ui| {
        // plot_ui.bar_chart(bar_data);
        plot_ui.box_plot(box_plot);
    });
}

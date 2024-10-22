use eframe::egui::{self, Color32, Stroke, Vec2};
use egui_plot::{BoxElem, BoxPlot, BoxSpread, Line, Orientation, Plot, PlotPoints};

pub fn candle_chart(ui: &mut egui::Ui) {
    let red = Color32::from_rgb(255, 0, 0);
    let green = Color32::from_rgb(0, 255, 0);
    let data = BoxPlot::new(vec![
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)) // 1월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(2.0, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5)) // 2월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(3.0, BoxSpread::new(1.8, 2.0, 2.4, 2.5, 2.7)) // 3월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(4.0, BoxSpread::new(1.5, 1.8, 1.8, 2.1, 2.2)) // 4월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(5.0, BoxSpread::new(1.4, 1.6, 1.6, 1.8, 2.1)) // 5월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(6.0, BoxSpread::new(0.5, 1.5, 1.5, 1.6, 1.7)) // 6월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(7.0, BoxSpread::new(1.2, 1.4, 1.4, 2.9, 3.2)) // 7월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(8.0, BoxSpread::new(2.1, 2.3, 2.3, 2.6, 2.7)) // 8월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(9.0, BoxSpread::new(1.9, 2.1, 2.1, 2.7, 3.5)) // 9월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(10.0, BoxSpread::new(2.0, 2.1, 2.1, 2.9, 3.3)) // 10월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(11.0, BoxSpread::new(2.3, 2.9, 2.9, 3.7, 4.1)) // 11월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(12.0, BoxSpread::new(3.1, 3.4, 3.4, 4.0, 4.2)) // 12월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
                   BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)) // 1월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(2.0, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5)) // 2월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(3.0, BoxSpread::new(1.8, 2.0, 2.4, 2.5, 2.7)) // 3월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(4.0, BoxSpread::new(1.5, 1.8, 1.8, 2.1, 2.2)) // 4월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(5.0, BoxSpread::new(1.4, 1.6, 1.6, 1.8, 2.1)) // 5월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(6.0, BoxSpread::new(0.5, 1.5, 1.5, 1.6, 1.7)) // 6월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(7.0, BoxSpread::new(1.2, 1.4, 1.4, 2.9, 3.2)) // 7월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(8.0, BoxSpread::new(2.1, 2.3, 2.3, 2.6, 2.7)) // 8월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(9.0, BoxSpread::new(1.9, 2.1, 2.1, 2.7, 3.5)) // 9월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(10.0, BoxSpread::new(2.0, 2.1, 2.1, 2.9, 3.3)) // 10월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(11.0, BoxSpread::new(2.3, 2.9, 2.9, 3.7, 4.1)) // 11월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(12.0, BoxSpread::new(3.1, 3.4, 3.4, 4.0, 4.2)) // 12월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
            BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)) // 1월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(13.0, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5)) // 2월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(14.0, BoxSpread::new(1.8, 2.0, 2.4, 2.5, 2.7)) // 3월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(15.0, BoxSpread::new(1.5, 1.8, 1.8, 2.1, 2.2)) // 4월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(16.0, BoxSpread::new(1.4, 1.6, 1.6, 1.8, 2.1)) // 5월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(17., BoxSpread::new(0.5, 1.5, 1.5, 1.6, 1.7)) // 6월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(18.0, BoxSpread::new(1.2, 1.4, 1.4, 2.9, 3.2)) // 7월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(19.0, BoxSpread::new(2.1, 2.3, 2.3, 2.6, 2.7)) // 8월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(20.0, BoxSpread::new(1.9, 2.1, 2.1, 2.7, 3.5)) // 9월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
        BoxElem::new(21.0, BoxSpread::new(2.0, 2.1, 2.1, 2.9, 3.3)) // 10월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(22.0, BoxSpread::new(2.3, 2.9, 2.9, 3.7, 4.1)) // 11월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(23.0, BoxSpread::new(3.1, 3.4, 3.4, 4.0, 4.2)) // 12월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
                   BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)) // 1월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(24.0, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5)) // 2월
            .whisker_width(0.0)
            .fill(green)
            .stroke(Stroke::new(2.0, green)),
        BoxElem::new(25.0, BoxSpread::new(1.8, 2.0, 2.4, 2.5, 2.7)) // 3월
            .whisker_width(0.0)
            .fill(red)
            .stroke(Stroke::new(2.0, red)),
       
    ]);

    let plot = Plot::new("candlestick chart")
        .view_aspect(10.0)
        .min_size(Vec2::from([12., 12.]));

    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data);
    });
}

#![no_main]
use plotters::prelude::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (Vec<(f32, f32)>, Vec<(f32, f32)>)| {
    let (lines, points) = input;
    if lines.iter().any(|(x, y)| x.is_nan() || y.is_nan() || x.abs() > 10000f32 || y.abs() > 10000f32) {
        return;
    }
    if points.iter().any(|(x, y)| x.is_nan() || y.is_nan() || x.abs() > 10000f32 || y.abs() > 10000f32) {
        return;
    }

    let root = BitMapBackend::new("test.png", (100, 100)).into_drawing_area();
    let root = root.margin(10, 10, 10, 10);
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0f32..10f32, 0f32..10f32).expect("unable to build 2d chart");

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw().expect("unable to configure mesh, x and y labels, and formatter");

    chart.draw_series(LineSeries::new(
        lines,
        &RED,
    )).expect("unable to draw line series");

    chart.draw_series(PointSeries::of_element(
        points,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)
            + Circle::new((0,0),s,st.filled())
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },
    )).unwrap();
    root.present().unwrap();
});
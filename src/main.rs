//Introduction
/*
use plotters::prelude::*;

fn main() {
    let root_drawing_area = BitMapBackend::new("images/0.1.png", (1920, 1080))
    .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
    .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
    .unwrap();

    chart.draw_series(LineSeries::new((-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())), &RED
    )).unwrap();
}
*/

//Mesh 
/*
use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/2.2.png", (1920, 1080))
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root).build_cartesian_2d(0..100, 0..100).unwrap();

    ctx.configure_mesh().draw().unwrap();
}
*/

//Title
/*
use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/2.4.png", (1920, 1080))
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root)
    .caption("Figure Sample", ("Arial", 30))
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();
}
*/

///Basic Data Plotting
//Line Series
/* 
use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/2.5.png", (1920, 1080))
    .into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Line Plot Demo", ("sans-serif", 40))
    .build_cartesian_2d(-10..10, 0..100)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((-10..=10).map(|x| (x, x* x)), &GREEN)).unwrap();

}
*/

//Scatter Plot
/*
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.6.png", (1920, 1080))
    .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Scatter Demo", ("sans-serif", 40))
    .build_cartesian_2d(-10..50, -10..50)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(DATA1.iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    ).unwrap();

    ctx.draw_series(DATA2.iter().map(|point| Circle::new(*point, 5, &RED)),
    ).unwrap();

}

const DATA1: [(i32, i32); 30] =  [(-3, 1), (-2, 3), (4, 2), (3, 0), (6, -5), (3, 
11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 
14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 
21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];

const DATA2: [(i32, i32); 30] = [(1, 22), (0, 22), (1, 20), (2, 24), (4, 26), 
(6, 24), (5, 27), (6, 27), (7, 27), (8, 30), (10, 30), (10, 33), (12, 34), (13, 
31), (15, 35), (14, 33), (17, 36), (16, 35), (17, 39), (19, 38), (21, 38), (22, 
39), (23, 43), (24, 44), (24, 46), (26, 47), (27, 48), (26, 49), (28, 47), (28, 
50)];
*/

//Area Chart
/*
use plotters::prelude::*;

fn main() {
    let root_area  = BitMapBackend::new("images/2.7.png", (1920, 1080))
    .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Scatter Demo", ("sans-serif", 40))
    .build_cartesian_2d(0..10, 0..50)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    ctx.draw_series(
        AreaSeries::new((0..).zip(data.iter().map(|x| *x)),
            0, 
            &RED.mix(0.2)
        ).border_style(&RED)
    ).unwrap();
}
*/

//Histrogram
/*
use plotters::prelude::*;

fn main() {
    //Histogram
    /*
    let root_area = BitMapBackend::new("images/2.8.png", (1920, 1080))
    .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Bar Demo", ("sans-serif", 40))
    .build_cartesian_2d((0..10).into_segmented(), 0..50)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x+1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    })).unwrap();
    */

    //Vertical Bar Chart
    let root_area = BitMapBackend::new("images/2.9.png", (1920, 1080))
    .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Bar Demo", ("sans-serif", 40))
    .build_cartesian_2d(0..50, (0..10).into_segmented())
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

    ctx.draw_series((0..).zip(data.iter()).map(|(y, x)| {
        let mut bar = Rectangle::new([
            (0, SegmentValue::Exact(y)),
            (*x, SegmentValue::Exact(y+1))],
            GREEN.filled()
        );

        bar.set_margin(5, 5, 0, 0);
        bar
    })).unwrap();

}
*/

//Visualize distribution
/*
use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.13.png", (1920, 1080))
    .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let mut ctx  = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Prime Distribution", ("sans-serif", 40))
    .build_cartesian_2d([true, false].into_segmented(), 0..50)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let prim:Vec<_> = (2..50).map(is_prime).collect();

    ctx.draw_series(
        Histogram::vertical(&ctx)
            .margin(100)
            .data(
                prim.iter().map(|x| (x, 1))
            )
    ).unwrap();
}

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true    
}
*/

//Time Series Chart - Here is where the money's at
use plotters::prelude::*;
use chrono::{Utc, TimeZone};

fn main() {
    let root_area = BitMapBackend::new("images/2.11.png", (1920, 1080))
    .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    let start_date = Utc.with_ymd_and_hms(2025, 01, 06, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2025, 02, 15, 0, 0, 0).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("MSFT daily close price", ("sans-serif", 40))
    .build_cartesian_2d(start_date..end_date, 130.0..145.0)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((0..).zip(DATA.iter()).map(|(idx, price)| {
        let day = (idx / 5) * 7 + idx % 5 + 1;
        let date = Utc.with_ymd_and_hms(2025, 01, day, 0, 0, 0).unwrap();
        (date, *price)
    }), &BLUE)).unwrap();
}

const DATA: [f64; 14] = [ 137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 
141.58, 139.55, 139.68, 139.10, 138.24, 135.67, 137.12, 138.12];
extern crate streaming_harness_hdrhist;
extern crate rand;
extern crate textplots;

use rand::distributions::{Normal, Distribution};

fn main() {
    let normal = Normal::new(20.0, 3.0);
    let mut rng = rand::thread_rng();

    let mut hist = streaming_harness_hdrhist::HDRHist::new();

    for _ in 0..1000000000 {
        let val = normal.sample(&mut rng) * 1_000_000_000f64;
        if val >= 0f64 {
            hist.add_value(val as u64);
        }
    }

    for (v, p, c) in hist.ccdf() {
        println!("{}\t{}\t{}", v, p, c);
    }

    eprintln!("summary {:#?}", hist.summary().collect::<Vec<_>>());
    eprintln!("summary_string\n{}", hist.summary_string());

    eprintln!("plot"); 
    use textplots::{Chart, Plot, Shape};
    let data: Vec<_> = hist.ccdf().map(|(v, p, _)| (v as f32, p as f32)).collect();
    Chart::new(180, 60, 0.0, 8e10).lineplot(Shape::Lines(&data)).nice();

    let mut hist2 = streaming_harness_hdrhist::HDRHist::new();
    for _ in 0..1000000000 {
        hist2.add_value(1000000);
    }
    eprintln!("summary_string\n{}", hist2.summary_string());
}

use butterworth::{Cutoff, Filter};
use csv::Writer;
use plotters::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};
use serde::Serialize;

use std::{f32::consts::PI, fmt::Display, os::unix::process};

mod read;

fn main_test() {
    let mut buffer = vec![
        Complex {
            re: 0.0f32,
            im: 0.0f32
        };
        128
    ];

    for i in 0..buffer.len() {
        buffer[i].re = (i as f32 / (0.1 * PI)).sin();
    }

    let x: Vec<f32> = buffer.iter().map(|c| c.re).collect();

    let mut y = analyze_frequencies(&x, 44100.0);
    y.sort_by(|a, b| a.amplitude.partial_cmp(&b.amplitude).unwrap());

    println!("results: {:#?}", y);

    // plot(&buffer);

    let root_drawing_area = SVGBackend::new("output.svg", (640, 480)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("FFT Result", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..128, 0f32..100f32)
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            y.iter()
                .enumerate()
                .map(|(i, freq_bin)| (i as i32, y[i].amplitude)),
            &RED,
        ))
        .unwrap();
}

// fn plot(buffer: &Vec<Complex<f32>>) {
//     let root_drawing_area = SVGBackend::new("output.svg", (640, 480)).into_drawing_area();
//
//     root_drawing_area.fill(&WHITE).unwrap();
//
//     let mut chart = ChartBuilder::on(&root_drawing_area)
//         .caption("FFT Result", ("sans-serif", 50).into_font())
//         .margin(20)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(0..128, 0f32..100f32)
//         .unwrap();
//
//     chart
//         .draw_series(LineSeries::new(
//             buffer.iter().enumerate().map(|(i, freq_bin)| {
//                 let magnitude = (freq_bin.re.powi(2) + freq_bin.im.powi(2)).sqrt();
//                 (i as i32, magnitude)
//             }),
//             &RED,
//         ))
//         .unwrap();
// }

#[derive(Debug, Clone, Serialize)]
pub struct Wave {
    pub frequency: f32,
    pub amplitude: f32,
    /// in radians
    pub phase: f32,
}

/// Performs FFT on a real-valued signal and returns wave components.
///
/// # Arguments
/// * `signal` - The input signal (real values)
/// * `sample_rate` - The sample rate in Hz
///
/// # Returns
/// A vector of `Wave` structs, one for each frequency bin (up to Nyquist)
pub fn analyze_frequencies(signal: &[f32], sample_rate: f32) -> Vec<Wave> {
    let fft_size = signal.len();

    let mut buffer: Vec<Complex<f32>> = signal.iter().map(|&x| Complex::new(x, 0.0)).collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    fft.process(&mut buffer);

    // Only take bins up to Nyquist frequency (first half + 1)
    let num_bins = fft_size / 2 + 1;

    buffer[..num_bins]
        .iter()
        .enumerate()
        .map(|(i, bin)| {
            let frequency = i as f32 * sample_rate / fft_size as f32;

            // Normalize magnitude by FFT size
            let magnitude = (bin.re.powi(2) + bin.im.powi(2)).sqrt() / fft_size as f32;

            let phase = bin.im.atan2(bin.re);

            Wave {
                frequency,
                amplitude: magnitude,
                phase,
            }
        })
        .collect()
}

fn main2() {
    let poses = read::read_csv("poses.csv");

    dbg!(poses.len());
    dbg!(&poses[0]);

    let mapped: Vec<PoseDistances> = poses.iter().map(distances).collect();

    let root_drawing_area = SVGBackend::new("output.svg", (640, 480)).into_drawing_area();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Poses vs time", ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..1i32, 0f32..1f32)
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            mapped.iter().map(|d| (d.frame as i32, d.shoulders)),
            &RED,
        ))
        .unwrap();

    let mut writer = csv::Writer::from_path("distances.csv").unwrap();

    for d in mapped {
        writer.serialize(d).unwrap();
    }

    writer.flush().unwrap();
}

fn main() {
    let poses = read::read_csv("poses.csv");

    let mapped: Vec<PoseDistances> = poses.iter().map(distances).collect();

    // let mut frequencies: Vec<Wave> = analyze_frequencies(
    //     &mapped.iter().map(|a| a.diagonal_left).collect::<Vec<f32>>(),
    //     10.0,
    // );

    let filtered: Vec<f32> = bandpass_filter(
        &mapped
            .iter()
            .map(|a| a.diagonal_left as f64)
            .collect::<Vec<f64>>(),
        10.0,
    )
    .iter()
    .map(|&x| x as f32)
    .collect();

    let mut processed: Vec<Wave> = analyze_frequencies(&filtered, 10.0);

    let mut writer = csv::Writer::from_path("frequencies.csv").unwrap();

    for f in &processed {
        writer.serialize(f).unwrap();
    }
    writer.flush().unwrap();

    processed.sort_by(|x, y| y.amplitude.partial_cmp(&x.amplitude).unwrap());

    let x = processed.first_chunk::<10>().unwrap();
    for wave in x {
        println!("{:?}", wave);
    }

    let max = filtered.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min = filtered.iter().cloned().fold(f32::INFINITY, f32::min);
    println!("Filtered range: {} to {}", min, max);
}

#[derive(Debug, Serialize)]
struct PoseDistances {
    frame: usize,
    shoulders: f32,
    hips: f32,
    left: f32,
    right: f32,
    diagonal_left: f32,
    diagonal_right: f32,
}

fn distances(pose: &read::Poses) -> PoseDistances {
    let shoulders = distance((pose.x12, pose.y12), (pose.x11, pose.y11));
    let hips = distance((pose.x24, pose.y24), (pose.x23, pose.y23));

    let left = distance((pose.x12, pose.y12), (pose.x24, pose.y24));
    let right = distance((pose.x11, pose.y11), (pose.x23, pose.y23));

    let diagonal_left = distance((pose.x12, pose.y12), (pose.x23, pose.y23));
    let diagonal_right = distance((pose.x11, pose.y11), (pose.x24, pose.y24));

    PoseDistances {
        frame: pose.frame as usize,
        shoulders,
        hips,
        left,
        right,
        diagonal_left,
        diagonal_right,
    }
}

fn distance((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn bandpass_filter(signal: &[f64], sample_rate: f64) -> Vec<f64> {
    let low_cutoff = 0.15; // Hz - removes slow drift/motion artifact
    let high_cutoff = 0.5; // Hz - removes high frequency noise
    let order = 2; // Higher order = steeper rolloff, but more ringing

    let filter = Filter::new(
        order,
        sample_rate,
        Cutoff::BandPass(low_cutoff, high_cutoff),
    )
    .unwrap();

    filter.bidirectional(&Vec::from(signal)).unwrap()
}

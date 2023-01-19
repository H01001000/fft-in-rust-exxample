#![feature(test)]
use plotters::prelude::*;
use rug::{float, Complex, Float};
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_fft() {
        assert_eq!(
            fft(vec![2.0, 4.0], &mut HashMap::new()),
            vec![Complex::with_val(53, 6), Complex::with_val(53, -2)]
        )
    }

    #[bench]
    fn bench_fft(b: &mut Bencher) {
        b.iter(|| fft(vec![2.; 16], &mut HashMap::new()));
    }

    #[bench]
    fn bench_slice(b: &mut Bencher) {
        b.iter(|| slice(&vec![2.; 16], 2, 0));
    }
}

fn slice<T: Clone + Debug>(p: &Vec<T>, step: usize, start: usize) -> Vec<T> {
    let mut sliced_array: Vec<T> = Vec::new();
    for index in 0..(p.len() / step) {
        sliced_array.push(p[index.mul(step).add(start)].clone());
    }
    return sliced_array;
}

fn get_w(n: usize, cache: &mut HashMap<usize, Complex>) -> Complex {
    match cache.get(&n) {
        Some(w) => return w.clone(),
        None => {
            let w = Complex::with_val(53, (0, 1))
                .mul(Complex::with_val(53, 2))
                .mul(Complex::with_val(53, float::Constant::Pi))
                .div(Complex::with_val(53, n))
                .exp();
            cache.insert(n, w.clone());
            return w;
        }
    }
}

pub fn fft(p: Vec<f32>, cache: &mut HashMap<usize, Complex>) -> Vec<Complex> {
    let n = p.len();
    if n == 1 {
        return vec![Complex::with_val(53, p[0])];
    }
    let w = get_w(n, cache);
    let mut w_current = Complex::with_val(53, 1);
    let p_even = slice(&p, 2, 0);
    let p_odd = slice(&p, 2, 1);
    let y_even = fft(p_even, cache);
    let y_odd = fft(p_odd, cache);
    let mut y = vec![Complex::with_val(53, 0); n];
    for j in 0..(n / 2) {
        let y_odd_ans = w_current.clone().mul(y_odd[j].clone());
        y[j] = y_even[j].clone().add(y_odd_ans.clone());
        y[j + (n / 2)] = y_even[j].clone().sub(y_odd_ans.clone());
        w_current = w_current.mul(w.clone())
    }
    return y;
}

pub fn function(time: usize, sample: usize, signel: &[String]) -> f32 {
    let mut y = Float::with_val(53, 0);
    let frequencys = slice(&signel.to_vec(), 2, 0);
    let mults = slice(&signel.to_vec(), 2, 1);
    for pair in 0..signel.len() / 2 {
        y = y.add(
            Float::with_val(53, 2)
                .mul(Float::with_val(53, float::Constant::Pi))
                .mul(Float::with_val(53, time))
                .div(Float::with_val(53, sample))
                .mul(Float::with_val(
                    53,
                    frequencys[pair].parse::<f32>().unwrap(),
                ))
                .sin()
                .mul(Float::with_val(53, mults[pair].parse::<f32>().unwrap())),
        );
    }
    return y.to_f32();
}

pub fn plot(
    x_value: Vec<f32>,
    y_value: Vec<f32>,
    sample: usize,
    length: usize,
    input_filename: &str,
    output_filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut y_sort = y_value.clone();
    y_sort.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let y_max = y_sort[y_sort.len() - 1];

    let root = BitMapBackend::new(output_filename, (1280, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("FFT of {}", input_filename),
            ("sans-serif", 25).into_font(),
        )
        .margin(5)
        .margin_left(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (20f32..(sample / 2) as f32)
                .log_scale()
                .with_key_points(vec![
                    20., 50., 100., 200., 500., 1000., 2000., 5000., 10000., 20000.,
                ]),
            0f32..(y_max * 1.1),
        )?;

    chart.configure_mesh().x_desc("Frequency (Hz)").draw()?;

    chart.draw_series(LineSeries::new(
        (0..length / 2).map(|x| (x_value[x], y_value[x])),
        &RED,
    ))?;

    Ok(())
}

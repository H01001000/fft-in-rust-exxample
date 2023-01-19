use fft::{fft, function, plot};
use ndarray::Array;
use rug::Float;
use std::{
    collections::HashMap,
    env,
    fs::File,
    ops::{Div, Mul},
    path::Path,
    usize,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(!args.get(1).is_none(), "missing input type");
    let (data_fn, length, sample, output_filename, start): (
        Box<dyn Fn(usize) -> f32>,
        usize,
        usize,
        &String,
        usize,
    ) = match args[1].as_str() {
        "gen" => {
            assert!(!args.get(2).is_none(), "missing length");
            assert!(!args.get(3).is_none(), "missing sample rate");
            assert!(!args.get(4).is_none(), "missing output filename");
            let mut signel: &[String] = &[];
            if args.len() > 4 {
                signel = &args[5..];
                assert!(signel.len() % 2 == 0);
            }
            println!("--------------------------------------------");
            println!("Input Data: Signal Generator, Output File: {}", &args[4]);
            println!(
                "Sample Rate: {}, Length: {}",
                args[3].parse::<usize>().unwrap(),
                args[2].parse::<usize>().unwrap()
            );
            println!("Signel:");
            for pair in (0..signel.len()).step_by(2) {
                println!(
                    "Frequency: {}, Multiplier: {}",
                    signel[pair],
                    signel[pair + 1]
                );
            }
            println!("--------------------------------------------");
            (
                Box::new(|t| function(t, args[3].parse::<usize>().unwrap(), signel)),
                args[2].parse::<usize>().unwrap(),
                args[3].parse::<usize>().unwrap(),
                &args[4],
                0,
            )
        }
        "audio" => {
            assert!(!args.get(2).is_none(), "missing input filename");
            assert!(!args.get(3).is_none(), "missing output filename");
            let mut inp_file = File::open(Path::new(&args[2])).unwrap();
            let (header, data) = wav::read(&mut inp_file).unwrap();
            let data_f32_p = data.as_sixteen().unwrap().clone();
            let length = data_f32_p.len();
            let sample: usize = header.sampling_rate as usize;
            let start = match args.get(4) {
                Some(arg) => {
                    ((arg.parse::<f32>().unwrap() / 1000.0) * sample as f32).round() as usize
                }
                None => 0,
            };
            let end = match args.get(5) {
                Some(arg) => {
                    ((arg.parse::<f32>().unwrap() / 1000.0) * sample as f32).round() as usize
                }
                None => start + length,
            };
            println!("--------------------------------------------");
            println!("Input Data: {}, Output File: {}", args[2], args[3]);
            println!("Sample Rate: {}, Length: {}", sample, (end - start));
            println!("Start Sample: {}, End Sample: {}", start, end);
            println!("--------------------------------------------");
            (
                Box::new(move |t: usize| data_f32_p[t] as f32),
                end - start,
                sample,
                &args[3],
                start,
            )
        }
        _ => panic!(),
    };

    let mut p: Vec<f32> = Vec::new();
    let mut f: Vec<f32> = Vec::new();
    for t in 0..length {
        //p.push(function(t, sample));
        p.push(data_fn(t + start));
        f.push(
            Float::with_val(53, t)
                .mul(Float::with_val(53, sample))
                .div(Float::with_val(53, length))
                .to_f32(),
        )
    }
    let ans = fft(p, &mut HashMap::new());
    let mut p2: Vec<f32> = ans
        .into_iter()
        .map(|f| f.abs().into_real_imag().0.to_f32())
        .collect();
    p2 = (Array::from(p2) / length as f32).to_vec();
    let mut p1 = p2[0..length / 2].to_vec();
    for i in 2..p1.len() {
        p1[i] = p1[i].mul(2.0);
    }
    f = f[0..length / 2].to_vec();
    //println!("{:?}", p1);
    plot(f, p1, sample, length, &args[1], &output_filename)?;
    Ok(())
}

extern crate clap;
extern crate png;
extern crate rand;

use clap::{App, AppSettings, Arg};
use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;

mod generator;
use generator::Lightning;

fn write_debug_lightning(
    charges: &Vec<[i32; 2]>,
    candidate_sites: &Vec<[i32; 2]>,
    width: u32,
    height: u32,
) {
    let filename = r"debug.png";
    let mut data = vec![0u8; (width * height) as usize];
    let center = [width as i32 / 2, height as i32 / 2];

    for (i, list) in [charges, candidate_sites].iter().enumerate() {
        let brightness = (128 * i - 1) as u8; // 注意: 負のオーバーフローを利用。

        for position in list.iter() {
            let x = (position[0] + center[0]) as u32;
            if x >= width {
                continue;
            };

            let y = (position[1] + center[1]) as u32;
            if y >= height {
                continue;
            };

            data[(x + width * y) as usize] = brightness;
        }
    }

    write_png(filename, &data, width, height);
}

fn write_png(filename: &str, data: &[u8], width: u32, height: u32) {
    let path = std::path::Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder
        .set(png::ColorType::Grayscale)
        .set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}

fn write_positions(filename: &str, positions: &Vec<[i32; 2]>) {
    let mut text: Vec<String> = vec![];

    for position in positions.iter() {
        let line = format!("{}, {}", position[0], position[1]);
        text.push(line.to_string());
    }

    std::fs::write(filename, text.join("\n")).expect("Unable to write file");
}

macro_rules! parse_value {
    ($matches:ident, $value_name:expr, $convert_to:ty) => {
        match $matches.value_of($value_name) {
            Some(value_str) => match value_str.parse::<$convert_to>() {
                Ok(value_parsed) => value_parsed,
                Err(error) => panic!("{}", error),
            },
            None => panic!(r"value_of() failed for {}.", $value_name),
        }
    };
}

fn render_lightning() {
    let matches = App::new("thunder")
        .version("0.1")
        .setting(AppSettings::AllowNegativeNumbers)
        .about("Thunder simulator.")
        .arg(
            Arg::with_name("iteration")
                .short("i")
                .long("iteration")
                .value_name("u32")
                .help("Number of iteration.")
                .takes_value(true)
                .default_value("2000"),
        )
        .arg(
            Arg::with_name("eta")
                .short("e")
                .long("eta")
                .value_name("f32")
                .help("Probability modifier. Usually range of 1 to 10.")
                .takes_value(true)
                .default_value("4.0"),
        )
        .get_matches();

    let num_iteration = parse_value!(matches, "iteration", u32);
    println!("iteration: {}", num_iteration);

    let eta = parse_value!(matches, "eta", f32);
    println!("eta: {}", eta);

    let lightning = Lightning::generate(num_iteration, eta);
    let charges = lightning.charges;
    let candidate_sites = lightning.candidate_sites;
    write_debug_lightning(&charges, &candidate_sites, 1920, 1200);
    write_positions("data_charges.txt", &charges);
    write_positions("data_candidate_sites.txt", &candidate_sites);
}

fn main() {
    render_lightning();
}

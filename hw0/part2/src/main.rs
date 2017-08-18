#[macro_use]
extern crate nom;

extern crate clap;

use clap::{App, Arg};

extern crate nalgebra;

use nalgebra::{Matrix4};

extern crate parse;

use parse::transformation::Transformation;

named!(transformations<&[u8], Vec<Transformation>>,
    many0!(parse::transformation::transformation)
);

fn main() {
    let matches = App::new("Homework 0: Part 2")
        .version("0.1")
        .author("Evan Davis <cptroot@gmail.com>")
        .arg(Arg::with_name("FILES")
            .required(true)
            .index(1)
            .multiple(true))
        .get_matches();

    let files = matches.values_of("FILES").unwrap();

    for file_name in files {
        use nom::Producer;
        let mut file_producer = nom::FileProducer::new(file_name, 256).unwrap();

        consumer_from_parser!(
            TransformationsConsumer<Vec<Transformation>>,
            transformations
        );

        let mut consumer = TransformationsConsumer::new();

        let transformations = file_producer.run(&mut consumer).unwrap();

        let matrix: Matrix4<f32> =
            transformations.iter()
                .map(|&transformation| Matrix4::<f32>::from(transformation))
                .fold(Matrix4::identity(), |a, b| { a * b });
        let inv_matrix = matrix.try_inverse().expect("matrix is non-invertible");

        print_matrix(&file_name, &inv_matrix);
    }
}

fn print_matrix(file_name: &str, matrix: &Matrix4<f32>) {
	for row in 0usize..4 {
        print!("|");

        for column in 0usize..4 {
            print!("{} ", matrix[(row, column)]);
        }
        println!("|");
    }
    println!("");
}

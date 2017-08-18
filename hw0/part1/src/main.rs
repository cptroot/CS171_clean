#[macro_use]
extern crate nom;

extern crate clap;

use clap::{App, Arg};

extern crate parse;

use parse::obj::{Obj, Vertex, Face};

fn main() {
    let matches = App::new("Homework 0: Part 1")
        .version("0.2")
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
        consumer_from_parser!(ObjConsumer<Obj>, parse::obj::obj);
        let mut consumer = ObjConsumer::new();

        let obj = file_producer.run(&mut consumer).unwrap();

        print_obj(&file_name, &obj);
    }
}

fn print_obj(name:&str, obj:&Obj) -> () {
    println!("{}:", name);
    println!("");

    for vertex in obj.vertices.iter() {
        match vertex {
            &Vertex(x, y, z) => println!("v {} {} {}", x, y, z)
        }
    }

    for face in obj.faces.iter() {
        match face {
            &Face(i1, i2, i3) => println!("f {} {} {}", i1, i2, i3)
        }
    }
    println!("");
}

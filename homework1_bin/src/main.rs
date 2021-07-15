use homework1_lib::area::*;
//use homework1_lib::volume::*;

use std::env;
use homework1_lib::volume::{find_volume, VolumeOf};

fn main() {

    let arguments: Vec<String> = env::args().collect();
    if arguments.len()>1 {

        if arguments[1] == "Circle" {
            let radius: f32 = match arguments[2].trim().parse() {
                Ok(num) => num,
                Err(_) => -1.0,
            };
            if radius > 0.0 { println!("{}", find_area(AreaOf::Circle { radius })); }
            else { println!("Circle: incorrect data!"); }
        }

        else if arguments[1] == "Rectangle" {
            println!("Hello, rectangle!");
            let width: f32 = match arguments[2].trim().parse() {
                Ok(num) => num,
                Err(_) => -1.0,
            };
            let height: f32 = match arguments[3].trim().parse() {
                Ok(num) => num,
                Err(_) => -1.0,
            };
            if width > 0.0 && height>0.0 { println!("{}", find_area(AreaOf::Rectangle { width,height })); }
            else { println!("Rectangle: incorrect data!"); }
        }

        else if arguments[1] == "Sphere" {
            println!("Hello, sphere!");
            let radius: f32 = match arguments[2].trim().parse() {
                Ok(num) => num,
                Err(_) => -1.0,
            };
            if radius > 0.0 { println!("{}", find_volume(VolumeOf::Sphere { radius })); }
            else { println!("Sphere: incorrect data!"); }
        }

        else if arguments[1] == "Parallelepiped" {
            let a: f32 = match arguments[2].trim().parse() {
                Ok(num) => num,
                Err(_) => -1.0,
            };
            let b: f32 = match arguments[3].trim().parse() {
                Ok(num) => num,
                Err(_) => -1.0,
            };
            let height: f32 = match arguments[4].trim().parse() {
                Ok(num) => num,
                Err(_) => -1.0,
            };
            if a > 0.0 && b>0.0 && c>0.0 { println!("{}", find_volume(VolumeOf::Parallelepiped { a,b,height })); }
            else { println!("Parallelepiped: incorrect data!"); }
        }

    }

}

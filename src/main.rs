use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    let width: f32;
    let height: f32;
    if args.len() == 2 {
        let (x, y) = args[1].split_once("x").unwrap();
        width = x.parse().unwrap();
        height = y.parse().unwrap();
    } else if args.len() == 3 {
        width = args[1].parse().unwrap();
        height = args[2].parse().unwrap();
    } else {
        println!("Invalid Input. Either input with an x combining the two numbers ( 1920x1080 ) or give the two numbers with a space ( 1920 1080 )");
        exit(1);
    }

    let mut i: f32 = 1.0;
    let mut newest_width: f32 = 0.0;
    let mut newest_height: f32 = 0.0;
    loop {
        let (width_ratio, height_ratio) = (width / i, height / i);
        if width_ratio.fract() == 0.0 && height_ratio.fract() == 0.0 {
            newest_width = width_ratio;
            newest_height = height_ratio;
        } else if width_ratio < 1.0 || height_ratio < 1.0 {
            println!("{}:{}", newest_width as i32, newest_height as i32);
            exit(0);
        }

        i += 1.0;
    }
}
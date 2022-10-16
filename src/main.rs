use image;
use image::GenericImageView;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Get the text file path from args
    let file_path: &String = &args[1];

    // Get the image file path from args
    let image_path: &String = &args[2];
    
    // Open image
    let image = image::open(image_path)
        .expect("Failed to open file");

    // For pixel in image pixels
    for (_, _, image::Rgba(pixel))in image.pixels() {
        // Extract RGBa values
        let [r, g, b, _a] = pixel;
        println!("{}", r);
        println!("{:x}", r);
        println!("colors: {} {} {}", r, g, b);
        println!("to hex: #{:x}{:x}{:x}", r, g, b);
        break
    }

    // Ensure that the file provided exists in current path
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(l) = line {
                println!("{}", l);
            }
        }
    }
}

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_lines<P>(file_path: &P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

//fn to_hex(r: &u8, g: &u8, b: &u8) {
//    format!("{:02X}{:02X}{:02X}", r, g, b)
//}

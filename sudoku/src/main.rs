extern crate array2;
use crate::array2::Arr;
use crate::array2::Array2;
use std::process::exit;
use std::env;
fn check(row: Vec<i32>) -> bool {
    let mut correct = true;
    let sum: i32 = row.iter().sum();
    if sum != 45 {
        correct = false;
    }
    for i in 0..9 {
        if row.iter().filter(|&n| *n == i).count() > 1 {
            correct = false;
        }
    }
    for i in 0..9 {
        if row[i] < 1 || row[i] > 9 {
            correct = false;
        }
    }
    return correct;
}
// CHECK BOXES 
/*fn check_box(box: Vec<i32>) -> bool {

}*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let img: csc411_image::image::Image;
    if args.len() == 2 {
        let filename = &args[1];
        img = csc411_image::image::Image::read(Some(filename)).unwrap();
    }
    else { img = csc411_image::image::Image::read(None).unwrap(); }
    
    let pixels = &img.pixels;
    let mut pixel_values: Vec<i32> = Vec::new();
    for pixel in pixels {
        match pixel {
            csc411_image::pixel::Pixel::Gray(gray) => pixel_values.push(gray.value as i32),
            csc411_image::pixel::Pixel::Rgb(_rgb) => exit(1)
        }
    }
    let n: i32 = 9;
    let m: i32 = 9;
    
    let a = Arr::from_row_major(pixel_values, n);
    let mut row_iter = a.iter_row_major();
    for _i in 0..n {
        let mut temp: Vec<i32> = Vec::new();
        for _j in 0..m {
            temp.push(row_iter.next().unwrap());
        }
        if !check(temp) {
            exit(1);
        }
    }
    let mut col_iter = a.iter_col_major();
    for _i in 0..n {
        let mut temp: Vec<i32> = Vec::new();
        for _j in 0..m {
            temp.push(col_iter.next().unwrap());
        }
        if !check(temp) {
            exit(1);
        }
    }
}
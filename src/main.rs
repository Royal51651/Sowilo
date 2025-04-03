use image::GenericImageView;
use std::io::{self, Write};

fn calculate_saturation(red: u8, green: u8, blue: u8) -> f32 { 
    let normal_red = red as f32 / 255.0;
    let normal_green = green as f32 / 255.0;
    let normal_blue = blue as f32 / 255.0;

    let mut max: f32 = 0.0;
    let mut min: f32 = 360.0;

    if max < normal_red {
        max = normal_red;
    }
    if min > normal_red {
        min = normal_red;
    }

    if max < normal_green {
        max = normal_green;
    }
    if min > normal_green {
        min = normal_green;
    }

    if max < normal_blue {
        max = normal_blue;
    }
    if min > normal_blue {
        min = normal_blue;
    }

    println!("Max: {max} | Min: {min}");

    if(max == min){
        return 0.0;
    }

    let lightness: f32 = (max + min) / 2.0;
    if lightness < 0.5 {
        ((max - min) / (max + min)) * 100.0
    } else {
        ((max - min) / (2.0 - max + min)) * 100.0
    }
}

fn calculate_hue(red: u8, green: u8, blue: u8) -> f32 {
    let normal_red = red as f32 / 255.0;
    let normal_green = green as f32 / 255.0;
    let normal_blue = blue as f32 / 255.0;

    let mut max: f32 = 0.0;
    let mut min: f32 = 360.0;

    if max < normal_red {
        max = normal_red;
    }
    if min > normal_red {
        min = normal_red;
    }

    if max < normal_green {
        max = normal_green;
    }
    if min > normal_green {
        min = normal_green;
    }

    if max < normal_blue {
        max = normal_blue;
    }
    if min > normal_blue {
        min = normal_blue;
    }

    let delta = max - min;
    let mut output: f32;
    if delta == 0.0 {
        return 0.0;
    } else if max == normal_red {
        output = 60.0 * ((normal_green - normal_blue)/ delta);
    } else if max == normal_green {
        output = 60.0 * (2.0 + (normal_blue - normal_red) / delta);
    } else {
        output = 60.0 * (4.0 + (normal_red - normal_green) / delta);
    }

    if output < 0.0 {
        output += 360.0;
    }

    output
}

fn calculate_luminosity(red: u8, green: u8, blue: u8) -> f32 {
    (red as f32 * 0.2126) + (green as f32 * 0.715) + (blue as f32 * 0.0722)
}

fn generate_palette_luminosity(path: &str, colors: u8){
    let img = image::open(path).unwrap();
    let (imgx, imgy) = img.dimensions();
    let mut pixels = Vec::new();
    let mut buffer = image::ImageBuffer::new(imgx, imgy);
    //looping through and creating a 1-dimensional array of each pixel and it's deviance
    for (x, y, pixel) in img.pixels() {
        let luminosity: f32 = calculate_luminosity(pixel[0], pixel[1], pixel[2]); 
        pixels.push((luminosity, x, y, pixel));
    }
    let size = pixels.len() / colors as usize;
    let mut pallete_colors = Vec::new();
    for i in 0..colors {
        let mut red_total: u128 = 0;
        let mut green_total: u128 = 0;
        let mut blue_total: u128 = 0;
        let mut count: u128 = 0;
        for index in (i as usize * size)..((i as usize + 1) * size) {
            red_total += pixels[index].3[0] as u128;
            green_total += pixels[index].3[1] as u128;
            blue_total += pixels[index].3[2] as u128;
            count += 1;
        }
        let total: [u8; 3] = [(red_total / count) as u8, (green_total / count) as u8, (blue_total / count) as u8];
        pallete_colors.push(total);
    }

    // copies the sorted pixel data back to the buffer starting from the top left and working down
    let mut i = 0;
    let mut index = 0;
    for d in 0..(imgy + imgx - 1) {
        for y in (0..=d).rev() {
            let x = d - y;
            if y < imgy && x < imgx {
                if i == size + 1 {
                    index += 1;
                    i = 0;
                }
                buffer.put_pixel(x, y, image::Rgb(pallete_colors[index]));
                i += 1; 
            }
        }
    }

    buffer.save("luminosity_sorted.png").unwrap();
}

fn generate_palette_hue(path: &str, colors: u8){
    let img = image::open(path).unwrap();
    let (imgx, imgy) = img.dimensions();
    let mut buffer = image::ImageBuffer::new(imgx, imgy);
    let number_of_pixels: u128 = imgx as u128 * imgy as u128;
    // calculate the min hue and the max hue
    let mut min_hue: f32 = 360.0;
    let mut max_hue: f32 = 0.0;
    for (_x, _y, pixel) in img.pixels() {
        let hue = calculate_hue(pixel[0], pixel[1], pixel[2]);
        if hue < min_hue {
            min_hue = hue;
        }
        if hue > max_hue {
            max_hue = hue;
        }
    }

    let delta_hue = max_hue - min_hue;
    let mut actual_colors = 0;
    let mut color_list = Vec::new();
    for i in 1..=colors{
        let mut red_total: u128 = 0;
        let mut green_total: u128 = 0;
        let mut blue_total: u128 = 0;
        let mut count: u128 = 0;
        let test: f32 = i as f32;
        for (_x, _y, pixel) in img.pixels() {
            let hue = calculate_hue(pixel[0], pixel[1], pixel[2]);
            if hue >= delta_hue * ((colors as f32 - test) / colors as f32) && hue <= delta_hue * ((colors as f32 - test + 1.0) / colors as f32) {
                red_total += pixel[0] as u128;
                green_total += pixel[1] as u128;
                blue_total += pixel[2] as u128;
                count += 1;
            }
        }
        let average: [u8; 3] = [(red_total / count) as u8, (green_total / count) as u8, (blue_total / count) as u8];
        for y in 0..imgy{ 
            for x in 0..imgx {
                buffer.put_pixel(x, y, image::Rgb(average));
            }
        }
        color_list.push(average);
        actual_colors += 1;
    }
    
    let mut i = 0;
    let mut index = 0;
    let size = number_of_pixels / actual_colors as u128;
    for d in 0..(imgy + imgx - 1) {
        for y in (0..=d).rev() {
            let x = d - y;
            if y < imgy && x < imgx {
                if i == size + 1 {
                    index += 1;
                    i = 0;
                }
                buffer.put_pixel(x, y, image::Rgb(color_list[index]));
                i += 1; 
            }
        }
    }

    buffer.save("hue_sorted.png").unwrap();

}

fn generate_palette_combo(path: &str, hues: u8, luminosities: u8){
    let img = image::open(path).unwrap();
    let (imgx, imgy) = img.dimensions();
    let mut buffer = image::ImageBuffer::new(imgx, imgy);
    let number_of_pixels: u128 = imgx as u128 * imgy as u128;
    // calculate the min hue and the max hue
    let mut min_hue: f32 = 360.0;
    let mut max_hue: f32 = 0.0;
    for (_x, _y, pixel) in img.pixels() {
        let hue = calculate_hue(pixel[0], pixel[1], pixel[2]);
        if hue < min_hue {
            min_hue = hue;
        }
        if hue > max_hue {
            max_hue = hue;
        }
    }

    let delta_hue = max_hue - min_hue;
    let hue_slice_size: f32 = delta_hue / hues as f32;

    // create the 2d vector with pixel data that will store the actual values later on
    let mut hue_slices: Vec<Vec<image::Rgba<u8>>> = Vec::new();
    for _i in 0..hues {
        let lum_vector: Vec<image::Rgba<u8>> = Vec::new();
        hue_slices.push(lum_vector);
    }

    // sort each pixel by it's hue
    for (_x, _y, pixel) in img.pixels() {
        let hue = calculate_hue(pixel[0], pixel[1], pixel[2]);
        let index: usize = (hue / hue_slice_size) as usize;
        if index == hues as usize {
            hue_slices[0].push(pixel);
        } else {
            hue_slices[index].push(pixel);
        }
        
    }

    let mut color_list: Vec<image::Rgba<u8>> = Vec::new();
    let mut actual_colors: u128 = 0;
    for i in hue_slices {
        let mut min_lum: f32 = 360.0;
        let mut max_lum: f32 = 0.0;
        // calculate the maximum and minimum luminosities for the sub-lists
        for pixel in &i {
            let lum = calculate_luminosity(pixel[0], pixel[1], pixel[2]);
            if lum > max_lum {
                max_lum = lum;
            }
            if lum < min_lum {
                min_lum = lum;
            }
        }
        let delta_lum = max_lum - min_lum;
        let lum_slice_size = delta_lum / luminosities as f32;
        // loop through the pixels
        for j in 0..luminosities {
            let mut red_total: u128 = 0;
            let mut green_total: u128 = 0;
            let mut blue_total: u128 = 0;
            let mut count: u128 = 0;
            
            // if they fit in a valid luminosity range, add them to the total;
            for pixel in &i {
                let lum = calculate_luminosity(pixel[0], pixel[1], pixel[2]);
                
                if lum >= lum_slice_size * (j as f32) && lum <= lum_slice_size * (j as f32 + 1.0) {
                    red_total += pixel[0] as u128;
                    green_total += pixel[1] as u128;
                    blue_total += pixel[2] as u128;
                    count += 1;
                }
            }
            if count != 0 {
                color_list.push(image::Rgba([(red_total / count) as u8, (green_total / count) as u8, (blue_total / count) as u8, 255]));
                actual_colors += 1;
            }
        }
    }

    let mut i = 0;
    let mut index = 0;
    let size = number_of_pixels / actual_colors;
    for d in 0..(imgy + imgx - 1) {
        for y in (0..=d).rev() {
            let x = d - y;
            if y < imgy && x < imgx {
                if i == size + 1 {
                    index += 1;
                    i = 0;
                }
                buffer.put_pixel(x, y, color_list[index]);
                i += 1; 
            }
        }
    }
    buffer.save("hue_sorted.png").unwrap();
}

/*

TODO:

add a function to calculate using saturation in addition to hue and luminosity

*/

fn main() {
    let path = String::from("wallpapers.png");
    println!("Generating By Combo");
    //generate_palette_combo(&path,8, 2);
    println!("{}", calculate_saturation(0, 1, 1));
    println!("Done");
}
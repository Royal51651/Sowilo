use image::GenericImageView;
use std::io::{self, Write};

fn calculate_saturation(red: u8, green: u8, blue: u8) -> f32 { 

    0.0
}

fn merge_sort(input: &Vec<(u32, u32, u32, image::Rgba<u8>)>) -> Vec<(u32, u32, u32, image::Rgba<u8>)> {
    // base case
    if input.len() < 2 {
        return input.to_vec();  // Ensure that we return the vector here
    } else {
        // splits the vector into two equal-ish halves
        let size = input.len() / 2;
        let left = merge_sort(&input[0..size].to_vec());
        let right = merge_sort(&input[size..].to_vec());

        // sorts and merges the two halves, then returns
        let merged = merge(&left, &right);
        merged
    }
}

fn merge(left: &Vec<(u32, u32, u32, image::Rgba<u8>)>, right: &Vec<(u32, u32, u32, image::Rgba<u8>)>) -> Vec<(u32, u32, u32, image::Rgba<u8>)> {
    let mut i = 0;
    let mut j = 0;
    let mut output: Vec<(u32, u32, u32, image::Rgba<u8>)> = Vec::new();

    // performing the actual sort, and appending values to the buffer
    while i < left.len() && j < right.len() {
        if left[i].0 < right[j].0 {
            output.push(left[i]);
            i += 1;
        } else {
            output.push(right[j]);
            j += 1;
        }
    }
    // typical merge sort stuff
    while i < left.len() {
        output.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        output.push(right[j]);
        j += 1;
    }

    output
}

fn merge_sort_f32(input: &Vec<(f32, u32, u32, image::Rgba<u8>)>) -> Vec<(f32, u32, u32, image::Rgba<u8>)> {
    // base case
    if input.len() < 2 {
        return input.to_vec();  // Ensure that we return the vector here
    } else {
        // splits the vector into two equal-ish halves
        let size = input.len() / 2;
        let left = merge_sort_f32(&input[0..size].to_vec());
        let right = merge_sort_f32(&input[size..].to_vec());

        // sorts and merges the two halves, then returns
        let merged = merge_f32(&left, &right);
        merged
    }
}

fn merge_f32(left: &Vec<(f32, u32, u32, image::Rgba<u8>)>, right: &Vec<(f32, u32, u32, image::Rgba<u8>)>) -> Vec<(f32, u32, u32, image::Rgba<u8>)> {
    let mut i = 0;
    let mut j = 0;
    let mut output: Vec<(f32, u32, u32, image::Rgba<u8>)> = Vec::new();

    // performing the actual sort, and appending values to the buffer
    while i < left.len() && j < right.len() {
        if left[i].0 < right[j].0 {
            output.push(left[i]);
            i += 1;
        } else {
            output.push(right[j]);
            j += 1;
        }
    }
    // typical merge sort stuff
    while i < left.len() {
        output.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        output.push(right[j]);
        j += 1;
    }

    output
}

fn calculate_hue(red: u8, green: u8, blue: u8) -> f32 {
    let normal_red = red as f32 / 255.0;
    let normal_green = green as f32 / 255.0;
    let normal_blue = blue as f32 / 255.0;

    let max;
    if normal_red > normal_green && normal_red > normal_blue {
        max = normal_red;
    } else if normal_green > normal_red && normal_green > normal_blue {
        max = normal_green;
    } else {
        max = normal_blue;
    }

    let min;   
    if normal_red < normal_green && normal_red < normal_blue {
        min = normal_red;
    } else if normal_green < normal_red && normal_green < normal_blue {
        min = normal_green;
    } else {
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

fn generate_palette(path: &str, colors: u8){
    println!("Making a palette at {colors} size");
    let img = image::open(path).unwrap();
    let (imgx, imgy) = img.dimensions();
    let mut pixels = Vec::new();
    let mut buffer = image::ImageBuffer::new(imgx, imgy);
    //looping through and creating a 1-dimensional array of each pixel and it's deviance
    for (x, y, pixel) in img.pixels() {
        let mut deviance: u32 = 0;
        if 255 > pixel[0] {
            deviance += 255 as u32 - pixel[0] as u32 ;
        } else {
            deviance += pixel[0] as u32 - 255 as u32 ;
        }

        if 255 > pixel[1] {
            deviance += 255 as u32  - pixel[1] as u32 ;
        } else {
            deviance +=  pixel[1] as u32  - 255 as u32 ;
        }

        if 255 > pixel[2] {
            deviance += 255 as u32  - pixel[2] as u32 ;
        } else {
            deviance +=  pixel[2] as u32  - 255 as u32;
        }
        pixels.push((deviance, x, y, pixel));
    }
    // merge sorts the vector based on their deviance from the selected color
    let sorted = merge_sort(&pixels);
    //let sorted = pixels;

    let size = sorted.len() / colors as usize;
    let mut pallete_colors = Vec::new();
    for i in 0..colors {
        let mut red_total: u128 = 0;
        let mut green_total: u128 = 0;
        let mut blue_total: u128 = 0;
        let mut count: u128 = 0;
        for index in (i as usize * size)..((i as usize + 1) * size) {
            red_total += sorted[index].3[0] as u128;
            green_total += sorted[index].3[1] as u128;
            blue_total += sorted[index].3[2] as u128;
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

    buffer.save("palette_sorted.png").unwrap();
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
    // merge sorts the vector based on their deviance from the selected color
    let sorted = merge_sort_f32(&pixels);
    //let sorted = pixels;

    let size = sorted.len() / colors as usize;
    let mut pallete_colors = Vec::new();
    for i in 0..colors {
        let mut red_total: u128 = 0;
        let mut green_total: u128 = 0;
        let mut blue_total: u128 = 0;
        let mut count: u128 = 0;
        for index in (i as usize * size)..((i as usize + 1) * size) {
            red_total += sorted[index].3[0] as u128;
            green_total += sorted[index].3[1] as u128;
            blue_total += sorted[index].3[2] as u128;
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
    let mut min_lum: f32 = 360.0;
    let mut max_lum: f32 = 0.0;
    for (_x, _y, pixel) in img.pixels() {
        let hue = calculate_hue(pixel[0], pixel[1], pixel[2]);
        let lum = calculate_luminosity(pixel[0], pixel[1], pixel[2]);
        if hue < min_hue {
            min_hue = hue;
        }
        if hue > max_hue {
            max_hue = hue;
        }
        if lum < min_lum {
            min_lum = lum;
        }
        if lum > max_lum {
            max_lum = lum;
        }
    }

    let delta_hue = max_hue - min_hue;
    let delta_lum = max_lum - min_lum;
    let mut actual_colors = 0;
    let mut color_list = Vec::new();
    // loop through each hue
    for i in 1..=hues{
        // loop through each luminosity
        for j in 1..=luminosities{
            let mut red_total: u128 = 0;
            let mut green_total: u128 = 0;
            let mut blue_total: u128 = 0;
            let mut count: u128 = 0;
            let hue_mod: f32 = i as f32;
            let lum_mod: f32 = j as f32;
            for (_x, _y, pixel) in img.pixels() {
                let hue = calculate_hue(pixel[0], pixel[1], pixel[2]);
                let lum = calculate_luminosity(pixel[0], pixel[1], pixel[2]);
                // first conditional checks hue, second checks luminsotiy
                if (hue >= delta_hue * ((hues as f32 - hue_mod) / hues as f32) && hue <= delta_hue * ((hues as f32 - hue_mod + 1.0) / hues as f32)) && (lum >= delta_lum * ((luminosities as f32 - lum_mod) / lum as f32) && lum <= delta_lum * ((luminosities as f32 - lum_mod + 1.0) / luminosities as f32)){
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

/*

TODO:

Fix the combo algorithm so they stop being O(w * l * h * b) (Create a vector that stores the needed categories instead of iterating through and checking only for one category) [ ]

Remove the parts of code that still have to do with sorting, as they aren't needed for this algo [ ]
*/

fn main() {
    let path = String::from("/Users/randallcandlewax/Sowilo/moon.webp");
    println!("Generating By Combo");
    generate_palette_combo(&path,32, 2);
    println!("Done");
}
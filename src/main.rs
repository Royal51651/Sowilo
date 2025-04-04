use image::GenericImageView;
use core::num;
use std::io::{self, Write};

type Buf = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
type Color = image::Rgba<u8>;

const WHITE: Color = image::Rgba([255, 255, 255, 255]);
const BLACK: Color = image::Rgba([0, 0, 0, 255]);
const BLANK: Color = image::Rgba([0, 0, 0, 0]);
const SOWILO: Color = image::Rgba([5, 154, 173, 255]);

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

    if max == min {
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

fn generate_palette_combo(path: &str, hues: u32, luminosities: u32) -> Vec<Vec<image::Rgba<u8>>> {
    let img = image::open(path).unwrap();
    let (imgx, imgy) = img.dimensions();
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

    let mut color_list: Vec<Vec<image::Rgba<u8>>> = Vec::new();
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
        let mut sub_list: Vec<image::Rgba<u8>> = Vec::new();
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
                sub_list.push(image::Rgba([(red_total / count) as u8, (green_total / count) as u8, (blue_total / count) as u8, 255]));        
            } else {
                sub_list.push(BLACK);
            }
            actual_colors += 1;
        }
        color_list.push(sub_list);
    }
    color_list
}

fn generate_palette_hue_and_sat(path: &str, hues: u32, saturations: u32) -> Vec<Vec<image::Rgba<u8>>> {
    let img = image::open(path).unwrap();
    let (imgx, imgy) = img.dimensions();
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

    let mut color_list: Vec<Vec<image::Rgba<u8>>> = Vec::new();
    let mut actual_colors: u128 = 0;
    for i in hue_slices {
        let mut min_sat: f32 = 360.0;
        let mut max_sat: f32 = 0.0;
        // calculate the maximum and minimum luminosities for the sub-lists
        for pixel in &i {
            let sat = calculate_saturation(pixel[0], pixel[1], pixel[2]);
            if sat > max_sat {
                max_sat = sat;
            }
            if sat < min_sat {
                min_sat = sat;
            }
        }
        let delta_sat = max_sat - min_sat;
        let sat_slice_size = delta_sat / saturations as f32;
        // loop through the pixels
        let mut sub_list: Vec<image::Rgba<u8>> = Vec::new();
        for j in 0..saturations {
            let mut red_total: u128 = 0;
            let mut green_total: u128 = 0;
            let mut blue_total: u128 = 0;
            let mut count: u128 = 0;
            
            // if they fit in a valid luminosity range, add them to the total;
            for pixel in &i {
                let sat = calculate_saturation(pixel[0], pixel[1], pixel[2]);
                
                if sat >= sat_slice_size * (j as f32) && sat <= sat_slice_size * (j as f32 + 1.0) {
                    red_total += pixel[0] as u128;
                    green_total += pixel[1] as u128;
                    blue_total += pixel[2] as u128;
                    count += 1;
                }
            }
            if count != 0 {
                sub_list.push(image::Rgba([(red_total / count) as u8, (green_total / count) as u8, (blue_total / count) as u8, 255]));        
            } else {
                sub_list.push(BLACK);
            }
            actual_colors += 1;
        }
        color_list.push(sub_list);
    }
    color_list
}

fn draw_pixel(x: u32, y: u32, color: [u8; 4], input: &mut Buf){
    let pixel = input.get_pixel_mut(x, y);
    *pixel = image::Rgba(color);

}

fn draw_line_horizontal(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: [u8; 4], input: &mut Buf){
    if x0 > x1{
        x0 ^= x1;
        x1 ^= x0;
        x0 ^= x1;
        y0 ^= y1;
        y1 ^= y0;
        y0 ^= y1;
    }
    let mut dx = x1 - x0;
    let mut dy = y1 - y0;

    let dir;
    if(dy < 0){
        dir = -1;
    } else {
        dir = 1;
    }
    dy *= dir;

    if dx != 0{
        let mut y = y0;
        let mut p = 2 * dy - dx;
        for i in 0..(dx + 1){
            draw_pixel(x0 as u32 + i as u32, y as u32, color, input);
            if p >= 0{
                y += dir;
                p = p - 2*dx;
            }
            p = p + 2*dy;
        }
    }
}

fn draw_line_vertical(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: [u8; 4], input: &mut Buf){
    if y0 > y1{
        x0 ^= x1;
        x1 ^= x0;
        x0 ^= x1;
        y0 ^= y1;
        y1 ^= y0;
        y0 ^= y1;
    }
    let mut dx = x1 - x0;
    let mut dy = y1 - y0;

    let dir;
    if(dx < 0){
        dir = -1;
    } else {
        dir = 1;
    }
    dx *= dir;

    if dy != 0{
        let mut x = x0;
        let mut p = 2 * dx - dy;
        for i in 0..(dy + 1){
            draw_pixel(x as u32, y0 as u32 + i as u32, color, input);
            if p >= 0{
                x += dir;
                p = p - 2*dy;
            }
            p = p + 2*dx;
        }
    }
}

fn draw_line(x0: u32, y0: u32, x1: u32, y1: u32, color: [u8; 4], input: &mut Buf){
    let dx: i32 = x1 as i32 - x0 as i32;
    let dy: i32 = y1 as i32 - y0 as i32;
    if(dx * dx > dy * dy){
        draw_line_horizontal(x0 as i32, y0 as i32, x1 as i32, y1 as i32, color, input);
    } else {
        draw_line_vertical(x0 as i32, y0 as i32, x1 as i32, y1 as i32, color, input);
    }
}

fn draw_rectangle(start_x: u32, start_y: u32, width: u32, height: u32, color: image::Rgba<u8>, input: &mut Buf) {
    for y in 0..height {
        for x in 0..width {
            let pixel = input.get_pixel_mut(start_x + x, start_y + y);
            *pixel = color;
        }
    }
}

fn distance(x1: u32, y1: u32, x2: u32, y2: u32) -> f32{
    let dx = x1 as i32 - x2 as i32;
    let dy = y1 as i32 - y2 as i32;
    (((dx * dx) + (dy * dy)) as f32).sqrt()
}

fn draw_circle(start_x: u32, start_y: u32, radius: u32, color: [u8; 4], input: &mut Buf){
    for y in (start_y - radius)..=(start_y + radius){
        for x in (start_x - radius)..=(start_y + radius){
            if distance(start_x, start_y, x, y) < radius as f32 {
                draw_pixel(x, y, color, input);
            }
        }
    }
}

// allow 128 pixels of padding
fn format_output(input: Vec<Vec<image::Rgba<u8>>>, override_spacing: bool){

    let mut spacing: u32 = 1920 / (input.len() as u32 * input.len() as u32);
    if spacing > 64 && !override_spacing {
        spacing = 64;
    } else if spacing < 4 || override_spacing {
        spacing = 0;
    }
    
    let square_height: u32 = (1920 - (spacing * (input.len() as u32 - 1))) / input.len() as u32;
    let mut output: Buf = image::ImageBuffer::new(2048, 2048);
    // black background
    draw_rectangle(0, 0, 2048, 2048, BLACK, &mut output);
    // sample image data
    for y in 0..input.len() as u32 {
        let square_width = (1920) / (input[y as usize]).len() as u32;
        for x in 0..input[y as usize].len() as u32 {
            draw_rectangle(64 + (square_width * x), 64 + (square_height * y) + (spacing * y), square_width, square_height, input[y as usize][x as usize], &mut output);
        }
    }
    output.save("hue_sorted.png").unwrap();
}
/*

TODO:

add a function to calculate using saturation in addition to hue and luminosity

*/

fn main() {
    let size: u32 = 8;
    let path = String::from("starry_night.jpg");
    println!("Generating By Combo");
    let colors = generate_palette_hue_and_sat(&path,size, size);
    //println!("{}", calculate_saturation(0, 1, 1));
    format_output(colors, true);
    println!("Done");
}
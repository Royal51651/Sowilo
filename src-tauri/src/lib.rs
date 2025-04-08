// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use image::{DynamicImage, GenericImageView, ImageEncoder, ExtendedColorType::Rgba8};
use image::codecs::png;
use base64::{Engine as _, engine::general_purpose};

type Buf = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
type Color = image::Rgba<u8>;
type SortingFn = fn(u8, u8, u8) -> f32;

const BLACK: Color = image::Rgba([0, 0, 0, 255]);

fn merge_sort(input: &Vec<Color>, sort_method: SortingFn) -> Vec<Color> {
    // base case
    if input.len() < 2 {
        return input.to_vec();  // Ensure that we return the vector here
    } else {
        // splits the vector into two equal-ish halves
        let size = input.len() / 2;
        let left = merge_sort(&input[0..size].to_vec(), sort_method);
        let right = merge_sort(&input[size..].to_vec(), sort_method);

        // sorts and merges the two halves, then returns
        let merged: Vec<Color> = merge(&left, &right, sort_method);
        merged
    }
}

fn calculate_vibrancy(red: u8, green: u8, blue: u8) -> f32{
    if green > red && green >= blue {
        green as f32 - (blue as f32 / 2.0 + red as f32 / 2.0)
    } else if blue >= red && blue >= green {
        blue as f32 - (green as f32 / 2.0 + red as f32 / 2.0)
    } else {
        red as f32 - (blue as f32 / 2.0 + green as f32 / 2.0)
    }
    
}

fn calculate_colorfulness(red: u8, green: u8, blue: u8) -> f32 {
   2.0 * calculate_saturation(red, green, blue) * calculate_luminosity(red, green, blue)
}

fn merge(left: &Vec<Color>, right: &Vec<Color>, sort_method: SortingFn) -> Vec<Color> {
    let mut i = 0;
    let mut j = 0;
    let mut output: Vec<Color> = Vec::new();

    // performing the actual sort, and appending values to the buffer
    while i < left.len() && j < right.len(){
        let left_value: f32 = sort_method(left[i][0], left[i][1], left[i][2]);
        let right_value: f32 = sort_method(right[j][0], right[j][1], right[j][2]);
        if left_value < right_value {
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

fn calculate_value(red: u8, green: u8, blue: u8) -> f32 {
    if red >= green && red >= blue {
        red as f32 / 255.0
    } else if green > red && green > blue {
        green as f32 / 255.0
    } else {
        blue as f32 / 255.0
    }
}

fn generate_palette(img: DynamicImage, hues: u32, saturations: u32) -> Vec<Vec<image::Rgba<u8>>> {
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
        let mut index: usize = (hue / hue_slice_size) as usize;
        while index >= hues as usize {
            index -= hues as usize;
        }
        hue_slices[index].push(pixel);
        
    }
    let mut color_list: Vec<Vec<image::Rgba<u8>>> = Vec::new();
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
        let delta_sat = max_sat as f64 - min_sat as f64;
        let sat_slice_size = delta_sat / saturations as f64;
        
        let mut saturation_slices: Vec<[u128; 4]> = Vec::new();

        for _i in 0..saturations {
            saturation_slices.push([0, 0, 0, 0]);
        }

        for pixel in &i {
            let sat = calculate_saturation(pixel[0], pixel[1], pixel[2]) as f64;
            let mut index: usize = ((sat as f64 - min_sat as f64) / sat_slice_size as f64) as usize;
            if sat == max_sat as f64 {
                index = saturations as usize- 1;
            }
            saturation_slices[index][0] += pixel[0] as u128;
            saturation_slices[index][1] += pixel[1] as u128;
            saturation_slices[index][2] += pixel[2] as u128;
            saturation_slices[index][3] += 1;
        }

        // loop through the pixels
        let mut sub_list: Vec<image::Rgba<u8>> = Vec::new();
        for i in saturation_slices {
            let count = i[3];
            if i[3] != 0 {
                sub_list.push(image::Rgba([(i[0] / count) as u8, (i[1] / count) as u8, (i[2] / count) as u8, 255]));        
            } else {
                sub_list.push(BLACK);
            }
        }
        color_list.push(sub_list);
    }

    color_list
}

fn draw_rectangle(start_x: u32, start_y: u32, width: u32, height: u32, color: image::Rgba<u8>, input: &mut Buf) {
    for y in 0..height {
        for x in 0..width {
            let pixel = input.get_pixel_mut(start_x + x, start_y + y);
            *pixel = color;
        }
    }
}

// allow 128 pixels of padding
fn format_output(input: Vec<Vec<image::Rgba<u8>>>, output_size: u32) -> String {   
    let square_height: u32 = output_size / input.len() as u32;
    let square_width = output_size / (input[0]).len() as u32;
    let mut output: Buf = image::ImageBuffer::new(output_size, output_size);

    for y in 0..input.len() as u32 {        
        for x in 0..input[y as usize].len() as u32 {
            draw_rectangle(square_width * x, square_height * y, square_width, square_height, input[y as usize][input.len() - 1 - x as usize], &mut output);
        }
    }

    let mut output_bytes = Vec::new();
    let encoder = png::PngEncoder::new(&mut output_bytes);
    encoder.write_image(&output, output_size, output_size, Rgba8).unwrap();
    let output_b64 = general_purpose::STANDARD.encode(output_bytes);
    output_b64

}
/*

TODO:

add a function to calculate using saturation in addition to hue and luminosity

*/
#[tauri::command]
fn process(input: &str, size: u32, sort_type: &str, output_size: u32) -> String {

    let decoded_image_bytes: Vec<u8> = general_purpose::STANDARD.decode(input).unwrap();
    // loading the image from memory
    let img = image::load_from_memory(&decoded_image_bytes).unwrap();
    
    let colors = generate_palette(img, size, size);
    let mut new_list: Vec<Vec<Color>> = Vec::new();
    for sub_list in &colors{
        if sort_type == "Colorfulness" {
            new_list.push(merge_sort(sub_list, calculate_colorfulness));
        } else if sort_type == "Luminosity" {
            new_list.push(merge_sort(sub_list, calculate_luminosity));
        } else if sort_type == "Value" {
            new_list.push(merge_sort(sub_list, calculate_value));
        } else if sort_type == "Vibrancy" {
            new_list.push(merge_sort(sub_list, calculate_vibrancy));
        } else if sort_type == "Unsorted" {
            new_list.push((sub_list).to_vec());
        }
        
    }
    //println!("{}", calculate_saturation(0, 1, 1));
    format_output(new_list, output_size)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

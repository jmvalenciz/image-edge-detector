//Gaussian blur:
//https://www.youtube.com/watch?v=C_zFhWdM4ic

//Sobel operator (Edge detection):
//https://www.youtube.com/watch?v=uihBwtPIBxM

mod config;

use clap::Parser;
use image::{Luma, Rgb, ImageBuffer};
use image::io::Reader as ImageReader;

use self::config::Config;

fn main() {
    // read args
    let config: Config = Config::parse();

    let image_path = config.input.as_str();
    let output_path = config.output.as_str();
   
    // Create image, set blur and transform to 8-bit gray image
    let img: image::ImageBuffer<Luma<u8>, Vec<u8>> = ImageReader::open(image_path)
        .expect("Unable To read image")
        .decode()
        .expect("Unable to decode image")
        .blur(config.blur)
        .into_luma8();
    
    // Output image
    let mut result: image::ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(img.width(), img.height());

    get_sobel_edges(&img, &mut result, config.color, config.noise_gate);

    result.save(output_path).expect("Unable to save result image");
    
}

// Apply 3x3 filter to specific pixel and return new pixel value
fn calculate_pixel_3x3(i:u32, j: u32, img: &ImageBuffer<Luma<u8>, Vec<u8>>, filter:&[f32;9])->f32{
    let mut g: f32 = 0.0;

    g += filter[0]*img[(i-1, j-1)].0[0] as f32;
    g += filter[1]*img[(i-1, j  )].0[0] as f32;
    g += filter[2]*img[(i-1, j+1)].0[0] as f32;
    g += filter[3]*img[(i,   j-1)].0[0] as f32;
    g += filter[4]*img[(i,   j  )].0[0] as f32;
    g += filter[5]*img[(i,   j+1)].0[0] as f32;
    g += filter[6]*img[(i+1, j-1)].0[0] as f32;
    g += filter[7]*img[(i+1, j  )].0[0] as f32;
    g += filter[8]*img[(i+1, j+1)].0[0] as f32;
    g
}

fn get_sobel_edges(
    in_img: &image::ImageBuffer<Luma<u8>,Vec<u8>>,
    out_img: &mut image::ImageBuffer<Rgb<u8>,Vec<u8>>,
    color: bool,
    noise_gate: f32){
   
    // Sobel operator on x direction
    let x_sobel: [f32;9] = [
        -1.0, 0.0, 1.0,
        -2.0, 0.0, 2.0,
        -1.0, 0.0, 1.0
    ];

    // Sobel operator on y direction
    let y_sobel: [f32;9] = [
        -1.0, -2.0, -1.0,
         0.0,  0.0,  0.0,
         1.0,  2.0,  1.0
    ];
    let width = in_img.width();
    let height = in_img.height();

    for j in 1..height-1{
        for i in 1..width-1{

            let g_x = calculate_pixel_3x3(i, j, in_img, &x_sobel);
            let g_y = calculate_pixel_3x3(i, j, in_img, &y_sobel);

            // apply Pythagorean theorem to get the final value of the pixel
            let g_final = (g_x*g_x + g_y*g_y).sqrt();

            // get the direction of the border using arctan function
            let angle = (g_y/g_x).atan();
            let (mut r, mut g, mut b) = (0.0,0.0,0.0);
            
            // check noise gate to remove pixels with low intensity
            if g_final >= noise_gate{
                if color {
                    (r,g,b) = (
                        (g_final/255.0)*256.0*(angle).cos(),
                        (g_final/255.0)*256.0*(angle + 120.0).cos(),
                        (g_final/255.0)*256.0*(angle - 120.0).cos()
                    );
                }
                else{
                    (r,g,b) = ( g_final, g_final, g_final);
                }
            }
            out_img[(i,j)] = Rgb([r as u8, g as u8, b as u8]);
        }
    }
}

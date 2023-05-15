use image::{GenericImageView};

struct CrawlChunk {
    // this is struct for the 2x2 chunks that I'll use to go through the image
    start_pixel: (u32, u32),
    pixels: [[RgbaPixel;2];2]
    
}

struct RgbaPixel {
    // this is a struct for storing the rgba values
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

fn get_image(dir: &str, scale: u32) -> (image::DynamicImage, u32, u32) {
    // this function takes a directory and scale and will return the DynamicImage data and the width and height of function as u32 integers
    let img = image::open(dir).unwrap();
    let width = &img.dimensions().0;
    let height = &img.dimensions().1;

    (img, *width, *height)
        
}

fn chunk(x: u32, y: u32, img: image::DynamicImage) -> CrawlChunk{ 
    // this function will get and return the RGBA values for a 2x2 'chunk' of pixels in an image in the form of an instance of CrawlChunk
    CrawlChunk {
        start_pixel: (x,y),
        pixels: [
            [
                RgbaPixel {
                    r: img.get_pixel(x,y)[0], g: img.get_pixel(x,y)[1], b: img.get_pixel(x,y)[2], a:img.get_pixel(x,y)[3]
                },
                RgbaPixel {
                    r: img.get_pixel(x+1,y)[0], g: img.get_pixel(x+1,y)[1], b: img.get_pixel(x+1,y)[2], a:img.get_pixel(x+1,y)[3]
                }
            ],
            [
                RgbaPixel {
                r: img.get_pixel(x,y+1)[0], g: img.get_pixel(x,y+1)[1], b: img.get_pixel(x,y+1)[2], a:img.get_pixel(x,y+1)[3]
                },
                RgbaPixel {
                    r: img.get_pixel(x+1,y+1)[0], g: img.get_pixel(x+1,y+1)[1], b: img.get_pixel(x+1,y+1)[2], a:img.get_pixel(x+1,y+1)[3]
                }
            ]
        ]
    }
}

fn main() {
    let (image, width, height) = get_image("testImage.png",1);
    println!("x: {:?}\ny: {}\nz: {}", image, width, height);
    
    for x in (0..width).step_by(2) {
        for y in (0..height).step_by(2) {
            let chunk: CrawlChunk = chunk(x, y, image.clone());
            
            println!("Chunk start pixel: {:?}\n",&chunk.start_pixel);
            // for row1 in 0..2 {
            //     for row2 in 0..2 {
            //         for rgba in 0..4 {
            //             if rgba == 0 {
            //                 println!("Pixel r: {:?}",chunk.pixels[row1][row2].r);

            //             } else if rgba == 1 {
            //                 println!("Pixel g: {:?}",chunk.pixels[row1][row2].g);
                                
            //             } else if rgba == 2 {
            //                 println!("Pixel b: {:?}",chunk.pixels[row1][row2].b);
                                
            //             } else if rgba == 3 {
            //                 println!("Pixel a: {:?}\n\n",chunk.pixels[row1][row2].a);
                                
            //             } 
                        
            //         }
            //     }
                
            // }

        }
        
    }
    
}

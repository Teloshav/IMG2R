use image::{GenericImageView};

struct CrawlChunk {
    start_pixel: (u32, u32),
    pixels: [[RgbaPixel;2];2]
    
}

struct RgbaPixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    let (width, height) = img.dimensions();

    for x in (0..width).step_by(2) {
        for y in (0..height).step_by(2) {
            

            let chunk = CrawlChunk {
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
            };

        }
        
    }
        
}

fn main() {
    get_image("testImage.png",1);
    
    
}

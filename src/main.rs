use image::{GenericImageView};

fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    println!("{:?}",img.dimensions());
    let (width, height) = img.dimensions();
    for x in 0..height {
        for y in 0..width {
            println!("x: {}, y: {}, {:?}",x,y,img.get_pixel(x,y));
        }
    }
}

fn main() {
    get_image("testImage.png",1);
    
}

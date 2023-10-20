use image::GenericImageView;

fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let (width, height) = img.dimensions();
}

fn main() {
    get_image("leaf.png", 4);
}

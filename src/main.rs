// Import necessary libraries
//use std::f32::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

// Define a vector struct for 3D space
#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

// Define color and camera settings
#[derive(Copy, Clone)]
struct Color(u8, u8, u8);

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    // Vector operations needed for ray tracing
    fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalized(self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

// Ray struct and sphere intersection
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Sphere {
    center: Vec3,
    radius: f32,
    color: Color,
}

impl Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // Ray-sphere intersection logic
        let oc = Vec3::new(
            ray.origin.x - self.center.x,
            ray.origin.y - self.center.y,
            ray.origin.z - self.center.z,
        );
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}

// Rendering the flight path
fn render(width: u32, height: u32) -> Vec<u8> {
    let mut img = vec![255; (width * height * 3) as usize];

    // Sphere representing Earth
    let earth = Sphere {
        center: Vec3::new(0.0, 0.0, -5.0),
        radius: 2.0,
        color: Color(0, 123, 255),
    };

    // Camera setup
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let viewport_height = 2.0;
    let viewport_width = 2.0 * (width as f32 / height as f32);

    for y in 0..height {
        for x in 0..width {
            let u = (x as f32 + 0.5) / width as f32;
            let v = (y as f32 + 0.5) / height as f32;
            let direction = Vec3::new(
                u * viewport_width - viewport_width / 2.0,
                v * viewport_height - viewport_height / 2.0,
                -1.0,
            )
            .normalized();

            let ray = Ray {
                origin,
                direction,
            };

            // Ray-sphere intersection check
            if let Some(_) = earth.intersect(&ray) {
                let idx = ((y * width + x) * 3) as usize;
                img[idx] = earth.color.0;
                img[idx + 1] = earth.color.1;
                img[idx + 2] = earth.color.2;
            }
        }
    }

    img
}

fn save_image(filename: &str, width: u32, height: u32, data: &[u8]) {
    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut png_writer = encoder.write_header().unwrap();
    png_writer.write_image_data(data).unwrap();
}

fn main() {
    let width = 800;
    let height = 600;
    let image_data = render(width, height);
    save_image("flight_paths.png", width, height, &image_data);
}

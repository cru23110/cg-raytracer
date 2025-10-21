mod vector;
mod ray;
mod camera;
mod material;
mod light;
mod sphere;
mod plane;
mod cube;
mod scene;
mod renderer;

use std::path::Path;
use image::{ImageBuffer, Rgb};

use vector::{Vec3, Color, Point3};
use camera::Camera;
use material::Material;
use light::Light;
use sphere::Sphere;
use plane::Plane;
use cube::Cube;
use scene::Scene;
use renderer::Renderer;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_DEPTH: u32 = 5;

fn main() {
    println!("🎨 Raytracer - Iniciando renderizado...");
    println!("Resolución: {}x{}", WIDTH, HEIGHT);

    // Crear cámara
    let camera = Camera::new(
        Point3::new(5.0, 3.0, 5.0),  // posición
        Point3::new(0.0, 0.5, 0.0),  // mira a
        Vec3::new(0.0, 1.0, 0.0),    // vector arriba
        45.0,                         // FOV
        WIDTH as f32 / HEIGHT as f32, // aspect ratio
        WIDTH,
        HEIGHT,
    );

    // Crear escena
    let mut scene = Scene::new(camera, Color::new(0.1, 0.15, 0.2)); // fondo azul oscuro

    // Agregar luces
    scene.add_light(Light::white(Point3::new(5.0, 5.0, 5.0), 1.2));
    scene.add_light(Light::white(Point3::new(-5.0, 4.0, 3.0), 0.8));

    // Agregar piso (plano)
    scene.add_plane(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::diffuse(Color::new(0.8, 0.8, 0.8)),
    ));

    // Agregar esferas con diferentes materiales
    // Esfera roja (difusa)
    scene.add_sphere(Sphere::new(
        Point3::new(-2.0, 0.5, 0.0),
        0.8,
        Material::diffuse(Color::new(1.0, 0.3, 0.3)),
    ));

    // Esfera verde (con especularidad)
    scene.add_sphere(Sphere::new(
        Point3::new(0.0, 0.5, -1.5),
        0.8,
        Material::shiny(Color::new(0.3, 1.0, 0.3)),
    ));

    // Esfera azul (reflectante)
    scene.add_sphere(Sphere::new(
        Point3::new(2.0, 0.8, 0.5),
        0.8,
        Material::reflective(Color::new(0.3, 0.3, 1.0)),
    ));

    // Agregar cubo (preparación para Fase 2)
    scene.add_cube(Cube::centered(
        Point3::new(-1.5, 1.5, 2.0),
        1.0,
        Material::shiny(Color::new(1.0, 1.0, 0.3)),
    ));

    // Renderizar escena
    println!("Renderizando escena...");
    let mut framebuffer: Vec<Vec<Color>> = vec![vec![Color::zero(); WIDTH as usize]; HEIGHT as usize];

    let start = std::time::Instant::now();

    for y in 0..HEIGHT {
        if y % 60 == 0 {
            let percentage = (y as f32 / HEIGHT as f32) * 100.0;
            println!("  Progreso: {:.1}%", percentage);
        }

        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;

            let ray = scene.camera.get_ray(u, v);
            let color = Renderer::trace_ray(&ray, &scene, MAX_DEPTH);
            framebuffer[y as usize][x as usize] = color;
        }
    }

    let elapsed = start.elapsed();
    println!("✓ Renderizado completado en {:.2}s", elapsed.as_secs_f32());

    // Guardar imagen
    println!("Guardando imagen...");
    save_image(&framebuffer, "src/output/scene.png").expect("Error al guardar la imagen");
    println!("✓ Imagen guardada en: src/output/scene.png");
}

/// Convierte un color (0.0-1.0) a RGB (0-255)
fn color_to_rgb(color: Color) -> Rgb<u8> {
    let r = (color.x * 255.0).clamp(0.0, 255.0) as u8;
    let g = (color.y * 255.0).clamp(0.0, 255.0) as u8;
    let b = (color.z * 255.0).clamp(0.0, 255.0) as u8;
    Rgb([r, g, b])
}

/// Guarda el framebuffer como una imagen PNG
fn save_image(framebuffer: &[Vec<Color>], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let height = framebuffer.len() as u32;
    let width = if height > 0 { framebuffer[0].len() as u32 } else { 0 };

    let mut img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let color = framebuffer[y as usize][x as usize];
            let rgb = color_to_rgb(color);
            img.put_pixel(x, y, rgb);
        }
    }

    // Crear directorio si no existe
    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    img.save(path)?;
    Ok(())
}

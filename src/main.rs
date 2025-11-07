mod vector;
mod ray;
mod camera;
mod material;
mod light;
mod sphere;
mod plane;
mod cube;
mod pyramid;
mod scene;
mod renderer;
mod texture;
mod layer_parser;

use std::path::Path;
use image::{ImageBuffer, Rgb};

use vector::{Vec3, Color, Point3};
use camera::Camera;
use material::Material;
use light::Light;
use sphere::Sphere;
use plane::Plane;
use cube::Cube;
use pyramid::Pyramid;
use scene::Scene;
use renderer::Renderer;
use texture::Texture;
use layer_parser::{Structure, BlockType};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_DEPTH: u32 = 5;

fn main() {
    println!("🎨 Raytracer - Escena Final: Templo Minecraft");
    println!("Resolución: {}x{}", WIDTH, HEIGHT);

    println!("Cargando estructura desde capas...");
    let structure = match Structure::from_file("guide/temple.txt") {
        Ok(s) => {
            println!("✓ Estructura cargada: {}x{}x{}", s.width(), s.height(), s.depth());
            println!("  Total de capas: {}", s.layers.len());
            s
        }
        Err(e) => {
            println!("Error al cargar estructura: {}", e);
            std::process::exit(1);
        }
    };

    let center_x = structure.width() as f32 / 2.0;
    let center_y = structure.height() as f32 / 2.0;
    let center_z = structure.depth() as f32 / 2.0;

    let camera = Camera::new(
        Point3::new(center_x - 18.0, center_y + 8.0, center_z + 18.0),
        Point3::new(center_x, center_y + 2.0, center_z),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        WIDTH as f32 / HEIGHT as f32,
        WIDTH,
        HEIGHT,
    );

    let mut scene = Scene::new(camera, Color::new(0.6, 0.7, 0.9));

    println!("Cargando texturas...");

    let texture_files = vec![
        ("textures/clearer-glass.png", "glass"),
        ("textures/cobblestone.png", "cobblestone"),
        ("textures/dirt-block.png", "dirt"),
    ];

    for (path, name) in &texture_files {
        match Texture::from_image(path) {
            Ok(tex) => {
                scene.add_texture(tex);
                println!("✓ Textura {} cargada", name);
            }
            Err(_) => {
                scene.add_texture(Texture {
                    width: 1,
                    height: 1,
                    data: vec![vec![Color::new(0.7, 0.7, 0.7)]],
                });
                println!("  Fallback para {}", name);
            }
        }
    }

    scene.add_light(Light::white(Point3::new(center_x, center_y + 15.0, center_z), 2.0));
    scene.add_light(Light::white(Point3::new(center_x - 10.0, center_y + 8.0, center_z + 10.0), 1.2));
    scene.add_light(Light::white(Point3::new(center_x + 10.0, center_y + 8.0, center_z - 10.0), 1.2));

    println!("Construyendo escena desde capas...");

    let block_size = 1.0;
    let mut added_count = 0;

    for layer in &structure.layers {
        let y = layer.level as f32;

        for (row_idx, row) in layer.grid.iter().enumerate() {
            for (col_idx, block_type) in row.iter().enumerate() {
                let x = col_idx as f32;
                let z = row_idx as f32;

                let position = Point3::new(x + 0.5, y - 0.5, z + 0.5);

                match block_type {
                    BlockType::Cobblestone => {
                        let mut material = Material::diffuse(Color::new(1.0, 1.0, 1.0));
                        material.albedo = 0.9;
                        material.specular = 0.1;
                        material = material.with_texture(1);
                        let cube = Cube::centered(position, block_size, material);
                        scene.add_cube(cube);
                        added_count += 1;
                    }
                    BlockType::Glass => {
                        let mut material = Material::shiny(Color::new(0.95, 0.95, 1.0));
                        material.albedo = 0.3;
                        material.reflectivity = 0.6;
                        material = material.with_texture(0);
                        let cube = Cube::centered(position, block_size, material);
                        scene.add_cube(cube);
                        added_count += 1;
                    }
                    BlockType::Dirt => {
                        let mut material = Material::diffuse(Color::new(1.0, 1.0, 1.0));
                        material.albedo = 0.95;
                        material = material.with_texture(2);
                        let cube = Cube::centered(position, block_size, material);
                        scene.add_cube(cube);
                        added_count += 1;
                    }
                    BlockType::Empty => {}
                }
            }
        }
    }

    println!("✓ Bloques añadidos: {}", added_count);

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
            let v = 1.0 - (y as f32 / HEIGHT as f32);

            let ray = scene.camera.get_ray(u, v);
            let color = Renderer::trace_ray(&ray, &scene, MAX_DEPTH);
            framebuffer[y as usize][x as usize] = color;
        }
    }

    let elapsed = start.elapsed();
    println!("✓ Renderizado completado en {:.2}s", elapsed.as_secs_f32());

    println!("Guardando imagen...");
    save_image(&framebuffer, "src/output/final_scene.png").expect("Error al guardar la imagen");
    println!("✓ Imagen guardada en: src/output/final_scene.png");
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

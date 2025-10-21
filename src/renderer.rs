use crate::vector::{Vec3, Color, Point3};
use crate::ray::Ray;
use crate::scene::Scene;

const EPSILON: f32 = 1e-4;
const MAX_DEPTH: u32 = 5;
const AMBIENT_STRENGTH: f32 = 0.2;

/// Estructura que contiene funciones para renderizar la escena
pub struct Renderer;

impl Renderer {
    /// Encuentra la intersección más cercana entre un rayo y la escena
    pub fn find_closest_intersection<'a>(
        ray: &Ray,
        scene: &'a Scene,
    ) -> Option<(f32, Point3, Vec3, &'a std::boxed::Box<dyn crate::scene::Intersectable>)> {
        if let Some((t, object)) = scene.find_closest_intersection(ray) {
            let hit_point = ray.at(t);
            let normal = object.normal_at(&hit_point);
            Some((t, hit_point, normal, object))
        } else {
            None
        }
    }

    /// Calcula el sombreado de un punto en una superficie
    /// Incluye componentes ambiente, difusa y especular (Phong)
    pub fn shade(
        hit_point: &Point3,
        normal: &Vec3,
        material: &crate::material::Material,
        scene: &Scene,
        view_dir: &Vec3,
    ) -> Color {
        // Componente ambiente
        let ambient = material.color * AMBIENT_STRENGTH;

        let mut color = ambient;

        // Para cada luz en la escena
        for light in &scene.lights {
            let light_dir = (light.position - *hit_point).normalize();

            // Test de sombra: lanzar rayo hacia la luz
            let shadow_ray = Ray::new(*hit_point + *normal * EPSILON, light_dir);
            let distance_to_light = (light.position - *hit_point).length();

            let is_in_shadow = if let Some((t, _, _, _)) = Self::find_closest_intersection(&shadow_ray, scene) {
                t < distance_to_light
            } else {
                false
            };

            if is_in_shadow {
                continue; // Este punto está en sombra, saltamos la luz
            }

            // Componente difusa (Lambertian)
            let diffuse_intensity = normal.dot(&light_dir).max(0.0);
            let diffuse = material.color * diffuse_intensity * material.albedo * light.intensity;

            // Componente especular (Phong)
            let reflected_light = light_dir.reflect(&(-*normal));
            let specular_intensity = reflected_light.dot(view_dir).max(0.0).powf(material.shininess);
            let specular = (light.color * specular_intensity * material.specular) * light.intensity;

            color = color + diffuse + specular;
        }

        color.clamp()
    }

    /// Traza un rayo en la escena de forma recursiva
    /// Maneja iluminación, sombras y reflexiones
    pub fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
        // Caso base: profundidad máxima alcanzada
        if depth == 0 {
            return scene.background_color;
        }

        // Buscar intersección más cercana
        if let Some((_t, hit_point, normal, object)) = Self::find_closest_intersection(ray, scene) {
            let material = object.get_material();

            // Calcular dirección hacia la cámara
            let view_dir = (scene.camera.position - hit_point).normalize();

            // Calcular color local (iluminación directa)
            let mut local_color = Self::shade(&hit_point, &normal, material, scene, &view_dir);

            // Si el material es reflectante, lanzar rayo reflejado
            if material.reflectivity > 0.0 && depth > 1 {
                let reflected_dir = ray.direction.reflect(&normal);
                let reflected_ray = Ray::new(hit_point + normal * EPSILON, reflected_dir);
                let reflected_color = Self::trace_ray(&reflected_ray, scene, depth - 1);

                // Mezclar color local con reflexión
                local_color = local_color * (1.0 - material.reflectivity) + reflected_color * material.reflectivity;
            }

            local_color
        } else {
            // No hay intersección, retornar color de fondo
            scene.background_color
        }
    }
}

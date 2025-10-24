use crate::vector::{Vec3, Color, Point3};
use crate::ray::Ray;
use crate::scene::Scene;

const EPSILON: f32 = 1e-4;
const MAX_DEPTH: u32 = 5;
const AMBIENT_STRENGTH: f32 = 0.2;

pub struct Renderer;

impl Renderer {
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

    pub fn shade(
        hit_point: &Point3,
        normal: &Vec3,
        material: &crate::material::Material,
        scene: &Scene,
        view_dir: &Vec3,
        uv_data: Option<(f32, f32, usize)>,
    ) -> Color {
        let base_color = if let Some((u, v, tex_id)) = uv_data {
            if tex_id < scene.textures.len() {
                scene.textures[tex_id].sample(u, v)
            } else {
                material.color
            }
        } else {
            material.color
        };

        let ambient = base_color * AMBIENT_STRENGTH;
        let mut color = ambient;

        for light in &scene.lights {
            let light_dir = (light.position - *hit_point).normalize();

            let shadow_ray = Ray::new(*hit_point + *normal * EPSILON, light_dir);
            let distance_to_light = (light.position - *hit_point).length();

            let is_in_shadow = if let Some((t, _, _, _)) = Self::find_closest_intersection(&shadow_ray, scene) {
                t < distance_to_light
            } else {
                false
            };

            if is_in_shadow {
                continue;
            }

            let diffuse_intensity = normal.dot(&light_dir).max(0.0);
            let diffuse = base_color * diffuse_intensity * material.albedo * light.intensity;

            let reflected_light = (-light_dir).reflect(normal);
            let specular_intensity = reflected_light.dot(view_dir).max(0.0).powf(material.shininess);
            let specular = (light.color * specular_intensity * material.specular) * light.intensity;

            color = color + diffuse + specular;
        }

        color.clamp()
    }

    pub fn trace_ray(ray: &Ray, scene: &Scene, depth: u32) -> Color {
        if depth == 0 {
            return scene.background_color;
        }

        if let Some((_t, hit_point, normal, object)) = Self::find_closest_intersection(ray, scene) {
            let material = object.get_material();
            let view_dir = (scene.camera.position - hit_point).normalize();
            let uv_data = object.get_uv(&hit_point);
            let mut local_color = Self::shade(&hit_point, &normal, material, scene, &view_dir, uv_data);

            if material.reflectivity > 0.0 && depth > 1 {
                let reflected_dir = ray.direction.reflect(&normal);
                let reflected_ray = Ray::new(hit_point + normal * EPSILON, reflected_dir);
                let reflected_color = Self::trace_ray(&reflected_ray, scene, depth - 1);
                local_color = local_color * (1.0 - material.reflectivity) + reflected_color * material.reflectivity;
            }

            local_color
        } else {
            scene.background_color
        }
    }
}

use crate::vector::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;

/// Estructura que representa una esfera en el espacio 3D
#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    /// Crea una nueva esfera
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// Calcula la intersección entre un rayo y esta esfera
    /// Resuelve: |origin + t*direction - center|^2 = radius^2
    /// Retorna Some(t) si hay intersección, None si no la hay
    /// Solo retorna t > 0 (intersecciones adelante del rayo)
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let t2 = (-b + discriminant_sqrt) / (2.0 * a);

        // Retornar la intersección más cercana que esté adelante del rayo
        if t1 > 1e-4 {
            Some(t1)
        } else if t2 > 1e-4 {
            Some(t2)
        } else {
            None
        }
    }

    /// Calcula la normal en un punto de la superficie de la esfera
    pub fn normal_at(&self, point: &Point3) -> Vec3 {
        (*point - self.center).normalize()
    }

    /// Retorna las coordenadas UV en la esfera (preparación para Fase 3)
    pub fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)> {
        let normal = self.normal_at(point);

        // Mapear la normal a coordenadas UV usando spherical coordinates
        let u = 0.5 + (normal.z.atan2(normal.x) / std::f32::consts::PI * 0.5);
        let v = 0.5 - (normal.y.asin() / std::f32::consts::PI);

        Some((u, v, 0))
    }
}

use crate::vector::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;

/// Estructura que representa un plano infinito en el espacio 3D
/// Ecuación del plano: (P - point) · normal = 0
#[derive(Clone, Copy)]
pub struct Plane {
    pub point: Point3,      // Punto en el plano
    pub normal: Vec3,       // Normal del plano (debe estar normalizada)
    pub material: Material,
}

impl Plane {
    /// Crea un nuevo plano
    pub fn new(point: Point3, normal: Vec3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            material,
        }
    }

    /// Calcula la intersección entre un rayo y este plano
    /// Resuelve: (origin + t*direction - point) · normal = 0
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let denom = ray.direction.dot(&self.normal);

        // Si el rayo es paralelo al plano
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = (self.point - ray.origin).dot(&self.normal) / denom;

        if t > 1e-4 {
            Some(t)
        } else {
            None
        }
    }

    /// Retorna la normal en cualquier punto del plano
    pub fn normal_at(&self, _point: &Point3) -> Vec3 {
        self.normal
    }

    pub fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)> {
        let tangent = if self.normal.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0).cross(&self.normal).normalize()
        } else {
            Vec3::new(1.0, 0.0, 0.0).cross(&self.normal).normalize()
        };

        let bitangent = self.normal.cross(&tangent).normalize();
        let relative_pos = *point - self.point;

        let u = (relative_pos.dot(&tangent) * 0.5) % 1.0;
        let v = (relative_pos.dot(&bitangent) * 0.5) % 1.0;

        Some((u.abs(), v.abs(), 1))
    }
}

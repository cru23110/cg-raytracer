use crate::vector::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;

/// Estructura que representa un cubo alineado con los ejes (AABB)
/// El cubo se define por sus puntos mínimo y máximo en los ejes
#[derive(Clone, Copy)]
pub struct Cube {
    pub min: Point3,        // Esquina mínima (x, y, z más bajos)
    pub max: Point3,        // Esquina máxima (x, y, z más altos)
    pub material: Material,
}

impl Cube {
    /// Crea un nuevo cubo a partir de los puntos mínimo y máximo
    pub fn new(min: Point3, max: Point3, material: Material) -> Self {
        Cube { min, max, material }
    }

    /// Crea un cubo centrado en un punto con un tamaño específico
    pub fn centered(center: Point3, size: f32, material: Material) -> Self {
        let half = size * 0.5;
        Cube {
            min: Point3::new(center.x - half, center.y - half, center.z - half),
            max: Point3::new(center.x + half, center.y + half, center.z + half),
            material,
        }
    }

    /// Calcula la intersección entre un rayo y este cubo usando algoritmo AABB
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let mut t_min = -f32::INFINITY;
        let mut t_max = f32::INFINITY;

        // Intersectar con los tres pares de planos (x, y, z)
        for i in 0..3 {
            let ray_start = match i {
                0 => ray.origin.x,
                1 => ray.origin.y,
                _ => ray.origin.z,
            };

            let ray_dir = match i {
                0 => ray.direction.x,
                1 => ray.direction.y,
                _ => ray.direction.z,
            };

            let min_bound = match i {
                0 => self.min.x,
                1 => self.min.y,
                _ => self.min.z,
            };

            let max_bound = match i {
                0 => self.max.x,
                1 => self.max.y,
                _ => self.max.z,
            };

            if ray_dir.abs() > 1e-6 {
                let t0 = (min_bound - ray_start) / ray_dir;
                let t1 = (max_bound - ray_start) / ray_dir;

                let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

                t_min = t_min.max(t0);
                t_max = t_max.min(t1);

                if t_min > t_max {
                    return None;
                }
            } else if ray_start < min_bound || ray_start > max_bound {
                return None;
            }
        }

        if t_min > 1e-4 {
            Some(t_min)
        } else if t_max > 1e-4 {
            Some(t_max)
        } else {
            None
        }
    }

    /// Calcula la normal en un punto de la superficie del cubo
    pub fn normal_at(&self, point: &Point3) -> Vec3 {
        // Encontrar qué cara del cubo está más cerca del punto
        let dx_min = (point.x - self.min.x).abs();
        let dx_max = (point.x - self.max.x).abs();
        let dy_min = (point.y - self.min.y).abs();
        let dy_max = (point.y - self.max.y).abs();
        let dz_min = (point.z - self.min.z).abs();
        let dz_max = (point.z - self.max.z).abs();

        let min_dist = dx_min.min(dx_max).min(dy_min).min(dy_max).min(dz_min).min(dz_max);

        // Retornar la normal de la cara más cercana
        if (min_dist - dx_min).abs() < 1e-6 {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (min_dist - dx_max).abs() < 1e-6 {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (min_dist - dy_min).abs() < 1e-6 {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (min_dist - dy_max).abs() < 1e-6 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (min_dist - dz_min).abs() < 1e-6 {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        }
    }

    /// Retorna coordenadas UV en la cara del cubo
    /// face_id: 0=X-, 1=X+, 2=Y-, 3=Y+, 4=Z-, 5=Z+
    pub fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)> {
        let epsilon = 1e-4;
        let size_x = self.max.x - self.min.x;
        let size_y = self.max.y - self.min.y;
        let size_z = self.max.z - self.min.z;

        // Determinar en qué cara está el punto
        if (point.x - self.min.x).abs() < epsilon {
            // Cara X-
            let u = (point.z - self.min.z) / size_z;
            let v = (point.y - self.min.y) / size_y;
            Some((u, v, 0))
        } else if (point.x - self.max.x).abs() < epsilon {
            // Cara X+
            let u = 1.0 - (point.z - self.min.z) / size_z;
            let v = (point.y - self.min.y) / size_y;
            Some((u, v, 1))
        } else if (point.y - self.min.y).abs() < epsilon {
            // Cara Y-
            let u = (point.x - self.min.x) / size_x;
            let v = 1.0 - (point.z - self.min.z) / size_z;
            Some((u, v, 2))
        } else if (point.y - self.max.y).abs() < epsilon {
            // Cara Y+
            let u = (point.x - self.min.x) / size_x;
            let v = (point.z - self.min.z) / size_z;
            Some((u, v, 3))
        } else if (point.z - self.min.z).abs() < epsilon {
            // Cara Z-
            let u = (point.x - self.min.x) / size_x;
            let v = (point.y - self.min.y) / size_y;
            Some((u, v, 4))
        } else if (point.z - self.max.z).abs() < epsilon {
            // Cara Z+
            let u = 1.0 - (point.x - self.min.x) / size_x;
            let v = (point.y - self.min.y) / size_y;
            Some((u, v, 5))
        } else {
            None
        }
    }
}

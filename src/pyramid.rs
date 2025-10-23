use crate::vector::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::Material;

/// Estructura que representa una pirámide triangular (tetraedro)
/// Formada por 4 caras triangulares
#[derive(Clone, Copy)]
pub struct Pyramid {
    pub apex: Point3,       // Vértice superior (punta)
    pub base_center: Point3, // Centro de la base
    pub height: f32,        // Altura de la pirámide
    pub base_radius: f32,   // Radio de la base (triángulo equilátero)
    pub material: Material,
}

impl Pyramid {
    /// Crea una nueva pirámide triangular
    pub fn new(apex: Point3, base_center: Point3, height: f32, base_radius: f32, material: Material) -> Self {
        Pyramid {
            apex,
            base_center,
            height,
            base_radius,
            material,
        }
    }

    /// Crea una pirámide centrada en un punto
    pub fn centered(center: Point3, size: f32, material: Material) -> Self {
        let height = size;
        let base_radius = size * 0.6;
        let apex = Point3::new(center.x, center.y + height * 0.5, center.z);
        let base_center = Point3::new(center.x, center.y - height * 0.5, center.z);

        Pyramid {
            apex,
            base_center,
            height,
            base_radius,
            material,
        }
    }

    /// Obtiene los 3 vértices de la base (triángulo equilátero)
    fn get_base_vertices(&self) -> [Point3; 3] {
        let angle1 = 0.0_f32;
        let angle2 = std::f32::consts::PI * 2.0 / 3.0;
        let angle3 = std::f32::consts::PI * 4.0 / 3.0;

        [
            Point3::new(
                self.base_center.x + self.base_radius * angle1.cos(),
                self.base_center.y,
                self.base_center.z + self.base_radius * angle1.sin(),
            ),
            Point3::new(
                self.base_center.x + self.base_radius * angle2.cos(),
                self.base_center.y,
                self.base_center.z + self.base_radius * angle2.sin(),
            ),
            Point3::new(
                self.base_center.x + self.base_radius * angle3.cos(),
                self.base_center.y,
                self.base_center.z + self.base_radius * angle3.sin(),
            ),
        ]
    }

    /// Intersección rayo-triángulo usando algoritmo de Möller-Trumbore
    fn intersect_triangle(&self, ray: &Ray, v0: Point3, v1: Point3, v2: Point3) -> Option<f32> {
        let epsilon = 1e-6;

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < epsilon {
            return None; // Rayo paralelo al triángulo
        }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > epsilon {
            Some(t)
        } else {
            None
        }
    }

    /// Calcula la intersección entre un rayo y la pirámide
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let base_verts = self.get_base_vertices();
        let mut closest_t = f32::INFINITY;

        // Intersección con las 3 caras laterales
        for i in 0..3 {
            let v0 = self.apex;
            let v1 = base_verts[i];
            let v2 = base_verts[(i + 1) % 3];

            if let Some(t) = self.intersect_triangle(ray, v0, v1, v2) {
                if t < closest_t {
                    closest_t = t;
                }
            }
        }

        // Intersección con la base (triángulo)
        if let Some(t) = self.intersect_triangle(ray, base_verts[0], base_verts[1], base_verts[2]) {
            if t < closest_t {
                closest_t = t;
            }
        }

        if closest_t < f32::INFINITY {
            Some(closest_t)
        } else {
            None
        }
    }

    /// Calcula la normal en un punto de la superficie de la pirámide
    pub fn normal_at(&self, point: &Point3) -> Vec3 {
        let base_verts = self.get_base_vertices();
        let epsilon = 1e-4;

        // Calcular el centro de la pirámide para asegurar que las normales apunten hacia afuera
        let center = Point3::new(
            (self.apex.x + self.base_center.x) * 0.5,
            (self.apex.y + self.base_center.y) * 0.5,
            (self.apex.z + self.base_center.z) * 0.5,
        );

        // Verificar si está en la base
        let dist_to_base = (point.y - self.base_center.y).abs();

        if dist_to_base < epsilon {
            return Vec3::new(0.0, -1.0, 0.0); // Base apunta hacia abajo
        }

        // Calcular normal de cada cara lateral y ver cuál es la más cercana
        let mut closest_normal = Vec3::new(0.0, -1.0, 0.0);
        let mut min_distance = f32::INFINITY;

        for i in 0..3 {
            let v0 = self.apex;
            let v1 = base_verts[i];
            let v2 = base_verts[(i + 1) % 3];

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let mut normal = edge1.cross(&edge2).normalize();

            // Asegurar que la normal apunte hacia AFUERA de la pirámide
            let face_center = Point3::new(
                (v0.x + v1.x + v2.x) / 3.0,
                (v0.y + v1.y + v2.y) / 3.0,
                (v0.z + v1.z + v2.z) / 3.0,
            );
            let outward = face_center - center;

            // Si la normal apunta hacia adentro, invertirla
            if normal.dot(&outward) < 0.0 {
                normal = normal * -1.0;
            }

            // Calcular distancia del punto al plano de esta cara
            let to_point = *point - v0;
            let distance = to_point.dot(&normal).abs();

            if distance < min_distance {
                min_distance = distance;
                closest_normal = normal;
            }
        }

        closest_normal.normalize()
    }

    /// Retorna coordenadas UV (preparación para Fase 3)
    pub fn get_uv(&self, _point: &Point3) -> Option<(f32, f32, usize)> {
        // Implementación básica para texturas en Fase 3
        Some((0.0, 0.0, 0))
    }
}

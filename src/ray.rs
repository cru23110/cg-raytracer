use crate::vector::{Point3, Vec3};

/// Estructura que representa un rayo en el espacio 3D
/// Ecuación de rayo: P(t) = origin + t * direction
/// donde t >= 0 representa la distancia a lo largo del rayo
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// Crea un nuevo rayo
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Retorna el punto en el rayo a una distancia t
    /// P(t) = origin + t * direction
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

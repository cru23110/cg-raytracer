use crate::vector::{Point3, Color};

/// Estructura que representa una fuente de luz
#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Point3,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    /// Crea una nueva luz puntual
    pub fn new(position: Point3, color: Color, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }

    /// Luz blanca estándar
    pub fn white(position: Point3, intensity: f32) -> Self {
        Light {
            position,
            color: Color::new(1.0, 1.0, 1.0),
            intensity,
        }
    }
}

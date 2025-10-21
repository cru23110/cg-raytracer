use crate::vector::Color;

/// Estructura que define las propiedades de un material
pub struct Material {
    pub color: Color,
    pub albedo: f32,         // Reflexión difusa (0.0 a 1.0)
    pub specular: f32,       // Componente especular (0.0 a 1.0)
    pub shininess: f32,      // Brillo (exponente de Phong)
    pub reflectivity: f32,   // Nivel de reflexión (0.0 a 1.0)

    // Preparación para Fase 3 (texturas)
    pub has_texture: bool,
    pub texture_id: Option<usize>,
}

impl Material {
    /// Crea un nuevo material
    pub fn new(color: Color) -> Self {
        Material {
            color,
            albedo: 0.8,
            specular: 0.2,
            shininess: 32.0,
            reflectivity: 0.0,
            has_texture: false,
            texture_id: None,
        }
    }

    /// Material difuso opaco (sin reflexión especular)
    pub fn diffuse(color: Color) -> Self {
        Material {
            color,
            albedo: 0.8,
            specular: 0.0,
            shininess: 1.0,
            reflectivity: 0.0,
            has_texture: false,
            texture_id: None,
        }
    }

    /// Material brillante (con especularidad)
    pub fn shiny(color: Color) -> Self {
        Material {
            color,
            albedo: 0.5,
            specular: 0.8,
            shininess: 64.0,
            reflectivity: 0.3,
            has_texture: false,
            texture_id: None,
        }
    }

    /// Material reflectante (espejo)
    pub fn reflective(color: Color) -> Self {
        Material {
            color,
            albedo: 0.1,
            specular: 0.9,
            shininess: 128.0,
            reflectivity: 0.9,
            has_texture: false,
            texture_id: None,
        }
    }

    /// Asignar una textura al material (Fase 3)
    pub fn with_texture(mut self, texture_id: usize) -> Self {
        self.has_texture = true;
        self.texture_id = Some(texture_id);
        self
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Material {
            color: self.color,
            albedo: self.albedo,
            specular: self.specular,
            shininess: self.shininess,
            reflectivity: self.reflectivity,
            has_texture: self.has_texture,
            texture_id: self.texture_id,
        }
    }
}

impl Copy for Material {}

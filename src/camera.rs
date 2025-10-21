use crate::vector::{Point3, Vec3};
use crate::ray::Ray;

/// Estructura de cámara que define la vista y parámetros de renderizado
pub struct Camera {
    pub position: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub width: u32,
    pub height: u32,

    // Vectores internos calculados
    forward: Vec3,
    right: Vec3,
    up_normalized: Vec3,
    viewport_height: f32,
    viewport_width: f32,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    /// Crea una nueva cámara
    pub fn new(
        position: Point3,
        look_at: Point3,
        up: Vec3,
        fov: f32,
        aspect_ratio: f32,
        width: u32,
        height: u32,
    ) -> Self {
        let mut camera = Camera {
            position,
            look_at,
            up: up.normalize(),
            fov,
            aspect_ratio,
            width,
            height,
            forward: Vec3::zero(),
            right: Vec3::zero(),
            up_normalized: Vec3::zero(),
            viewport_height: 0.0,
            viewport_width: 0.0,
            horizontal: Vec3::zero(),
            vertical: Vec3::zero(),
            lower_left_corner: Point3::zero(),
        };

        camera.update_vectors();
        camera
    }

    fn update_vectors(&mut self) {
        // Calcular vectores de la cámara
        self.forward = (self.look_at - self.position).normalize();
        self.right = self.forward.cross(&self.up).normalize();
        self.up_normalized = self.right.cross(&self.forward).normalize();

        // Calcular dimensiones del viewport
        let theta = self.fov.to_radians();
        let h = (theta / 2.0).tan();
        self.viewport_height = 2.0 * h;
        self.viewport_width = self.aspect_ratio * self.viewport_height;

        // Calcular vectores del plano de visión
        self.horizontal = self.right * self.viewport_width;
        self.vertical = self.up_normalized * self.viewport_height;

        // Calcular la esquina inferior izquierda del plano de visión
        self.lower_left_corner =
            self.position +
            self.forward -
            self.horizontal / 2.0 -
            self.vertical / 2.0;
    }

    /// Genera un rayo desde la cámara hacia coordenadas (u, v) del framebuffer
    /// u y v están en el rango [0, 1]
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction =
            self.lower_left_corner +
            self.horizontal * u +
            self.vertical * v -
            self.position;

        Ray::new(self.position, direction.normalize())
    }
}

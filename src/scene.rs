use crate::vector::{Point3, Vec3, Color};
use crate::ray::Ray;
use crate::material::Material;
use crate::light::Light;
use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::plane::Plane;
use crate::cube::Cube;
use crate::pyramid::Pyramid;

/// Trait que define la interfaz común para todos los objetos intersectables
pub trait Intersectable: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn normal_at(&self, point: &Point3) -> Vec3;
    fn get_material(&self) -> &Material;
    fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)>;
}

// Implementar trait para Sphere
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        Sphere::intersect(self, ray)
    }

    fn normal_at(&self, point: &Point3) -> Vec3 {
        Sphere::normal_at(self, point)
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)> {
        Sphere::get_uv(self, point)
    }
}

// Implementar trait para Plane
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        Plane::intersect(self, ray)
    }

    fn normal_at(&self, point: &Point3) -> Vec3 {
        Plane::normal_at(self, point)
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)> {
        Plane::get_uv(self, point)
    }
}

// Implementar trait para Cube
impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        Cube::intersect(self, ray)
    }

    fn normal_at(&self, point: &Point3) -> Vec3 {
        Cube::normal_at(self, point)
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)> {
        Cube::get_uv(self, point)
    }
}

// Implementar trait para Pyramid
impl Intersectable for Pyramid {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        Pyramid::intersect(self, ray)
    }

    fn normal_at(&self, point: &Point3) -> Vec3 {
        Pyramid::normal_at(self, point)
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_uv(&self, point: &Point3) -> Option<(f32, f32, usize)> {
        Pyramid::get_uv(self, point)
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub background_color: Color,
    pub textures: Vec<()>,
}

impl Scene {
    /// Crea una nueva escena vacía
    pub fn new(camera: Camera, background_color: Color) -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            camera,
            background_color,
            textures: Vec::new(),
        }
    }

    /// Agrega un objeto a la escena
    pub fn add_object(&mut self, object: Box<dyn Intersectable>) {
        self.objects.push(object);
    }

    /// Agrega una esfera a la escena
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.objects.push(Box::new(sphere));
    }

    /// Agrega un plano a la escena
    pub fn add_plane(&mut self, plane: Plane) {
        self.objects.push(Box::new(plane));
    }

    /// Agrega un cubo a la escena
    pub fn add_cube(&mut self, cube: Cube) {
        self.objects.push(Box::new(cube));
    }

    /// Agrega una pirámide a la escena
    pub fn add_pyramid(&mut self, pyramid: Pyramid) {
        self.objects.push(Box::new(pyramid));
    }

    /// Agrega una luz a la escena
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Encuentra la intersección más cercana en la escena
    pub fn find_closest_intersection(&self, ray: &Ray) -> Option<(f32, &Box<dyn Intersectable>)> {
        let mut closest_t = f32::INFINITY;
        let mut closest_object: Option<&Box<dyn Intersectable>> = None;

        for object in &self.objects {
            if let Some(t) = object.intersect(ray) {
                if t < closest_t {
                    closest_t = t;
                    closest_object = Some(object);
                }
            }
        }

        closest_object.map(|obj| (closest_t, obj))
    }
}

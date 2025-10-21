/// Estructura de vector 3D utilizada para posiciones, direcciones y colores
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// Alias para mayor claridad semántica
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    /// Crea un nuevo vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    /// Vector cero
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Retorna la magnitud (longitud) del vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Retorna la magnitud al cuadrado (más eficiente si no necesitas sqrt)
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Retorna un vector normalizado (dirección unitaria)
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            *self / len
        } else {
            Vec3::zero()
        }
    }

    /// Producto punto (dot product) entre dos vectores
    /// Usado para calcular ángulos y proyecciones
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Producto cruz (cross product) entre dos vectores
    /// Retorna un vector perpendicular a ambos
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Refleja el vector incidente contra una normal
    /// Fórmula: reflect = incident - 2 * (incident · normal) * normal
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - (*normal * (2.0 * self.dot(normal)))
    }

    /// Limita los componentes del vector entre 0 y 1 (útil para colores)
    pub fn clamp(&self) -> Self {
        Vec3 {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            z: self.z.clamp(0.0, 1.0),
        }
    }
}

// Implementar operadores aritméticos

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Implementar operadores compuestos

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    fn approx_equal(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_vector_creation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert!(approx_equal(v.x, 1.0));
        assert!(approx_equal(v.y, 2.0));
        assert!(approx_equal(v.z, 3.0));
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert!(approx_equal(result.x, 5.0));
        assert!(approx_equal(result.y, 7.0));
        assert!(approx_equal(result.z, 9.0));
    }

    #[test]
    fn test_vector_subtraction() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = v1 - v2;
        assert!(approx_equal(result.x, 3.0));
        assert!(approx_equal(result.y, 3.0));
        assert!(approx_equal(result.z, 3.0));
    }

    #[test]
    fn test_scalar_multiplication() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        assert!(approx_equal(result.x, 2.0));
        assert!(approx_equal(result.y, 4.0));
        assert!(approx_equal(result.z, 6.0));
    }

    #[test]
    fn test_scalar_division() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let result = v / 2.0;
        assert!(approx_equal(result.x, 1.0));
        assert!(approx_equal(result.y, 2.0));
        assert!(approx_equal(result.z, 3.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1.dot(&v2);
        // (1*4) + (2*5) + (3*6) = 4 + 10 + 18 = 32
        assert!(approx_equal(result, 32.0));
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        // i × j = k = (0, 0, 1)
        assert!(approx_equal(result.x, 0.0));
        assert!(approx_equal(result.y, 0.0));
        assert!(approx_equal(result.z, 1.0));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let length = v.length();
        assert!(approx_equal(length, 5.0));
    }

    #[test]
    fn test_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let normalized = v.normalize();
        let length = normalized.length();
        assert!(approx_equal(length, 1.0));
        assert!(approx_equal(normalized.x, 0.6));
        assert!(approx_equal(normalized.y, 0.8));
    }

    #[test]
    fn test_reflection() {
        // Rayo golpeando una superficie vertical
        let incident = Vec3::new(1.0, -1.0, 0.0).normalize();
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let reflected = incident.reflect(&normal);
        // El reflejo debería ser (1, 1, 0) normalizado
        let expected = Vec3::new(1.0, 1.0, 0.0).normalize();
        assert!(approx_equal(reflected.x, expected.x));
        assert!(approx_equal(reflected.y, expected.y));
        assert!(approx_equal(reflected.z, expected.z));
    }

    #[test]
    fn test_clamp() {
        let v = Vec3::new(1.5, -0.5, 0.5);
        let clamped = v.clamp();
        assert!(approx_equal(clamped.x, 1.0));
        assert!(approx_equal(clamped.y, 0.0));
        assert!(approx_equal(clamped.z, 0.5));
    }
}

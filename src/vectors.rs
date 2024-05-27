use std::fmt;
use std::ops;

#[derive(Clone, Copy, Default)]
pub struct _3DVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl _3DVector {
    pub fn unitary(self: _3DVector) -> _3DVector {
        let length = (self * self).powf(0.5f64);
        _3DVector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

impl fmt::Debug for _3DVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "(x: {:+.5}  y: {:+.5}  z: {:+.5})",
            &self.x, &self.y, &self.z
        ))
    }
}

impl ops::Add<_3DVector> for _3DVector {
    type Output = _3DVector;
    fn add(self: _3DVector, other: _3DVector) -> _3DVector {
        _3DVector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<_3DVector> for _3DVector {
    type Output = _3DVector;
    fn sub(self: _3DVector, other: _3DVector) -> _3DVector {
        _3DVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<_3DVector> for _3DVector {
    type Output = f64;
    fn mul(self: _3DVector, other: _3DVector) -> f64 {
        self.x * other.y + self.y * other.y + self.z * other.z
    }
}

impl ops::Mul<_3DVector> for f64 {
    type Output = _3DVector;
    fn mul(self: f64, other: _3DVector) -> _3DVector {
        _3DVector {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl ops::Mul<f64> for _3DVector {
    type Output = _3DVector;
    fn mul(self: _3DVector, other: f64) -> _3DVector {
        other * self
    }
}

impl ops::Neg for _3DVector {
    type Output = _3DVector;
    fn neg(self: _3DVector) -> _3DVector {
        _3DVector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign for _3DVector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Div<f64> for _3DVector {
    type Output = _3DVector;
    fn div(self, rhs: f64) -> Self::Output {
        _3DVector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<f64> for _3DVector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::SubAssign for _3DVector {
    fn sub_assign(&mut self, rhs: _3DVector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::MulAssign<f64> for _3DVector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

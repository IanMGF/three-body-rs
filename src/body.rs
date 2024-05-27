#[path = "./vectors.rs"]
pub mod vectors;
use std::ops;
use vectors::_3DVector;

#[derive(Debug, Copy, Clone, Default)]
pub struct Body {
    pub mass: f64,
    pub pos: _3DVector,
    pub velocity: _3DVector,
}

impl ops::Shl<Body> for Body {
    type Output = _3DVector;
    fn shl(self, rhs: Body) -> Self::Output {
        const G: f64 = 6.6743e-11;
        let m1 = self.mass;
        let m2 = rhs.mass;
        let r: _3DVector = rhs.pos - self.pos;

        G * ((m1 * m2) / (r * r)) * r.unitary()
    }
}

impl ops::BitOr<Body> for Body {
    type Output = Body;

    fn bitor(self, rhs: Body) -> Self::Output {
        Body {
            pos: (self.pos * self.mass + rhs.pos * rhs.mass) / (self.mass + rhs.mass),
            velocity: (self.velocity * self.mass + rhs.velocity * rhs.mass)
                / (self.mass + rhs.mass),
            mass: self.mass + rhs.mass,
        }
    }
}

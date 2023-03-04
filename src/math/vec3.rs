use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(val: [f32; 3]) -> Self {
        Self { e: val }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::new([0.0, 0.0, 0.0])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new([-self.x(), -self.y(), -self.z()])
    }
}

macro_rules! impl_vec3_op {
    ($type:ident $op:ident $op_func:ident $op_symbol:tt) => {
        // Implement first & case and use it for others
        // example: impl Sub for &Vec3
        impl $op for &$type {
            type Output = $type;

            fn $op_func(self, rhs: Self) -> Self::Output {
                $type::new([self.x() $op_symbol rhs.x(), self.y() $op_symbol rhs.y(), self.z() $op_symbol rhs.z()])
            }
        }

        // example: impl Sub for Vec3
        impl $op for $type {
            type Output = $type;

            fn $op_func(self, rhs: Self) -> Self::Output {
                &self $op_symbol &rhs
            }
        }

        // example: impl Sub<Vec3> for &Vec3
        impl $op<$type> for &$type {
            type Output = $type;

            fn $op_func(self, rhs: $type) -> Self::Output {
                self $op_symbol &rhs
            }
        }

        // example: impl Sub<&Vec3> for Vec3
        impl $op<&$type> for $type {
            type Output = $type;

            fn $op_func(self, rhs: &$type) -> Self::Output {
                &self $op_symbol rhs
            }
        }

        // example: impl Sub<f32> for &Vec3
        impl $op<f32> for &$type {
            type Output = $type;

            fn $op_func(self, rhs: f32) -> Self::Output {
                $type::new([self.x() $op_symbol rhs, self.y() $op_symbol rhs, self.z() $op_symbol rhs])
            }
        }

        // example: impl Sub<f32> for Vec3
        impl $op<f32> for $type {
            type Output = $type;

            fn $op_func(self, rhs: f32) -> Self::Output {
                &self $op_symbol rhs
            }
        }

    }
}

impl_vec3_op!(Vec3 Add add +);
impl_vec3_op!(Vec3 Sub sub -);
impl_vec3_op!(Vec3 Div div /);
impl_vec3_op!(Vec3 Mul mul *);

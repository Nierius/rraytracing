use std::ops::{Add, Div, Mul, Neg, Sub};

use super::random::{rand_f32_clamped, Random};

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
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new([
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ])
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        return self.x().abs() < f32::EPSILON
            && self.y().abs() < f32::EPSILON
            && self.z().abs() < f32::EPSILON;
    }

    /** LOOPS UNTIL ONE FOUND */
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let random = Vec3::random_clamped(-1.0, 1.0);
            if random.length_squared() < 1.0 {
                return random;
            }
        }
    }

    /** LOOPS UNTIL ONE FOUND */
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let random = Vec3::new([
                rand_f32_clamped(-1.0, 1.0),
                rand_f32_clamped(-1.0, 1.0),
                0.0,
            ]);
            if random.length_squared() < 1.0 {
                return random;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        }

        -in_unit_sphere
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

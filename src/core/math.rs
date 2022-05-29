use std::ops;

pub type Point2D = Point<f32, 2>;
pub type Point3D = Point<f32, 3>;
pub type PointInt2D = Point<i32, 2>;
pub type PointInt3D = Point<i32, 3>;

#[derive(Clone, Copy)]
pub struct Point<T, const N: usize> {
    value: [T; N],
}

impl Point2D {
    pub fn x(&self) -> f32 { self.value[0] }
    pub fn y(&self) -> f32 { self.value[1] }
    pub fn sqr_length(&self) -> f32 { f32::powi(self.value[0], 2) + f32::powi(self.value[1], 2) }
    pub fn length(&self) -> f32 { f32::sqrt(self.sqr_length()) }
}

impl Point3D {
    pub const fn new(x: f32, y: f32, z: f32) -> Self { Point3D { value: [x, y, z] } }
    pub fn x(&self) -> f32 { self.value[0] }
    pub fn y(&self) -> f32 { self.value[1] }
    pub fn z(&self) -> f32 { self.value[2] }
    pub fn sqr_length(&self) -> f32 {
        f32::powi(self.value[0], 2) + f32::powi(self.value[1], 2) + f32::powi(self.value[2], 2)
    }
    pub fn length(&self) -> f32 { f32::sqrt(self.sqr_length()) }
    pub fn mul_length(&self, v: f32) -> Point3D {
        Point3D::new(self.value[0] * v, self.value[1] * v, self.value[2] * v)
    }

    pub const ZERO: Point3D = Point3D::new(0.0, 0.0, 0.0);
    pub const UP: Point3D = Point3D::new(0.0, 1.0, 0.0);
    pub const RIGHT: Point3D = Point3D::new(1.0, 0.0, 0.0);
    pub const FORWARD: Point3D = Point3D::new(0.0, 0.0, 1.0);
}

impl ops::Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point3D::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point3D::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self {
        let x = x * 0.5;
        let y = y * 0.5;
        let z = z * 0.5;
        let c1 = f32::cos(x);
        let c2 = f32::cos(y);
        let c3 = f32::cos(z);
        let s1 = f32::sin(x);
        let s2 = f32::sin(y);
        let s3 = f32::sin(z);
        Self::new(
            (s1 * c2 * c3) + (c1 * s2 * s3),
            (c1 * s2 * c3) - (s1 * c2 * s3),
            (c1 * c2 * s3) + (s1 * s2 * c3),
            (c1 * c2 * c3) - (s1 * s2 * s3),
        )
    }
}

pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

#[derive(Clone, Copy)]
pub struct Matrix<const N: usize> {
    value: [[f32; N]; N],
}

impl ops::Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4 {
            value: [
                [
                    self.value[0][0] * rhs.value[0][0]
                        + self.value[0][1] * rhs.value[1][0]
                        + self.value[0][2] * rhs.value[2][0]
                        + self.value[0][3] * rhs.value[3][0],
                    self.value[0][0] * rhs.value[0][1]
                        + self.value[0][1] * rhs.value[1][1]
                        + self.value[0][2] * rhs.value[2][1]
                        + self.value[0][3] * rhs.value[3][1],
                    self.value[0][0] * rhs.value[0][2]
                        + self.value[0][1] * rhs.value[1][2]
                        + self.value[0][2] * rhs.value[2][2]
                        + self.value[0][3] * rhs.value[3][2],
                    self.value[0][0] * rhs.value[0][3]
                        + self.value[0][1] * rhs.value[1][3]
                        + self.value[0][2] * rhs.value[2][3]
                        + self.value[0][3] * rhs.value[3][3],
                ],
                [
                    self.value[1][0] * rhs.value[0][0]
                        + self.value[1][1] * rhs.value[1][0]
                        + self.value[1][2] * rhs.value[2][0]
                        + self.value[1][3] * rhs.value[3][0],
                    self.value[1][0] * rhs.value[0][1]
                        + self.value[1][1] * rhs.value[1][1]
                        + self.value[1][2] * rhs.value[2][1]
                        + self.value[1][3] * rhs.value[3][1],
                    self.value[1][0] * rhs.value[0][2]
                        + self.value[1][1] * rhs.value[1][2]
                        + self.value[1][2] * rhs.value[2][2]
                        + self.value[1][3] * rhs.value[3][2],
                    self.value[1][0] * rhs.value[0][3]
                        + self.value[1][1] * rhs.value[1][3]
                        + self.value[1][2] * rhs.value[2][3]
                        + self.value[1][3] * rhs.value[3][3],
                ],
                [
                    self.value[2][0] * rhs.value[0][0]
                        + self.value[2][1] * rhs.value[1][0]
                        + self.value[2][2] * rhs.value[2][0]
                        + self.value[2][3] * rhs.value[3][0],
                    self.value[2][0] * rhs.value[0][1]
                        + self.value[2][1] * rhs.value[1][1]
                        + self.value[2][2] * rhs.value[2][1]
                        + self.value[2][3] * rhs.value[3][1],
                    self.value[2][0] * rhs.value[0][2]
                        + self.value[2][1] * rhs.value[1][2]
                        + self.value[2][2] * rhs.value[2][2]
                        + self.value[2][3] * rhs.value[3][2],
                    self.value[2][0] * rhs.value[0][3]
                        + self.value[2][1] * rhs.value[1][3]
                        + self.value[2][2] * rhs.value[2][3]
                        + self.value[2][3] * rhs.value[3][3],
                ],
                [
                    self.value[3][0] * rhs.value[0][0]
                        + self.value[3][1] * rhs.value[1][0]
                        + self.value[3][2] * rhs.value[2][0]
                        + self.value[3][3] * rhs.value[3][0],
                    self.value[3][0] * rhs.value[0][1]
                        + self.value[3][1] * rhs.value[1][1]
                        + self.value[3][2] * rhs.value[2][1]
                        + self.value[3][3] * rhs.value[3][1],
                    self.value[3][0] * rhs.value[0][2]
                        + self.value[3][1] * rhs.value[1][2]
                        + self.value[3][2] * rhs.value[2][2]
                        + self.value[3][3] * rhs.value[3][2],
                    self.value[3][0] * rhs.value[0][3]
                        + self.value[3][1] * rhs.value[1][3]
                        + self.value[3][2] * rhs.value[2][3]
                        + self.value[3][3] * rhs.value[3][3],
                ],
            ],
        }
    }
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        value: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn raw(&self) -> &[[f32; 4]; 4] { &self.value }

    pub fn perspective_fov(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
        let max_y = near * f32::tan(0.5 * fov);
        let min_y = -max_y;
        let min_x = min_y * aspect;
        let max_x = max_y * aspect;
        Self::perspective_off(min_x, max_x, min_y, max_y, near, far)
    }
    pub fn perspective_off(
        left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32,
    ) -> Matrix4 {
        let x = 2.0 * near / (right - left);
        let y = 2.0 * near / (top - bottom);
        let a = (right + left) / (right - left);
        let b = (top + bottom) / (top - bottom);
        let c = -(far + near) / (far - near);
        let d = -(2.0 * far * near) / (far - near);
        Matrix4 {
            value: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [a, b, c, -1.0],
                [0.0, 0.0, d, 0.0],
            ],
        }
    }

    pub fn scale(point: Point3D) -> Matrix4 {
        Matrix4 {
            value: [
                [point.x(), 0.0, 0.0, 0.0],
                [0.0, point.y(), 0.0, 0.0],
                [0.0, 0.0, point.z(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(point: Point3D) -> Matrix4 {
        let m = Matrix4 {
            value: [
                [1.0, 0.0, 0.0, point.x()],
                [0.0, 1.0, 0.0, point.y()],
                [0.0, 0.0, 1.0, point.z()],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        m
    }

    pub fn rotation(&mut self, degrees: f32, axis: Point3D) {
        let radians = f32::to_radians(degrees);
        let cos = f32::cos(radians);
        let sin = f32::sin(radians);
        let x = axis.x();
        let y = axis.y();
        let z = axis.z();
        self.value = [
            [
                cos + f32::powi(x, 2) * (1.0 - cos),
                x * y * (1.0 - cos) - z * sin,
                x + z * (1.0 - cos) + y * sin,
                0.0,
            ],
            [
                y * x * (1.0 - cos) + z * sin,
                cos + f32::powi(y, 2) * (1.0 - cos),
                y * z * (1.0 - cos) - x * sin,
                0.0,
            ],
            [
                z * x * (1.0 - cos) - y * sin,
                z * y * (1.0 - cos) + x * sin,
                cos + f32::powi(z, 2) * (1.0 - cos),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ];
    }

    pub fn combine(
        scale: Option<Matrix4>, rotation: Option<Matrix4>, translation: Option<Matrix4>,
    ) -> Matrix4 {
        let mut m = Matrix4::IDENTITY;
        let mut apply = |x: &Option<Matrix<4>>| {
            if let Some(v) = x {
                m = m * *v;
            }
        };
        apply(&scale);
        apply(&rotation);
        apply(&translation);
        m
    }
}

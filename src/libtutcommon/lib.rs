#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![feature(float_extras)]

#![doc = "Common stuff for tutorials."]
#![crate_name = "tutcommon"]

// Include SDL2 library.
extern crate sdl2;

extern crate byteorder;

extern crate libc;

extern crate gl;

use std::default::Default;
use std::ops::Mul;

pub mod glutils;

pub mod sdl;

#[doc = "Vector with 3 components (x,y,z)"]
pub struct Vector3f (pub f32, pub f32, pub f32);

impl Vector3f {
    #[doc = "Normalize vector to length 1."]
    pub fn normalize(&self) -> Vector3f {
        let l = (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt();

        Vector3f(self.0 / l, self.1 / l, self.2 / l)
    }

    #[doc = "Cross product."]
    pub fn cross(&self, _rhs:&Vector3f) -> Vector3f {
        Vector3f (
            self.1 * _rhs.2 - self.2 * _rhs.1,
            self.2 * _rhs.0 - self.0 * _rhs.2,
            self.0 * _rhs.1 - self.1 * _rhs.0,
        )
    }

    fn sub(&self, _rhs:&Vector3f) -> Vector3f {
        Vector3f (
            self.0 - _rhs.0,
            self.1 - _rhs.1,
            self.2 - _rhs.2,
        )
    }
}

#[doc = "Matrix 4x4 to send into OpenGL."]
#[derive(Debug)]
pub struct Matrix4f {
    data : [[f32; 4]; 4], //column major order data[j][i] points to j-th column i-th row.
}

impl Matrix4f {
    #[doc = "Get raw data for OpenGL."]
    pub fn as_ptr(&self) -> * const f32 {
        self.data[0].as_ptr()
    }

    #[doc = "Get perspective projection matrix."]
    pub fn perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Matrix4f {
        let mut res : Matrix4f = Default::default();

        let f = 1.0 / (fov / 2.0 ).to_radians().tan();

        res.data[0][0] = f / aspect;
        res.data[1][1] = f;
        res.data[2][2] = (zfar + znear) / (znear - zfar);
        res.data[2][3] = - 1.0;
        res.data[3][2] = 2.0 * zfar * znear / (znear - zfar);
        res.data[3][3] = 0.0;

        res
    }

    #[doc = "Get orthographic projection matrix."]
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, znear: f32, zfar: f32) -> Matrix4f {
        let mut res : Matrix4f = Default::default();

        res.data[0][0] = 2.0 / (right - left);
        res.data[1][1] = 2.0 / (top - bottom);
        res.data[2][2] = -2.0 / (zfar - znear);
        res.data[3][0] = - (right + left) / (right - left);
        res.data[3][1] = - (top + bottom) / (top - bottom);
        res.data[3][2] = - (zfar + znear) / (zfar - znear);

        res
    }

    #[doc = "Generate matrix for camera."]
    pub fn look_at(eye:Vector3f, center:Vector3f, up:Vector3f) -> Matrix4f {
        let f = center.sub(&eye).normalize();
        let up1 = up.normalize();
        let s = f.cross(&up1).normalize();
        let u = s.cross(&f);

        let mut m : Matrix4f = Default::default();
        m.data[0][0] = s.0;
        m.data[1][0] = s.1;
        m.data[2][0] = s.2;

        m.data[0][1] = u.0;
        m.data[1][1] = u.1;
        m.data[2][1] = u.2;

        m.data[0][2] = -f.0;
        m.data[1][2] = -f.1;
        m.data[2][2] = -f.2;

        let mut t : Matrix4f = Default::default();

        t.data[3][0] = -eye.0;
        t.data[3][1] = -eye.1;
        t.data[3][2] = -eye.2;

        m.mul(&t)
    }

    #[doc = "Generate translation matrix."]
    pub fn translate(t:Vector3f) -> Matrix4f {
        let mut res : Matrix4f = Default::default();

        res.data[3][0] = t.0;
        res.data[3][1] = t.1;
        res.data[3][2] = t.2;

        res
    }

    #[doc = "Generate translation matrix."]
    pub fn scale(s:Vector3f) -> Matrix4f {
        let mut res : Matrix4f = Default::default();

        res.data[0][0] = s.0;
        res.data[1][1] = s.1;
        res.data[2][2] = s.2;

        res
    }

    #[doc = "Generate rotate matrix."]
    pub fn rotate(angle:f32, axis:Vector3f) -> Matrix4f {
        let mut res : Matrix4f = Default::default();

        let a = (angle / 2.0).to_radians().sin();
        let vn = axis.normalize();

        let x = vn.0 * a;
        let y = vn.1 * a;
        let z = vn.2 * a;
        let w = (angle / 2.0).to_radians().cos();

        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        res.data[0][0] = 1.0 - 2.0 * (y2 + z2);
        res.data[0][1] = 2.0 * (xy + wz);
        res.data[0][2] = 2.0 * (xz - wy);

        res.data[1][0] = 2.0 * (xy - wz);
        res.data[1][1] = 1.0 - 2.0 * (x2 + z2);
        res.data[1][2] = 2.0 * (yz + wx);

        res.data[2][0] = 2.0 * (xz + wy);
        res.data[2][1] = 2.0 * (yz - wx);
        res.data[2][2] = 1.0 - 2.0 * (x2 + y2);

        res
    }

    #[doc = "Matrix multiplication."]
    pub fn mul(&self, _rhs: &Matrix4f) -> Matrix4f {
        let mut res : Matrix4f = Default::default();

        for i in 0..4 { // row
            for j in 0..4 { // column
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[k][i] * _rhs.data[j][k];
                }
                res.data[j][i] = sum;
            }
        }

        res
    }
}

impl Default for Matrix4f {
    fn default() -> Matrix4f {
        Matrix4f { data : [
            [1.0, 0.0, 0.0, 0.0,],
            [0.0, 1.0, 0.0, 0.0,],
            [0.0, 0.0, 1.0, 0.0,],
            [0.0, 0.0, 0.0, 1.0,],
        ] }
    }
}




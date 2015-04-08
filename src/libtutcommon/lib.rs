#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![feature(std_misc)]

#![doc = "Common stuff for tutorials."]
#![crate_name = "tutcommon"]

use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::ops::Mul;
use std::path::Path;

#[doc = "Read content of file into string."]
pub fn read_source_from_file<P: AsRef<Path>>(path : P) -> String {
    let mut res = String::new();
    File::open(path).unwrap().read_to_string(&mut res).unwrap();
    return res;
}

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

        m * t
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

impl Mul for Matrix4f {
    type Output = Matrix4f;

    fn mul(self, _rhs: Matrix4f) -> Matrix4f {
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


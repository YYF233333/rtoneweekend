use std::ops::Deref;

use arrayfire::*;

use crate::vec3::*;
pub type Point3Array = Vec3Array;
pub type ColorArray = Vec3Array;

pub struct Vec3Array {
    elements: Array<f32>,
}

impl Vec3Array {
    pub fn new(elements: Vec<Vec3>) -> Self {
        let mut t = vec![];
        for e in &elements {
            t.push(e.x());
        }
        for e in &elements {
            t.push(e.y());
        }
        for e in &elements {
            t.push(e.z());
        }
        let dims = dim4!(elements.len() as u64, 3);
        Self {
            elements: Array::new(&t, dims),
        }
    }

    pub fn print(&self) {
        af_print!("", self.elements)
    }

    pub fn x(&self) -> Array<f32> {
        col(&self.elements, 0)
    }

    pub fn y(&self) -> Array<f32> {
        col(&self.elements, 1)
    }

    pub fn z(&self) -> Array<f32> {
        col(&self.elements, 2)
    }
}

impl Deref for Vec3Array {
    type Target = Array<f32>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

pub fn dot(u: Vec3Array, v: Vec3Array) -> Vec3Array {
    Vec3Array {
        elements: sum(&(u.elements * v.elements), 1),
    }
}

pub fn cross(u: Vec3Array, v: Vec3Array) -> Vec3Array {
    let x = u.y() * v.z() - u.z() * v.y();
    let y = u.z() * v.x() - u.x() * v.z();
    let z = u.x() * v.y() - u.y() * v.x();
    Vec3Array {
        elements: join_many![1; &x, &y, &z],
    }
}

pub fn unit(v: Vec3Array) -> Vec3Array {
    let len = sum(&(v.clone() * v.clone()), 1);
    let len1 = sqrt(&len);
    let len2 = sqrt(&len);
    let len3 = sqrt(&len);
    let len = join_many![1; &len1, &len2, &len3];
    Vec3Array {
        elements: v.elements / len,
    }
}

impl Vec3Array {
    pub fn random(num: u64) -> Self {
        Self {
            elements: randu!(num, 3),
        }
    }

    pub fn random_in_range(num: u64, min: f32, max: f32) -> Self {
        Self {
            elements: min + (max - min) * (randu!(num, 3)),
        }
    }

    pub fn near_zero(&self) -> Array<bool> {
        static S: f32 = 1e-8f32;
        let x = lt(&abs(&self.x()), &S, true);
        let y = lt(&abs(&self.y()), &S, true);
        let z = lt(&abs(&self.z()), &S, true);
        selectl(1.0, &x, &selectl(1.0, &y, &z))
    }
}

pub fn reflect(v: Vec3Array, n: Vec3Array) -> Vec3Array {
    Vec3Array {
        elements: v.clone()
            - 2.0f32
                * dot(
                    v,
                    Vec3Array {
                        elements: n.clone(),
                    },
                )
                .clone()
                * n.clone(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_array() -> Vec3Array {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let c = Vec3::new(7.0, 8.0, 9.0);
        let d = Vec3::new(1.0, 2.0, 3.0);
        let e = Vec3::new(4.0, 5.0, 6.0);
        let f = Vec3::new(7.0, 8.0, 9.0);
        Vec3Array::new(vec![a, b, c, d, e, f])
    }
    fn small_array() -> Vec3Array {
        let a = Vec3::new(1e-9, 2.0, 3.0);
        let b = Vec3::new(4.0, 1e-9, 6.0);
        let c = Vec3::new(7.0, 8.0, 1e-9);
        let d = Vec3::new(1e-8, 2.0, 3.0);
        let e = Vec3::new(1e-9, 1e-9, 1e-9);
        let f = Vec3::new(7.0, 8.0, 9.0);
        Vec3Array::new(vec![a, b, c, d, e, f])
    }

    #[test]
    fn sub_array() {
        let array = test_array();
        array.print();
        af_print!("", array.x());
        af_print!("", array.y());
        af_print!("", array.z());
    }

    #[test]
    fn test_dot() {
        let array = test_array();
        let array2 = test_array();
        let res = dot(array, array2);
        print(&res);
    }

    #[test]
    fn test_cross() {
        let array = test_array();
        let array2 = test_array();
        let res = cross(array, array2);
        print(&res);
    }

    #[test]
    fn test_unit() {
        let array = test_array();
        let res = unit(array);
        print(&res);
    }

    #[test]
    fn test_near_zero() {
        let array = small_array();
        let res = array.near_zero();
        print(&res);
    }
}

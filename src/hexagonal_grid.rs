//! Hexagonal grid algebra.
//!
//! Based on:  https://www.redblobgames.com/grids/hexagons/
//!
//! Implementation choices:
//! - Pointy top orientation
//! - Odd row offset
use std::ops::{Add, Sub};

use nannou::prelude::*;

/// In a regular hexagon the interior angles are 120°. There are six “wedges”, each an equilateral
/// triangle with 60° angles inside. Each corner is size units away from the center. In code:
pub fn pointy_hex_corner(center: Point2, size: f32, corner: usize) -> Point2 {
    assert!(corner <= 6, "Hexagons only have six corners");
    let angle_deg = 60.0 * corner as f32 - 30.0;
    let angle_rad = deg_to_rad(angle_deg);
    Point2::new(
        center.x + size * angle_rad.cos(),
        center.y + size * angle_rad.sin(),
    )
}

/// Moving one space in hex coordinates involves changing one of the 3 cube coordinates by +1 and
/// changing another one by -1 (the sum must remain 0). There are 3 possible coordinates to change
/// by +1, and 2 remaining that could be changed by -1. This results in 6 possible changes. Each
/// corresponds to one of the hexagonal directions. The simplest and fastest approach is to
/// precompute the permutations and put them into a table of Cube(dq, dr, ds):
const CUBE_DIRECTION_VECTORS: [CubeCoord; 6] = [
    CubeCoord::new_unchecked(1, 0, -1),
    CubeCoord::new_unchecked(1, -1, 0),
    CubeCoord::new_unchecked(0, -1, 1),
    CubeCoord::new_unchecked(-1, 0, 1),
    CubeCoord::new_unchecked(-1, 1, 0),
    CubeCoord::new_unchecked(0, 1, -1),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CubeCoord {
    q: i32,
    r: i32,
    s: i32,
}

impl CubeCoord {
    const fn new_unchecked(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s }
    }
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        assert_eq!(0, q + s + r);
        Self { q, r, s }
    }

    pub fn into_parts(self) -> (i32, i32, i32) {
        (self.q, self.r, self.s)
    }

    pub fn direction(direction: usize) -> CubeCoord {
        CUBE_DIRECTION_VECTORS[direction]
    }

    pub fn neighbor(&self, direction: usize) -> Self {
        *self + Self::direction(direction)
    }

    pub fn distance(&self, other: &CubeCoord) -> i32 {
        let vec = *self - *other;
        *[vec.q.abs(), vec.r.abs(), vec.s.abs()]
            .iter()
            .max()
            .unwrap()
    }

    pub fn scale(cube: &Self, factor: i32) -> Self {
        CubeCoord::new(cube.q * factor, cube.r * factor, cube.s * factor)
    }

    pub fn ring(&self, radius: u16) -> Vec<Self> {
        let mut results = vec![];
        let mut hex = *self + Self::scale(&Self::direction(4), radius as i32);
        for i in 0..6 {
            for _ in 0..radius as usize {
                results.push(hex);
                hex = CubeCoord::neighbor(&hex, i);
            }
        }
        results
    }

    pub fn spiral(&self, radius: u16) -> Vec<Self> {
        if radius == 0 {
            return vec![];
        }
        let mut results = vec![*self];
        for k in 1..radius + 1 {
            let ring = self.ring(k);
            results.extend(ring.into_iter());
        }
        results
    }

    /// The way to think about hex to pixel conversion is to look at the basis vectors. The arrow
    /// (0,0)→(1,0) is the q basis vector (x=sqrt(3), y=0) and (0,0)→(0,1) is the r basis vector
    /// (x=sqrt(3)/2, y=3/2). The pixel coordinate is q_basis * q + r_basis * r. For example, the
    /// hex at (1,1) is the sum of 1 q vector and 1 r vector. A hex at (3,2) would be the sum of 3 q
    /// vectors and 2 r vectors.
    pub fn cartesian(&self, size: f32) -> Point2 {
        let sqrt3 = 3.0f32.sqrt();
        let sqrt3_2 = sqrt3 / 2.0;
        let x = size * (sqrt3 * self.q as f32 + sqrt3_2 * self.r as f32);
        let y = size * ((3.0 / 2.0) * self.r as f32);
        Point2::new(x, y)
    }
}

impl Add for CubeCoord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        CubeCoord::new(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
    }
}

impl Sub for CubeCoord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        CubeCoord::new(self.q - rhs.q, self.r - rhs.r, self.s - rhs.s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring() {
        let origin = CubeCoord::new(0, 0, 0);
        for n in 0..100 {
            let r = origin.ring(n);
            assert_eq!(r.len(), 6 * n as usize);
        }
    }

    #[test]
    fn spiral() {
        let origin = CubeCoord::new(0, 0, 0);
        for n in 1..100 {
            let len = 1 + 3 * n * (n + 1);
            let r = origin.spiral(n);
            assert_eq!(len as usize, r.len());
        }
    }
}

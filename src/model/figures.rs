use core::*;
use model::Snapshot;
use motion::rotation::{
    rotation_matrix_x,
    rotation_matrix_y,
    rotation_matrix_z
};

use itertools::Itertools;
use std::f32::consts::PI;
use std::rc::Rc;

pub fn triangle(a: Point, b: Point, c: Point, color: Color) -> Snapshot {
    Snapshot {
        positions: Rc::new(vec![a, b, c]),
        colors: Rc::new(vec![color, color.clone(), color.clone()]),
        indices: vec![0, 1, 2],
        size: 3
    }
}

#[allow(dead_code)]
pub fn diamond(a: Point, b: Point, c: Point, d: Point,
               color_abc: Color, color_bcd: Color) -> Snapshot {
    Snapshot {
        positions: Rc::new(vec![a, b, c, d]),
        colors: Rc::new(vec![color_abc.clone(), color_abc.clone(), color_abc, color_bcd]),
        indices: vec![0, 1, 2, 1, 2, 3],
        size: 4
    }
}

pub fn tetrahedron(a: Point, b: Point, c: Point, d: Point,
                   color_base: Color, color_peak: Color) -> Snapshot {
    Snapshot {
        positions: Rc::new(vec![a, b, c, d]),
        colors: Rc::new(vec![color_base.clone(), color_base.clone(), color_base, color_peak]),
        indices: vec![0, 1, 2, 0, 1, 3, 1, 2, 3, 2, 0, 3],
        size: 4
    }
}

pub fn circle_xy(center: Point, radius: CoordFloat, color: Color, detailing: u16) -> Snapshot {
    let k = detailing;
    js_assert(k > 2, format!("impossible to build circle with detailing {}", k));

    let angle = 2. * PI / (k as CoordFloat);
    let rotation = rotation_matrix_z(angle);

    let mut points = vec![center];
    let mut indices = vec![];

    let mut vector: Vector = scale(radius) * unit_x().coords;
    points.push(center + vector);

    for i in 0..k {
        vector = rotation * vector;
        let current = center + vector;

        indices.push(0);
        indices.push(i + 1);
        if i != k - 1 {
            points.push(current);
            indices.push(i + 2);
        } else {
            indices.push(1);
        }
    }

    let size = points.len() as u16;
    assert!(size == k + 1);

    Snapshot {
        positions: Rc::new(points),
        colors: Rc::new(vec![color; size as usize]),
        indices: indices,
        size
    }
}

pub fn sphere_xyz(center: Point, radius: CoordFloat, color: Color, detailing: u16) -> Snapshot {
    sphere_xyz_colored(center, radius, detailing, color, color, color)
}

pub fn sphere_xyz_colored(center: Point, radius: CoordFloat, detailing: u16,
                          color_north: Color, color_equator: Color, color_south: Color) -> Snapshot {
    let k = detailing;
    js_assert(k > 0 && k % 2 == 0 && k % 3 == 0,
              format!("impossible to build sphere with detailing {}", k));

    let ry = scale(radius) * unit_y().coords;
    let north: Point = center + ry;
    let south: Point = center - ry;

    let mut points = vec![north, south];
    let mut indices = vec![]; //0, 1 are reserved for north and south
    let mut colors = vec![color_north, color_south];

    let m = k / 2 - 1; //amount of points on every half-circle between poles
    // 6 -> 2, 12 -> 5, etc.

    let rotation_y = rotation_matrix_y(2. * PI / (k as CoordFloat));
    let rotation_z = rotation_matrix_z(PI / ((m + 1) as CoordFloat));

    let arc = {
        let mut result = vec![];
        let mut vector = ry;
        for _ in 0..m + 1 {
            vector = rotation_z * vector;
            result.push(vector.clone());
        }
        result
    };

    let grid = {
        let mut result = vec![];
        let mut curr_arc = arc.clone();
        for _ in 0..k {
            for i in 0..((m + 1) as usize) {
                curr_arc[i] = rotation_y * curr_arc[i];
                result.push(curr_arc[i]);
            }
        }
        result
    };

    for j in 0..k {
        let top = 2 + j * m;
        let bottom = top + m - 1;

        let equator = (top + bottom) as ColorFloat / 2.;
        let factor = m as ColorFloat / 2.;

        for i in top..bottom + 1 {
            let vector = grid[(j * (m + 1) + (i - top)) as usize];

            points.push(center + vector);
            let neighbour = if j == k - 1 {
                (i - 2) % m + 2
            } else {
                i + m
            };

            if i == top {
                push_triangle(&mut indices, top, neighbour, 0);
            }

            if i < bottom {
                push_square(&mut indices, i, i + 1, neighbour, neighbour + 1);
            } else {
                push_triangle(&mut indices, bottom, neighbour, 1);
            }

            let color = {
                let distance = (i as ColorFloat) - equator;
                if distance == 0. {
                    color_equator
                } else if distance < 0. {
                    mix(color_equator, color_north, - distance / factor)
                } else {
                    mix(color_equator, color_south,   distance / factor)
                }
            };
            colors.push(color);
        }
    }

    let size = points.len() as u16;
    js_assert(size == m * k + 2, format!("wrong size of sphere: {}", size));

    Snapshot {
        positions: Rc::new(points),
        colors: Rc::new(colors),
        indices: indices,
        size
    }
}

pub fn tower(bottom: Snapshot, top: Snapshot) -> Snapshot {
    let n = bottom.size;
    if n != top.size {
        panic!("Can't attach top to bottom of different amount of vertices");
    }

    let n = n as u16;

    let mut positions = Rc::try_unwrap(bottom.positions).unwrap();
    positions.append(&mut Rc::try_unwrap(top.positions).unwrap());

    let mut colors = Rc::try_unwrap(bottom.colors).unwrap();
    colors.append(&mut Rc::try_unwrap(top.colors).unwrap());

    let mut indices = bottom.indices;

    fn renumerate(n: u16, iter: impl Iterator<Item=u16>) -> impl Iterator<Item=u16> {
        iter.map(move |i| i + n)
    }

    let mut top: Vec<u16> = renumerate(n, top.indices.into_iter())
            .collect();

    indices.append(&mut top);

    let mut edges: Vec<(u16, u16)> = (0..n)
        .zip(renumerate(n, 0..n))
        .collect();
    edges.push((0, n));

    for ((a,b),(c,d)) in edges.into_iter().tuple_windows() {
        push_triangle(&mut indices, a, c, d);
        push_triangle(&mut indices, a, b, d);
    }

    Snapshot {
        positions: Rc::new(positions),
        colors: Rc::new(colors),
        indices,
        size: n + n
    }
}

fn push_square(indices: &mut Vec<u16>, a: u16, b: u16, c: u16, d: u16) {
    push_triangle(indices, a, b, c);
    push_triangle(indices, b, c, d);
}

fn push_triangle(indices: &mut Vec<u16>, a: u16, b: u16, c: u16) {
    indices.push(a);
    indices.push(b);
    indices.push(c);
}
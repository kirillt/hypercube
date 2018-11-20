use core::{Point, Color};
use model::Snapshot;

use itertools::Itertools;
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

pub fn prism(bottom: Snapshot, top: Snapshot) -> Snapshot {
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
        indices.push(a);
        indices.push(c);
        indices.push(d);

        indices.push(a);
        indices.push(b);
        indices.push(d);
    }

    Snapshot {
        positions: Rc::new(positions),
        colors: Rc::new(colors),
        indices,
        size: n + n
    }
}
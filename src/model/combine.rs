use core::*;
use model::Snapshot;

use itertools::Itertools;
use std::rc::Rc;

pub fn merge(bottom: Snapshot, top: Snapshot) -> Snapshot {
    let n = bottom.size;
    let m = top.size;

    let (positions, colors, indices) = merge_internal(bottom, top);

    Snapshot {
        positions: Rc::new(positions),
        colors: Rc::new(colors),
        indices: indices,
        size: n + m
    }
}

pub fn merge_group(mut group: Vec<Snapshot>) -> Snapshot {
    let n = group.len();

    if n > 2 {
        let m = n / 2;
        let half: Vec<Snapshot> = group.drain(0..m).collect();
        merge(merge_group(half),
              merge_group(group))
    } else if n == 2 {
        let first = group.remove(0);
        let second = group.remove(0);
        merge(first, second)
    } else {
        js_assert(n > 0, true,
                  "merging group of 0 snapshots makes no sense".to_string());
        group.remove(0)
    }
}

pub fn tower(bottom: Snapshot, top: Snapshot) -> Snapshot {
    js_assert(bottom.size == top.size, false,
              format!("bottom and top are of different size: {} != {}",
                      bottom.size, top.size));

    let n = bottom.size as u16;
    let (positions, colors, mut indices) = merge_internal(bottom, top);

    let mut edges: Vec<(u16, u16)> = (0..n)
        .zip((0..n).map(move |i| i + n))
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

fn merge_internal(bottom: Snapshot, top: Snapshot) -> (Vec<Point>, Vec<Color>, Vec<u16>) {
    let mut positions = Rc::try_unwrap(bottom.positions).unwrap();
    positions.append(&mut Rc::try_unwrap(top.positions).unwrap());

    let mut colors = Rc::try_unwrap(bottom.colors).unwrap();
    colors.append(&mut Rc::try_unwrap(top.colors).unwrap());

    let n = bottom.size as u16;
    let mut top: Vec<u16> = top.indices
        .into_iter()
        .map(move |i| i + n)
        .collect();

    let mut indices = bottom.indices;
    indices.append(&mut top);

    (positions, colors, indices)
}
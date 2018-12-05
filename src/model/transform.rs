use core::*;
use model::Snapshot;

use std::rc::Rc;

pub fn rotation_matrix_x(angle: CoordFloat) -> Matrix4 {
    rotation_matrix(vec![angle, 0., 0., 0., 0., 0.])
}

pub fn rotation_matrix_y(angle: CoordFloat) -> Matrix4 {
    rotation_matrix(vec![0., angle, 0., 0., 0., 0.])
}

pub fn rotation_matrix_z(angle: CoordFloat) -> Matrix4 {
    rotation_matrix(vec![0., 0., angle, 0., 0., 0.])
}

pub fn rotation_matrix(angles: Vec<CoordFloat>) -> Matrix4 {
    assert!(angles.len() == 6);
    let angle_wx = angles[0];
    let angle_wy = angles[1];
    let angle_wz = angles[2];
    let angle_xy = angles[3];
    let angle_xz = angles[4];
    let angle_yz = angles[5];

    let transformation_wx = Matrix4::new(
        1.,  0.,             0.,                   0.,
        0.,  1.,             0.,                   0.,
        0.,  0., angle_wx.cos(), -1. * angle_wx.sin(),
        0.,  0., angle_wx.sin(),       angle_wx.cos()
    ); //yz

    let transformation_wy = Matrix4::new(
        1.,             0., 0.,                   0.,
        0., angle_wy.cos(), 0., -1. * angle_wy.sin(),
        0.,             0., 1.,                   0.,
        0., angle_wy.sin(), 0.,       angle_wy.cos()
    ); //xz

    let transformation_wz = Matrix4::new(
        1.,             0.,                   0., 0.,
        0., angle_wz.cos(), -1. * angle_wz.sin(), 0.,
        0., angle_wz.sin(),       angle_wz.cos(), 0.,
        0.,             0.,                   0., 1.,
    ); //xy

    let transformation_xy = Matrix4::new(
        angle_xy.cos(), 0., 0., -1. * angle_xy.sin(),
        0., 1., 0.,                   0.,
        0., 0., 1.,                   0.,
        angle_xy.sin(), 0., 0.,       angle_xy.cos()
    ); //wz

    let transformation_xz = Matrix4::new(
        angle_xz.cos(), 0., -1. * angle_xz.sin(), 0.,
        0., 1.,                   0., 0.,
        angle_xz.sin(), 0.,       angle_xz.cos(), 0.,
        0., 0.,                   0., 1.
    ); //wy

    let transformation_yz = Matrix4::new(
        angle_yz.cos(), -1. * angle_yz.sin(), 0., 0.,
        angle_yz.sin(),       angle_yz.cos(), 0., 0.,
        0.,                   0., 1., 0.,
        0.,                   0., 0., 1.
    ); //wx


    transformation_wx * transformation_wy * transformation_wz *
        transformation_xy * transformation_xz *
        transformation_yz
}
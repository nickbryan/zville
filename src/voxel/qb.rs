use bevy::{math, render::color};
use byteorder::{ByteOrder, LittleEndian};
use std::io::{self, Read};
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ZAxisOrientation {
    LeftHanded,
    RightHanded,
}

fn z_axis_orientation_from_data(data: u32) -> ZAxisOrientation {
    if data > 0 {
        return ZAxisOrientation::RightHanded;
    }

    ZAxisOrientation::LeftHanded
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub version: u32,
    pub color_format: u32,
    pub z_axis_orientation: ZAxisOrientation,
    pub compressed: bool,
    pub visibility_mask_encoded: bool,
    pub num_matrices: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct VoxelData {
    pub position: math::Vec3,
    pub color: color::Color,
    pub visible: bool,
    // TODO: what about face visibility?
}

#[derive(Debug)]
pub struct Matrix {
    pub name: String,
    pub size: Size,
    pub position: math::Vec3,
    pub voxels: Vec<VoxelData>,
}

#[derive(Debug)]
pub struct QubicleData {
    pub header: Header,
    pub matrices: Vec<Matrix>,
}

pub fn parse<T: io::Read>(data: T) -> QubicleData {
    let mut buf_reader = io::BufReader::new(data);

    let mut qb = QubicleData {
        header: Header {
            version: read_u32(&mut buf_reader),
            color_format: read_u32(&mut buf_reader),
            z_axis_orientation: z_axis_orientation_from_data(read_u32(&mut buf_reader)),
            compressed: read_u32(&mut buf_reader) != 0,
            visibility_mask_encoded: read_u32(&mut buf_reader) != 0,
            num_matrices: read_u32(&mut buf_reader),
        },
        matrices: vec![],
    };

    for _ in 0..qb.header.num_matrices {
        let name_len = read_byte(&mut buf_reader);

        let mut m = Matrix {
            name: String::from_utf8(read(&mut buf_reader, name_len as usize)).unwrap(),
            size: Size {
                x: read_u32(&mut buf_reader) as usize,
                y: read_u32(&mut buf_reader) as usize,
                z: read_u32(&mut buf_reader) as usize,
            },
            position: math::Vec3::new(
                read_u32(&mut buf_reader) as f32,
                read_u32(&mut buf_reader) as f32,
                read_u32(&mut buf_reader) as f32,
            ),
            voxels: vec![],
        };

        if !qb.header.compressed {
            for z in 0..m.size.z {
                for y in 0..m.size.y {
                    for x in 0..m.size.x {
                        let c1 = read_byte(&mut buf_reader);
                        let c2 = read_byte(&mut buf_reader);
                        let c3 = read_byte(&mut buf_reader);
                        let a = read_byte(&mut buf_reader);

                        m.voxels.push(VoxelData {
                            position: math::Vec3::new(x as f32, y as f32, z as f32),
                            color: color::Color::rgb_u8(c1, c2, c3),
                            visible: a > 0,
                        })
                    }
                }
            }
        }

        qb.matrices.push(m);
    }

    qb
}

fn read<T: io::Read>(reader: &mut T, size: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size);

    let mut part_reader = reader.take(size as u64);

    part_reader.read_to_end(&mut buf).unwrap();

    buf
}

fn read_u32<T: io::Read>(reader: &mut T) -> u32 {
    LittleEndian::read_u32(&read(reader, mem::size_of::<u32>()))
}

fn read_byte<T: io::Read>(reader: &mut T) -> u8 {
    read(reader, mem::size_of::<u8>())[0]
}

impl 

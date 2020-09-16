use crate::voxel::{Matrix, Voxel};
use bevy::{asset, math, render::color};
use byteorder::{ByteOrder, LittleEndian};
use std::io::{self, Read};
use std::mem;
use std::path::Path;

#[derive(Default)]
pub struct QubicleBinaryLoader;

impl asset::AssetLoader<Matrix> for QubicleBinaryLoader {
    fn from_bytes(&self, _: &Path, bytes: Vec<u8>) -> anyhow::Result<Matrix, anyhow::Error> {
        // Due to the way the .qb files are encoded we have to read the data even if we don't use it.
        // Where data is read and not used the variable is prefixed with an _.
        let mut bytes = bytes.as_slice();

        let _version = read_u32(&mut bytes);
        let _color_format = read_u32(&mut bytes);
        let _z_axis_orientation = read_u32(&mut bytes);

        let compressed = read_u32(&mut bytes) != 0;
        if compressed {
            return Err(anyhow::anyhow!(
                "compressed Qubicle Binary files are not supported"
            ));
        }

        let _visibility_mask_encoded = read_u32(&mut bytes) != 0;

        let num_matrices = read_u32(&mut bytes);
        if num_matrices != 1 {
            return Err(anyhow::anyhow!(
                "only one matrix expected in Qubicle Binary file"
            ));
        }

        let name_len = read_byte(&mut bytes);
        let _name = String::from_utf8(read(&mut bytes, name_len as usize)).unwrap();

        let size_x = read_u32(&mut bytes) as usize;
        let size_y = read_u32(&mut bytes) as usize;
        let size_z = read_u32(&mut bytes) as usize;

        let _matrix_position = math::Vec3::new(
            read_u32(&mut bytes) as f32,
            read_u32(&mut bytes) as f32,
            read_u32(&mut bytes) as f32,
        );

        let mut matrix = Matrix::new(size_x, size_y, size_z);

        for z in 0..size_z {
            for y in 0..size_y {
                for x in 0..size_x {
                    let position = math::Vec3::new(x as f32, y as f32, z as f32);

                    let color = color::Color::rgb_u8(
                        read_byte(&mut bytes),
                        read_byte(&mut bytes),
                        read_byte(&mut bytes),
                    );

                    // Read the alpha from color. If it is 0 then this voxel is empty.
                    let visible = read_byte(&mut bytes) > 0;

                    matrix.set(
                        position,
                        if visible {
                            Voxel::Solid(color)
                        } else {
                            Voxel::Empty
                        },
                    );
                }
            }
        }

        Ok(matrix)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["qb"];
        EXTENSIONS
    }
}

fn read_byte<T: io::Read>(reader: &mut T) -> u8 {
    read(reader, mem::size_of::<u8>())[0]
}

fn read_u32<T: io::Read>(reader: &mut T) -> u32 {
    LittleEndian::read_u32(&read(reader, mem::size_of::<u32>()))
}

fn read<T: io::Read>(reader: &mut T, size: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(size);

    let mut part_reader = reader.take(size as u64);

    part_reader.read_to_end(&mut buf).unwrap();

    buf
}

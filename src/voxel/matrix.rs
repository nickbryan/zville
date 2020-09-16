use crate::voxel::Voxel;
use bevy::{
    math,
    render::{color, mesh},
};

#[derive(Debug)]
struct Size {
    x: usize,
    y: usize,
    z: usize,
}

impl Size {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn volume(&self) -> usize {
        self.x * self.y * self.z
    }
}

#[derive(Debug)]
pub struct Matrix {
    size: Size,
    voxels: Vec<Voxel>,
}

impl Matrix {
    pub fn new(size_x: usize, size_y: usize, size_z: usize) -> Self {
        let size = Size::new(size_x, size_y, size_z);
        let capacity = size.volume();

        Self {
            size,
            voxels: vec![Voxel::Empty; capacity],
        }
    }

    pub fn set(&mut self, pos: math::Vec3, v: Voxel) {
        let index = self.index(pos);
        self.voxels[index] = v;
    }

    pub fn lookup(&self, pos: math::Vec3) -> &Voxel {
        let index = self.index(pos);
        &self.voxels[index]
    }

    fn index(&self, pos: math::Vec3) -> usize {
        let size_x = self.size.x as f32;
        let size_y = self.size.y as f32;
        let index = pos.x() + pos.y() * size_x + pos.z() * size_x * size_y;

        index as usize
    }

    pub fn mesh_parts(&self) -> Vec<(mesh::Mesh, color::Color)> {
        let mut parts: Vec<(mesh::Mesh, color::Color)> = Vec::new();

        let dimensions = [self.size.x, self.size.y, self.size.z];

        // Iterate over each face of the Matrix.
        for face in 0..6 {
            let is_back_face = face > 2;
            let direction = face % 3;
            let axis_a = (direction + 1) % 3;
            let axis_b = (direction + 2) % 3;

            let mut start_pos = math::Vec3::zero();
            let mut axis_offset = math::Vec3::zero();
            axis_offset[direction] = 1.0;

            let mut mask: Vec<Option<&Voxel>> = vec![None; dimensions[axis_a] * dimensions[axis_b]];

            #[derive(Debug)]
            enum Side {
                Top,
                Bottom,
                Front,
                Back,
                Right,
                Left,
            };

            let side = match direction {
                0 => {
                    if is_back_face {
                        Side::Left
                    } else {
                        Side::Right
                    }
                }
                1 => {
                    if is_back_face {
                        Side::Bottom
                    } else {
                        Side::Top
                    }
                }
                2 => {
                    if is_back_face {
                        Side::Back
                    } else {
                        Side::Front
                    }
                }
                _ => panic!("should not reach this"),
            };

            // Iterate over the matrix layer by layer.
            start_pos[direction] = -1.0;
            while start_pos[direction] < dimensions[direction] as f32 {
                let mut n = 0;

                start_pos[axis_b] = 0.0;
                while start_pos[axis_b] < dimensions[axis_b] as f32 {
                    start_pos[axis_a] = 0.0;
                    while start_pos[axis_a] < dimensions[axis_a] as f32 {
                        let voxel_a = if start_pos[direction] >= 0.0 {
                            Some(self.lookup(start_pos))
                        } else {
                            None
                        };

                        let voxel_b = if start_pos[direction] < (dimensions[direction] - 1) as f32 {
                            Some(self.lookup(start_pos + axis_offset))
                        } else {
                            None
                        };

                        mask[n] = if voxel_a.is_some()
                            && voxel_b.is_some()
                            && voxel_a.unwrap() == voxel_b.unwrap()
                        {
                            None
                        } else if is_back_face {
                            voxel_b
                        } else {
                            voxel_a
                        };

                        n += 1;

                        start_pos[axis_a] += 1.0;
                    }

                    start_pos[axis_b] += 1.0;
                }

                start_pos[direction] += 1.0;

                n = 0;

                for j in 0..dimensions[axis_b] {
                    let mut i = 0;
                    while i < dimensions[axis_a] {
                        if let Some(vox) = mask[n] {
                            // Calculate the width.
                            let mut w = 1;
                            while (i + w) < dimensions[axis_a]
                                && mask[n + w].is_some()
                                && mask[n + w].unwrap() == mask[n].unwrap()
                            {
                                w += 1;
                            }

                            // Calculate the hight.
                            let mut h = 1;
                            'outer: while (j + h) < dimensions[axis_b] {
                                for k in 0..w {
                                    if mask[n + k + h * dimensions[axis_a]].is_none()
                                        || mask[n + k + h * dimensions[axis_a]].unwrap()
                                            != mask[n].unwrap()
                                    {
                                        break 'outer;
                                    }
                                }

                                h += 1;
                            }

                            if let Voxel::Solid(color) = vox {
                                start_pos[axis_a] = i as f32;
                                start_pos[axis_b] = j as f32;

                                let mut du = math::Vec3::zero();
                                du[axis_a] = w as f32;

                                let mut dv = math::Vec3::zero();
                                dv[axis_b] = h as f32;

                                let normal = match side {
                                    Side::Top => [0.0, 1.0, 0.0],
                                    Side::Bottom => [0.0, -1.0, 0.0],
                                    Side::Right => [1.0, 0.0, 0.0],
                                    Side::Left => [-1.0, 0.0, 0.0],
                                    Side::Front => [0.0, 0.0, 1.0],
                                    Side::Back => [0.0, 0.0, -1.0],
                                };

                                let vertices = [
                                    (Into::<[f32; 3]>::into(start_pos), normal, [0.0, 0.0]),
                                    (Into::<[f32; 3]>::into(start_pos + dv), normal, [0.0, 0.0]),
                                    (Into::<[f32; 3]>::into(start_pos + du), normal, [0.0, 0.0]),
                                    (
                                        Into::<[f32; 3]>::into(start_pos + du + dv),
                                        normal,
                                        [0.0, 0.0],
                                    ),
                                ];

                                let indices = if is_back_face {
                                    vec![2, 0, 1, 1, 3, 2]
                                } else {
                                    vec![2, 3, 1, 1, 0, 2]
                                };

                                let mut positions = Vec::new();
                                let mut normals = Vec::new();
                                let mut uvs = Vec::new();
                                for (position, normal, uv) in vertices.iter() {
                                    positions.push(*position);
                                    normals.push(*normal);
                                    uvs.push(*uv);
                                }

                                let m = mesh::Mesh {
                                    primitive_topology:
                                        bevy::render::pipeline::PrimitiveTopology::TriangleList,
                                    attributes: vec![
                                        mesh::VertexAttribute::position(positions),
                                        mesh::VertexAttribute::normal(normals),
                                        mesh::VertexAttribute::uv(uvs),
                                    ],
                                    indices: Some(indices),
                                };

                                parts.push((m, *color));
                            }

                            for l in 0..h {
                                for k in 0..w {
                                    mask[n + k + l * dimensions[axis_a]] = None;
                                }
                            }

                            i += w;
                            n += w;
                        } else {
                            i += 1;
                            n += 1;
                        }
                    }
                }
            }
        }

        parts
    }
}

use std::collections::HashMap;
use std::sync::Arc;

use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Material};
use crate::triangle::Triangle;
use crate::vec3::Point3;

pub fn load_obj(path: &str, material_map: &HashMap<String, Arc<dyn Material>>) -> HittableList {
    let mut world = HittableList::new();

    let (models, materials_result) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .expect("Failed to load OBJ file");

    let materials = materials_result.unwrap_or_default();

    // Build material name lookup: material_id -> material name
    let mat_names: Vec<String> = materials.iter().map(|m| m.name.clone()).collect();

    // Default fallback material (white Lambertian)
    let default_mat: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(0.9, 0.9, 0.9)));

    for model in models {
        let mesh = &model.mesh;
        let positions = &mesh.positions;
        let indices = &mesh.indices;
        let texcoords = &mesh.texcoords;

        // Determine this mesh's material from the map
        let mat: Arc<dyn Material> = match mesh.material_id {
            Some(mid) if mid < mat_names.len() => material_map
                .get(&mat_names[mid])
                .unwrap_or(&default_mat)
                .clone(),
            _ => default_mat.clone(),
        };

        // Helper: get UV from texcoords, fallback to (0,0) if none
        let get_uv = |idx: usize| -> (f64, f64) {
            if texcoords.len() > idx * 2 + 1 {
                (texcoords[idx * 2] as f64, texcoords[idx * 2 + 1] as f64)
            } else {
                (0.0, 0.0)
            }
        };

        for chunk in indices.chunks(3) {
            if chunk.len() < 3 {
                continue;
            }

            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = Point3::new(
                positions[i0 * 3] as f64,
                positions[i0 * 3 + 1] as f64,
                positions[i0 * 3 + 2] as f64,
            );
            let v1 = Point3::new(
                positions[i1 * 3] as f64,
                positions[i1 * 3 + 1] as f64,
                positions[i1 * 3 + 2] as f64,
            );
            let v2 = Point3::new(
                positions[i2 * 3] as f64,
                positions[i2 * 3 + 1] as f64,
                positions[i2 * 3 + 2] as f64,
            );

            let uv0 = get_uv(i0);
            let uv1 = get_uv(i1);
            let uv2 = get_uv(i2);

            world.add(Arc::new(Triangle::new(
                &v0,
                &v1,
                &v2,
                uv0,
                uv1,
                uv2,
                mat.clone(),
            )));
        }
    }

    world
}

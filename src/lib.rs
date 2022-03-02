use wasm_bindgen::prelude::*;

fn lcg_random(seed: f32) -> f32{
    let a: f32 = 214013.0 * seed + 2531011.0;
    let b: f32 = (a as f32) % 2147483648.0;
    let c: f32 = b / 65536.0;
    let d: f32 = (c as f32) * (js_sys::Math::cos(seed as f64) as f32);
    let e: f32 = d % 1.0;
    return if e > 0.0 { e } else { -e };
}
fn get_noise(seed: u32, x: f32, z: f32) -> f32{
    return lcg_random(js_sys::Math::ceil((x * 123123.0 + z * 324234.0 + (seed as f32)) as f64) as f32);
}
#[wasm_bindgen]
pub fn generate_terrain_mesh(resolution: u8, tile_size: u16) -> js_sys::Float32Array{
    let vertices_per_row: u64 = (resolution as u64) + 1;
    let vertex_count: u32 = js_sys::Math::pow(vertices_per_row as f64, 2.0) as u32;
    let index_count: u64 = (js_sys::Math::pow(resolution as f64, 2.0) * 6.0) as u64;
    let vertex_offset: u32 = 0;
    let normal_offset: u32 = vertex_offset + vertex_count * 3;
    let texture_cord_offset: u32 = normal_offset + vertex_count * 3;
    let index_offset: u32 = texture_cord_offset + vertex_count * 2;
    let step_size: f32 = (tile_size as f32) / (resolution as f32);
    let data: js_sys::Float32Array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from_f64((vertex_count * 3 + vertex_count * 3 + vertex_count * 2 + (index_count as u32)) as f64));
    for z in 0..vertices_per_row{
        for x in 0..vertices_per_row{
            let index: u32 = (z + x * vertices_per_row) as u32;
            data.set_index(index * 3 + vertex_offset, ((x * 2 - 1) as f32) * step_size);
            data.set_index(index * 3 + vertex_offset + 2, ((z * 2 - 1) as f32) * step_size);
        }
    }
    return data;
}
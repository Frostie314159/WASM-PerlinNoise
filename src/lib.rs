use wasm_bindgen::prelude::*;

static AMPLITUDE: u8 = 10;
static OCTAVES: u8 = 2;
static ROUGHNESS: f32 = 0.3;

fn lcg_random(seed: f32) -> f32{
    let rand: f32 = (((214013.0 * seed + 2531011.0) % 2147483648.0) / 65536.0) * seed.cos() % 1.0;
    return if rand > 0.0 { rand } else { -rand };
}
fn get_noise(seed: i32, x: f32, z: f32) -> f32{
    return lcg_random((x * 123123.0 + z * 324234.0 + (seed as f32)).ceil());
}
fn get_smooth_noise(seed: i32, x: f32, z: f32) -> f32{
    let corners: f32 = (get_noise(seed, x - 1.0, z - 1.0) + get_noise(seed, x - 1.0, z + 1.0) + get_noise(seed, x + 1.0, z - 1.0) + get_noise(seed, x + 1.0, z + 1.0)) / 16.0;
    let sides: f32 = (get_noise(seed, x - 1.0, z) + get_noise(seed, x, z + 1.0) + get_noise(seed, x + 1.0, z) + get_noise(seed, x, z - 1.0)) / 8.0;
    let middle: f32 = get_noise(seed, x, z) / 4.0;
    return corners + sides + middle;
}
fn interpolate(a: f32, b: f32, blend: f32) -> f32{
    let f: f32 = (1.0 - (blend * std::f32::consts::PI)) * 0.5;
    return a * (1.0 - f) + b * f;
}
fn get_interpolated_noise(seed: i32, x: f32, z: f32) -> f32{
    let int_x: i32 = x as i32;
    let int_z: i32 = z as i32;
    let frac_x: f32 = x - (int_x as f32);
    let frac_z: f32 = z - (int_z as f32);

    let v1: f32 = get_smooth_noise(seed, int_x as f32, int_z as f32);
    let v2: f32 = get_smooth_noise(seed, (int_x as f32) + 1.0, int_z as f32);
    let v3: f32 = get_smooth_noise(seed, int_x as f32, (int_z as f32) + 1.0);
    let v4: f32 = get_smooth_noise(seed, (int_x as f32) + 1.0, (int_z as f32) + 1.0);

    let i1: f32 = interpolate(v1, v2, frac_x);
    let i2: f32 = interpolate(v3, v4, frac_x);
    return interpolate(i1, i2, frac_z);
}
fn get_height(seed: i32, x: f32, z: f32) -> f32{
    let mut total: f32 = 0.0;
    let d: u32 = (OCTAVES - 1).pow(2) as u32;
    for index in 0..OCTAVES{
        let freq: f32 = f32::powf(index as f32, 2.0) / (d as f32);
        let amp: f32 = ROUGHNESS.powf(index as f32) * (AMPLITUDE as f32);
        total += get_interpolated_noise(seed, x * freq, z * freq) * amp;
    }
    return total;
}
#[wasm_bindgen]
pub fn generate_terrain_mesh(resolution: u32, tile_size: u32) -> js_sys::Float32Array{
    let vertices_per_row: u32 = (resolution as u32) + 1;
    let vertex_count: u32 = vertices_per_row.pow(2) as u32;
    let index_count: u32 = resolution.pow(2) * 6;
    let vertex_offset: u32 = 0;
    let normal_offset: u32 = vertex_offset + vertex_count * 3;
    let texture_cord_offset: u32 = normal_offset + vertex_count * 3;
    let index_offset: u32 = texture_cord_offset + vertex_count * 2;
    let step_size: f32 = (tile_size as f32) / (resolution as f32);
    let seed: i32 = js_sys::Math::random() as i32;
    let data: js_sys::Float32Array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from_f64((vertex_count * 3 + vertex_count * 3 + vertex_count * 2 + index_count) as f64));
    for z in 0..vertices_per_row{
        for x in 0..vertices_per_row{
            let index: u32 = (z + x * vertices_per_row) as u32;
            data.set_index(index * 3 + vertex_offset, (((x as i32) * 2 - 1) as f32) * step_size);
            data.set_index(index * 3 + vertex_offset + 2, (((z as i32) * 2 - 1) as f32) * step_size);
            data.set_index(index * 3 + vertex_offset + 1, 0.0);
            
            data.set_index(index * 2 + texture_cord_offset, (x as f32) * step_size);
            data.set_index(index * 2 + texture_cord_offset + 1, (z as f32) * step_size);
            
            let height_l: f32 = get_height(seed, ((x - 1) as f32) * step_size, (z as f32) * step_size);
            let height_r: f32 = get_height(seed, ((x + 1) as f32) * step_size, (z as f32) * step_size);
            let height_d: f32 = get_height(seed, (x as f32) * step_size, ((z + vertices_per_row) as f32) * step_size);
            let height_u: f32 = get_height(seed, (x as f32) * step_size, ((z - vertices_per_row) as f32) * step_size);
            let vector: [f32; 3] = [height_u - height_d, 2.0, height_l - height_r];
            let mag: f32 = f32::sqrt(vector[0].powf(2.0) + vector[1].powf(2.0) + vector[2].powf(2.0));
            data.set_index(index * 3 + normal_offset, vector[0] / mag);
            data.set_index(index * 3 + normal_offset + 1, vector[1] / mag);
            data.set_index(index * 3 + normal_offset + 2, vector[2] / mag);
        }
    }
    for z in 0..resolution{
        for x in 0..resolution{
            let index: u32 = z * resolution + x;
            let upper_left_vertex: f32 = index as f32;
            let upper_right_vertex: f32 = upper_left_vertex + 1.0;
            let lower_left_vertex: f32 = (index as f32) + (vertices_per_row as f32);
            let lower_right_vertex: f32 = lower_left_vertex + 1.0;
            data.set_index(index * 3 + index_offset, upper_left_vertex);
            data.set_index(index * 3 + index_offset + 1, upper_right_vertex);
            data.set_index(index * 3 + index_offset + 2, lower_left_vertex);
            data.set_index(index * 3 + index_offset + 3, lower_left_vertex);
            data.set_index(index * 3 + index_offset + 4, upper_right_vertex);
            data.set_index(index * 3 + index_offset + 5, lower_right_vertex);
        }
    }
    return data;
}
#[wasm_bindgen]
pub fn get_range_from_array(array: js_sys::Float32Array, first: u32, last: u32) -> js_sys::Float32Array{
    let temp_array: js_sys::Float32Array = js_sys::Float32Array::new(&wasm_bindgen::JsValue::from_f64((last - first) as f64));
    for index in first..last{
        temp_array.set_index(index - first, array.get_index(index));
    }
    return temp_array;
}
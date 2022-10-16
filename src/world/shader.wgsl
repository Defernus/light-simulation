struct Params {
    size: u32,
    amount: u32,
};

struct Photon {
    pos: vec4<f32>,
    dir: vec4<f32>,
    wavelength: f32,
}

@group(0)
@binding(0)
var<uniform> params : Params;

@group(0)
@binding(1)
var<storage, read_write> v_indices: array<Photon>; // this is used as both input and output for convenience

fn process(photon: Photon) -> Photon {
    return Photon (
        photon.pos + photon.dir,
        photon.dir,
        photon.wavelength,
    );
}

@compute
@workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x >= params.size || global_id.y >= params.size || global_id.z >= params.size) {
        return;
    }

    let index: u32 = global_id.x + global_id.y * params.size + global_id.z * params.size * params.size;
    v_indices[index] = process(v_indices[index]);
}

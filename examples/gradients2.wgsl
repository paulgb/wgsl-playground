struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

struct Uniforms {
    mouse: vec2<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

fn fs_rot(angle: f32) -> mat2x2<f32> {
    return mat2x2<f32>(
        vec2<f32>(cos(angle), -sin(angle)),
        vec2<f32>(sin(angle), cos(angle)),
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let r_angle = sin(uniforms.time / 1.4) / 3.;
    let ra = fs_rot(r_angle) * in.coord;
    let r = sin(ra.x * 3.) / 2. + 0.5;

    let g_angle = sin(uniforms.time / 1.9) / 2.;
    let ga = fs_rot(g_angle) * in.coord;
    let g = sin(ga.x * 4.) / 2. + 0.5;

    let b_angle = sin(uniforms.time / 1.6) / 1.2;
    let ba = fs_rot(b_angle) * in.coord;
    let b = sin(ba.x * 5.) / 2. + 0.5;

    return vec4<f32>(r, g, b, 1.0);
}
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

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normalized = (in.coord + vec2<f32>(1., 1.)) / 2.;

    let angle = sin(uniforms.time);
    let cc = sin(uniforms.time / 1.4 + 0.3);
    let r = sin(uniforms.time / 4.4) / 2. + 0.5;
    let rot: mat2x2<f32> = mat2x2<f32>(
        vec2<f32>(cos(angle), -sin(angle)),
        vec2<f32>(sin(angle), cos(angle))
    );

    let tc: vec2<f32> = rot * in.coord;

    let c = sin(tc.x * 12. * cc) / 2. + 0.5;
    let d = sin(tc.y * 12.) / 2. + 0.5;

    return vec4<f32>(r, c, d, 1.0);
}
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let r: f32 = dot(in.coord, in.coord);

    if (r > .95) {
        discard;
    }

    let normalized = (in.coord + vec2<f32>(1., 1.)) / 2.;
    return vec4<f32>(normalized.rg, 0., 1.0);
}
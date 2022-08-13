struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {

    var vertices = array<vec2<f32>, 3>(
        vec2<f32>(-1., 1.),
        vec2<f32>(3.0, 1.),
        vec2<f32>(-1., -3.0),
    );

    var out: VertexOutput;
    out.coord =vertices[in.vertex_index];
    out.position = vec4<f32>(out.coord, 0.0, 1.0);

    return out;
}
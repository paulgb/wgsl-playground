var vertices: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(-1., 1.),
    vec2<f32>(3.0, 1.),
    vec2<f32>(-1., -3.0),
);

struct VertexOutput {
    [[location(0)]] coord: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    out.coord = vertices[in_vertex_index];
    out.position = vec4<f32>(out.coord, 0.0, 1.0);
    return out;
}
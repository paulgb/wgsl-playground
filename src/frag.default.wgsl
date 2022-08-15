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
    // fs_main is run once for every pixel in the image, each frame.
    // You can access the pixel's location in two ways:
    // - `in.coord` is a vec2 containing the pixel location in clip
    //    space, i.e. scaled from -1.0 to 1.0 left to right, and 1.0
    //    to -1 top to bottom.
    // - `in.position` is a vec4 containing the pixel location in
    //   device (i.e. pixel/screen unit) coordinates.
    let normalized = (in.coord + vec2<f32>(1., 1.)) / 2.;

    // fs_main must return a vec4 representing an RGBA value scaled
    // between 0 and 1. E.g. vec4<f32>(0., 0., 0., 1.) is black and
    // vec4<f32>(1., 1., 1., 1.) is white.
    //return vec4<f32>(normalized.rg, 0., 1.0);
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
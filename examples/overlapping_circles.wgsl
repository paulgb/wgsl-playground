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

fn circle(c: vec2<f32>, r: f32, probe: vec2<f32>) -> bool {
    let delta = probe - c;
    if (dot(delta, delta) > r * r) {
        return false;
    } else {
        return true;
    }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var c: u32 = 0u;

    var colors: array<vec3<f32>, 10> = array<vec3<f32>, 10>(
        vec3<f32>(0., 0., 0.),
        vec3<f32>(0.01, 0.01, 0.01),
        vec3<f32>(0.4, 0.0, 0.0),
        vec3<f32>(0.5, 0.2, 0.2),
        vec3<f32>(0.6, 0.5, 0.4),
        vec3<f32>(0.7, 0.5, 0.3),
        vec3<f32>(0.9, 0.6, 0.4),
        vec3<f32>(1., 1., 1.),
        vec3<f32>(1., 1., 1.),
        vec3<f32>(1., 1., 1.),
    );

    for (var i: u32 = 1u; i < 500u; i = i + 1u) {
        let rx: f32 = fract(sin(f32(i)) * 1000.);
        let ry: f32 = fract(sin(f32(i)) * 1001.);
        let sx: f32 = fract(sin(f32(i)) * 1009.) - 0.5;
        let sy: f32 = fract(sin(f32(i)) * 1011.);

        let xx = fract(rx + sx * uniforms.time / 4.) * 2.4 - 1.2;
        let yy = fract(ry + sy * uniforms.time / 4.) * 2.4 - 1.2;

        let r = (fract(sin(f32(i)) * 1044.) + 1. / 2.) * 0.06 + 0.01;
        let rr = r * (sin(uniforms.time) + 2.) / 3.;

        if (circle(vec2<f32>(xx, yy), rr, in.coord)) {
            c = c + 1u;
        }
    }

    if (c >= 1u && abs(in.coord.y - uniforms.mouse.y) < 0.3 && uniforms.mouse.y < 0.9) {
        c = 7u - c;
    }

    return vec4<f32>(colors[c], 1.0);
}
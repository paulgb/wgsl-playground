// This is a work in progress.

struct VertexOutput {
    [[location(0)]] coord: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

let ITERATIONS: i32 = 10;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var x: f32 = 0.;
    var y: f32 = 0.;
    var i: i32 = 0;
    
    for (; i < ITERATIONS; i = i + 1) {
        if (x*x + y*y > 4.) {
            break;
        }
        let xtemp = (x * x) - (y * y) + in.coord.x;
        y = 2. * x * y + in.coord.y;
        x = xtemp;
    }

    if (i == ITERATIONS) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    } else {
        return vec4<f32>(0., 0., 0., 1.0);
    }
}
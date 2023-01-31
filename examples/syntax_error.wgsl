let TAU:f32 = 6.28318530718;

struct Uniforms {
    mouse: vec2<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};


fn plot(uv: vec2<f32>, b: f32) -> f32 {
    return smoothstep(b - 0.02, b, uv.y)
         - smoothstep(b, b + 0.02, uv.y);
}

fn hsb2rgb(c: vec3<f32>) -> vec3<f32> {
    let v = vec3<f32>(0.0, 4.0, 2.0) + c.x * 6.0;
    let m = v % vec3<f32>(6.0);
    let a = abs(m - 3.0);
    var rgb = clamp(a - 1.0, vec3<f32>(0.0), vec3<f32>(1.0));

    rgb = rgb * rgb * (3.0 - 2.0 * rgb);
    return c.z * mix( vec3<f32>(1.0), rgb, c.y);
}

fn random(p: vec2<f32>) -> f32 {
    return fract(
        sin(dot(p, vec2<f32>(12.9898, 78.233)))
        * 43758.5453123
    );
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
   let uv = (in.coord + vec2<f32>(1., 1.)) / 2.;

 /*
     col = col * (1.0 - smoothstep(r, r + sharpness, length( q )))
               * (smoothstep(r - rim, r - rim + sharpness, length( q )))
               ;
*/
/*
    var col = vec3<f32>(1.0, 0.0, 1.0);
    let thickness = 0.01;
     col = col * smoothstep(1.0 - thickness, 1.0, uv.x)
         + col * smoothstep(1.0 - thickness, 1.0, uv.y)
         + col * smoothstep(1.0 - thickness, 1.0, 1.0 - uv.x)
         + col * smoothstep(1.0 - thickness, 1.0, 1.0 - uv.y)
*/



//
//         + col * sin(uv.y * 2.0)
//         + col / atan(uv.x * 10.0)
     ;

     //let y = pow(uv.x - 0.5, 2.0);
//     let y = smoothstep(0.1, 0.9, uv.x);
//
//     let pct = plot(uv, y);
//
//     let color = pct * vec3<f32>(0.0, 1.0, 0.0);
/*

    var color = vec3<f32>(0.0);

    let to_center = vec2<f32>(0.5) - uv;
    let angle = atan2(to_center.y, to_center.x);
    let radius = length(to_center) * 2.0;


    color = hsb2rgb(vec3<f32>(
        1.0,
        radius,
        1.0
    ));

*/

    let rnd = random(floor(uv * 10.0));
    let off = uniforms.time * 0.2;
    let i = floor(uv * 10.0);
    let f = fract(uv * 10.0 );


    let y = mix(random(i), random(i + 1.0), 0.4);


     return vec4<f32>(vec3<f32>(y), 1.0);
}
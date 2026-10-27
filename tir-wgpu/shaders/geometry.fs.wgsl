struct Output {
    @location(0) out_color: vec4<f32>,
};

// Stable pseudo-random value in [0, 1) for screen-space grain.
fn hash21(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    p3 += dot(p3, p3.yzx + vec3<f32>(33.33));
    return fract((p3.x + p3.y) * p3.z);
}

@fragment
fn main(
    @location(0) v_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
) -> Output {
    // Subtle monochrome grain, like the surface of a glazed ceramic tile.
    let grain = (hash21(position.xy) - 0.5) * 0.035;
    let rgb = clamp(
        v_color.rgb + vec3<f32>(grain, grain, grain),
        vec3<f32>(0.0, 0.0, 0.0),
        vec3<f32>(1.0, 1.0, 1.0),
    );
    return Output(vec4<f32>(rgb, v_color.a));
}

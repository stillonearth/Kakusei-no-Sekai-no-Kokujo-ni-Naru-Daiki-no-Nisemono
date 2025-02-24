#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

const PIXEL_SIZE_FAC: f32 = 700.0;
const SPIN_EASE: f32 = 0.5;
const COLOUR_2: vec4<f32> = vec4<f32>(0.0, 156.0/255.0, 1.0, 1.0);
const COLOUR_1: vec4<f32> = vec4<f32>(0.85, 0.2, 0.2, 1.0);
const COLOUR_3: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
const SPIN_AMOUNT: f32 = 0.7;
const CONTRAST: f32 = 1.5;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Convert UV coordinates to pixel-like coordinates
    let screen_size = vec2<f32>(1.0, 1.0); // Approximation for iResolution
    let pixel_size = length(screen_size) / PIXEL_SIZE_FAC;
    
    // Convert to pixelated coordinates
    let frag_coord = in.uv * screen_size;
    let quantized_uv = floor(frag_coord / pixel_size) * pixel_size;
    var uv = (quantized_uv - 0.5 * screen_size) / length(screen_size);
    
    let uv_len = length(uv);
    
    // Swirl effect
    let speed = globals.time * SPIN_EASE * 0.1 + 302.2;
    let base_angle = atan2(uv.y, uv.x);
    let angle_offset = SPIN_EASE * 20.0 * (SPIN_AMOUNT * uv_len + (1.0 - SPIN_AMOUNT));
    let new_angle = base_angle + speed - angle_offset;
    
    let mid = (screen_size / length(screen_size)) * 0.5;
    uv = vec2<f32>(
        uv_len * cos(new_angle) + mid.x,
        uv_len * sin(new_angle) + mid.y
    ) - mid;

    // Paint distortion effect
    uv *= 30.0;
    let time_speed = globals.time;
    var uv2 = vec2<f32>(uv.x + uv.y, uv.x + uv.y);

    for(var i: i32 = 0; i < 5; i++) {
        uv2 += uv + cos(length(uv));
        uv += 0.5 * vec2<f32>(
            cos(5.1123314 + 0.353 * uv2.y + time_speed * 0.131121),
            sin(uv2.x - 0.113 * time_speed)
        );
        uv -= cos(uv.x + uv.y) - sin(uv.x * 0.711 - uv.y);
    }

    // Color calculation
    let contrast_mod = 0.25 * CONTRAST + 0.5 * SPIN_AMOUNT + 1.2;
    let paint_value = clamp(length(uv) * 0.035 * contrast_mod, 0.0, 2.0);
    
    let c1 = max(0.0, 1.0 - contrast_mod * abs(1.0 - paint_value));
    let c2 = max(0.0, 1.0 - contrast_mod * abs(paint_value));
    let c3 = 1.0 - min(c1 + c2, 1.0);

    // Combine colors
    let base_strength = 0.3 / CONTRAST;
    let blended = COLOUR_1 * c1 + COLOUR_2 * c2 + vec4<f32>(COLOUR_3.rgb * c3, c3);
    let highlights = 0.3 * max(5.0 * c1 - 4.0, 0.0) + 0.4 * max(5.0 * c2 - 4.0, 0.0);
    
    return base_strength * COLOUR_1 + 
           (1.0 - base_strength) * blended + 
           highlights;
}
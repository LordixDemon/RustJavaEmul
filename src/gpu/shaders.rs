pub(crate) const FRAME_SHADER: &str = r#"
struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOut {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0),
    );

    var out: VertexOut;
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = uvs[vertex_index];
    return out;
}

@group(0) @binding(0) var frame_texture: texture_2d<f32>;
@group(0) @binding(1) var frame_sampler: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(frame_texture, frame_sampler, in.uv);
}
"#;

pub(crate) const V3_SHADER: &str = r#"
struct VertexIn {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.uv = in.uv;
    out.color = in.color;
    return out;
}

@group(0) @binding(0) var tri_texture: texture_2d<f32>;
@group(0) @binding(1) var tri_sampler: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let color = textureSample(tri_texture, tri_sampler, in.uv) * in.color;
    if (color.a <= 0.003) {
        discard;
    }
    return color;
}
"#;

pub(crate) const M3G_SHADER: &str = r#"
struct VertexIn {
    @location(0) position: vec4<f32>,
    @location(1) uv01: vec4<f32>,
    @location(2) color: vec4<f32>,
    @location(3) misc: vec4<f32>,
    @location(4) misc2: vec4<f32>,
    @location(5) fog_range: vec4<f32>,
    @location(6) fog_color: vec4<f32>,
    @location(7) blend0: vec4<f32>,
    @location(8) blend1: vec4<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv01: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) misc: vec4<f32>,
    @location(3) misc2: vec4<f32>,
    @location(4) fog_range: vec4<f32>,
    @location(5) fog_color: vec4<f32>,
    @location(6) blend0: vec4<f32>,
    @location(7) blend1: vec4<f32>,
};

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(in.position.xy, in.position.z, 1.0);
    out.uv01 = in.uv01;
    out.color = in.color;
    out.misc = in.misc;
    out.misc2 = in.misc2;
    out.fog_range = in.fog_range;
    out.fog_color = in.fog_color;
    out.blend0 = in.blend0;
    out.blend1 = in.blend1;
    return out;
}

@group(0) @binding(0) var tri_texture0: texture_2d<f32>;
@group(0) @binding(1) var tri_sampler0: sampler;
@group(0) @binding(2) var tri_texture1: texture_2d<f32>;
@group(0) @binding(3) var tri_sampler1: sampler;

fn apply_texture(mode: f32, format: f32, frag: vec4<f32>, texel: vec4<f32>, blend_color: vec4<f32>) -> vec4<f32> {
    let m = i32(mode + 0.5);
    if (m == 0) {
        return frag;
    }
    let f = i32(format + 0.5);
    let is_alpha = f == 96;
    let is_rgb = f == 99 || f == 97;
    if (m == 228) {
        if (is_alpha) {
            return vec4<f32>(frag.rgb, texel.a);
        }
        if (is_rgb) {
            return vec4<f32>(texel.rgb, frag.a);
        }
        return texel;
    }
    if (m == 226) {
        return vec4<f32>(mix(frag.rgb, texel.rgb, texel.a), frag.a);
    }
    if (m == 224) {
        let a = select(frag.a * texel.a, frag.a, is_rgb);
        return vec4<f32>(min(frag.rgb + texel.rgb, vec3<f32>(1.0)), a);
    }
    if (m == 225) {
        let a = select(frag.a * texel.a, frag.a, is_rgb);
        return vec4<f32>(frag.rgb * (vec3<f32>(1.0) - texel.rgb) + blend_color.rgb * texel.rgb, a);
    }
    if (is_alpha) {
        return vec4<f32>(frag.rgb, frag.a * texel.a);
    }
    if (is_rgb) {
        return vec4<f32>(frag.rgb * texel.rgb, frag.a);
    }
    return frag * texel;
}

fn apply_fog(color: vec4<f32>, z: f32, mode: f32, density: f32, near: f32, far: f32, fog_color: vec3<f32>) -> vec4<f32> {
    if (mode < 0.5) {
        return color;
    }
    var factor: f32;
    if (mode > 80.5) {
        let range = far - near;
        if (abs(range) <= 1.0e-6) {
            factor = 1.0;
        } else {
            factor = clamp((far - z) / range, 0.0, 1.0);
        }
    } else {
        factor = clamp(exp(-max(density, 0.0) * max(z, 0.0)), 0.0, 1.0);
    }
    return vec4<f32>(mix(fog_color, color.rgb, factor), color.a);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let texel0 = textureSample(tri_texture0, tri_sampler0, in.uv01.xy);
    let texel1 = textureSample(tri_texture1, tri_sampler1, in.uv01.zw);
    var color = apply_texture(in.misc.y, in.misc.z, in.color, texel0, in.blend0);
    color = apply_texture(in.misc2.x, in.misc2.y, color, texel1, in.blend1);
    color = apply_fog(color, in.misc.w, in.misc2.z, in.misc2.w, in.fog_range.x, in.fog_range.y, in.fog_color.rgb);
    if (color.a <= max(in.misc.x, 0.003)) {
        discard;
    }
    return color;
}
"#;

use super::super::{
    CompositingMode, Fog, Light, PolygonMode,
    math::{dot3, normalize3},
};
use super::texture::*;
#[allow(unused_imports)]
use super::types::*;

pub(crate) fn rasterize_triangle(
    pixels: &mut [i32],
    depth: &mut [f32],
    width: i32,
    height: i32,
    v0: M3gDrawVertex,
    v1: M3gDrawVertex,
    v2: M3gDrawVertex,
    appearance: &M3gAppearanceState,
    depth_enabled: bool,
    camera_near: f32,
    camera_far: f32,
    depth_range_near: f32,
    depth_range_far: f32,
    dirty: &mut M3gDirtyRect,
) -> usize {
    let area = edge(v0.x, v0.y, v1.x, v1.y, v2.x, v2.y);
    if area.abs() <= 0.0001 {
        return 0;
    }
    if should_cull_triangle(area, appearance.culling, appearance.winding) {
        return 0;
    }

    let min_x = floor_i32(v0.x.min(v1.x).min(v2.x)).clamp(0, width - 1);
    let max_x = ceil_i32(v0.x.max(v1.x).max(v2.x)).clamp(0, width - 1);
    let min_y = floor_i32(v0.y.min(v1.y).min(v2.y)).clamp(0, height - 1);
    let max_y = ceil_i32(v0.y.max(v1.y).max(v2.y)).clamp(0, height - 1);
    if min_x > max_x || min_y > max_y {
        return 0;
    }

    let inv_area = 1.0 / area;
    let edge_epsilon = 0.0001 * area.abs();
    let positive_area = area > 0.0;
    let max_edge = ((v1.x - v0.x).abs() + (v1.y - v0.y).abs())
        .max((v2.x - v1.x).abs() + (v2.y - v1.y).abs())
        .max((v0.x - v2.x).abs() + (v0.y - v2.y).abs())
        .max(1.0);
    let depth0 = depth_buffer_value(v0.z, camera_near, camera_far, depth_range_near, depth_range_far);
    let depth1 = depth_buffer_value(v1.z, camera_near, camera_far, depth_range_near, depth_range_far);
    let depth2 = depth_buffer_value(v2.z, camera_near, camera_far, depth_range_near, depth_range_far);
    let depth_slope = (depth1 - depth0).abs().max((depth2 - depth0).abs()).max((depth2 - depth1).abs()) / max_edge;
    let depth_bias = appearance.depth_offset_factor * depth_slope * 0.001 + appearance.depth_offset_units * 0.0001;
    let depth_test = depth_enabled && appearance.depth_test;
    let depth_write = depth_enabled && appearance.depth_write;
    let constant_fragment_color = if v0.color == v1.color && v1.color == v2.color {
        Some(v0.color)
    } else {
        None
    };
    let alpha_threshold_u32 = (appearance.alpha_threshold * 255.0).ceil().max(0.0) as u32;
    let inv_z0 = 1.0 / v0.z.max(0.0001);
    let inv_z1 = 1.0 / v1.z.max(0.0001);
    let inv_z2 = 1.0 / v2.z.max(0.0001);
    let u0_inv_z = v0.u * inv_z0;
    let u1_inv_z = v1.u * inv_z1;
    let u2_inv_z = v2.u * inv_z2;
    let v0_inv_z = v0.v * inv_z0;
    let v1_inv_z = v1.v * inv_z1;
    let v2_inv_z = v2.v * inv_z2;
    let u10_inv_z = v0.u1 * inv_z0;
    let u11_inv_z = v1.u1 * inv_z1;
    let u12_inv_z = v2.u1 * inv_z2;
    let v10_inv_z = v0.v1 * inv_z0;
    let v11_inv_z = v1.v1 * inv_z1;
    let v12_inv_z = v2.v1 * inv_z2;
    let camera_depth_scale = if camera_far > camera_near {
        Some(1.0 / (camera_far - camera_near))
    } else {
        None
    };
    let edge0 = EdgeStep::new(v1.x, v1.y, v2.x, v2.y, min_x, min_y);
    let edge1 = EdgeStep::new(v2.x, v2.y, v0.x, v0.y, min_x, min_y);
    let edge2 = EdgeStep::new(v0.x, v0.y, v1.x, v1.y, min_x, min_y);
    let inv_z_plane = AttrStep::new(inv_z0, inv_z1, inv_z2, edge0, edge1, edge2, inv_area);
    let u_inv_z_plane = AttrStep::new(u0_inv_z, u1_inv_z, u2_inv_z, edge0, edge1, edge2, inv_area);
    let v_inv_z_plane = AttrStep::new(v0_inv_z, v1_inv_z, v2_inv_z, edge0, edge1, edge2, inv_area);
    let u1_inv_z_plane = AttrStep::new(u10_inv_z, u11_inv_z, u12_inv_z, edge0, edge1, edge2, inv_area);
    let v1_inv_z_plane = AttrStep::new(v10_inv_z, v11_inv_z, v12_inv_z, edge0, edge1, edge2, inv_area);
    let u_plane = AttrStep::new(v0.u, v1.u, v2.u, edge0, edge1, edge2, inv_area);
    let v_plane = AttrStep::new(v0.v, v1.v, v2.v, edge0, edge1, edge2, inv_area);
    let u1_plane = AttrStep::new(v0.u1, v1.u1, v2.u1, edge0, edge1, edge2, inv_area);
    let v1_plane = AttrStep::new(v0.v1, v1.v1, v2.v1, edge0, edge1, edge2, inv_area);
    let z_plane = AttrStep::new(v0.z, v1.z, v2.z, edge0, edge1, edge2, inv_area);
    let mut written = 0;
    let mut row0 = edge0.start;
    let mut row1 = edge1.start;
    let mut row2 = edge2.start;
    let mut row_inv_z = inv_z_plane.start;
    let mut row_u_inv_z = u_inv_z_plane.start;
    let mut row_v_inv_z = v_inv_z_plane.start;
    let mut row_u1_inv_z = u1_inv_z_plane.start;
    let mut row_v1_inv_z = v1_inv_z_plane.start;
    let mut row_u = u_plane.start;
    let mut row_v = v_plane.start;
    let mut row_u1 = u1_plane.start;
    let mut row_v1 = v1_plane.start;
    let mut row_z = z_plane.start;
    for y in min_y..=max_y {
        let mut edge_value0 = row0;
        let mut edge_value1 = row1;
        let mut edge_value2 = row2;
        let mut inv_z_value = row_inv_z;
        let mut u_inv_z_value = row_u_inv_z;
        let mut v_inv_z_value = row_v_inv_z;
        let mut u1_inv_z_value = row_u1_inv_z;
        let mut v1_inv_z_value = row_v1_inv_z;
        let mut u_value = row_u;
        let mut v_value = row_v;
        let mut u1_value = row_u1;
        let mut v1_value = row_v1;
        let mut z_value = row_z;
        for x in min_x..=max_x {
            let pixel_edge0 = edge_value0;
            let pixel_edge1 = edge_value1;
            let pixel_edge2 = edge_value2;
            edge_value0 += edge0.x_step;
            edge_value1 += edge1.x_step;
            edge_value2 += edge2.x_step;
            let inv_z = inv_z_value;
            let u_inv_z = u_inv_z_value;
            let v_inv_z = v_inv_z_value;
            let u1_inv_z = u1_inv_z_value;
            let v1_inv_z = v1_inv_z_value;
            let affine_u = u_value;
            let affine_v = v_value;
            let affine_u1 = u1_value;
            let affine_v1 = v1_value;
            let affine_z = z_value;
            inv_z_value += inv_z_plane.x_step;
            u_inv_z_value += u_inv_z_plane.x_step;
            v_inv_z_value += v_inv_z_plane.x_step;
            u1_inv_z_value += u1_inv_z_plane.x_step;
            v1_inv_z_value += v1_inv_z_plane.x_step;
            u_value += u_plane.x_step;
            v_value += v_plane.x_step;
            u1_value += u1_plane.x_step;
            v1_value += v1_plane.x_step;
            z_value += z_plane.x_step;
            let inside = if positive_area {
                pixel_edge0 >= -edge_epsilon && pixel_edge1 >= -edge_epsilon && pixel_edge2 >= -edge_epsilon
            } else {
                pixel_edge0 <= edge_epsilon && pixel_edge1 <= edge_epsilon && pixel_edge2 <= edge_epsilon
            };
            if !inside {
                continue;
            }

            let reciprocal_z = if inv_z.is_finite() && inv_z.abs() > f32::EPSILON {
                Some(1.0 / inv_z)
            } else {
                None
            };
            let camera_z = reciprocal_z.unwrap_or(affine_z);
            let z = depth_buffer_value_fast(camera_z, camera_near, depth_range_near, depth_range_far, camera_depth_scale) + depth_bias;
            let index = (y * width + x) as usize;
            if depth_test && z > depth[index] {
                continue;
            }

            let fragment_color = match constant_fragment_color {
                Some(c) => c,
                None => {
                    let w0 = pixel_edge0 * inv_area;
                    let w1 = pixel_edge1 * inv_area;
                    let w2 = pixel_edge2 * inv_area;
                    interpolate_color(v0.color, v1.color, v2.color, w0, w1, w2)
                }
            };
            let (u, v) = if appearance.perspective_correction {
                if let Some(reciprocal_z) = reciprocal_z {
                    (u_inv_z * reciprocal_z, v_inv_z * reciprocal_z)
                } else {
                    (affine_u, affine_v)
                }
            } else {
                (affine_u, affine_v)
            };
            let (u1, v1) = if appearance.perspective_correction {
                if let Some(reciprocal_z) = reciprocal_z {
                    (u1_inv_z * reciprocal_z, v1_inv_z * reciprocal_z)
                } else {
                    (affine_u1, affine_v1)
                }
            } else {
                (affine_u1, affine_v1)
            };
            let mut color = if let Some(texture) = appearance.texture.as_ref() {
                blend_texture(texture, fragment_color, sample_texture(texture, u, v))
            } else {
                fragment_color
            };
            if let Some(texture) = appearance.texture1.as_ref() {
                color = blend_texture(texture, color, sample_texture(texture, u1, v1));
            }
            if let Some(fog) = appearance.fog {
                color = apply_distance_fog(color, fog, camera_z);
            }

            let alpha = ((color as u32) >> 24) & 0xff;
            if alpha == 0 || alpha < alpha_threshold_u32 {
                continue;
            }

            if depth_write {
                depth[index] = z;
            }
            if appearance.color_write {
                if appearance.blending == CompositingMode::REPLACE && appearance.alpha_write {
                    pixels[index] = color;
                } else {
                    if !appearance.alpha_write {
                        let dst_alpha = (pixels[index] as u32) & 0xff00_0000;
                        let alpha = if dst_alpha == 0 { 0xff00_0000 } else { dst_alpha };
                        color = (alpha | ((color as u32) & 0x00ff_ffff)) as i32;
                    }
                    pixels[index] = compose_m3g_pixel(pixels[index], color, appearance.blending);
                }
            }
            written += 1;
        }
        row0 += edge0.y_step;
        row1 += edge1.y_step;
        row2 += edge2.y_step;
        row_inv_z += inv_z_plane.y_step;
        row_u_inv_z += u_inv_z_plane.y_step;
        row_v_inv_z += v_inv_z_plane.y_step;
        row_u1_inv_z += u1_inv_z_plane.y_step;
        row_v1_inv_z += v1_inv_z_plane.y_step;
        row_u += u_plane.y_step;
        row_v += v_plane.y_step;
        row_u1 += u1_plane.y_step;
        row_v1 += v1_plane.y_step;
        row_z += z_plane.y_step;
    }
    if written > 0 {
        dirty.include_rect(min_x, min_y, max_x, max_y);
    }
    written
}

#[derive(Clone, Copy)]
struct EdgeStep {
    start: f32,
    x_step: f32,
    y_step: f32,
}

impl EdgeStep {
    fn new(ax: f32, ay: f32, bx: f32, by: f32, min_x: i32, min_y: i32) -> Self {
        let x_step = by - ay;
        let y_step = -(bx - ax);
        let px = min_x as f32 + 0.5;
        let py = min_y as f32 + 0.5;
        Self {
            start: (px - ax) * x_step + (py - ay) * y_step,
            x_step,
            y_step,
        }
    }
}

#[derive(Clone, Copy)]
struct AttrStep {
    start: f32,
    x_step: f32,
    y_step: f32,
}

impl AttrStep {
    fn new(a0: f32, a1: f32, a2: f32, edge0: EdgeStep, edge1: EdgeStep, edge2: EdgeStep, inv_area: f32) -> Self {
        Self {
            start: (a0 * edge0.start + a1 * edge1.start + a2 * edge2.start) * inv_area,
            x_step: (a0 * edge0.x_step + a1 * edge1.x_step + a2 * edge2.x_step) * inv_area,
            y_step: (a0 * edge0.y_step + a1 * edge1.y_step + a2 * edge2.y_step) * inv_area,
        }
    }
}

pub(crate) fn depth_buffer_value(camera_z: f32, camera_near: f32, camera_far: f32, range_near: f32, range_far: f32) -> f32 {
    let normalized = if camera_far > camera_near {
        ((camera_z - camera_near) / (camera_far - camera_near)).clamp(0.0, 1.0)
    } else {
        camera_z.max(0.0)
    };
    range_near + normalized * (range_far - range_near)
}

fn depth_buffer_value_fast(camera_z: f32, camera_near: f32, range_near: f32, range_far: f32, camera_scale: Option<f32>) -> f32 {
    let normalized = if let Some(scale) = camera_scale {
        ((camera_z - camera_near) * scale).clamp(0.0, 1.0)
    } else {
        camera_z.max(0.0)
    };
    range_near + normalized * (range_far - range_near)
}

fn edge(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
}

pub(crate) fn should_cull_triangle(area: f32, culling: i32, winding: i32) -> bool {
    if culling == PolygonMode::CULL_NONE {
        return false;
    }

    let front_facing = if winding == PolygonMode::WINDING_CW { area < 0.0 } else { area > 0.0 };
    match culling {
        PolygonMode::CULL_FRONT => front_facing,
        _ => !front_facing,
    }
}

pub(crate) fn material_base_color(material: Option<(i32, i32, i32)>, default_color: i32, lighting: &M3gSceneLighting) -> i32 {
    let Some((ambient, diffuse, emissive)) = material else {
        return default_color;
    };
    if lighting.ambient_lights == 0 && lighting.lights.is_empty() {
        return diffuse;
    }

    let alpha = ((diffuse as u32) >> 24) & 0xff;
    let r = (color_channel_f32(ambient, 16) * lighting.ambient_r + color_channel_f32(emissive, 16)).clamp(0.0, 1.0);
    let g = (color_channel_f32(ambient, 8) * lighting.ambient_g + color_channel_f32(emissive, 8)).clamp(0.0, 1.0);
    let b = (color_channel_f32(ambient, 0) * lighting.ambient_b + color_channel_f32(emissive, 0)).clamp(0.0, 1.0);
    pack_argb(alpha, (r * 255.0) as u32, (g * 255.0) as u32, (b * 255.0) as u32)
}

pub(crate) fn shade_vertex(
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    vertex_color: i32,
    appearance: &M3gAppearanceState,
    lighting: &M3gSceneLighting,
    mesh_scope: i32,
) -> i32 {
    let Some(material) = appearance.material else {
        return vertex_color;
    };
    let diffuse = if material.vertex_color_tracking {
        vertex_color
    } else {
        material.diffuse
    };
    let alpha = ((diffuse as u32) >> 24) & 0xff;
    let mut r = color_channel_f32(material.emissive, 16) + color_channel_f32(material.ambient, 16) * lighting.ambient_r;
    let mut g = color_channel_f32(material.emissive, 8) + color_channel_f32(material.ambient, 8) * lighting.ambient_g;
    let mut b = color_channel_f32(material.emissive, 0) + color_channel_f32(material.ambient, 0) * lighting.ambient_b;

    let mut used = 0usize;
    if let Some(normal) = normal {
        let mut normal = normal;
        let view = if appearance.local_camera_lighting {
            normalize3([-position[0], -position[1], -position[2]]).unwrap_or([0.0, 0.0, 1.0])
        } else {
            [0.0, 0.0, 1.0]
        };
        if appearance.two_sided_lighting && dot3(normal, view) < 0.0 {
            normal = [-normal[0], -normal[1], -normal[2]];
        }
        for light in lighting.lights.iter() {
            if used >= super::super::common::M3G_MAX_LIGHTS as usize {
                break;
            }
            if (light.scope & mesh_scope) == 0 {
                continue;
            }
            used += 1;
            let (to_light, distance, atten) = match light.mode {
                Light::DIRECTIONAL => {
                    let to_light = [-light.direction[0], -light.direction[1], -light.direction[2]];
                    (normalize3(to_light).unwrap_or(to_light), 0.0, 1.0)
                }
                Light::OMNI | Light::SPOT => {
                    let to_light = [
                        light.position[0] - position[0],
                        light.position[1] - position[1],
                        light.position[2] - position[2],
                    ];
                    let distance = dot3(to_light, to_light).sqrt();
                    let to_light = normalize3(to_light).unwrap_or([0.0, 0.0, 1.0]);
                    let atten = 1.0 / (light.constant + light.linear * distance + light.quadratic * distance * distance).max(0.0001);
                    (to_light, distance, atten)
                }
                _ => continue,
            };
            let mut spot = 1.0;
            if light.mode == Light::SPOT {
                let from_light = [-to_light[0], -to_light[1], -to_light[2]];
                let cos_angle = dot3(from_light, light.direction);
                if cos_angle < light.spot_cos {
                    continue;
                }
                spot = cos_angle.max(0.0).powf(light.spot_exponent);
            }
            let ndotl = dot3(normal, to_light).max(0.0);
            let diffuse_term = ndotl * atten * spot;
            r += color_channel_f32(diffuse, 16) * light.color[0] * diffuse_term;
            g += color_channel_f32(diffuse, 8) * light.color[1] * diffuse_term;
            b += color_channel_f32(diffuse, 0) * light.color[2] * diffuse_term;

            if ndotl > 0.0 && material.shininess > 0.0 {
                let reflect = [
                    2.0 * ndotl * normal[0] - to_light[0],
                    2.0 * ndotl * normal[1] - to_light[1],
                    2.0 * ndotl * normal[2] - to_light[2],
                ];
                let rdotv = dot3(reflect, view).max(0.0);
                let spec = rdotv.powf(material.shininess) * atten * spot;
                r += color_channel_f32(material.specular, 16) * light.color[0] * spec;
                g += color_channel_f32(material.specular, 8) * light.color[1] * spec;
                b += color_channel_f32(material.specular, 0) * light.color[2] * spec;
            }
            let _ = distance;
        }
    }

    pack_argb(
        alpha,
        (r.clamp(0.0, 1.0) * 255.0) as u32,
        (g.clamp(0.0, 1.0) * 255.0) as u32,
        (b.clamp(0.0, 1.0) * 255.0) as u32,
    )
}

pub(crate) fn apply_distance_fog(color: i32, fog: M3gFogState, camera_z: f32) -> i32 {
    let factor = if fog.mode == Fog::LINEAR {
        let range = fog.far - fog.near;
        if range.abs() <= f32::EPSILON {
            1.0
        } else {
            ((fog.far - camera_z) / range).clamp(0.0, 1.0)
        }
    } else {
        (-fog.density.max(0.0) * camera_z.max(0.0)).exp().clamp(0.0, 1.0)
    };
    let src = color as u32;
    let fog_color = fog.color as u32;
    let mix = |shift: u32| -> u32 {
        let a = ((src >> shift) & 0xff) as f32;
        let b = ((fog_color >> shift) & 0xff) as f32;
        (a * factor + b * (1.0 - factor)) as u32
    };
    (((src >> 24) & 0xff) << 24 | mix(16) << 16 | mix(8) << 8 | mix(0)) as i32
}

pub(crate) fn color_channel_f32(color: i32, shift: u32) -> f32 {
    (((color as u32) >> shift) & 0xff) as f32 / 255.0
}

pub(crate) fn compose_m3g_pixel(dst: i32, src: i32, blending: i32) -> i32 {
    match blending {
        CompositingMode::ALPHA => source_over(dst, src),
        CompositingMode::ALPHA_ADD => alpha_add(dst, src),
        CompositingMode::MODULATE => framebuffer_modulate(dst, src, 1),
        CompositingMode::MODULATE_X2 => framebuffer_modulate(dst, src, 2),
        _ => src,
    }
}

pub(crate) fn source_over(dst: i32, src: i32) -> i32 {
    let src = src as u32;
    let dst = dst as u32;
    let src_a = (src >> 24) & 0xff;
    if src_a == 0xff {
        return src as i32;
    }
    if src_a == 0 {
        return dst as i32;
    }

    let dst_a = (dst >> 24) & 0xff;
    let inv_a = 255 - src_a;
    let out_a = src_a + mul_u8(dst_a, inv_a);
    if out_a == 0 {
        return 0;
    }

    let src_r = (src >> 16) & 0xff;
    let src_g = (src >> 8) & 0xff;
    let src_b = src & 0xff;
    let dst_r = (dst >> 16) & 0xff;
    let dst_g = (dst >> 8) & 0xff;
    let dst_b = dst & 0xff;
    let dst_weight = mul_u8(dst_a, inv_a);
    pack_argb(
        out_a,
        ((src_r * src_a + dst_r * dst_weight) / out_a).min(255),
        ((src_g * src_a + dst_g * dst_weight) / out_a).min(255),
        ((src_b * src_a + dst_b * dst_weight) / out_a).min(255),
    )
}

fn alpha_add(dst: i32, src: i32) -> i32 {
    let src = src as u32;
    let dst = dst as u32;
    let src_a = (src >> 24) & 0xff;
    let dst_a = (dst >> 24) & 0xff;
    let src_r = mul_u8((src >> 16) & 0xff, src_a);
    let src_g = mul_u8((src >> 8) & 0xff, src_a);
    let src_b = mul_u8(src & 0xff, src_a);
    pack_argb(
        (src_a + dst_a).min(255),
        (((dst >> 16) & 0xff) + src_r).min(255),
        (((dst >> 8) & 0xff) + src_g).min(255),
        ((dst & 0xff) + src_b).min(255),
    )
}

fn framebuffer_modulate(dst: i32, src: i32, factor: u32) -> i32 {
    let src = src as u32;
    let dst = dst as u32;
    if ((dst >> 24) & 0xff) == 0 {
        return src as i32;
    }
    pack_argb(
        (((src >> 24) & 0xff) * ((dst >> 24) & 0xff) * factor / 255).min(255),
        (((src >> 16) & 0xff) * ((dst >> 16) & 0xff) * factor / 255).min(255),
        (((src >> 8) & 0xff) * ((dst >> 8) & 0xff) * factor / 255).min(255),
        ((src & 0xff) * (dst & 0xff) * factor / 255).min(255),
    )
}

pub(crate) fn floor_i32(value: f32) -> i32 {
    let integer = value as i32;
    if value < integer as f32 { integer - 1 } else { integer }
}

pub(crate) fn ceil_i32(value: f32) -> i32 {
    let integer = value as i32;
    if value > integer as f32 { integer + 1 } else { integer }
}

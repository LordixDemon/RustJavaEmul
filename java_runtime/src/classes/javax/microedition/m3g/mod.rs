mod animation;
mod animation_track;
mod appearance;
mod background;
pub mod benchmark;
mod camera;
mod common;
mod compositing_mode;
mod file_loader;
mod file_loader_mesh;
mod fog;
mod gpu_scene;
mod graphics3d;
mod group;
mod image2d;
mod keyframe;
mod lighting;
mod loader;
mod m3g_reader;
mod material;
mod math;
mod mesh;
mod morphing_mesh;
mod node;
mod object3d;
mod object3d_duplicate;
mod polygon_mode;
mod prelude;
mod raw_arrays;
mod ray_intersection;
mod render;
mod skinned_mesh;
mod sprite3d;
mod texture2d;
mod transform;
mod transformable;
mod triangle_strip_array;
mod types;
mod vertex_array;
mod vertex_buffer;
mod vertex_index;
mod world;

pub use self::common::latest_render_diagnostics;
pub use self::gpu_scene::{
    M3gGpuFog, M3gGpuFrame, M3gGpuTex, M3gGpuTexture, M3gGpuTriangle, M3gGpuVertex, clear_m3g_gpu_frames, latest_m3g_gpu_frame_after,
};
pub use self::types::*;

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        AnimationController,
        AnimationTrack,
        Appearance,
        Background,
        Camera,
        CompositingMode,
        Fog,
        Graphics3D,
        Group,
        Image2D,
        IndexBuffer,
        KeyframeSequence,
        Light,
        Loader,
        Material,
        Mesh,
        MorphingMesh,
        Node,
        Object3D,
        PolygonMode,
        RayIntersection,
        SkinnedMesh,
        Sprite3D,
        Texture2D,
        Transform,
        Transformable,
        TriangleStripArray,
        VertexArray,
        VertexBuffer,
        World,
    ]
}

#[cfg(test)]
mod tests {
    use alloc::{sync::Arc, vec};

    use super::{
        Image2D, PolygonMode, Texture2D,
        common::decode_m3g_section_data,
        math::{identity_matrix, multiply_matrix, rotation_matrix, rotation_matrix_to_axis_angle},
        render::{M3gTexture, decode_m3g_image_pixels, sample_texture, should_cull_triangle},
    };

    #[test]
    fn uncompressed_m3g_section_uses_borrowed_data() {
        let data = b"m3g-section";
        let decoded = decode_m3g_section_data(0, data, data.len()).unwrap();

        assert_eq!(decoded.as_slice(), data);
    }

    #[test]
    fn zlib_m3g_section_inflates_to_declared_length() {
        let compressed_hello = [0x78, 0x9c, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x07, 0x00, 0x06, 0x2c, 0x02, 0x15];
        let decoded = decode_m3g_section_data(1, &compressed_hello, 5).unwrap();

        assert_eq!(decoded.as_slice(), b"hello");
    }

    #[test]
    fn zlib_m3g_section_rejects_length_mismatch() {
        let compressed_hello = [0x78, 0x9c, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x07, 0x00, 0x06, 0x2c, 0x02, 0x15];

        assert!(decode_m3g_section_data(1, &compressed_hello, 6).is_err());
    }

    #[test]
    fn image2d_embedded_rgba_pixels_are_decoded_as_argb() {
        let pixels = decode_m3g_image_pixels(Image2D::RGBA, 1, 1, &[], &[0x11, 0x22, 0x33, 0x44]);

        assert_eq!(pixels, vec![0x4411_2233]);
    }

    #[test]
    fn image2d_palette_indices_decode_palette_entries() {
        let palette = [0x10, 0x20, 0x30, 0x40, 0xaa, 0xbb, 0xcc, 0xdd];
        let pixels = decode_m3g_image_pixels(Image2D::RGBA, 2, 1, &palette, &[1, 0]);

        assert_eq!(pixels, vec![0xddaa_bbccu32 as i32, 0x4010_2030]);
    }

    #[test]
    fn linear_texture_filter_blends_neighboring_texels() {
        let texture = M3gTexture {
            width: 2,
            height: 2,
            pixels: Arc::new(vec![
                0xff00_0000u32 as i32,
                0xffff_0000u32 as i32,
                0xff00_ff00u32 as i32,
                0xff00_00ffu32 as i32,
            ]),
            format: Image2D::RGBA,
            blend_color: 0,
            blending: Texture2D::FUNC_REPLACE,
            wrap_s: Texture2D::WRAP_CLAMP,
            wrap_t: Texture2D::WRAP_CLAMP,
            image_filter: Texture2D::FILTER_LINEAR,
            transform: identity_matrix(),
            transform_identity: true,
        };

        let sampled = sample_texture(&texture, 0.5, 0.5) as u32;

        assert_eq!((sampled >> 24) & 0xff, 0xff);
        assert!(((sampled >> 16) & 0xff).abs_diff(63) <= 1);
        assert!(((sampled >> 8) & 0xff).abs_diff(63) <= 1);
        assert!((sampled & 0xff).abs_diff(63) <= 1);
    }

    #[test]
    fn polygon_mode_culls_by_projected_winding() {
        assert!(!should_cull_triangle(1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CCW));
        assert!(should_cull_triangle(-1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CCW));
        assert!(should_cull_triangle(1.0, PolygonMode::CULL_FRONT, PolygonMode::WINDING_CCW));
        assert!(!should_cull_triangle(-1.0, PolygonMode::CULL_FRONT, PolygonMode::WINDING_CCW));
        assert!(!should_cull_triangle(-1.0, PolygonMode::CULL_NONE, PolygonMode::WINDING_CCW));
        assert!(!should_cull_triangle(-1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CW));
        assert!(should_cull_triangle(1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CW));
    }

    #[test]
    fn composed_rotation_round_trips_through_axis_angle() {
        let composed = multiply_matrix(rotation_matrix(31.0, 0.0, 0.0, 1.0), rotation_matrix(79.0, 1.0, 0.0, 0.0));
        let (angle, ax, ay, az) = rotation_matrix_to_axis_angle(composed);
        let round_trip = rotation_matrix(angle, ax, ay, az);

        for (actual, expected) in round_trip.iter().zip(composed) {
            assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
        }
    }
}

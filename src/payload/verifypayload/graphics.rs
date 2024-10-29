use serde::Serialize;

#[derive(Debug, Serialize)]

pub struct Canvas {
    hash: isize,
    #[serde(rename = "emailHash")]
    email_hash: Option<usize>,
    #[serde(rename = "histogramBins")]
    histogram_bins: Vec<usize>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            hash: 243977283,
            email_hash: None,
            histogram_bins: vec![
                14330, 43, 44, 41, 53, 35, 36, 31, 63, 38, 24, 26, 25, 37, 36, 32, 37, 19, 25, 40,
                31, 27, 24, 27, 42, 23, 29, 27, 43, 15, 25, 36, 19, 20, 22, 22, 29, 23, 38, 33, 21,
                19, 45, 30, 15, 23, 34, 35, 35, 30, 42, 72, 29, 14, 30, 30, 23, 26, 22, 23, 20, 35,
                18, 25, 153, 15, 25, 28, 12, 16, 18, 13, 13, 33, 23, 23, 22, 20, 23, 32, 19, 16,
                10, 30, 19, 43, 15, 18, 22, 12, 20, 29, 18, 23, 17, 18, 51, 25, 37, 40, 75, 24,
                523, 35, 30, 15, 16, 19, 24, 58, 38, 10, 22, 24, 23, 41, 16, 16, 25, 41, 24, 21,
                20, 13, 20, 21, 35, 31, 193, 42, 28, 14, 10, 19, 13, 21, 26, 33, 16, 13, 18, 11,
                21, 38, 15, 31, 58, 32, 22, 17, 18, 21, 19, 87, 13, 19, 20, 14, 45, 22, 21, 19, 28,
                14, 42, 14, 14, 35, 33, 11, 33, 18, 24, 30, 13, 16, 13, 21, 23, 23, 21, 18, 31, 13,
                17, 26, 16, 19, 24, 16, 13, 136, 29, 16, 27, 21, 32, 23, 26, 18, 30, 38, 41, 52,
                62, 90, 33, 40, 30, 53, 31, 24, 31, 34, 48, 30, 26, 37, 31, 24, 23, 30, 24, 33, 40,
                39, 37, 39, 22, 23, 33, 36, 37, 28, 34, 63, 43, 55, 34, 40, 42, 40, 35, 45, 39, 60,
                48, 47, 40, 46, 45, 68, 58, 61, 80, 13315,
            ],
        }
    }
}

#[derive(Debug, Serialize)]

pub struct Gpu {
    vendor: String,
    model: String,
    extensions: Vec<String>,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            vendor: String::from("Google Inc. (Apple)"),
            model: String::from(
                "ANGLE (Apple, ANGLE Metal Renderer: Apple M3 Pro, Unspecified Version)",
            ),
            extensions: vec![
                "ANGLE_instanced_arrays".to_string(),
                "EXT_blend_minmax".to_string(),
                "EXT_clip_control".to_string(),
                "EXT_color_buffer_half_float".to_string(),
                "EXT_depth_clamp".to_string(),
                "EXT_disjoint_timer_query".to_string(),
                "EXT_float_blend".to_string(),
                "EXT_frag_depth".to_string(),
                "EXT_polygon_offset_clamp".to_string(),
                "EXT_shader_texture_lod".to_string(),
                "EXT_texture_compression_bptc".to_string(),
                "EXT_texture_compression_rgtc".to_string(),
                "EXT_texture_filter_anisotropic".to_string(),
                "EXT_texture_mirror_clamp_to_edge".to_string(),
                "EXT_sRGB".to_string(),
                "KHR_parallel_shader_compile".to_string(),
                "OES_element_index_uint".to_string(),
                "OES_fbo_render_mipmap".to_string(),
                "OES_standard_derivatives".to_string(),
                "OES_texture_float".to_string(),
                "OES_texture_float_linear".to_string(),
                "OES_texture_half_float".to_string(),
                "OES_texture_half_float_linear".to_string(),
                "OES_vertex_array_object".to_string(),
                "WEBGL_blend_func_extended".to_string(),
                "WEBGL_color_buffer_float".to_string(),
                "WEBGL_compressed_texture_astc".to_string(),
                "WEBGL_compressed_texture_etc".to_string(),
                "WEBGL_compressed_texture_etc1".to_string(),
                "WEBGL_compressed_texture_pvrtc".to_string(),
                "WEBGL_compressed_texture_s3tc".to_string(),
                "WEBGL_compressed_texture_s3tc_srgb".to_string(),
                "WEBGL_debug_renderer_info".to_string(),
                "WEBGL_debug_shaders".to_string(),
                "WEBGL_depth_texture".to_string(),
                "WEBGL_draw_buffers".to_string(),
                "WEBGL_lose_context".to_string(),
                "WEBGL_multi_draw".to_string(),
                "WEBGL_polygon_mode".to_string(),
            ],
        }
    }
}

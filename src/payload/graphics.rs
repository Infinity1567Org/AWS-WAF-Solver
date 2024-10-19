use serde::Serialize;

#[derive(Debug, Serialize)]

pub struct Canvas {
    hash: usize,
    #[serde(rename = "emailHash")]
    email_hash: Option<usize>,
    #[serde(rename = "histogramBins")]
    histogram_bins: Vec<usize>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            hash: 1108890895,
            email_hash: None,
            histogram_bins: vec![
                14574, 35, 54, 37, 42, 67, 29, 25, 39, 31, 37, 50, 43, 36, 33, 67, 55, 21, 67, 24,
                24, 25, 23, 39, 46, 27, 44, 12, 31, 18, 48, 46, 35, 57, 26, 12, 39, 27, 23, 29, 33,
                33, 16, 10, 34, 24, 11, 34, 33, 40, 34, 37, 66, 10, 21, 23, 21, 19, 6, 17, 48, 45,
                11, 14, 19, 18, 32, 7, 14, 15, 8, 17, 17, 17, 40, 15, 11, 12, 43, 66, 30, 18, 19,
                14, 31, 12, 15, 16, 20, 54, 31, 27, 17, 18, 13, 17, 36, 20, 24, 115, 80, 29, 546,
                24, 9, 39, 29, 16, 14, 15, 11, 36, 23, 16, 17, 16, 15, 31, 20, 23, 70, 40, 17, 12,
                31, 29, 29, 20, 23, 59, 32, 54, 22, 13, 16, 38, 32, 34, 72, 15, 24, 17, 12, 16, 14,
                12, 25, 21, 10, 12, 60, 19, 7, 92, 8, 51, 29, 21, 8, 30, 10, 13, 19, 22, 51, 36, 6,
                14, 9, 15, 14, 29, 17, 40, 11, 24, 60, 80, 7, 10, 9, 47, 9, 25, 9, 7, 18, 10, 7,
                54, 15, 19, 7, 11, 61, 22, 14, 9, 66, 7, 18, 57, 38, 49, 66, 72, 84, 6, 26, 15, 8,
                14, 9, 27, 37, 31, 16, 29, 18, 35, 27, 20, 68, 84, 33, 67, 19, 42, 52, 23, 21, 40,
                13, 38, 19, 35, 35, 16, 20, 73, 124, 43, 45, 34, 66, 35, 33, 78, 36, 32, 80, 55,
                43, 63, 43, 13187,
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
        Self { vendor: String::from("Google Inc. (NVIDIA)"), model: String::from("ANGLE (NVIDIA, NVIDIA GeForce GTX 1660 (0x00002184) Direct3D11 vs_5_0 ps_5_0, D3D11)"), extensions: vec![
            String::from("ANGLE_instanced_arrays"),
            String::from("EXT_blend_minmax"),
            String::from("EXT_color_buffer_half_float"),
            String::from("EXT_float_blend"),
            String::from("EXT_frag_depth"),
             String::from("EXT_shader_texture_lod"),
             String::from("EXT_sRGB"),
             String::from("EXT_texture_compression_bptc"),
             String::from("EXT_texture_compression_rgtc"),
             String::from("EXT_texture_filter_anisotropic"),
             String::from("OES_element_index_uint"),
             String::from("OES_fbo_render_mipmap"),
             String::from("OES_standard_derivatives"),
             String::from("OES_texture_float"),
             String::from("OES_texture_float_linear"),
             String::from("OES_texture_half_float"),
             String::from("OES_texture_half_float_linear"),
             String::from("OES_vertex_array_object"),
             String::from("WEBGL_color_buffer_float"),
             String::from("WEBGL_compressed_texture_s3tc"),
             String::from( "WEBGL_compressed_texture_s3tc_srgb"),
             String::from("WEBGL_debug_renderer_info"),
             String::from("WEBGL_debug_shaders"),
             String::from("WEBGL_depth_texture"),
             String::from("WEBGL_draw_buffers"),
             String::from("WEBGL_lose_context"),
             String::from("WEBGL_provoking_vertex")
          ] }
    }
}

use serde::{Deserialize,Serialize};



#[derive(Debug, Serialize, Deserialize)]

pub struct Canvas {
    hash : usize,
    email_hash : Option<usize>,
    histogram_bins : Vec<usize>
}


#[derive(Debug, Serialize, Deserialize)]

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

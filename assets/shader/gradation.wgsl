// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct InstanceInput {
    @location(4) model_texcoord: vec4<f32>,
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};



struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) ndc_coords: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {

    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.tex_coords = vec2(
    instance.model_texcoord[0] * model.tex_coords[0] + instance.model_texcoord[1] * (1.0-model.tex_coords[0])  ,
    instance.model_texcoord[2] * model.tex_coords[1] + instance.model_texcoord[3] * (1.0-model.tex_coords[1])
    );// model.tex_coords + instance.model_texcoord;
    out.clip_position =  camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
//    let model_position = model_matrix * vec4<f32>(model.position, 1.0);
//    out.clip_position = camera.view_proj * model_position;
    out.ndc_coords = out.clip_position.xy / out.clip_position.w;
    return out;
}


@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;


fn hue_to_rgb(hue: f32) -> vec3<f32> {
    let r = abs(hue * 6.0 - 3.0) - 1.0;
    let g = 2.0 - abs(hue * 6.0 - 2.0);
    let b = 2.0 - abs(hue * 6.0 - 4.0);
    return clamp(vec3(r, g, b), vec3(0.0), vec3(1.0));
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//    return vec4(1.0);

    let centered_coords = in.ndc_coords * vec2(0.5, -0.5) + vec2(0.5);
    let angle = atan2(centered_coords.y - 0.5, centered_coords.x - 0.5) / (2.0 * 3.14159265359) + 0.5;
    let color = hue_to_rgb(angle);
    return vec4(color, 1.0);
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) l_pos: vec3<f32>,
    @location(3) normal: vec3<f32>,
};

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mvp * vec4<f32>(model.position, 1.0);
    out.l_pos = model.position;
    out.color = model.color;
    out.uv = model.uv;
    return out;
}

fn fresnel_schlick(cosTheta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}  

fn distribution_ggx(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    var a = roughness * roughness;
    var a2 = a * a;
    var NdotH = max(dot(N, H), 0.0);
    var NdotH2 = NdotH * NdotH;

    var num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    var r = (roughness + 1.0);
    var k = (r * r) / 8.0;

    var num = NdotV;
    var denom = NdotV * (1.0 - k) + k;

    return num / denom;
}
fn geometry_smith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32 {
    var NdotV = max(dot(N, V), 0.0);
    var NdotL = max(dot(N, L), 0.0);
    var ggx2 = geometry_schlick_ggx(NdotV, roughness);
    var ggx1 = geometry_schlick_ggx(NdotL, roughness);

    return ggx1 * ggx2;
}

var<private> PI : f32 = 3.141592;
var<private> roughness : f32 = 0.5;
var<private> metallic : f32 = 0.5;
var<private> light_dir : vec3<f32> = vec3<f32>(0.3, 0.5, 0.3);
var<private> light_color : vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
var<private> view_pos : vec3<f32> = vec3<f32>(0.0, 0.0, 0.0); //TODO: Set this properly

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var albedo = textureSample(t_diffuse, s_diffuse, in.uv) * in.color;
    var N = normalize(in.normal);
    var V = normalize(view_pos - in.l_pos); //This might require world pos instead of local
    var F0 = vec3<f32>(0.04);
    F0 = mix(F0, albedo.rgb, metallic);
    var Lo = vec3<f32>(0.0);

    //Get directional radiance
    var L = normalize(light_dir);
    var H = normalize(V + L);
    var radiance = light_color;

    //Cook BRDF
    var NDF = distribution_ggx(N, H, roughness);
    var G = geometry_smith(N, V, L, roughness);
    var F = fresnel_schlick(max(dot(H, V), 0.0), F0);

    var kS = F;
    var kD = vec3(1.0) - kS;
    kD *= 1.0 - metallic;

    var numerator = NDF * G * F;
    var denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
    var specular = numerator / denominator;

    var NdotL = max(dot(N, L), 0.0);
    Lo += (kD * albedo.rgb / PI + specular) * radiance * NdotL;

    var ambient = vec3(0.03) * albedo.rgb;
    var color = ambient + Lo;
    color = color / (color + vec3(1.0));
    color = pow(color, vec3(1.0 / 2.2));

    return vec4(color, 1.0);
}

#version 410

#define ATLAS_SIZE 12

struct material {
    vec4 ka;
    vec4 kd;
    vec4 ks;

    ivec2 ka_map;
    ivec2 kd_map;
    ivec2 ks_map;

    float ns;
    float ni;
};

struct fetch_result {
    vec4 value;
    bool found;
};

layout(std140) uniform Materials {
    material materials[512];
};

uniform sampler2D normal;
uniform sampler2D uv;
uniform usampler2D pixel_drawn_by;
uniform sampler2D depth;

uniform sampler2DArray atlas[ATLAS_SIZE];
uniform int atlas_base;

uniform vec4 light_position;
uniform float light_intensity;

uniform mat4 mat_proj;
uniform mat4 mat_view;

in vec2 TexPos;
out vec4 color;

fetch_result fetch_material(vec4 d, ivec2 map) {
    if (map.x == -1) {
        return fetch_result(d, true);
    } else if (map.x >= atlas_base && map.x < atlas_base + ATLAS_SIZE) {
        vec2 uv_value = texture(uv, TexPos).xy;
        vec4 text = texture(atlas[map.x], vec3(uv_value, float(map.y)));
        return fetch_result(vec4(text.xyz, 1.), true);
    } else {
        fetch_result(vec4(0., 0., 0., 0.), false);
    }
}

vec4 calc_eye_from_window(vec3 window_space) {
    vec4 viewport = vec4(0., 0., 1280., 800.);
    vec2 depthrange = vec2(0., 1.);
    vec3 ndc_pos;
    ndc_pos.xy = ((2.0 * window_space.xy) - (2.0 * viewport.xy)) / (viewport.zw) - 1;
    ndc_pos.z = (2.0 * window_space.z - depthrange.x - depthrange.y) /
               (depthrange.y - depthrange.x);

    vec4 clip_pose;
    clip_pose.w = mat_proj[3][2] / (ndc_pos.z - (mat_proj[2][2] / mat_proj[2][3]));
    clip_pose.xyz = ndc_pos * clip_pose.w;

    return inverse(mat_proj) * clip_pose;
}

void main() {
    uvec2 object = texture(pixel_drawn_by, TexPos).xy;
    fetch_result kd = fetch_material(materials[object.y].kd,
                                     materials[object.y].kd_map);
    fetch_result ks = fetch_material(materials[object.y].ks,
                                     materials[object.y].ks_map);

    vec4 eye = calc_eye_from_window(vec3(gl_FragCoord.x,
                                         gl_FragCoord.y,
                                         texture(depth, TexPos).x));
    vec4 position = inverse(mat_view) * eye;
    vec4 delta = light_position - position;
    
    float dist = length(delta);

    vec4 light_normal = normalize(delta);
    vec4 surface_normal = vec4(texture(normal, TexPos).xyz, 0.);

    if (kd.found) {
        color = (kd.value * light_intensity * max(0, dot(light_normal, surface_normal))) / (dist * dist);
    }
}
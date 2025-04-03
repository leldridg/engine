#version 330 core

layout (location = 0) in vec3 Position;

uniform vec2 u_resolution;
uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

// executed in parallel for each vertex
void main() {
    vec4 uv = u_model_matrix * vec4(Position, 1.0); // 0.0 is z, 1.0 is w
    uv = u_projection_matrix * u_view_matrix * uv;

    // make ((-1.0, -1.0), (1.0, -1.0)), (1.0, 1.0, (-1.0, 1.0)) a square always in the center of the viewport
    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    }

    // gl_Position = vec4(uv, 1.0);
    gl_Position = uv;
}
#version 150 core

in vec3 a_Pos;
in vec2 a_Uv;
in vec4 a_Color;

out vec2 v_Uv;
out vec4 t_Color;

uniform b_Projection {
    mat4 u_Model;
    mat4 u_Proj;
    mat4 u_Scale;
};

void main() {
    v_Uv = a_Uv;
    gl_Position = u_Proj * u_Model * u_Scale * vec4(a_Pos, 1.0);
    t_Color = a_Color;
}
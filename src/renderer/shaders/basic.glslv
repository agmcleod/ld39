#version 150 core

in vec2 a_Pos;
in vec2 a_Uv;
in vec4 a_Color;

out vec2 v_Uv;
out vec4 t_Color;

uniform b_Projection {
    mat4 u_Model;
    mat4 u_Proj;
};

void main() {
    v_Uv = a_Uv;
    gl_Position = u_Proj * u_Model * vec4(a_Pos, 0.0, 1.0);
    t_Color = a_Color;
}
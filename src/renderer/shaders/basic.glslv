#version 150 core

in vec2 a_Pos;
in vec2 a_Uv;

out vec2 v_Uv;

uniform b_Projection {
    mat4 u_Model;
    mat4 u_Proj;
};

void main() {
    v_Uv = a_Uv;
    gl_Position = u_Proj * u_Model * vec4(a_Pos, 0.0, 1.0);
}
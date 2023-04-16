#version 300 es

in vec3 a_Position;
in vec3 a_Normal;

uniform mat4 u_Model;
uniform mat4 u_View;
uniform mat4 u_Projection;

out vec3 v_Position;
out vec3 v_Normal;

void main() {
    mat4 mvp = u_Projection * u_View * u_Model;
    v_Position = vec3(u_Model * vec4(a_Position, 1.0));
    v_Normal = mat3(u_Model) * a_Normal;

    gl_Position = mvp * vec4(a_Position, 1.0);
}
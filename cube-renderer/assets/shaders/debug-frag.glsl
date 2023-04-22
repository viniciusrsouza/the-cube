#version 300 es

precision highp float;
out vec4 outColor;

in vec3 v_Position;
in vec3 v_Normal;

#define MAX_LIGHTS 8

struct Material {
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
};

uniform Material u_Material;
uniform int u_LightCount;
uniform Light u_Lights[MAX_LIGHTS];
uniform vec3 u_ViewPos;

void main() {
    vec3 normal = normalize(v_Normal);
    vec3 viewDir = normalize(u_ViewPos - v_Position);

    vec4 result = vec4(0.0);
    for(int i = 0; i < u_LightCount; i++) {
        vec3 lightDir = normalize(u_Lights[i].position - v_Position);
        vec3 reflectDir = reflect(-lightDir, normal);

        float diff = max(dot(normal, lightDir), 0.0);
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), u_Material.shininess);

        vec4 ambient = u_Lights[i].ambient * u_Material.ambient;
        vec4 diffuse = u_Lights[i].diffuse * u_Material.diffuse * diff;
        vec4 specular = u_Lights[i].specular * u_Material.specular * spec;

        result += (ambient + diffuse + specular);
    }

    outColor = result;
}
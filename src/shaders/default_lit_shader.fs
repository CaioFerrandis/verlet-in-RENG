#version 330 core

out vec4 Result;

in vec3 FragPos;
in vec3 transformedNormal;
in vec2 frag_texCoord;
in vec4 out_color;

uniform vec3 lightColor[32];
uniform vec3 lightPos[32];
uniform sampler2D textureSampler;
uniform vec3 viewPos;

void main()
{
    vec3 ambient = vec3(0.);
    vec3 diffuse = vec3(0.);
    vec3 specular = vec3(0.);

    for (int i = 0; i < 32; i++){
        // Ambient
        float ambientStrength = 0.1;
        ambient += lightColor[i] * ambientStrength;
        
        // Diffuse
        vec3 norm = normalize(transformedNormal);
        vec3 lightDir = normalize(lightPos[i] - FragPos);
        float diff = max(dot(norm, lightDir), 0.0);
        diffuse += lightColor[i] * diff;
        
        // Specular
        float specularStrength = 0.3;
        vec3 viewDir = normalize(viewPos - FragPos);
        vec3 reflectDir = reflect(-lightDir, norm);  
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);  // Specular exponent
        specular += lightColor[i] * specularStrength * spec;  
    }

    // Combine all lighting effects
    vec4 texColor = texture(textureSampler, frag_texCoord);
    Result = vec4(ambient + diffuse + specular, 1.0) * texColor * out_color;
}

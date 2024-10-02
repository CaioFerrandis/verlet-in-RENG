#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec4 out_color;

void main()
{
	out_color = color;
	gl_Position = projection * view * model * vec4(aPos, 1.0);
}
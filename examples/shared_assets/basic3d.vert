#version 330 core
in vec3 pos;
in vec4 color;
in vec2 uv;

uniform mat4 mvp;

out vec4 frag_color;
out vec2 frag_uv;

void main(){
    frag_uv=uv;
    frag_color=color;
    gl_Position=mvp*vec4(pos,1.);
}
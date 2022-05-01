#version 330 core
in vec4 frag_color;
in vec2 frag_uv;

uniform sampler2D main_tex;

out vec4 out_color;

void main(){
    out_color=vec4(frag_color)+texture(main_tex,frag_uv);
}
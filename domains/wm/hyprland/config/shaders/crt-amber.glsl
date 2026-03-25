#version 300 es

precision mediump float;
in vec2 v_texcoord;
layout(location = 0) out vec4 fragColor;
uniform sampler2D tex;

void main() {
    vec2 uv = v_texcoord;
    
    vec2 texSize = vec2(1920.0, 1080.0);
    float pixelSize = 1.5;
    vec2 pixelUV = floor(uv * texSize / pixelSize) * pixelSize / texSize;
    
    vec4 color = texture(tex, pixelUV);
    float luma = dot(color.rgb, vec3(0.299, 0.587, 0.114));
    
    float scanline = sin(uv.y * 800.0 * 3.14159) * 0.5 + 0.5;
    luma *= 1.0 - scanline * 0.1;
    
    vec3 amber = vec3(1.0, 0.4, 0.0);
    vec3 dark = vec3(0.05, 0.02, 0.0);
    vec3 final_color = mix(dark, amber, luma);
    
    fragColor = vec4(final_color, 1.0);
}

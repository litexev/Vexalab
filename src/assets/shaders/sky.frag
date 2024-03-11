#version 100
precision lowp float;
uniform vec2 canvasSize;
uniform sampler2D Texture;
uniform vec4 startColor; // Specify the starting color
uniform vec4 endColor;   // Specify the ending color

void main() {
    vec2 coord = gl_FragCoord.xy / canvasSize; // Normalize to [0,1] using canvas size
    if (abs(coord.x) < 1. && abs(coord.y) < 1.) {
        coord = (coord + 1.) / 2.;
        vec4 gradientColor = mix(startColor, endColor, 1.0 - coord.y);
        gl_FragColor = gradientColor;
    } else {
        gl_FragColor = vec4(1, 1, 1, 1); // White
    }
}
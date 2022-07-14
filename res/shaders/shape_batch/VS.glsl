#version 420 core

layout (location = 0) in float aRoundness;
layout (location = 1) in vec4 aColor;

out vec4 fColor;
out vec2 fPos;
out vec2 fSize;
out float fThickness;
flat out float fRoundness;

uniform vec2 uPos[200];
uniform vec2 uSize[200];
uniform float uThickness[200];

uniform mat4 uProjection;

void main() {
    int id = gl_VertexID / 4;
    int vId = gl_VertexID % 4;

    fPos = uPos[id];
    fSize = uSize[id];
    fThickness = uThickness[id];
    fColor = aColor;
    fRoundness = aRoundness;

    if (vId == 0) {
        // Bottom right
        gl_Position = vec4(ceil(fPos.x + fSize.x), ceil(fPos.y + fSize.y), 0.0, 1.0);
    } else if (vId == 1) {
        // Top right
        gl_Position = vec4(ceil(fPos.x + fSize.x), floor(fPos.y), 0.0, 1.0);
    } else if (vId == 2) {
        // Top left
        gl_Position = vec4(floor(fPos.x), floor(fPos.y), 0.0, 1.0);
    } else {
        // Bottom left
        gl_Position = vec4(floor(fPos.x), ceil(fPos.y + fSize.y), 0.0, 1.0);
    }
    gl_Position = uProjection * gl_Position;
}
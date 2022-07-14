#version 420 core

in vec4 fColor;
in vec2 fPos;
in vec2 fSize;
in float fThickness;
flat in float fRoundness;

out vec4 oColor;

uniform vec2 uScreenSize;

void main() {
    // Compute pixel position
    vec2 pos = vec2(gl_FragCoord.x, uScreenSize.y - gl_FragCoord.y);

    // Compute alpha level
    float alpha = -1.0;
    if (floor(pos.x) < ceil(fPos.x)) { // On the x-axis
        alpha = 1.0 - (fPos.x - int(fPos.x));
    } else if (ceil(pos.x) > floor(fPos.x + fSize.x)) {
        alpha = fPos.x - int(fPos.x) + fSize.x - int(fSize.x);
    }

    if (floor(pos.y) < ceil(fPos.y)) { // on the y-axis
        alpha = max(alpha, 1.0 - (fPos.y - int(fPos.y) + fSize.y - int(fSize.y)));
    } else if (ceil(pos.y) > floor(fPos.y + fSize.y)) {
        alpha = max(alpha, (fPos.y - int(fPos.y) + fSize.y - int(fSize.y)));
    }

    if (alpha == -1.0) alpha = 1.0;

    // Detect Shape
    if (fRoundness == -1.0 || fRoundness >= min(fSize.x, fSize.y) / 2.0) { // Circle
        float radius = fSize.x / 2.0;
        if (fThickness > 0.0 && fThickness < radius) {
            vec2 center = fPos + vec2(radius, radius);
            float d = distance(pos, center);
            float delta = fwidth(d);
            alpha = min(1.0 - smoothstep(radius - delta, radius, d), smoothstep(radius - fThickness - delta, radius - fThickness, d));
        } else {
            vec2 center = fPos + vec2(radius, radius);
            float d = distance(pos, center);
            alpha = 1.0 - smoothstep(radius - fwidth(d), radius, d);
        }
    } else if (fRoundness == 0.0) { // Rectangle
        // Detect Draw Mode
        if (fThickness > 0.0 && fThickness < min(fSize.x, fSize.y) / 2) {
            // Fill corners
            if (pos.x < ceil(fPos.x + fThickness)) {
                if (pos.y < ceil(fPos.y + fThickness)) { // Top left
                    alpha = -1.0;
                } else if (pos.y > floor(fPos.y + fSize.y - fThickness)) { // Bottom left
                    alpha = -1.0;
                }
            } else if (pos.x > floor(fPos.x + fSize.x - fThickness)) { // Top right
                if (pos.y < ceil(fPos.y + fThickness)) {
                    alpha = -1.0;
                } else if (pos.y > floor(fPos.y + fSize.y - fThickness)) { // Bottom right
                    alpha = -1.0;
                }
            }
            // Fill sides
            if (alpha != -1.0) {
                if (pos.x < ceil(fPos.x + fThickness)) { // Left
                    if (ceil(pos.x) > floor(fPos.x + fThickness)) {
                        alpha = fPos.x - int(fPos.x) + fSize.x - int(fSize.x) + fThickness - int(fThickness);
                    }
                } else if (pos.x > floor(fPos.x + fSize.x - fThickness)) { // Right
                    if (floor(pos.x) < ceil(fPos.x + fSize.x - fThickness)) {
                        alpha = fPos.x - int(fPos.x) + fSize.x - int(fSize.x) - fThickness - int(fThickness);
                    }
                } else if (pos.y < ceil(fPos.y + fThickness)) { // Top
                    if (ceil(pos.y) > floor(fPos.y + fThickness)) {
                        alpha = fPos.y - int(fPos.y) + fSize.y - int(fSize.y) - fThickness - int(fThickness);
                    }
                } else if (pos.y > floor(fPos.y + fSize.y - fThickness)) { // Bottom
                    if (floor(pos.y) < ceil(fPos.y + fSize.y - fThickness)) {
                        alpha = fPos.y - int(fPos.y) + fSize.y - int(fSize.y) - fThickness - int(fThickness);
                    }
                } else {
                    alpha = 0.0;
                }
            }
        }
    } else { // Round Rectangle
        if (fThickness > 0.0 && fThickness < min(fSize.x, fSize.y) / 2) { // TODO: Draw
            // Fill corners
            // FIXME: fix ceil and floor
            bool immut = false;
            if (pos.x < ceil(fPos.x + fRoundness)) {
                if (pos.y < ceil(fPos.y + fRoundness)) { // Top left
                    vec2 center = fPos + vec2(fRoundness, fRoundness);
                    float d = distance(pos, center);
                    float delta = fwidth(d);
                    alpha = min(1.0 - smoothstep(fRoundness - delta, fRoundness, d), smoothstep(fRoundness - fThickness - delta, fRoundness - fThickness, d));
                    immut = true;
                } else if (pos.y > floor(fPos.y + fSize.y - fRoundness)) { // Bottom left
                    vec2 center = fPos + vec2(fRoundness, fSize.y - fRoundness);
                    float d = distance(pos, center);
                    float delta = fwidth(d);
                    alpha = min(1.0 - smoothstep(fRoundness - delta, fRoundness, d), smoothstep(fRoundness - fThickness - delta, fRoundness - fThickness, d));
                    immut = true;
                }
            } else if (pos.x > floor(fPos.x + fSize.x - fRoundness)) { // Top right
                if (pos.y < ceil(fPos.y + fRoundness)) {
                    vec2 center = fPos + vec2(fSize.x - fRoundness, fRoundness);
                    float d = distance(pos, center);
                    float delta = fwidth(d);
                    alpha = min(1.0 - smoothstep(fRoundness - delta, fRoundness, d), smoothstep(fRoundness - fThickness - delta, fRoundness - fThickness, d));
                    immut = true;
                } else if (pos.y > floor(fPos.y + fSize.y - fRoundness)) { // Bottom right
                    vec2 center = fPos + vec2(fSize.x - fRoundness, fSize.y - fRoundness);
                    float d = distance(pos, center);
                    float delta = fwidth(d);
                    alpha = min(1.0 - smoothstep(fRoundness - delta, fRoundness, d), smoothstep(fRoundness - fThickness - delta, fRoundness - fThickness, d));
                    immut = true;
                }
            }
            // Fill sides
            if (alpha != -1.0 && !immut) {
                if (pos.x < ceil(fPos.x + fThickness)) { // Left
                    if (ceil(pos.x) > floor(fPos.x + fThickness)) {
                        alpha = fPos.x - int(fPos.x) + fSize.x - int(fSize.x) + fThickness - int(fThickness);
                    }
                } else if (pos.x > floor(fPos.x + fSize.x - fThickness)) { // Right
                    if (floor(pos.x) < ceil(fPos.x + fSize.x - fThickness)) {
                        alpha = fPos.x - int(fPos.x) + fSize.x - int(fSize.x) - fThickness - int(fThickness);
                    }
                } else if (pos.y < ceil(fPos.y + fThickness)) { // Top
                    if (ceil(pos.y) > floor(fPos.y + fThickness)) {
                        alpha = fPos.y - int(fPos.y) + fSize.y - int(fSize.y) - fThickness - int(fThickness);
                    }
                } else if (pos.y > floor(fPos.y + fSize.y - fThickness)) { // Bottom
                    if (floor(pos.y) < ceil(fPos.y + fSize.y - fThickness)) {
                        alpha = fPos.y - int(fPos.y) + fSize.y - int(fSize.y) - fThickness - int(fThickness);
                    }
                } else {
                    alpha = 0.0;
                }
            }
        } else {
            if (pos.x < fPos.x + fRoundness) { // Left
                if (pos.y < fPos.y + fRoundness) { // Top
                    vec2 center = fPos + vec2(fRoundness, fRoundness);
                    float d = distance(pos, center);
                    alpha = 1.0 - smoothstep(fRoundness - fwidth(d), fRoundness, d);
                } else if (pos.y > fPos.y + fSize.y - fRoundness) { // Bottom
                    vec2 center = fPos + vec2(fRoundness, fSize.y - fRoundness);
                    float d = distance(pos, center);
                    alpha = 1.0 - smoothstep(fRoundness - fwidth(d), fRoundness, d);
                }
            } else if (pos.x > fPos.x + fSize.x - fRoundness) { // Right
                if (pos.y < fPos.y + fRoundness) { // Top
                    vec2 center = fPos + vec2(fSize.x - fRoundness, fRoundness);
                    float d = distance(pos, center);
                    alpha = 1.0 - smoothstep(fRoundness - fwidth(d), fRoundness, d);
                } else if (pos.y > fPos.y + fSize.y - fRoundness) { // Bottom
                    vec2 center = fPos + vec2(fSize.x - fRoundness, fSize.y - fRoundness);
                    float d = distance(pos, center);
                    alpha = 1.0 - smoothstep(fRoundness - fwidth(d), fRoundness, d);
                }
            }
        }
    }

    if (alpha == -1.0) alpha = 1.0;

    // Apply color
    oColor = fColor * vec4(1.0, 1.0, 1.0, alpha);
}
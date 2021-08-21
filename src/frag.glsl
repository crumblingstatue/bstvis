uniform vec2 res;
uniform vec2 view_cen;

void main(){
    vec2 pos = gl_FragCoord.xy / res.xy;
    float r = abs(sin(pos.x + view_cen.x * 0.001));
    float g = abs(sin(pos.y + view_cen.y * 0.001));
    float b = abs(sin(1.0 - pos.x + view_cen.x * 0.001));
    gl_FragColor = vec4(r, g, b, 1.0);
}

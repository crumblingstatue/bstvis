vec4 firstColor = vec4(1.0,0.0,0.0,1.0); //red
vec4 middleColor = vec4(0.0,1.0,0.0,1.0); // green
vec4 endColor = vec4(0.0,0.0,1.0,1.0); // blue
vec2 res = vec2(800., 600.);

void main()
{
    vec2 xy = gl_FragCoord.xy / res.xy;

    float h = 0.5; // adjust position of middleColor
    vec4 col = mix(mix(firstColor, middleColor, xy.x/h), mix(middleColor, endColor, (xy.x - h)/(1.0 - h)), step(h, xy.x));

    gl_FragColor = vec4(col.x, col.y, col.z, 1);
}
#version 140

#define NUM_LAYERS 5

uniform float lum;
uniform int layers;
uniform int seed;

uniform vec2 res;

mat2 rot(float a) {
    float s = sin(a), c = cos(a);
    return mat2(c, -s, s, c);
}

float star(vec2 uv, float flare) {
    float d = length(uv);
    float star = pow(0.035/d, 1.4)*0.3;

    float rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));
    star += rays*flare;
    uv *= rot(3.14159/4.0);
    rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));
    star += rays*flare*0.3;

    star *= smoothstep(0.2, 3.0, 0.15/d);
    return star;
}

// https://stackoverflow.com/questions/4200224/random-noise-functions-for-glsl
float rand(vec2 co){
    return fract(sin(dot(co, vec2(12.9898 + seed, 78.233 - seed))) * 43758.5453);
}

vec4 orange = vec4(1.0,0.3,0.0,1.0);
vec4 white = vec4(1.0,1.0,1.0,1.0); 
vec4 blue = vec4(0.0,0.0,1.0,1.0); 

vec3 randcolor(vec2 id) {
    float x = fract(rand(id)*23.872);

    float h = 0.45; // adjust position of white
    vec4 col = mix(mix(orange, white, x*0.9/h), mix(white, blue, (x - (1.0-h))/(h)), smoothstep(0.5, 0.7, x));
        
    float p = smoothstep(0.2, 0.8, x);
    col += 0.2 * (orange * (1.0 - p) + blue * 0.2 * (p));
    return col.rgb;
}

vec3 field(vec2 uv) {
    vec3 col = vec3(0.0);
    vec2 gv = fract(uv) - 0.5;
    vec2 id = floor(uv);

    for (int y = -1; y <= 1; y++) {
        for (int x = -1; x <= 1; x++) {
            vec2 offset = vec2(x, y);  
            float num = rand(id + offset);
            vec2 nums = vec2(num, fract(num*135.129));      

            float size = fract(num*345.32) * lum;

            float star = star(gv - offset - vec2(num, fract(num*34.)) + 0.5, smoothstep(.9, 1., size)*0.6);
                
            col += star*size*randcolor(id + offset);
        }
    }
        return col;
}

void main() {
    vec2 fragCoord = gl_FragCoord.xy;

    vec2 uv = (fragCoord - 0.5*res)/res.y;
    uv *= 3.0;

    vec3 col = vec3(0);
    
    for(float i=0.; i<1.; i+=1./layers) {
        float depth = fract(i);
            
        float scale = mix(20., .5, depth);
        float fade = depth*smoothstep(1., .9, depth);
        col += field(uv*scale+i*453.2)*fade;
    }
        
    col = pow(col, vec3(.4545));	// gamma correction

    gl_FragColor = vec4(col, 1.0);
}
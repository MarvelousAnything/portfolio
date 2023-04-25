precision mediump float;

uniform float u_time;
uniform vec2 u_mouse;
uniform vec2 u_resolution;

void main() {
  // vec2 uv = (2.0 * gl_FragCoord.xy - u_resolution.xy) / u_resolution.y * 0.5;
  // uv.y = -uv.y;
  // if (distance(uv, (2.0 * u_mouse - u_resolution.xy) / u_resolution.y) > 0.5) {
  //   gl_FragColor = vec4(1.0);
  // } else {
  //   gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
  // }

  vec2 uv = gl_FragCoord.xy / u_resolution.xy;
  vec3 col = 0.5 + 0.5 * cos(u_time * 0.001 + uv.xyx + vec3(0.0, 2.0, 4.0));
  gl_FragColor = vec4(col, 1.0);
}

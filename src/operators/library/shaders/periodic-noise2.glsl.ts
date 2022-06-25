import glsl from '../glsl';

export default glsl`
float periodicNoise2(
    vec2 uv,
    int scale_x,
    int scale_y,
    float offset_z,
    int num_octaves,
    float roughness) {
  float coeff = 1.0;
  float accum = 0.0;
  float total = 0.0;
  float sx = float(scale_x);
  float sy = float(scale_y);
  for (int i = 1; i <= 16; i += 1) {
    if (i <= num_octaves) {
      accum += pnoise(vec3(uv.x * sx, uv.y * sy, offset_z), vec3(sx, sy, 1000)) * coeff;
      total += coeff;
      coeff *= roughness;
    }
    sx *= 2.0;
    sy *= 2.0;
  }
  return (accum / total) + 0.5;
}
`;

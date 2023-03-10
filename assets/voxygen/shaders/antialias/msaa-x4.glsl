vec4 aa_apply(
    texture2D tex, sampler smplr,
    texture2D depth_tex, sampler depth_smplr,
    vec2 fragCoord,
    vec2 resolution
) {
    ivec2 texel_coord = ivec2(fragCoord.x, fragCoord.y);

        vec4 sample1 = texelFetch(sampler2DMS(tex, smplr), texel_coord, 0);
        vec4 sample2 = texelFetch(sampler2DMS(tex, smplr), texel_coord, 1);
        vec4 sample3 = texelFetch(sampler2DMS(tex, smplr), texel_coord, 2);
        vec4 sample4 = texelFetch(sampler2DMS(tex, smplr), texel_coord, 3);

    // Average Samples
    vec4 msaa_color = (sample1 + sample2 + sample3 + sample4) / 4.0;

    return msaa_color;
}

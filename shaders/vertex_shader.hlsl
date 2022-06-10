struct VOut
{
	float4 position: SV_POSITION;
	float2 uv: TEXCOORD;
	float4 color: COLOR;
};

VOut VShader(float4 position : POSITION, float2 uv : TEXCOORD, float4 color : COLOR)
{
    VOut output;

    output.position = position;
    output.uv = uv;
    output.color = color;

    return output;
}

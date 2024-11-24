
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32>
{
    var out = vec4<f32>();
}

/*
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals

#define TAU 6.2831853071

@group(2) @binding(100) var<uniform> g_mouse: vec2<f32>;

struct Uniforms {
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

const day = vec3f(0.2,0.3,0.5);
const sun = vec3f(2.0,2.0,2.0);

fn background(night: vec3f, pos: vec2<f32>, mouse: vec2<f32>) -> vec3<f32> {
	let m: f32 = length(pos);
	let ml: f32 = length(mouse);
	var bgcol: vec3<f32> = mix(night, day, ml / 0.5);
	if (ml > 0.5) {
		bgcol = day;
	}
	let moomcircle: f32 = step(0.08, distance(mouse, pos));
	let suncircle: f32 = step(0.09, m);
	var returncol: vec3<f32> = mix(sun, bgcol, suncircle);
	returncol = returncol * (moomcircle);
	return returncol;
}

fn rand(x: f32) -> f32 {
	var res: f32 = 0.;

	for (var i: i32 = 0; i < 5; i = i + 1) {
		res = res + (0.24 * f32(i) * sin(x * 0.68171 * f32(i)));
	}

	return res;
}


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {

    var night: vec3f = vec3f(0.0,0.0,0.0);

	var fragColor: vec4<f32>;
	var fragCoord = vec2<f32>(f32(g_mouse.x), f32(g_mouse.y) );

	let uv2: vec2<f32> = fragCoord.xy;
	var v: f32 = uv2.y + 0.1;
	v = 1. - abs(v * 2. - 1.);
	v = pow(v, 2. + sin((globals.time * 0.2 + 0.05) * 6.2831855) * 0.5);
	var color: vec3<f32> = vec3<f32>(0.);
	var x: f32 = 1. - uv2.x * 0.75;
	var y: f32 = 1. - abs(uv2.y * 2. - 1.);
	color = color + (vec3<f32>(x * 0.5, y, x) * v);
	let seed: vec2<f32> = fragCoord.xy;
	var r: vec2<f32>;
	r.x = fract(sin(seed.x * 12.9898 + seed.y * 78.233) * 43758.547);
	r.y = fract(sin(seed.x * 53.7842 + seed.y * 47.5134) * 43758.547);
	let s: f32 = mix(r.x, (sin((globals.time * 2.5 + 60.) * r.y) * 0.5 + 0.5) * (r.y * r.y * (r.y * r.y)), 0.04);
	color = color + (pow(s, 70.) * (1. - v));
	night = color;
	fragColor.a = 1.;
	var uv: vec2<f32> = fragCoord.xy;
	uv.x = uv.x ;
	let floorBounceCount: f32 = 0.;
	let rk: f32 = 0.025;
	let speed: f32 = 5.;
	var mouse: vec2<f32> = vec2<f32>(50., 50.);//inputs.mouse.xy;

		mouse = vec2<f32>(abs(((globals.time / speed) % (2.)) - 1.) * (1. - 2. * rk) + rk, pow(0.5, floorBounceCount) * abs(sin(globals.time / speed)) * 0.45 + rk);
		mouse.x = mouse.x - 0.5;
		mouse.y = mouse.y * 2.;
		mouse.y = mouse.y - 0.5;



	var col: vec3<f32> = background(night, uv, mouse);
	var light: vec3<f32> = vec3<f32>(0., 0., 0.);
	var iterations: i32 = 50;
	var incr: vec2<f32> = uv / f32(iterations);
	var p: vec2<f32> = vec2<f32>(0., 0.) + incr;

	for (var i: i32 = 2; i < iterations; i = i + 1) {
		light = light + (background(night, p, mouse));
		p = p + (incr);
	}

	light = light / (f32(iterations) * max(0.001, dot(uv, uv)) * 80.);
	let star: vec2<f32> = fragCoord.xy;
	let moomcircle: f32 = smoothstep(0.08, 0.085, distance(mouse, uv));
	col = col * (moomcircle);
	if (moomcircle < 1.) {
		col = col + (vec3<f32>((1. - moomcircle) * length(mouse)));
	}
	fragColor = vec4<f32>(col + light, 1.);

	return fragColor;
}

*/

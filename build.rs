use hlsl_compiler::{compile_from_file_to_file, CompileFlags, CompileFromFileToFileError, EffectCompileFlags};

fn main() {
	println!("Compiling Pixel Shader");
	let (result, messages) = compile_from_file_to_file(
		"shaders/pixel_shader.hlsl".to_string(),
		"shaders/pixel_shader.cso".to_string(),
		None,
		None,
		"PShader".to_string(),
		"ps_5_0".to_string(),
		CompileFlags::empty(),
		EffectCompileFlags::empty()
	);
	check_compile_result(result, messages);
	println!("Compiling Vertex Shader");
	let (result, messages) = compile_from_file_to_file(
		"shaders/vertex_shader.hlsl".to_string(),
		"shaders/vertex_shader.cso".to_string(),
		None,
		None,
		"VShader".to_string(),
		"vs_5_0".to_string(),
		CompileFlags::empty(),
		EffectCompileFlags::empty()
	);
	check_compile_result(result, messages);
	println!("cargo:rerun-if-changed=shaders/")
}

fn check_compile_result(result: Result<(), CompileFromFileToFileError>, messages: Option<Vec<u8>>) {
	if let Some(message_blob) = messages {
		print_messages(message_blob)
	}

	result.unwrap();
}

fn print_messages(message_blob: Vec<u8>) {
	let message_string = String::from_utf8(message_blob).unwrap();
	let messages = message_string.lines();
	for line in messages {
		println!("{}", line);
	}
}
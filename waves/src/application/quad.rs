use crate::application::shaders::ShaderContext;
use crate::application::vertex::{BasicMesh, Vertex};
use crate::render_core::animation_params::AnimationParams;
use crate::render_core::frame_sequencer::FrameGate;
use crate::render_core::mesh::{
	add_mesh, clear_frame, draw_meshes_always, DrawBuffers, DrawMode, MeshMode,
};
#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn generate_drawable_quad(shader_context: ShaderContext) -> Vec<(BasicMesh, DrawBuffers)> {
	let meshes = generate_mesh();
	let buffers: Vec<DrawBuffers> =
		meshes.iter().map(|m| add_mesh(&shader_context, m, MeshMode::Static).unwrap()).collect();

	meshes.into_iter().zip(buffers.into_iter()).collect()
}

fn generate_mesh() -> Vec<BasicMesh> {
	let mut mesh = BasicMesh::with_capacities(4, 6);

	mesh.push_vertex(Vertex::from_vecs_2d(nglm::vec2(-1.0, -1.0), nglm::vec4(1.0, 0.0, 0.0, 1.0)));
	mesh.push_vertex(Vertex::from_vecs_2d(nglm::vec2(-1.0, 1.0), nglm::vec4(0.0, 1.0, 0.0, 1.0)));
	mesh.push_vertex(Vertex::from_vecs_2d(nglm::vec2(1.0, -1.0), nglm::vec4(0.0, 0.0, 1.0, 1.0)));
	mesh.push_vertex(Vertex::from_vecs_2d(nglm::vec2(1.0, 1.0), nglm::vec4(1.0, 1.0, 0.0, 1.0)));

	mesh.push_index(2);
	mesh.push_index(0);
	mesh.push_index(1);
	mesh.push_index(1);
	mesh.push_index(3);
	mesh.push_index(2);

	vec![mesh]
}

pub async fn draw(gate: FrameGate<AnimationParams>, shader: ShaderContext) {
	let meshes_and_buffers = generate_drawable_quad(shader.clone());

	loop {
		let params = (&gate).await;

		clear_frame(params.viewport.context());

		shader.use_shader();
		draw_meshes_always(params.viewport.context(), &meshes_and_buffers, DrawMode::Surface);
	}
}

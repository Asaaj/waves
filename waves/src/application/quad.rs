
use crate::application::shaders::ShaderContext;
use crate::application::vertex::{BasicMesh, Vertex};
use crate::render_core::mesh::{
	add_mesh, DrawBuffers, MeshMode,
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

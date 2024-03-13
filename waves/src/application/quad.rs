use std::time::Duration;
use async_std::task::sleep;
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

pub async fn draw(gate: FrameGate<AnimationParams>, simulation_frame: async_channel::Receiver<u64>, shader: ShaderContext) {
	let clear_color = nglm::vec4(0.0, 0.0, 0.0, 1.0);
	let meshes_and_buffers = generate_drawable_quad(shader.clone());

	let check_frame = async move |current_frame: u64| {
		let last_simulation_frame = simulation_frame.recv().await.expect("Failed to receive frame number");
		if last_simulation_frame != current_frame {
			panic!("Draw is out of sync with simulation! Expected frame {}, got {}", current_frame, last_simulation_frame);
		}
	};

	loop {
		let params = (&gate).await;
		check_frame(params.frame_number).await;

		clear_frame(params.viewport.context(), clear_color);

		shader.use_shader();
		draw_meshes_always(params.viewport.context(), &meshes_and_buffers, DrawMode::Surface);
	}
}

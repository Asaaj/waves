use image::Rgba;
use web_sys::WebGl2RenderingContext;

use crate::application::shaders::ShaderContext;
use crate::application::vertex::{BasicMesh, Vertex};
use crate::render_core::animation_params::AnimationParams;
use crate::render_core::frame_sequencer::FrameGate;
use crate::render_core::mesh::{
	add_mesh, clear_frame, draw_meshes_always, DrawBuffers, DrawMode, MeshMode,
};
use crate::render_core::{texture, uniform};
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

pub async fn draw_indirect(
	gate: FrameGate<AnimationParams>,
	simulation_frame: async_channel::Receiver<u64>,
	render_shader: ShaderContext,
	texture_passthrough: ShaderContext,
) {
	let check_frame = async move |current_frame: u64| {
		let last_simulation_frame =
			simulation_frame.recv().await.expect("Failed to receive frame number");
		if last_simulation_frame != current_frame {
			panic!(
				"Draw is out of sync with simulation! Expected frame {}, got {}",
				current_frame, last_simulation_frame
			);
		}
	};

	let clear_color = nglm::vec4(0.0, 0.0, 0.0, 1.0);
	let meshes_and_buffers = generate_drawable_quad(render_shader.clone());

	let mut texture_dimensions = nglm::vec2(300.0, 300.0);
	let texture_index = 0;
	let texture_target = WebGl2RenderingContext::TEXTURE0 + texture_index as u32;
	let color_attachment = WebGl2RenderingContext::COLOR_ATTACHMENT0 + 0u32;
	let (framebuffer, texture) = texture::create_render_target::<Rgba<u8>>(
		render_shader.context.clone(),
		texture_target,
		color_attachment,
		texture_dimensions,
	)
	.expect("Failed to create render target");

	texture_passthrough.use_shader();
	uniform::init_i32("s_texture", &texture_passthrough, texture_index);

	loop {
		let params = (&gate).await;
		check_frame(params.frame_number).await;

		let context = params.viewport.context();

		let current_dimensions = params.viewport.dimensions();
		if current_dimensions != texture_dimensions {
			waves_log!(
				"Resizing texture: {} x {} -> {} x {}",
				texture_dimensions.x,
				texture_dimensions.y,
				current_dimensions.x,
				current_dimensions.y,
			);
			texture_dimensions = current_dimensions;
			texture::regenerate_texture::<Rgba<u8>>(
				context.clone(),
				texture_target,
				texture_dimensions,
			)
			.expect("Failed to update texture size");
		}

		{
			render_shader.use_shader();

			context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));
			context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

			context.viewport(0, 0, texture_dimensions.x as i32, texture_dimensions.y as i32);
			clear_frame(context, clear_color);

			draw_meshes_always(context, &meshes_and_buffers, DrawMode::Surface);
		}

		{
			texture_passthrough.use_shader();

			context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
			context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

			context.viewport(0, 0, params.viewport.width() as i32, params.viewport.height() as i32);
			clear_frame(context, clear_color);

			draw_meshes_always(context, &meshes_and_buffers, DrawMode::Surface);
		}
	}
}

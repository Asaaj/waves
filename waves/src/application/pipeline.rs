use image::Rgba;
use web_sys::WebGl2RenderingContext;

use crate::application::quad::generate_drawable_quad;
use crate::application::shaders::ShaderContext;
use crate::render_core::animation_params::AnimationParams;
use crate::render_core::frame_sequencer::FrameGate;
use crate::render_core::mesh::{clear_frame, draw_meshes_always, DrawMode};
use crate::render_core::ping_pong_buffer::PingPongBuffer;
use crate::render_core::uniform;

pub async fn draw_indirect(
	gate: FrameGate<AnimationParams>,
	simulation_frame: async_channel::Receiver<u64>,
	new_frame_shader: ShaderContext,
	// combine_frames_shader: ShaderContext,
	render_to_texture: ShaderContext,
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
	let meshes_and_buffers = generate_drawable_quad(new_frame_shader.clone());

	let mut texture_dimensions = nglm::vec2(300u32, 300u32);
	// let render_target =
	// 	RenderTarget::<Rgba<u8>>::new(new_frame_shader.context.clone(), 0, 0,
	// texture_dimensions).expect("Failed to create render target");
	let mut pingpong = PingPongBuffer::<Rgba<u8>, 2>::new(
		new_frame_shader.context.clone(),
		[0, 1],
		texture_dimensions,
	)
	.expect("Failed to create pingpong buffer");

	render_to_texture.use_shader();
	let mut texture_to_draw = uniform::init_smart_i32(
		"s_texture",
		&render_to_texture,
		pingpong.current_texture_index() as i32,
	);

	loop {
		let params = (&gate).await;
		check_frame(params.frame_number).await;

		let f_dimensions = params.viewport.dimensions();
		texture_dimensions = nglm::vec2(f_dimensions.x as u32, f_dimensions.y as u32);
		pingpong.update_size(texture_dimensions);

		let context = params.viewport.context();

		{
			new_frame_shader.use_shader();
			context
				.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&pingpong.framebuffer));

			context.viewport(0, 0, texture_dimensions.x as i32, texture_dimensions.y as i32);
			clear_frame(context, clear_color);

			draw_meshes_always(context, &meshes_and_buffers, DrawMode::Surface);
		}

		{
			render_to_texture.use_shader();
			context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

			context.viewport(0, 0, params.viewport.width() as i32, params.viewport.height() as i32);
			clear_frame(context, clear_color);

			texture_to_draw.smart_write(pingpong.current_texture_index() as i32);

			draw_meshes_always(context, &meshes_and_buffers, DrawMode::Surface);
		}

		pingpong.bind_next();
	}
}

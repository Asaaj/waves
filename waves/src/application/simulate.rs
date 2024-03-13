use std::f32::consts::{PI, TAU};

use nglm::Vec2;

use crate::application::shaders::ShaderContext;
use crate::render_core::animation_params::AnimationParams;
use crate::render_core::frame_sequencer::FrameGate;
use crate::render_core::uniform;
use crate::utils::prelude::*;

pub async fn waves(gate: FrameGate<AnimationParams>, shader: ShaderContext) {
	let mut phase = 0.0f32;
	let wavelength = 0.1f32;
	let phase_step_per_sec = TAU; // One full period

	shader.use_shader();

	let _u_oscillator_location = uniform::init_smart_mat2(
		"u_oscillatorLocation",
		&shader,
		nglm::mat2x2(0.5, -0.5, 0.5, -0.5),
	);
	let _u_wavelength = uniform::init_smart_f32("u_wavelength", &shader, wavelength);

	let mut u_phase = uniform::init_smart_vec2("u_phase", &shader, Vec2::repeat(phase));
	let mut u_viewport_size = uniform::new_smart_vec2("u_viewportSize", &shader);

	loop {
		let params = (&gate).await;
		shader.use_shader();

		let width = params.viewport.width();
		let height = params.viewport.height();
		u_viewport_size.smart_write(nglm::vec2(width, height));

		phase += phase_step_per_sec * params.delta_time.as_secs_f32();
		if phase > TAU {
			phase -= TAU;
		}
		u_phase.smart_write(Vec2::repeat(phase));
	}
}

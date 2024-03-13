use std::rc::Rc;

use single_thread_executor::new_executor_and_spawner;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::application::shaders::{load_render_texture_shaders, load_simulation_shaders};
use crate::application::{quad, simulate};
use crate::render_core::animation::{wrap_animation_body, AnimationFn};
use crate::render_core::animation_params::AnimationParams;
use crate::render_core::frame_sequencer::{FrameGate, FrameMarker, FrameSequencer};
use crate::utils::prelude::*;

pub fn get_animation_loop(
	canvas: HtmlCanvasElement,
	context: WebGl2RenderingContext,
) -> Result<AnimationFn, JsValue> {
	let (executor, spawner) = new_executor_and_spawner();
	spawn_local(async move {
		executor.run().await;
	});

	let frame_sequencer = Rc::new(FrameSequencer::<AnimationParams>::new());
	let simulation_shader =
		load_simulation_shaders(&context).expect("Failed to load simulation shaders");
	let render_texture_shader =
		load_render_texture_shaders(&context).expect("Failed to load render shaders");

	let (sender, receiver) = async_channel::unbounded::<u64>();

	spawner.spawn(simulate::waves(
		FrameGate::new(frame_sequencer.clone(), "Simulate Waves".to_owned()),
		sender,
		simulation_shader.clone(),
	));

	spawner.spawn(quad::draw_indirect(
		FrameGate::new(frame_sequencer.clone(), "Draw Quad".to_owned()),
		receiver,
		simulation_shader.clone(),
		render_texture_shader,
	));

	let frame_marker = FrameMarker::new(frame_sequencer.clone());

	Ok(wrap_animation_body(move |params: AnimationParams| {
		frame_marker.frame(params);
	}))
}

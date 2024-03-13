use std::rc::Rc;

use single_thread_executor::new_executor_and_spawner;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::application::shaders::load_shaders;
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
	let shader = load_shaders(&context).expect("Failed to load shaders");

	let (sender, receiver) = async_channel::unbounded::<u64>();

	spawner.spawn(quad::draw(
		FrameGate::new(frame_sequencer.clone(), "Draw Quad".to_owned()),
		receiver,
		shader.clone(),
	));

	spawner.spawn(simulate::waves(
		FrameGate::new(frame_sequencer.clone(), "Simulate Waves".to_owned()),
		sender,
		shader.clone(),
	));

	let frame_marker = FrameMarker::new(frame_sequencer.clone());

	Ok(wrap_animation_body(move |params: AnimationParams| {
		frame_marker.frame(params);
	}))
}

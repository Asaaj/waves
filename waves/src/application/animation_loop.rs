use std::rc::Rc;

use single_thread_executor::new_executor_and_spawner;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::application::quad;
// use crate::application::data::load_temp_data;
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

	spawner.spawn(quad::draw(
		FrameGate::new(frame_sequencer.clone(), "Draw Quad".to_owned()),
		context.clone(),
	));

	let frame_marker = FrameMarker::new(frame_sequencer.clone());

	Ok(wrap_animation_body(move |params: AnimationParams| {
		frame_marker.frame(params);
	}))
}

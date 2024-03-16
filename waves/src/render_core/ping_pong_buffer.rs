use std::marker::PhantomData;

use itertools::Itertools;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

use crate::render_core::image::LoadableImageType;
use crate::render_core::texture::{
	bind_texture_to_framebuffer, generate_and_bind_framebuffer, generate_and_bind_texture,
	regenerate_texture,
};
use crate::utils::prelude::*;

pub struct PingPongBuffer<T: LoadableImageType, const N: usize> {
	pub framebuffer: WebGlFramebuffer,
	context: WebGl2RenderingContext,
	textures: [WebGlTexture; N], // TODO: This should use [RenderTarget; N] or similar
	texture_indices: [u32; N],
	current_index: usize,
	dimensions: nglm::U32Vec2,
	_phantom_type: PhantomData<T>,
}

impl<T: LoadableImageType, const N: usize> PingPongBuffer<T, N> {
	pub fn new(
		context: WebGl2RenderingContext,
		texture_indices: [u32; N],
		dimensions: nglm::U32Vec2,
	) -> Result<Self, JsValue> {
		let texture_results: Vec<_> = texture_indices
			.iter()
			.map(|index| {
				let texture_target = WebGl2RenderingContext::TEXTURE0 + index;
				generate_and_bind_texture::<T>(
					context.clone(),
					texture_target,
					WebGl2RenderingContext::LINEAR,
					WebGl2RenderingContext::NEAREST,
					dimensions,
					None,
				)
			})
			.collect();

		if let Some(invalid) =
			texture_results.iter().find(|result| result.is_err()).map(|e| e.clone().err())
		{
			return Err(invalid.expect("Problem detecting error in ping-pong buffer creation"));
		}

		let textures: [WebGlTexture; N] = texture_results
			.into_iter()
			.map(|result| result.expect("Undetected error in ping-pong texture creation"))
			.collect_vec()
			.try_into()
			.expect("Failed to collect textures into array");

		let color_attachment_index = 0u32;
		let color_attachment = WebGl2RenderingContext::COLOR_ATTACHMENT0 + color_attachment_index;

		let current_index = 0;

		let frame_buffer = generate_and_bind_framebuffer(
			context.clone(),
			color_attachment,
			&textures[current_index],
		)?;

		Ok(Self {
			framebuffer: frame_buffer,
			context,
			textures,
			texture_indices,
			current_index,
			dimensions,
			_phantom_type: Default::default(),
		})
	}

	pub fn bind_next(&mut self) {
		self.current_index += 1;
		self.current_index %= N;

		bind_texture_to_framebuffer(
			self.context.clone(),
			WebGl2RenderingContext::COLOR_ATTACHMENT0,
			&self.framebuffer,
			&self.textures[self.current_index],
		)
	}

	pub fn current_texture_index(&self) -> u32 { self.texture_indices[self.current_index] }

	pub fn current_texture_target(&self) -> u32 {
		WebGl2RenderingContext::TEXTURE0 + self.current_texture_index()
	}

	pub fn update_size(&mut self, dimensions: nglm::U32Vec2) {
		if dimensions != self.dimensions {
			waves_log!(
				"Resizing ping-pong textures: {} x {} -> {} x {}",
				self.dimensions.x,
				self.dimensions.y,
				dimensions.x,
				dimensions.y
			);
			self.texture_indices.iter().for_each(|index| {
				regenerate_texture::<T>(
					self.context.clone(),
					WebGl2RenderingContext::TEXTURE0 + index,
					dimensions,
				)
				.expect("Failed to update texture dimensions")
			});
			self.dimensions = dimensions;
		}
	}
}

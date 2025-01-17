use std::marker::PhantomData;

use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

use crate::render_core::image::LoadableImageType;

pub struct RenderTarget<T: LoadableImageType> {
	pub framebuffer: WebGlFramebuffer,
	pub texture: WebGlTexture,
	texture_target_index: u32,
	color_attachment_index: u32,
	_phantom_image_type: PhantomData<T>,
}

impl<T: LoadableImageType> RenderTarget<T> {
	pub fn new(
		context: WebGl2RenderingContext,
		texture_target_index: u32,
		color_attachment_index: u32,
		dimensions: nglm::U32Vec2,
	) -> Result<Self, JsValue> {
		let texture_target = WebGl2RenderingContext::TEXTURE0 + texture_target_index;
		let color_attachment = WebGl2RenderingContext::COLOR_ATTACHMENT0 + color_attachment_index;
		let (framebuffer, texture) =
			create_render_target::<T>(context, texture_target, color_attachment, dimensions)?;

		Ok(Self {
			framebuffer,
			texture,
			texture_target_index,
			color_attachment_index,
			_phantom_image_type: Default::default(),
		})
	}

	pub fn texture_index(&self) -> u32 { self.texture_target_index }

	pub fn texture_target(&self) -> u32 {
		WebGl2RenderingContext::TEXTURE0 + self.texture_target_index
	}
}

fn create_render_target<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	texture_target: u32,
	color_attachment: u32,
	dimensions: nglm::U32Vec2,
) -> Result<(WebGlFramebuffer, WebGlTexture), JsValue> {
	let texture = generate_and_bind_texture::<T>(
		context.clone(),
		texture_target,
		WebGl2RenderingContext::LINEAR,
		WebGl2RenderingContext::NEAREST,
		dimensions,
		None,
	)?;

	let framebuffer = generate_and_bind_framebuffer(context, color_attachment, &texture)?;

	Ok((framebuffer, texture))
}

pub fn generate_and_bind_texture<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	texture_target: u32,
	min_filter: u32,
	mag_filter: u32,
	dimensions: nglm::U32Vec2,
	bytes: Option<&[u8]>,
) -> Result<WebGlTexture, JsValue> {
	let texture = context.create_texture().ok_or("Failed to create texture")?;

	context.active_texture(texture_target);
	context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D,
		WebGl2RenderingContext::TEXTURE_MIN_FILTER,
		min_filter as i32,
	);
	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D,
		WebGl2RenderingContext::TEXTURE_MAG_FILTER,
		mag_filter as i32,
	);

	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D,
		WebGl2RenderingContext::TEXTURE_WRAP_S,
		WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
	);
	context.tex_parameteri(
		WebGl2RenderingContext::TEXTURE_2D,
		WebGl2RenderingContext::TEXTURE_WRAP_T,
		WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
	);

	context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
		WebGl2RenderingContext::TEXTURE_2D,
		0,
		T::texture_internal_format() as i32,
		dimensions.x as i32,
		dimensions.y as i32,
		0,
		T::texture_format(),
		T::texture_type(),
		bytes,
	)?;

	Ok(texture)
}

pub fn regenerate_texture<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	texture_target: u32,
	dimensions: nglm::U32Vec2,
) -> Result<(), JsValue> {
	context.active_texture(texture_target);

	context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
		WebGl2RenderingContext::TEXTURE_2D,
		0,
		T::texture_internal_format() as i32,
		dimensions.x as i32,
		dimensions.y as i32,
		0,
		T::texture_format(),
		T::texture_type(),
		None,
	)?;

	Ok(())
}

pub fn generate_and_bind_framebuffer(
	context: WebGl2RenderingContext,
	attachment: u32,
	texture: &WebGlTexture,
) -> Result<WebGlFramebuffer, JsValue> {
	let framebuffer = context.create_framebuffer().ok_or("Failed to create framebuffer")?;

	bind_texture_to_framebuffer(context, attachment, &framebuffer, texture);

	Ok(framebuffer)
}

pub fn bind_texture_to_framebuffer(
	context: WebGl2RenderingContext,
	attachment: u32,
	framebuffer: &WebGlFramebuffer,
	texture: &WebGlTexture,
) {
	context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));

	context.framebuffer_texture_2d(
		WebGl2RenderingContext::FRAMEBUFFER,
		attachment,
		WebGl2RenderingContext::TEXTURE_2D,
		Some(texture),
		0,
	);
}

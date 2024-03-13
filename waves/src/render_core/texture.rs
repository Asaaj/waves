use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlTexture};

use crate::render_core::image::LoadableImageType;

pub fn create_render_target<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	texture_target: u32,
	color_attachment: u32,
	dimensions: nglm::Vec2,
) -> Result<(WebGlFramebuffer, WebGlTexture), JsValue> {
	let texture = generate_and_bind_texture::<T>(
		context.clone(),
		texture_target,
		WebGl2RenderingContext::LINEAR,
		WebGl2RenderingContext::NEAREST,
		dimensions,
	)?;

	let framebuffer = generate_and_bind_framebuffer(context, color_attachment, &texture)?;

	Ok((framebuffer, texture))
}

fn generate_and_bind_texture<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	texture_target: u32,
	min_filter: u32,
	mag_filter: u32,
	dimensions: nglm::Vec2,
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
		None,
	)?;

	Ok(texture)
}

pub fn regenerate_texture<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	texture_target: u32,
	dimensions: nglm::Vec2,
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

fn generate_and_bind_framebuffer(
	context: WebGl2RenderingContext,
	attachment: u32,
	texture: &WebGlTexture,
) -> Result<WebGlFramebuffer, JsValue> {
	let framebuffer = context.create_framebuffer().ok_or("Failed to create framebuffer")?;
	context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));

	context.framebuffer_texture_2d(
		WebGl2RenderingContext::FRAMEBUFFER,
		attachment,
		WebGl2RenderingContext::TEXTURE_2D,
		Some(texture),
		0,
	);

	Ok(framebuffer)
}

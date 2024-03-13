use image::{
	DynamicImage, EncodableLayout, GenericImageView, GrayImage, Luma, Rgb, RgbImage, Rgba,
	RgbaImage,
};
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext;

use crate::render_core::texture;

/// This feels like it probably duplicates something that can be done in the
/// image library already.
pub trait LoadableImageType {
	type ImageType: GenericImageView;

	// Combinations: https://registry.khronos.org/webgl/specs/latest/2.0/#TEXTURE_TYPES_FORMATS_FROM_DOM_ELEMENTS_TABLE
	fn texture_internal_format() -> u32;
	fn texture_format() -> u32;
	fn texture_type() -> u32;

	fn cast_to(dynamic: &DynamicImage) -> Option<&Self::ImageType>;
	fn copy_to(dynamic: &DynamicImage) -> Self::ImageType;
	fn raw(img: &Self::ImageType) -> &[u8];

	fn name() -> String;
}

impl LoadableImageType for Luma<u8> {
	type ImageType = GrayImage;

	fn texture_internal_format() -> u32 { WebGl2RenderingContext::LUMINANCE }

	fn texture_format() -> u32 { WebGl2RenderingContext::LUMINANCE }

	fn texture_type() -> u32 { WebGl2RenderingContext::UNSIGNED_BYTE }

	fn cast_to(dynamic: &DynamicImage) -> Option<&Self::ImageType> { dynamic.as_luma8() }

	fn copy_to(dynamic: &DynamicImage) -> Self::ImageType { dynamic.to_luma8() }

	fn raw(img: &Self::ImageType) -> &[u8] { img.as_bytes() }

	fn name() -> String { "Luma8".to_owned() }
}

impl LoadableImageType for Rgb<u8> {
	type ImageType = RgbImage;

	fn texture_internal_format() -> u32 { WebGl2RenderingContext::RGB }

	fn texture_format() -> u32 { WebGl2RenderingContext::RGB }

	fn texture_type() -> u32 { WebGl2RenderingContext::UNSIGNED_BYTE }

	fn cast_to(dynamic: &DynamicImage) -> Option<&Self::ImageType> { dynamic.as_rgb8() }

	fn copy_to(dynamic: &DynamicImage) -> Self::ImageType { dynamic.to_rgb8() }

	fn raw(img: &Self::ImageType) -> &[u8] { img.as_bytes() }

	fn name() -> String { "Rgb8".to_owned() }
}

impl LoadableImageType for Rgba<u8> {
	type ImageType = RgbaImage;

	fn texture_internal_format() -> u32 { WebGl2RenderingContext::RGBA }

	fn texture_format() -> u32 { WebGl2RenderingContext::RGBA }

	fn texture_type() -> u32 { WebGl2RenderingContext::UNSIGNED_BYTE }

	fn cast_to(dynamic: &DynamicImage) -> Option<&Self::ImageType> { dynamic.as_rgba8() }

	fn copy_to(dynamic: &DynamicImage) -> Self::ImageType { dynamic.to_rgba8() }

	fn raw(img: &Self::ImageType) -> &[u8] { img.as_bytes() }

	fn name() -> String { "Rgba8".to_owned() }
}

pub fn load_into_texture<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	png_bytes: &[u8],
	texture_number: u32,
) -> Result<(), JsValue> {
	load_into_texture_with_filters::<T>(
		context,
		png_bytes,
		texture_number,
		WebGl2RenderingContext::LINEAR,
		WebGl2RenderingContext::LINEAR,
	)
}

pub fn load_into_texture_with_filters<T: LoadableImageType>(
	context: WebGl2RenderingContext,
	png_bytes: &[u8],
	texture_number: u32,
	min_filter: u32,
	mag_filter: u32,
) -> Result<(), JsValue> {
	let decoder = png::Decoder::new(png_bytes);
	let mut reader = decoder.read_info().map_err(|s| s.to_string())?;
	let mut buf = vec![0; reader.output_buffer_size()];

	let info = reader.next_frame(&mut buf).map_err(|s| s.to_string())?;
	let bytes = &buf[..info.buffer_size()];

	let dimensions = nglm::vec2(info.width, info.height);

	// TODO: Probably slower, but worth profiling:
	// let dyn_img = image::load_from_memory_with_format(png_bytes,
	// ImageFormat::Png)     .map_err(|e| e.to_string())?;
	// let name = T::name();
	// let concrete_image = T::cast_to(&dyn_img)
	//     .expect(format!("Image was not stored with type {name}").as_str());
	// .ok_or(format!("Image was not stored with type {name}"));
	// let dimensions = concrete_image.dimensions();

	texture::generate_and_bind_texture::<T>(
		context,
		texture_number,
		min_filter,
		mag_filter,
		dimensions,
		Some(bytes),
	)?;

	Ok(())
}

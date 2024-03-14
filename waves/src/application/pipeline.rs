use image::Rgba;
use web_sys::WebGl2RenderingContext;
use crate::application::quad::generate_drawable_quad;
use crate::application::shaders::ShaderContext;
use crate::render_core::animation_params::AnimationParams;
use crate::render_core::frame_sequencer::FrameGate;
use crate::render_core::{texture, uniform};
use crate::render_core::mesh::{clear_frame, draw_meshes_always, DrawMode};
use crate::render_core::viewport::Viewport;

use crate::utils::prelude::*;

pub async fn draw_indirect(
    gate: FrameGate<AnimationParams>,
    simulation_frame: async_channel::Receiver<u64>,
    new_frame_shader: ShaderContext,
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
    let texture_index = 0;
    let texture_target = WebGl2RenderingContext::TEXTURE0 + texture_index as u32;
    let color_attachment = WebGl2RenderingContext::COLOR_ATTACHMENT0 + 0u32;
    let (framebuffer, texture) = texture::create_render_target::<Rgba<u8>>(
        new_frame_shader.context.clone(),
        texture_target,
        color_attachment,
        texture_dimensions,
    )
        .expect("Failed to create render target");

    render_to_texture.use_shader();
    uniform::init_i32("s_texture", &render_to_texture, texture_index);

    loop {
        let params = (&gate).await;
        check_frame(params.frame_number).await;
        texture_dimensions = update_texture_size(params.viewport.clone(), texture_dimensions, texture_target);

        let context = params.viewport.context();

        {
            new_frame_shader.use_shader();

            context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&framebuffer));
            context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

            context.viewport(0, 0, texture_dimensions.x as i32, texture_dimensions.y as i32);
            clear_frame(context, clear_color);

            draw_meshes_always(context, &meshes_and_buffers, DrawMode::Surface);
        }

        {
            render_to_texture.use_shader();

            context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
            context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

            context.viewport(0, 0, params.viewport.width() as i32, params.viewport.height() as i32);
            clear_frame(context, clear_color);

            draw_meshes_always(context, &meshes_and_buffers, DrawMode::Surface);
        }
    }
}

fn update_texture_size(
    viewport: Viewport,
    previous: nglm::U32Vec2,
    texture_target: u32,
) -> nglm::U32Vec2 {
    let current = viewport.dimensions();
    let current = nglm::vec2(current.x as u32, current.y as u32);
    if previous != current {
        waves_log!(
			"Resizing texture: {} x {} -> {} x {}",
			previous.x,
			previous.y,
			current.x,
			current.y
		);
        texture::regenerate_texture::<Rgba<u8>>(
            viewport.context().clone(),
            texture_target,
            current,
        )
            .expect("Failed to update texture size");
    }
    current
}

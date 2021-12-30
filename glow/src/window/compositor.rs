use crate::{Backend, Color, Error, Renderer, Settings, Viewport};
use core::ffi::c_void;
use glow::HasContext;
use iced_graphics::{Antialiasing, Size};
use iced_native::screenshot::{ColorType, Screenshot};
use std::convert::TryInto;
/// A window graphics backend for iced powered by `glow`.
#[allow(missing_debug_implementations)]
pub struct Compositor {
    gl: glow::Context,
    framebuffer: Option<glow::Framebuffer>,
    framebuffer_size: Option<Size<u32>>,
}

impl iced_graphics::window::VirtualCompositor for Compositor {
    fn read(&self) -> Option<Screenshot> {
        let gl = &self.gl;

        // TODO: Validate the buffer
        let mut rv = Vec::new();

        // assert_eq!(buffer.len(), 3 * region.width as usize * region.height as usize);
        let width_height =
            self.framebuffer_size.expect("Uninitialized framebuffer");
        let (width, height) =
            (width_height.width as i32, width_height.height as i32);
        rv.resize((width * height * 3) as usize, 0);
        unsafe {
            gl.read_pixels(
                0,      //region.x as i32,
                0,      //region.y as i32,
                width,  //region.width as i32,
                height, //region.height as i32,
                glow::RGB,
                glow::UNSIGNED_BYTE,
                glow::PixelPackData::Slice(rv.as_mut_slice()),
            );
        }

        //png expects data starting at the top left corner; opengl starts reading from the bottem
        //right. this makes the opengl standard consistent with the expectation of png
        let new_rv: Vec<u8> = rv
            .as_mut_slice()
            .chunks_exact_mut(width as usize * 3)
            .rev()
            .map(|chunk| chunk.to_vec())
            .flatten()
            .collect();

        
        
        
        Some(
            Screenshot::new(
                new_rv,
                width.try_into().unwrap(),
                height.try_into().unwrap(),
            )
            .encoding(ColorType::Rgb),
        )
    }
}

impl iced_graphics::window::GLCompositor for Compositor {
    type Settings = Settings;
    type Renderer = Renderer;

    unsafe fn new(
        settings: Self::Settings,
        loader_function: impl FnMut(&str) -> *const c_void,
    ) -> Result<(Self, Self::Renderer), Error> {
        let gl = glow::Context::from_loader_function(loader_function);

        let version = gl.version();
        log::info!("Version: {:?}", version);
        log::info!("Embedded: {}", version.is_embedded);

        let renderer = gl.get_parameter_string(glow::RENDERER);
        log::info!("Renderer: {}", renderer);

        // Enable auto-conversion from/to sRGB
        gl.enable(glow::FRAMEBUFFER_SRGB);

        // Enable alpha blending
        gl.enable(glow::BLEND);
        gl.blend_func_separate(
            glow::SRC_ALPHA,
            glow::ONE_MINUS_SRC_ALPHA,
            glow::ONE,
            glow::ONE_MINUS_SRC_ALPHA,
        );

        // Disable multisampling by default
        gl.disable(glow::MULTISAMPLE);

        let renderer = Renderer::new(Backend::new(&gl, settings));

        let framebuffer = None;
        Ok((
            Self {
                gl,
                framebuffer,
                framebuffer_size: None,
            },
            renderer,
        ))
    }

    fn sample_count(settings: &Settings) -> u32 {
        settings
            .antialiasing
            .map(Antialiasing::sample_count)
            .unwrap_or(0)
    }

    fn resize_viewport(&mut self, physical_size: Size<u32>) {
        unsafe {
            self.gl.viewport(
                0,
                0,
                physical_size.width as i32,
                physical_size.height as i32,
            );

            self.framebuffer_size = Some(physical_size)
        }
    }

    fn present<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        color: Color,
        overlay: &[T],
    ) {
        let gl = &self.gl;

        let [r, g, b, a] = color.into_linear();

        unsafe {
            gl.clear_color(r, g, b, a);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        renderer.with_primitives(|backend, primitive| {
            backend.present(gl, primitive, viewport, overlay, self.framebuffer);
        });
        self.framebuffer_size = Some(viewport.physical_size());
    }
}

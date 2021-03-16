use anyhow::Result;
use ash::vk;
use nalgebra_glm as glm;
use support::{
    camera::{update_free_camera, CameraDirection, FreeCamera},
    run_app,
    vulkan::{
        Cube, CubeRender, Image, ImageNode, RawImage, RenderDevice, RenderGraph, ShaderCache,
    },
    App, ApplicationState,
};
use winit::event::VirtualKeyCode;

#[derive(Default)]
struct DemoApp {
    rendergraph: RenderGraph,
    shader_cache: ShaderCache,
    cube: Option<CubeRender>,
    angle: f32,
    camera: FreeCamera,
}

impl App for DemoApp {
    fn initialize(&mut self, state: &ApplicationState, render_device: &RenderDevice) -> Result<()> {
        state.capture_mouse(true)?;
        state.set_cursor_visible(false);

        self.rendergraph = create_rendergraph(&render_device)?;

        let cube = Cube::new(
            render_device.context.allocator.clone(),
            &render_device.command_pool,
        )?;
        let mut cube_render = CubeRender::new(render_device.context.device.clone(), cube);

        cube_render.create_pipeline(
            &mut self.shader_cache,
            self.rendergraph.pass("color")?.render_pass.clone(),
            vk::SampleCountFlags::TYPE_1,
        )?;
        self.cube = Some(cube_render);

        Ok(())
    }

    fn update(&mut self, state: &ApplicationState) -> Result<()> {
        self.angle += 10.0 * state.system.delta_time as f32;
        update_free_camera(&mut self.camera, state)?;
        Ok(())
    }

    fn render(&mut self, state: &ApplicationState, render_device: &mut RenderDevice) -> Result<()> {
        let perspective = glm::perspective_zo(
            state.system.aspect_ratio(),
            90_f32.to_radians(),
            0.01,
            1000.0,
        );
        let model = glm::rotate(
            &glm::Mat4::identity(),
            self.angle.to_radians(),
            &glm::Vec3::y(),
        );
        let mvp = perspective * self.camera.view_matrix() * model;

        let logical_size = state.window.inner_size();
        let window_dimensions = [logical_size.width, logical_size.height];
        let device = render_device.context.device.clone();
        render_device
            .frame
            .render(&window_dimensions, |command_buffer, image_index| {
                self.rendergraph.execute_pass(
                    command_buffer,
                    "color",
                    image_index,
                    |pass, command_buffer| {
                        device.update_viewport(command_buffer, pass.extent, false)?;
                        if let Some(cube) = self.cube.as_ref() {
                            cube.issue_commands(
                                command_buffer,
                                mvp,
                                glm::vec4(1.0, 1.0, 1.0, 1.0),
                                false,
                            )?;
                        }
                        Ok(())
                    },
                )?;

                Ok(())
            })?;

        if render_device.frame.recreated_swapchain {
            self.rendergraph = create_rendergraph(render_device)?;
            if let Some(cube) = self.cube.as_mut() {
                cube.create_pipeline(
                    &mut self.shader_cache,
                    self.rendergraph.pass("color")?.render_pass.clone(),
                    vk::SampleCountFlags::TYPE_1,
                )?;
            }
        }

        Ok(())
    }
}

pub fn create_rendergraph(render_device: &RenderDevice) -> Result<RenderGraph> {
    let swapchain = render_device.frame.swapchain()?;
    let swapchain_properties = render_device.frame.swapchain_properties;
    let device = render_device.context.device.clone();
    let allocator = render_device.context.allocator.clone();

    let color = "color";
    let backbuffer = &RenderGraph::backbuffer_name(0);
    let mut rendergraph = RenderGraph::new(
        &[color],
        vec![ImageNode {
            name: backbuffer.to_string(),
            extent: swapchain_properties.extent,
            format: swapchain_properties.surface_format.format,
            clear_value: vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.39, 0.58, 0.92, 1.0],
                },
            },
            samples: vk::SampleCountFlags::TYPE_1,
            force_store: false,
            force_shader_read: false,
        }],
        &[(color, backbuffer)],
    )?;

    rendergraph.build(device.clone(), allocator)?;

    let swapchain_images = swapchain
        .images()?
        .into_iter()
        .map(|handle| Box::new(RawImage(handle)) as Box<dyn Image>)
        .collect::<Vec<_>>();
    rendergraph.insert_backbuffer_images(device, swapchain_images)?;

    Ok(rendergraph)
}

fn main() -> Result<()> {
    run_app(DemoApp::default(), "Cube")
}

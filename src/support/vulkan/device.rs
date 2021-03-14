use anyhow::Result;
use ash::vk;
use raw_window_handle::HasRawWindowHandle;
use std::sync::Arc;

use super::core::{CommandPool, Context, Frame};

pub struct RenderDevice {
    pub command_pool: CommandPool,
    pub frame: Frame,
    pub context: Arc<Context>,
}

impl RenderDevice {
    const MAX_FRAMES_IN_FLIGHT: usize = 2;

    pub fn new(window_handle: &impl HasRawWindowHandle, dimensions: &[u32; 2]) -> Result<Self> {
        let context = Arc::new(Context::new(window_handle)?);
        let frame = Frame::new(context.clone(), dimensions, Self::MAX_FRAMES_IN_FLIGHT)?;

        let create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(context.physical_device.graphics_queue_family_index)
            .flags(vk::CommandPoolCreateFlags::TRANSIENT);
        let command_pool = CommandPool::new(
            context.device.clone(),
            context.graphics_queue(),
            create_info,
        )?;

        Ok(Self {
            command_pool,
            frame,
            context,
        })
    }
}

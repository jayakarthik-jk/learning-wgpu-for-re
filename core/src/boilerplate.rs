use pollster::FutureExt;
use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages,
};
use winit::dpi::PhysicalSize;

pub(crate) fn create_adapter(instance: &Instance) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions::default())
        .block_on()
        .expect("Adapter not found")
}

pub(crate) fn create_device(adapter: &Adapter) -> (Device, Queue) {
    adapter
        .request_device(
            &DeviceDescriptor {
                required_features: Features::empty(),
                ..Default::default()
            },
            None,
        )
        .block_on()
        .expect("Device with the required feature not found")
}

pub(crate) fn create_surface_config(
    surface: &Surface,
    adapter: &Adapter,
    size: &PhysicalSize<u32>,
) -> SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(adapter);
    let format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    }
}

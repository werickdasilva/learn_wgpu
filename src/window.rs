use winit::{dpi::PhysicalSize, window::WindowBuilder};

pub struct Window {
    pub title: String,
    pub size: (u32, u32),
}

impl Window {
    pub fn new() -> Self {
        Self {
            title: String::from("Learn Wgpu"),
            size: (500, 400),
        }
    }

    pub fn builder(self) -> WindowBuilder {
        WindowBuilder::new()
            .with_title(self.title)
            .with_inner_size(PhysicalSize::new(self.size.0, self.size.1))
    }
}

use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    winit::{WINIT_WINDOWS, WinitWindows},
};
pub use winit::window::BadIcon;
use winit::window::Icon as WinitIcon;

#[derive(Resource, Clone, Default)]
pub struct Icon(Option<WinitIcon>);

impl Icon {
    /// No icon
    pub const NONE: Self = Self(None);

    /// Creates an icon from 32bpp RGBA data.
    ///
    /// The length of `rgba` must be divisible by 4, and `width * height` must equal
    /// `rgba.len() / 4`. Otherwise, this will return a `BadIcon` error.
    pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, BadIcon> {
        Ok(Icon(Some(WinitIcon::from_rgba(rgba, width, height)?)))
    }
}

#[cfg(feature = "image")]
impl From<image::DynamicImage> for Icon {
    fn from(image: image::DynamicImage) -> Self {
        use image::GenericImageView;
        
        let rgba = image.to_rgba8().into_raw();
        let (width, height) = image.dimensions();
        Icon::from_rgba(rgba, width, height).unwrap_or_default()
    }
}

/// Apply the icon to all windows
pub(crate) fn apply(
    icon: Res<Icon>,
    _: NonSendMarker, // must run on the main thread
) {
    WINIT_WINDOWS.with_borrow_mut(|WinitWindows { windows, .. }| {
        if windows.is_empty() {
            return;
        }
        for window in windows.values() {
            window.set_window_icon(icon.0.clone());
        }
    });
}

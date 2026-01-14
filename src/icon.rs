use bevy::{
    ecs::system::NonSendMarker,
    prelude::*,
    winit::{WINIT_WINDOWS, WinitWindows},
};
#[cfg(feature = "image")]
pub use winit::window::BadIcon;
use winit::window::Icon as WinitIcon;

#[derive(Resource, Clone)]
pub struct Icon(WinitIcon);

impl From<WinitIcon> for Icon {
    fn from(icon: WinitIcon) -> Self {
        Icon(icon)
    }
}

impl Icon {
    #[cfg(feature = "image")]
    pub fn from_image(img: image::DynamicImage) -> Result<Self, BadIcon> {
        use image::GenericImageView;

        let rgba = img.to_rgba8().into_raw();
        let (width, height) = img.dimensions();

        Ok(Icon(WinitIcon::from_rgba(rgba, width, height)?))
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
            window.set_window_icon(Some(icon.0.clone()));
        }
    });
}

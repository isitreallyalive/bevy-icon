use bevy::prelude::*;
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

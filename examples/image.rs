use bevy::prelude::*;
use bevy_icon::prelude::*;

const ICON_DATA: &[u8] = include_bytes!("icon.jpg");

fn main() -> Result<()> {
    let icon = image::load_from_memory(ICON_DATA)?;
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(BevyIconPlugin::builder(Icon::from_image(icon)?).build())
        .run();
    Ok(())
}

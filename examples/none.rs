use bevy::prelude::*;
use bevy_icon::prelude::*;

fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        // BevyIconPlugin::default has the same effect
        .add_plugins(BevyIconPlugin::new(Icon::NONE))
        .run();

    Ok(())
}

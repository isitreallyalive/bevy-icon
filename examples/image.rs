use std::sync::LazyLock;

use bevy::prelude::*;
use bevy_icon::prelude::*;

macro_rules! load_icon {
    ($path:expr) => {
        Icon::from_image(
            image::load_from_memory(include_bytes!($path))
                .expect(concat!("failed to load ", $path)),
        )
        .expect(concat!("failed to create icon from ", $path))
    };
}

const CAT_ICON: LazyLock<Icon> = LazyLock::new(|| load_icon!("cat.png"));
const DOG_ICON: LazyLock<Icon> = LazyLock::new(|| load_icon!("dog.png"));

/// System that changes the icon resource every second
fn change(mut icon: ResMut<Icon>, mut is_cat: ResMut<IsCat>) {
    *icon = if is_cat.0 {
        DOG_ICON.clone()
    } else {
        CAT_ICON.clone()
    };
    is_cat.0 = !is_cat.0;
}

/// Resource to track which icon is currently set
#[derive(Resource)]
struct IsCat(bool);

fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyIconPlugin::builder(CAT_ICON.clone()).build())
        // change icon every second
        .add_systems(FixedUpdate, change)
        .insert_resource(Time::from_seconds(1.))
        .insert_resource(IsCat(true))
        .run();

    Ok(())
}

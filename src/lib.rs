#![no_std]

#[cfg(feature = "std")]
extern crate std;

use bevy::prelude::*;
use icon::Icon;

mod icon;

pub mod prelude {
    pub use crate::BevyIconPlugin;
    pub use crate::icon::BadIcon;
    pub use crate::icon::Icon;
}

#[derive(Default)]
pub struct BevyIconPlugin(Icon);

impl BevyIconPlugin {
    /// Initialize the plugin with the given icon
    pub fn new(icon: Icon) -> Self {
        BevyIconPlugin(icon)
    }
}

impl From<Icon> for BevyIconPlugin {
    fn from(icon: Icon) -> Self {
        BevyIconPlugin(icon)
    }
}

impl Plugin for BevyIconPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone())
            .add_systems(Update, icon::apply.run_if(resource_changed::<Icon>));
    }
}

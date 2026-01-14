use bevy::prelude::*;
use icon::Icon;

mod icon;

pub mod prelude {
    pub use crate::BevyIconPlugin;
    #[cfg(feature = "image")]
    pub use crate::icon::BadIcon;
    pub use crate::icon::Icon;
}

#[derive(bon::Builder)]
pub struct BevyIconPlugin {
    #[builder(start_fn)]
    icon: Icon,
}

impl From<Icon> for BevyIconPlugin {
    fn from(icon: Icon) -> Self {
        BevyIconPlugin { icon }
    }
}

impl Plugin for BevyIconPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.icon.clone())
            .add_systems(Update, icon::apply.run_if(resource_changed::<Icon>));
    }
}

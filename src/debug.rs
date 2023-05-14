use bevy::prelude::*;

pub(crate) struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    #[cfg(not(debug_assertions))]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "inspector"))]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
    }

    #[cfg(debug_assertions)]
    #[cfg(feature = "inspector")]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        bevy::app::PluginGroupBuilder::start::<Self>().add(WorldInspectorPlugin::new())
    }
}

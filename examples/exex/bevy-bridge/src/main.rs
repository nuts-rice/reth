use bevy::prelude::*;
use reth_exex::ExExContext;

// async fn init<Node:>
//

#[derive(Default, Resource)]
struct InitResource {
    balance: u64,
}

fn bevy_init(mut commands: Commands) {}

fn bevy_bridge() {}

pub struct BevyChainInspectorPlugin;
impl Plugin for BevyChainInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InitResource>();
        app.add_systems(Startup, bevy_init);
        app.add_systems(Update, bevy_bridge);
    }
}
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window { prevent_default_event_handling: false, ..default() }),
                ..Default::default()
            }),
            BevyChainInspectorPlugin,
        ))
        .run();
}

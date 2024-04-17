use bevy::prelude::*;
use futures::Future;
use reth_exex::{ExExContext, ExExEvent};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use reth_provider::CanonStateNotification;

async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> eyre::Result<impl Future<Output = eyre::Result<()>>> {
    Ok(bevy_bridge_exex(ctx))
}

async fn bevy_bridge_exex<Node: FullNodeComponents>(
    mut ctx: ExExContext<Node>,
    // account: Res<AccountResource>,
) -> eyre::Result<()> {
    unimplemented!()
}

#[derive(Default, Resource)]
struct InitResource {
    address: String,
    balance: u64,
}

#[derive(Default, Component)]
struct PlayerAccountResource {
    handle: usize,
    address: String,
    balance: u64,
}

fn bevy_init(mut commands: Commands) {}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        PlayerAccountResource {
            handle: 0,
            address: "0x7d873FbFE8e16f5F55740a52a356c2f52c613cdF".to_string(),
            balance: 0,
        },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-2., 0., 0.)),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    ));
    commands.spawn((
        PlayerAccountResource {
            handle: 1,
            address: "0x7f93B033D18dcA9fD2BA4CbF2bf73A2DF840756c".to_string(),
            balance: 0,
        },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-2., 0., 0.)),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    ));
}

fn bevy_bridge() {}

pub struct BevyChainInspectorPlugin;
impl Plugin for BevyChainInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InitResource>();
        app.add_systems(Startup, bevy_init);
        app.add_systems(Update, bevy_bridge);
    }
}
fn main() -> eyre::Result<()> {
    reth::cli::Cli::parse_args().run(|builder, _| async move {
        let handle = builder
            .node(EthereumNode::default())
            .install_exex("BevyBridge", exex_init)
            .launch()
            .await?;
        handle.wait_for_node_exit().await
    });
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window { prevent_default_event_handling: false, ..default() }),
                ..Default::default()
            }),
            BevyChainInspectorPlugin,
        ))
        .run();
    Ok(())
}

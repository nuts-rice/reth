use bevy::{log::LogPlugin, prelude::*};
use futures::Future;
use reth_exex::{ExExContext, ExExEvent};
use reth_node_api::FullNodeComponents;
use reth_node_ethereum::EthereumNode;
use reth_provider::CanonStateNotification;

pub mod bevy_data;
use bevy_data::PlayerBundle;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Lobby,
    InGame,
}

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

#[derive(Default, Component)]
struct PlayerAccountResource {
    handle: usize,
    address: String,
    balance: u64,
}

fn bevy_init(mut commands: Commands) {}

fn spawn_players(mut commands: Commands) {
    commands.spawn((PlayerBundle {
        account: PlayerAccountResource {
            handle: 0,
            address: "0x7d873FbFE8e16f5F55740a52a356c2f52c613cdF".to_string(),
            balance: 0,
        },
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-2., 0., 0.)),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    },));
    commands.spawn((PlayerBundle {
        account: PlayerAccountResource {
            handle: 1,
            address: "0x7f93B033D18dcA9fD2BA4CbF2bf73A2DF840756c".to_string(),
            balance: 0,
        },
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-2., 0., 0.)),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
    },));
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    commands.spawn(Camera2dBundle::default());
    spawn_players(commands);
}

fn bevy_bridge() {}

fn lobby_startup(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
    spawn_players(commands);
}

pub struct BevyChainInspectorPlugin;
impl Plugin for BevyChainInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, bevy_init);
        app.add_systems(Update, bevy_bridge);
    }
}
// async fn exex<Node: FullNodeComponents>(mut ctx: ExExContext<Node>, world: ) -> eyre::Result<()>
// {     unimplemented!()
// }
//

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { prevent_default_event_handling: false, ..default() }),
            ..Default::default()
        }))
        // .add_plugins(DefaultPlugins.set(LogPlugin {
        // filter: "info,wgpu_core=warn,wgpu_hal=warn,".into(),
        // level: bevy::log::Level::DEBUG,
        // ..default()
        // }))
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Lobby), (lobby_startup,))
        .add_plugins(BevyChainInspectorPlugin)
        .run();
    Ok(())
}

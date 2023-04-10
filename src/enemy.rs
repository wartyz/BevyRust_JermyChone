use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::{Rng, thread_rng};
use crate::{ActiveEnemies, Enemy, ENEMY_SPRITE, SCALE, WIN_SIZE_HEIGHT, WIN_SIZE_WIDTH};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_spawn.run_if(on_timer(Duration::from_secs_f32(1.))));
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    //time: Res<Time>,
    //game_textures: Res<GameTextures>,
    //win_size: Res<WinSize>,

    asset_server: Res<AssetServer>,
    //mut windows: Query<&Window, With<PrimaryWindow>>,
) {
    if active_enemies.0 < 1 {
        // calcular la posición aleatória
        let mut rng = thread_rng();
        let w_span = WIN_SIZE_WIDTH / 2. - 100.;
        let h_span = WIN_SIZE_HEIGHT / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        // spawn enemigo

        commands.spawn(SpriteBundle {
            // sprite: Sprite {
            //     color: Color::rgb(1., 0.7, 0.7),
            //     custom_size: Some(Vec2::new(200.0, 100.0)),
            //     ..default()
            // },
            texture: asset_server.load(ENEMY_SPRITE),
            //texture: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..default()
            },
            ..default()
        })
            .insert(Enemy);
        active_enemies.0 += 1;
    }
}
use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use crate::enemy::formation::FormationMaker;
use crate::{ENEMY_MAX, ENEMY_SIZE, EnemyCount, GameTextures, SPRITE_SCALE, WinSize};
use crate::components::{Enemy, SpriteSize};

mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FormationMaker::default())

            .add_system(enemy_spawn_system.run_if(on_timer(Duration::from_secs_f32(1.))));
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    mut formation_maker: ResMut<FormationMaker>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        // get formation and start x/y
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;

        commands
            .spawn(SpriteBundle {
                texture: game_textures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(formation)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

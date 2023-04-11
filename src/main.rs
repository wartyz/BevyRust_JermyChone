// https://www.youtube.com/watch?v=Yb3vInxzKGE&t=445s

mod player;
mod enemy;
mod components;

//use std::collections::HashSet;
use std::default::Default;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::prelude::KeyCode::Sysrq;

use bevy::window::PrimaryWindow;
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use components::SpriteSize;
//use bevy::sprite::collide_aabb::collide;
use bevy::math::Vec3Swizzles;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::HashSet;
use crate::components::ExplosionTimer;

//use crate::components::{Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
//use crate::_enemy::EnemyPlugin;
//use crate::player::PlayerPlugin;

// region:    --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

const SCALE: f32 = 0.5;

// endregion: --- Asset Constants

// region:    --- Game Constants

const TIME_STEP: f32 = 1. / 60.;
const WIN_SIZE_WIDTH: f32 = 598.0;
const WIN_SIZE_HEIGHT: f32 = 676.0;
const BASE_SPEED: f32 = 500.;

const PLAYER_RESPAWN_DELAY: f64 = 2.;
const ENEMY_MAX: u32 = 2;
const MAX_FORMATION_MEMBERS: u32 = 2;

// endregion: --- Game Constants

pub const APP_TITLE: &str = "Rust Invaders!";

// region:    --- Resources
#[derive(Resource, Default)]
pub struct Materials {
    pub player_materials: Handle<Image>,
}
//texture: asset_server.load(PLAYER_SPRITE),

// #[derive(Resource)]
// pub struct WinSize {
//     pub w: f32,
//     pub h: f32,
// }
//
// impl Default for WinSize {
//     fn default() -> WinSize {
//         WinSize {
//             w: 800.,
//             h: 600.,
//         }
//     }
// }

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    _enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

// #[derive(Resource)]
// struct EnemyCount(u32);

#[derive(Resource)]
struct PlayerState {
    on: bool,
    // alive
    last_shot: f64, // -1 if not shot
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
        }
    }
}

impl PlayerState {
    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }
    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}

#[derive(Resource)]
struct ActiveEnemies(u32);

impl Default for ActiveEnemies {
    fn default() -> Self {
        Self(0)
    }
}

// endregion: --- Resources

// region: Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct FromPlayer;

#[derive(Component)]
struct PlayerReadyFire(bool);

#[derive(Component)]
struct Laser;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct FromEnemy;

#[derive(Component)]
struct Explosion;

#[derive(Component)]
struct ExplosionToSpawn(Vec3);

#[derive(Component)]
struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}
// endregion: Components

fn main() {
    let window = Window
    {
        title: APP_TITLE.to_string(),
        resolution: (WIN_SIZE_WIDTH, WIN_SIZE_HEIGHT).into(),
        position: WindowPosition::new(IVec2::new(100, 100)),
        resizable: false,
        ..default()
    };

    let primary_window = Some(window);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { primary_window, ..default() }))

        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .init_resource::<ActiveEnemies>()

        .add_startup_system(setup)

        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(explosion_to_spawn)

//         .add_system(movable_system)
        .add_system(player_laser_hit_enemy)
        .add_system(enemy_laser_hit_player)
        .add_system(animate_explosion)
//
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: Query<&Window, With<PrimaryWindow>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // Creamos recursos
    commands.insert_resource(Materials {
        player_materials: asset_server.load(PLAYER_SPRITE),
    });
    //window.set_position(IVec2::new(3870, 4830));

    // capture window size
    let Ok(window) = windows.get_single() else {
        return;
    };

    //let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
//
    // position window (for tutorial)
    //let mut window = windows.get_single().unwrap();

    //window.set_position(IVec2::new(2780, 4900));

    // spawn a sprite
    /* commands
         .spawn(SpriteBundle {
             //material: materials.add(Color::rgb(1., 0.7, 0.7).into()),
             //sprite: Sprite::new(Vec2::new(200.0, 100.))
             ..Default::default()
         });*/

//
    // add WinSize resource
    //let win_size = WinSize { w: win_w, h: win_h };
    //commands.insert_resource(win_size);
//
    // create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        _enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion,
    };
    commands.insert_resource(game_textures);
    //commands.insert_resource(EnemyCount(0));
}

// fn movable_system(
//     mut commands: Commands,
//     win_size: Res<WinSize>,
//     mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
// ) {
//     for (entity, velocity, mut transform, movable) in query.iter_mut() {
//         let translation = &mut transform.translation;
//         translation.x += velocity.x * TIME_STEP * BASE_SPEED;
//         translation.y += velocity.y * TIME_STEP * BASE_SPEED;
//
//         if movable.auto_despawn {
//             // despawn when out of screen
//             const MARGIN: f32 = 200.;
//             if translation.y > win_size.h / 2. + MARGIN
//                 || translation.y < -win_size.h / 2. - MARGIN
//                 || translation.x > win_size.w / 2. + MARGIN
//                 || translation.x < -win_size.w / 2. - MARGIN
//             {
//                 commands.entity(entity).despawn();
//             }
//         }
//     }
// }

fn player_laser_hit_enemy(
    mut commands: Commands,
    //mut enemy_count: ResMut<EnemyCount>,
    mut laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    mut enemy_query: Query<(Entity, &Transform, &SpriteSize, With<Enemy>)>,
    mut active_enemies: ResMut<ActiveEnemies>,
) {
    let mut enemies_blasted: HashSet<Entity> = HashSet::new();
//
//     // iterate through the lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter_mut() {
//         if despawned_entities.contains(&laser_entity) {
//             continue;
//         }

        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate through the enemies
        for (enemy_entity, enemy_tf, enemy_size, _) in enemy_query.iter_mut() {
//             if despawned_entities.contains(&enemy_entity)
//                 || despawned_entities.contains(&laser_entity)
//             {
//                 continue;
//             }
//
            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // perform collision
            if let Some(_) = collision {
                if enemies_blasted.get(&enemy_entity).is_none() {

                    // remove the enemy
                    commands.entity(enemy_entity).despawn();
                    active_enemies.0 -= 1;

                    // spawn the explosionToSpawn
                    commands
                        .spawn(ExplosionToSpawn(enemy_tf.translation.clone()));

                    enemies_blasted.insert(enemy_entity);
                }

                // remove the laser
                commands.entity(laser_entity).despawn();
                //despawned_entities.insert(laser_entity);
            }
        }
    }
}

fn enemy_laser_hit_player(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
    if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
        let player_scale = Vec2::from(player_tf.scale.xy());
        // Para cada laser de enemigo
        for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from(laser_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );

            // perform the collision
            if let Some(_) = collision {
                // borrar el player
                commands.entity(player_entity).despawn();
                player_state.shot(time.elapsed_seconds_f64());

                // borrar el laser
                commands.entity(laser_entity).despawn();

                // crear explosionToSpawn
                commands.spawn(ExplosionToSpawn(player_tf.translation.clone()));

                break;
            }
        }
    }
}

fn explosion_to_spawn(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &ExplosionToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
//         // spawn the explosion sprite


        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: game_textures.explosion.clone(),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Explosion)
            .insert(ExplosionTimer::default());

        // despawn the explosionToSpawn
        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut ExplosionTimer,
        &mut TextureAtlasSprite),
        //&Handle<TextureAtlas>,
        With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= EXPLOSION_LEN {
                commands.entity(entity).despawn()
            }
        }
    }
}

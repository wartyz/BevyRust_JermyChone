use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use crate::{Laser, LASER_SPRITE, Materials, Player, PLAYER_SPRITE, PlayerReadyFire, Speed, TIME_STEP, WIN_SIZE_HEIGHT};

//use crate::{GameTextures, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE, PlayerState, SPRITE_SCALE, WinSize};
//use crate::components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(PlayerState::default())
        //app.add_system(player_spawn_system.run_if(on_timer(Duration::from_secs_f32(0.5))))
        app.init_resource::<Materials>()

            .add_startup_system(player_spawn_system)
            .add_system(player_movement)
            //app.add_startup_stage("game_setup_actors", SystemStage::single(player_spawn_system))
            //.add_system(player_keyboard_event_system)
            .add_system(player_fire)
            .add_system(laser_movement);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    //mut player_state: ResMut<PlayerState>,
    //time: Res<Time>,
    //game_textures: Res<GameTextures>,
    //win_size: Res<WinSize>,
    materials: Res<Materials>,
    asset_server: Res<AssetServer>,
    //mut windows: Query<&Window, With<PrimaryWindow>>,
) {
    //let mut window = windows.get_single().unwrap();

    // spawn un sprite
    let bottom = -WIN_SIZE_HEIGHT / 2.;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 0.7, 0.7),
            custom_size: Some(Vec2::new(200.0, 100.0)),
            ..default()
        },
        texture: asset_server.load(PLAYER_SPRITE),
        //texture: materials.player_materials.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
            scale: Vec3::new(0.5, 0.5, 1.),
            ..default()
        },
        ..default()
    })
        .insert(Player)
        .insert(PlayerReadyFire(true))
        .insert(Speed::default());
    // let now = time.elapsed_seconds_f64();
    // let last_shot = player_state.last_shot;
    //
    // if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
    //     // add player
    //     let bottom = -win_size.h / 2.;
    //     commands
    //         .spawn(SpriteBundle {
    //             texture: game_textures.player.clone(),
    //             transform: Transform {
    //                 translation: Vec3::new(
    //                     0.,
    //                     bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.,
    //                     10.,
    //                 ),
    //                 scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         })
    //         .insert(Player)
    //         .insert(SpriteSize::from(PLAYER_SIZE))
    //         .insert(Movable { auto_despawn: false })
    //         .insert(Velocity { x: 0., y: 0. });
    //
    //     player_state.spawned();
    // }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform, With<Player>)>,
) {
    // En video es un Result

    for (speed, mut transform, _) in query.iter_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

// fn player_fire_system(
//     mut commands: Commands,
//     kb: Res<Input<KeyCode>>,
//     game_textures: Res<GameTextures>,
//     query: Query<&Transform, With<Player>>,
// ) {
//     if let Ok(player_tf) = query.get_single() {
//         if kb.just_pressed(KeyCode::Space) {
//             let (x, y) = (player_tf.translation.x, player_tf.translation.y);
//             let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;
//
//             let mut spawn_laser = |x_offset: f32| {
//                 commands
//                     .spawn(SpriteBundle {
//                         texture: game_textures.player_laser.clone(),
//                         transform: Transform {
//                             translation: Vec3::new(x + x_offset, y + 15., 0.),
//                             scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     })
//                     .insert(Laser)
//                     .insert(FromPlayer)
//                     .insert(SpriteSize::from(PLAYER_LASER_SIZE))
//                     .insert(Movable { auto_despawn: true })
//                     .insert(Velocity { x: 0., y: 1. });
//             };
//
//             spawn_laser(x_offset);
//             spawn_laser(-x_offset);
//         }
//     }
// }
//
// fn player_keyboard_event_system(
//     kb: Res<Input<KeyCode>>,
//     mut query: Query<&mut Velocity, With<Player>>,
// ) {
//     if let Ok(mut velocity) = query.get_single_mut() {
//         velocity.x = if kb.pressed(KeyCode::Left) {
//             -1.
//         } else if kb.pressed(KeyCode::Right) {
//             1.
//         } else {
//             0.
//         }
//     }
// }

fn player_fire(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    //game_textures: Res<GameTextures>,
    mut query: Query<(&Transform, &mut PlayerReadyFire, With<Player>)>,
    asset_server: Res<AssetServer>,
) {
    for (player_tf, mut ready_fire, _) in query.iter_mut() {
        if ready_fire.0 && kb.pressed(KeyCode::Space) {
            let x = player_tf.translation.x;
            let y = player_tf.translation.y;
            commands.spawn(SpriteBundle {
                texture: asset_server.load(LASER_SPRITE),
                transform: Transform {
                    translation: Vec3::new(x, y + 15., 0.),
                    ..default()
                },
                ..default()
            })
                .insert(Laser)
                .insert(Speed::default());
            ready_fire.0 = false;
        }

        if kb.just_released(KeyCode::Space) {
            ready_fire.0 = true;
        }
    }
}

fn laser_movement(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    //game_textures: Res<GameTextures>,
    mut query: Query<(Entity, &Speed, &mut Transform, With<Laser>)>,
    asset_server: Res<AssetServer>,
) {
    for (laser_entity, speed, mut laser_tf, _) in query.iter_mut() {
        let translation = &mut laser_tf.translation;
        translation.y += speed.0 * TIME_STEP;
        if translation.y > WIN_SIZE_HEIGHT {
            commands.entity(laser_entity).despawn();
        }
    }
}
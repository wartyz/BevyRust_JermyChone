use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::{Rng, thread_rng};
use crate::{ActiveEnemies, Enemy, ENEMY_SPRITE, SCALE, WIN_SIZE_HEIGHT, WIN_SIZE_WIDTH, ENEMY_SIZE, ENEMY_MAX, ENEMY_LASER_SPRITE, Laser, FromEnemy, ENEMY_LASER_SIZE, Speed, TIME_STEP, BASE_SPEED};
use crate::MAX_FORMATION_MEMBERS;
use crate::components::{Movable, SpriteSize, Velocity};

// Región Formation *****************
/// Component - Enemy Formation (per _enemy)
#[derive(Clone, Component, Default)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub offset: (f32, f32),
    //pub pivot: (f32, f32),
    //pub speed: f32,
    pub angle: f32,
    // change per tick
    group_id: u32,
}

/// Resource - Formation Maker
#[derive(Default, Resource)]
pub struct FormationMaker {
    group_seq: u32,
    current_formation: Option<Formation>,
    current_formation_members: u32,

}

/// Formation factory implementation
impl FormationMaker {
    pub fn make(&mut self) -> Formation {
        match (&self.current_formation,
               self.current_formation_members >= MAX_FORMATION_MEMBERS) {
            // if has current template and still within max members
            (Some(tmpl), false) => {
                self.current_formation_members += 1;
                tmpl.clone()
            }
            // if first formation or previous formation is full (need to create a new one)
            (None, _) | (_, true) => {
                let mut rng = thread_rng();

                // compute the start x/y
                let h_span = WIN_SIZE_HEIGHT / 2. - 100.;
                let w_span = WIN_SIZE_WIDTH / 4.;

                let x = if rng.gen_bool(0.5) {
                    w_span
                } else {
                    -w_span
                };
                let y = rng.gen_range(-h_span..h_span) as f32;
                let start = (x, y);

                // Calcula el offset y radio
                let offset = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));
                let radius = (rng.gen_range(80.0..150.), 100.);
                let angle = (y - offset.0).atan2(x - offset.1);

                // create the formation
                self.group_seq += 1;
                let group_id = self.group_seq;

                let formation = Formation {
                    start,
                    offset,
                    radius,
                    angle,
                    group_id,
                };

                // store as template
                self.current_formation = Some(formation.clone());
                // reset members to 1
                self.current_formation_members = 1;

                formation
            }
        }
    }
}

// Fin Región Formation ************

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FormationMaker>()
            .add_system(enemy_spawn.run_if(on_timer(Duration::from_secs_f32(1.))))
            .add_system(enemy_fire.run_if(on_timer(Duration::from_secs_f32(0.9))))
            .add_system(enemy_laser_movement)
            .add_system(enemy_movement);
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    mut formation_maker: ResMut<FormationMaker>,
    //time: Res<Time>,
    //game_textures: Res<GameTextures>,
    //win_size: Res<WinSize>,

    asset_server: Res<AssetServer>,
    //mut windows: Query<&Window, With<PrimaryWindow>>,
) {
    if active_enemies.0 < ENEMY_MAX {
        /*// calcular la posición aleatória
        let mut rng = thread_rng();
        let w_span = WIN_SIZE_WIDTH / 2. - 100.;
        let h_span = WIN_SIZE_HEIGHT / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;*/

        // Obtiene la formación e inicia x/y
        let formation = formation_maker.make();
        let (x, y) = formation.start;

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
            .insert(Enemy)
            .insert(Speed::default())
            .insert(SpriteSize::from(ENEMY_SIZE))
            .insert(formation);
        active_enemies.0 += 1;
    }
}

fn enemy_fire(
    mut commands: Commands,
    //game_textures: Res<GameTextures>,
    asset_server: Res<AssetServer>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    // Para cada enemigo dispara laser
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        // spawn enemy laser sprite
        commands
            .spawn(SpriteBundle {
                //texture: game_textures.enemy_laser.clone(),
                texture: asset_server.load(ENEMY_LASER_SPRITE),
                transform: Transform {
                    translation: Vec3::new(x, y - 15., 0.),
                    //rotation: Quat::from_rotation_x(PI),
                    scale: Vec3::new(SCALE, -SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))
            .insert(FromEnemy)
            //.insert(Movable { auto_despawn: true })
            //.insert(Velocity { x: 0., y: -1. });
            .insert(Speed::default());
    }
}

fn enemy_laser_movement(
    mut commands: Commands,
    //game_textures: Res<GameTextures>,
    asset_server: Res<AssetServer>,
    mut laser_query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromEnemy>)>,
) {
    // Para cada laser del enemigo
    for (entity, speed, mut tf) in laser_query.iter_mut() {
        tf.translation.y -= speed.0 * TIME_STEP;
        if tf.translation.y < -WIN_SIZE_HEIGHT / 2. - 50. {
            commands.entity(entity).despawn();
        }
    }
}

fn enemy_movement(
    //time: Res<Time>,
    mut commands: Commands,
    //game_textures: Res<GameTextures>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Transform, &Speed, &mut Formation), With<Enemy>>,
) {
    //let now = time.elapsed_seconds();

    // Por cada enemigo
    for (mut tf, speed, mut formation) in query.iter_mut() {
        // current position
        let (x_org, y_org) = (tf.translation.x, tf.translation.y);

        // max distance
        let max_distance = TIME_STEP * speed.0;

        // Obtiene la elipse
        let (x_offset, y_offset) = formation.offset;
        let (x_radius, y_radius) = formation.radius;

        // // 1 for counter clockwise, -1 clockwise
        // let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. };
        // let (x_pivot, y_pivot) = formation.pivot;
        // let (x_radius, y_radius) = formation.radius;

        // Calcula el siguiente ángulo (basado en el tiempo)
        let dir = if formation.start.0 > 0. { 1. } else { -1. };
        let angle = formation.angle + dir * speed.0 * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);

        // Calcula el destino
        let x_dst = x_radius * angle.cos() + x_offset;
        let y_dst = y_radius * angle.sin() + y_offset;

        // let angle = formation.angle
        //     + dir * formation.speed * TIME_STEP / (x_radius.min(y_radius) * PI / 2.);
        //
        // // compute target x/y
        // let x_dst = x_radius * angle.cos() + x_pivot;
        // let y_dst = y_radius * angle.sin() + y_pivot;
        //
        // Calcula la distancia
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance == 0. {
            0.
        } else {
            max_distance / distance
        };
        //
        // Calculamos el final x/y
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) };
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        // Comience a rotar el ángulo de formación solo cuando el sprite esté en o cerca de la elipse
        if distance < max_distance * speed.0 / 20. {
            formation.angle = angle;
        }

        // Aplicar transformación
        tf.translation.x = x;
        tf.translation.y = y;

        // let translation = &mut transform.translation;
        // (translation.x, translation.y) = (x, y);
    }
}
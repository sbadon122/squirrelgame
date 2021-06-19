use bevy::prelude::*;

// This resource provides rules for our "game".
struct Player {
    x_velocity: f32,
    y_velocity: f32,
    flip_sprite: bool,
    max_speed: f32,
    acceleration: f32,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Squirrel".to_string(),
            width: 600.,
            height: 800.,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(keyboard_input_system.system())
        .run();
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>,mut  player: ResMut<Player>) {
   
    if keyboard_input.pressed(KeyCode::A) {

        if player.x_velocity > player.max_speed*-1.0 
        {
            player.x_velocity -= player.acceleration;
        }
        
        player.flip_sprite = true;
    }

    if keyboard_input.just_released(KeyCode::A) {
        player.x_velocity = 0.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        if player.x_velocity < player.max_speed
        {
            player.x_velocity += player.acceleration;
        }
      
        player.flip_sprite = false;
      
    }

    if keyboard_input.just_released(KeyCode::D) {
    
        player.x_velocity = 0.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        if player.y_velocity < player.max_speed
        {
            player.y_velocity += player.acceleration;
        }
    }

    if keyboard_input.just_released(KeyCode::W) {

        player.y_velocity = 0.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        if player.y_velocity > player.max_speed * -1.0
        {
            player.y_velocity -= player.acceleration;
        }
    
    }

    if keyboard_input.just_released(KeyCode::S) {
        player.y_velocity = 0.0;
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, &mut Transform)>,
    player: Res<Player>,
) {
    //12-19
   
    for (mut timer, mut sprite, texture_atlas_handle, mut transform) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.flip_x = player.flip_sprite;
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            //Idle Animation
            if  player.x_velocity == 0.0 && player.y_velocity == 0.0
            {
                if sprite.index < 8 || sprite.index+1 > 12
                {
                    sprite.index=8;
                
                }
               
            }
            else {
                if sprite.index < 17 || sprite.index+1 > 25 
                {
                    sprite.index=17;
                   
                }
                let translation_x = transform.translation.x + player.x_velocity;
                let translation_y = transform.translation.y + player.y_velocity;
                if translation_x > -300.0 && translation_x < 300.0
                {
                    transform.translation.x = translation_x;
                }

                if translation_y > -350.0 && translation_y < 425.0
                {
                    transform.translation.y = translation_y;
                }
                println!("{}", transform.translation.y);
                
                
            }
        
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>,>
) {
    let texture_handle = asset_server.load("squirrel_sprite_sheet.png");

    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true));

    commands.insert_resource(Player {
        x_velocity: 0.0,
        y_velocity: 0.0,
        flip_sprite: false,
        max_speed: 20.0,
        acceleration: 2.5,
    })
}
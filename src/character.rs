use bevy::prelude::*;

#[derive(Component)]
struct Character;

pub struct CharacterPlugin;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(Update, animate_character);
    }
}

fn spawn_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("char_idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(96), 9, 1, None, None);
    let texture_atlas = texture_atlas.add(layout);
    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas,
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Character,
    ));
}

fn animate_character(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Character>>,
) {
    for (mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == 8 { 0 } else { atlas.index + 1 };
        }
    }
}

fn move_character(query: Query<&mut Transform, With<Character>>, time: Res<Time>) {}

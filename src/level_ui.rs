use bevy::{color::palettes::css, prelude::*};

use crate::{player::Stamina, storyteller::LevelTimer, GameSet, GameStuff};

pub struct LevelUiPlugin;

impl Plugin for LevelUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateLevelUi>()
            .add_systems(Update, create_level_ui_system)
            .add_systems(Update, show_stamina.in_set(GameSet::Playing));
    }
}

#[derive(Event)]
pub struct CreateLevelUi;

#[derive(Component)]
pub struct LevelUi;

#[derive(Component)]
pub struct TaskText;

#[derive(Component)]
pub struct StaminaState;

fn create_level_ui_system(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut ev_create_level_ui: EventReader<CreateLevelUi>,
) {
    if ev_create_level_ui.is_empty() {
        return;
    }

    let text_font = TextFont {
        font_size: 24.0,
        ..default()
    };

    //Spawn top info bar
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(60.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        min_height: Val::Px(80.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                    LevelUi,
                    GameStuff,
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new(""), text_font.clone(), LevelUi, LevelTimer));

                    parent.spawn((Text::new(""), text_font.clone(), LevelUi, TaskText));

                    spawn_bar(parent);
                });
        });

    ev_create_level_ui.clear();
}

fn spawn_bar(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            height: Val::Px(10.0),
            width: Val::Px(200.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((Text::new("Stamina"), TextFont::default()));

            parent
                .spawn((
                    Node {
                        width: Val::Px(100.),
                        height: Val::Px(10.),
                        padding: UiRect::all(Val::Px(1.)),
                        align_items: AlignItems::Stretch,
                        top: Val::Px(2.0),
                        left: Val::Px(6.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Percent(50.0),
                            ..Default::default()
                        },
                        BackgroundColor(css::GREEN.into()),
                        StaminaState,
                    ));
                });
        });
}

fn show_stamina(
    mut query: Query<(&mut Node, &mut BackgroundColor), With<StaminaState>>,
    staminas: Query<&Stamina>,
) {
    let Ok(stamina) = staminas.single() else {
        warn!("Stamina not found");
        return;
    };

    let Ok((mut node, mut background_color)) = query.single_mut() else {
        warn!("Stamina ui not found");
        return;
    };

    node.width = Val::Percent(stamina.value * 100.0);

    if stamina.blocked {
        background_color.0 = css::ORANGE_RED.into();
    } else {
        background_color.0 = css::GREEN.into();
    }
}

use bevy::{color::palettes::css, prelude::*};

use crate::{GameSet, GameState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_main_menu);
        app.add_systems(OnExit(GameState::Menu), clear_menu);
        app.add_systems(Update, button_system.in_set(GameSet::Menu));
    }
}

#[derive(Component)]
pub struct MainMenu;

fn setup_main_menu(mut commands: Commands) {
    let text_font = TextFont {
        font_size: 24.0,
        ..default()
    };
    commands
        .spawn((
            MainMenu,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("You're a shepherd's dog. Your master is a vampire. Don't ask why, you're just a dog. \n
Don't let the sheep flock run away and get eaten by wolves. Complete your master's tasks. \n
If you fail, you'll be replaced. Or whatever your bloodhungry master deems to think will be justified punishment. So... Be a good dog. \n
Good luck!\n"),
                TextFont::default(),
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(100.0),
                        height: Val::Px(50.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::WHITE),
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start"),
                        text_font.clone(),
                    ));
                });
        });
}

fn clear_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(css::DARK_GRAY.into());
            }
            Interaction::None => {
                *color = BackgroundColor(Color::BLACK);
            }
        }
    }
}

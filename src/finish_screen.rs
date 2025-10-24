use bevy::prelude::*;

use crate::{
    storyteller::{FailReason, Score},
    GameSet, GameState,
};

pub struct FinishScreenPlugin;

impl Plugin for FinishScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Finish), setup_finish_screen);

        app.add_systems(OnExit(GameState::Finish), cleanup_finish_screen);
        app.add_systems(Update, finish_screen_system.in_set(GameSet::Finish));
    }
}

#[derive(Component)]
struct FinishScreen;

fn setup_finish_screen(mut commands: Commands, score: Res<Score>, fail: Option<Res<FailReason>>) {
    let text_font = TextFont {
        font_size: 24.0,
        ..default()
    };

    commands.spawn((
        FinishScreen,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.7)),
    )).with_children(|parent| {
        let text = if let Some(fail) = fail {
            match fail.as_ref() {
                FailReason::SheepDied => format!("Uh-oh. \nWhat a bad dog. Half your sheep have been eaten, you fleay mutt. Await your punishment. \nIf you're alive afterwards, give it a decent try."),
                FailReason::TaskFailed(reason) => format!("Uh-oh. Bad dog. \nYou failed, filthy mutt. \nReason: {} \nPrepare to be punished.", reason),
            }
        } else {
            format!("Good dog! \nYou get to live another day. \nYou did well enough. Your master will be waiting for you tomorrow.")
        };

        parent.spawn((
            Text::new(format!("{} \nScore: {:.1}", text, score.0)),
            TextFont::default(),
        ));

        parent.spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            BorderColor(Color::WHITE),)
        ).with_children(|parent| {
            parent.spawn((
                Text::new("Ok"),
                text_font.clone(),
            ));
            
        });
    });
}

fn cleanup_finish_screen(mut commands: Commands, query: Query<Entity, With<FinishScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn finish_screen_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Menu);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

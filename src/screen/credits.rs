use bevy::prelude::*;

use super::Screen;
use crate::util::ui::*;

pub(super) fn plugin(app: &mut App) {
    // Screen setup and teardown.
    app.add_systems(OnEnter(Screen::Credits), enter_credits)
        .add_systems(OnExit(Screen::Credits), exit_credits);

    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );
}

#[derive(Component, PartialEq, Eq)]
enum CreditsAction {
    Back,
}

fn enter_credits(mut commands: Commands) {
    let list = commands.my_root().insert(StateScoped(Screen::Credits)).id();

    commands.my_label("Alice - Foo").set_parent(list);
    commands.my_label("Bob - Bar").set_parent(list);

    commands
        .my_button("Back")
        .insert(CreditsAction::Back)
        .set_parent(list);
}

fn exit_credits() {}

fn handle_credits_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: ButtonInteractionQuery<CreditsAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}

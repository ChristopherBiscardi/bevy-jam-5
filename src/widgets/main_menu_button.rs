use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_mod_picking::prelude::*;
use std::time::Duration;
use woodpecker_ui::prelude::*;

use crate::{
    assets::{AudioAssets, FontVelloAssets},
    states::AppState,
    widgets::*,
};
// use woodpecker_ui_macros::Widget;

pub fn main_menu_interaction(
    mut interactions: Query<(Entity, &PickingInteraction)>,
    children: Query<&Children>,
    mut colors: Query<
        &mut WoodpeckerStyle,
        With<PickingInteractionSubscriber>,
    >,
    transitions: Query<&TransitionTimer>,
) {
    for (entity, interaction) in &mut interactions {
        let all_timers_finished = children
            .iter_descendants(entity)
            .filter_map(|entity| {
                transitions.get(entity).ok()
            })
            // note: all will return true if filter_map
            // filters *all* items out,
            // resulting in an empty iterator
            .all(|transition| {
                return transition
                    .timeouts
                    .iter()
                    .all(|timer| timer.finished());
            });
        if !all_timers_finished {
            continue;
        }
        let interaction_color: Color = match interaction {
            PickingInteraction::Pressed => SKY_200.into(),
            PickingInteraction::Hovered => SKY_200.into(),
            PickingInteraction::None => Color::WHITE,
        };

        let color_updates = children
            .iter_descendants(entity)
            .filter(|entity| colors.get(*entity).is_ok())
            .collect::<Vec<Entity>>();
        for entity in color_updates {
            let mut style = colors.get_mut(entity).unwrap();
            let mut new_style = *style;
            new_style.background_color = interaction_color;
            *style = new_style;
        }
    }
}

#[derive(Component, Clone)]
pub struct PickingInteractionSubscriber;

// We can derive widget here and pass in our
// systems passing in the widget_systems is
// optional and if we don't pass them in we need
// to call `app.add_widget_systems`!
#[derive(Component, Widget, Clone, Reflect)]
#[widget_systems(update, render)]
pub struct MainMenuButtonWidget {
    pub content: String,
    // TODO: can we measure the inner text to calculate
    // this width?
    pub width: Units,
    /// Offset the start of the animation chain
    /// in milliseconds
    pub offset: u64,
}

impl Default for MainMenuButtonWidget {
    fn default() -> Self {
        Self {
            content: "A button".to_string(),
            width: Units::Pixels(300.),
            offset: 0,
        }
    }
}

#[derive(Debug, Component, Clone)]
struct State {
    color_1_timer: Timer,
    color_2_timer: Timer,
}

impl Default for State {
    fn default() -> Self {
        Self {
            color_1_timer: Timer::new(
                Duration::from_millis(400),
                TimerMode::Once,
            ),
            color_2_timer: Timer::new(
                Duration::from_millis(400),
                TimerMode::Once,
            ),
        }
    }
}

#[derive(Bundle, Default, Clone)]
pub struct MainMenuButtonWidgetBundle {
    pub props: MainMenuButtonWidget,
    pub style: WoodpeckerStyle,
}

pub fn update(
    entity: Res<CurrentWidget>,
    mut widgets: Query<&mut MainMenuButtonWidget>,
    mut states: Query<&mut State>,
    time: Res<Time>,
    mut hooks: ResMut<HookHelper>,
    children: Query<&Children>,
) -> bool {
    let Ok(props) = widgets.get_mut(**entity) else {
        warn!(
            "MainMenuButtonWidget not available in update query"
        );
        return false;
    };

    // let Some(state_entity) =
    //     hooks.get_state::<State>(*entity)
    // else {
    //     // state doesn't exist on first render, so
    //     // all we care about is props so that we
    //     // get a first render
    //     return props.is_changed();
    // };

    // let Ok(mut state) =
    // states.get_mut(state_entity) else {
    //     warn!(
    //         "MainMenuButtonWidget::state not
    // available in update query"
    //     );
    //     return false;
    // };

    // if !state.color_1_timer.finished() {
    //     if state
    //         .color_1_timer
    //         .tick(time.delta())
    //         .just_finished()
    //     {
    //         return true;
    //     }
    // } else if !state.color_2_timer.finished() {
    //     if state
    //         .color_2_timer
    //         .tick(time.delta())
    //         .just_finished()
    //     {
    //         return true;
    //     }
    // }

    // let Ok(interaction_color) =
    //     interaction.get_mut(**entity)
    // else {
    //     return props.is_changed();
    // };

    props.is_changed() //|| state.is_changed()
}

pub fn render(
    mut commands: Commands,
    entity: Res<CurrentWidget>,
    fonts: Res<FontVelloAssets>,
    widgets: Query<&MainMenuButtonWidget>,
    mut states: Query<&mut State>,
    mut hooks: ResMut<HookHelper>,
    audio: Res<Audio>,
    audios: Res<AudioAssets>,
) {
    audio.play(audios.data_show.clone());
    // let state_entity = hooks.use_state(
    //     &mut commands,
    //     *entity,
    //     State::default(),
    // );
    let Ok(props) = widgets.get(**entity) else {
        warn!(
            "MainMenuButtonWidget not available in render query"
        );
        return;
    };
    // let Ok(mut state) =
    // states.get_mut(state_entity) else {
    //     warn!(
    //         "MainMenuButtonWidget State not
    // available in render query"
    //     );
    //     return;
    // };

    let mut inner_container_children =
        WidgetChildren::default();

    inner_container_children.add::<Element>((
            Name::new("Card"),
            PickingInteractionSubscriber,
            ElementBundle::default(),
            WidgetRender::Quad,
            TransitionTimer {
                easing: timer_transition::TransitionEasing::QuinticOut,
                start: Timer::new(
                    Duration::from_millis(props.offset + 300),
                    TimerMode::Once,
                ),
                timeouts: vec![Timer::new(
                    Duration::from_millis(200),
                    TimerMode::Once,
                )],
                looping: false,
                styles: vec![WoodpeckerStyle {
                    position: WidgetPosition::Absolute,
                    background_color: Color::WHITE.into(),
                    width: Units::Pixels(0.),
                    height: Units::Pixels(60.),
                    ..default()
                },
                WoodpeckerStyle {
                    position: WidgetPosition::Absolute,
                    background_color: Color::WHITE.into(),
                    width: props.width,
                    height: Units::Pixels(60.),
                    border: Edge::all(1.),
                    border_color: SLATE_300.into(),
                    ..default()
                }],
                ..default()
            }
        ));

    // inner_container_children.add::<Element>((
    //     Name::new("Card"),
    //     ElementBundle::default(),
    //     WidgetRender::Quad,
    //     TransitionTimer {
    //         easing:
    // timer_transition::TransitionEasing::QuinticOut,
    //         start: Timer::new(
    //             Duration::from_millis(300),
    //             TimerMode::Once,
    //         ),
    //         timeouts: vec![Timer::new(
    //             Duration::from_millis(200),
    //             TimerMode::Once,
    //         )],
    //         looping: false,
    //         styles: vec![WoodpeckerStyle {
    //             position: WidgetPosition::Absolute,
    //             background_color:
    // Color::WHITE.into(),             width:
    // Units::Pixels(0.),             height:
    // Units::Pixels(60.),             ..default()
    //         },
    //         WoodpeckerStyle {
    //             position: WidgetPosition::Absolute,
    //             background_color:
    // Color::WHITE.into(),             width:
    // props.width,             height:
    // Units::Pixels(60.),             ..default()
    //         }],
    //         ..default()
    //     }
    // ));

    inner_container_children.add::<Element>((
        Name::new("Card::text"),
        ElementBundle::default(),
        WidgetRender::Text {
            content: props.content.clone(),
            word_wrap: false,
        },
        TransitionTimer {
            easing: timer_transition::TransitionEasing::QuinticOut,
            start: Timer::new(
                Duration::from_millis(props.offset + 400),
                TimerMode::Once,
            ),
            timeouts: vec![Timer::new(
                Duration::from_millis(100),
                TimerMode::Once,
            )],
            looping: false,
            styles: vec![WoodpeckerStyle {
                margin: Edge::all(10.),
                font_size: 30.0,
                color: SLATE_950.with_alpha(0.).into(),
                font: Some(fonts.outfit_bold.id()),
                ..default()
            },
            WoodpeckerStyle {
                margin: Edge::all(10.),
                font_size: 30.0,
                color: SLATE_950.into(),
                font: Some(fonts.outfit_bold.id()),
                ..default()
            }],
            ..default()
        }
    ));

    inner_container_children.add::<Element>((
        Name::new("Primary Reveal"),
        ElementBundle::default(),
        WidgetRender::Quad,
        TransitionTimer {
            easing: timer_transition::TransitionEasing::QuinticOut,
            start: Timer::new(
                Duration::from_millis(props.offset + 0),
                TimerMode::Once,
            ),
            timeouts: vec![Timer::new(
                Duration::from_millis(200),
                TimerMode::Once,
            ),Timer::new(
                Duration::from_millis(100),
                TimerMode::Once,
            ),Timer::new(
                Duration::from_millis(100),
                TimerMode::Once,
            )],
            looping: false,
            styles: vec![WoodpeckerStyle {
                position: WidgetPosition::Absolute,
                background_color: SKY_400.into(),
                width: Units::Pixels(0.),
                height: Units::Pixels(60.),
                ..default()
            },
            WoodpeckerStyle {
                position: WidgetPosition::Absolute,
                background_color: SKY_400.into(),
                width: props.width,
                height: Units::Pixels(60.),
                ..default()
            },
            WoodpeckerStyle {
                position: WidgetPosition::Absolute,
                background_color: SKY_400.into(),
                width: props.width,
                height: Units::Pixels(60.),
                ..default()
            }, WoodpeckerStyle {
                position: WidgetPosition::Absolute,
                background_color: SKY_400.into(),
                width: Units::Pixels(0.),
                height: Units::Pixels(60.),
                ..default()
            }],
            ..default()
        },
    ));

    inner_container_children.add::<Element>((
        Name::new("Secondary Reveal"),
        ElementBundle::default(),
        WidgetRender::Quad,
        TransitionTimer {
            easing: timer_transition::TransitionEasing::QuinticOut,
            start: Timer::new(
                Duration::from_millis(props.offset + 200),
                TimerMode::Once,
            ),
            timeouts: vec![Timer::new(
                Duration::from_millis(200),
                TimerMode::Once,
            ),Timer::new(
                Duration::from_millis(100),
                TimerMode::Once,
            )],
            looping: false,
            styles: vec![WoodpeckerStyle {
                position: WidgetPosition::Absolute,
                background_color: SLATE_950.into(),
                width: Units::Pixels(0.),
                height: Units::Pixels(60.),
                ..default()
            },
            WoodpeckerStyle {
                position: WidgetPosition::Absolute,
                background_color: SLATE_950.into(),
                width: props.width,
                height: Units::Pixels(60.),
                ..default()
            },
            WoodpeckerStyle {
                position: WidgetPosition::Absolute,
                background_color: SLATE_950.into(),
                width: Units::Pixels(0.),
                height: Units::Pixels(60.),
                ..default()
            }],
            ..default()
        },
    ));

    // // We tell the widget system runner that the
    // children should be processed at this widget.
    inner_container_children.apply(entity.as_parent());

    let mut container_children = WidgetChildren::default();

    container_children.add::<Element>((
        ElementBundle {
            styles: WoodpeckerStyle {
                background_color: BLUE_400.into(),
                width: props.width,
                height: Units::Pixels(60.),

                ..default()
            },
            children: inner_container_children,
            ..default()
        },
        // WidgetRender::Quad,
        PickingInteraction::default(),
        Pickable::default(),
        On::<Pointer<Over>>::run(
            |audio: Res<Audio>,
             audios: Res<AudioAssets>| {
                audio.play(audios.data_short.clone());
                // button.hovering = true;
            },
        ),
        On::<Pointer<Out>>::listener_component_mut::<
            PickingInteraction,
        >(|_, button| {
            // button.hovering = false;
        }),
        On::<Pointer<Click>>::run(
            |audio: Res<Audio>,
             audios: Res<AudioAssets>| {
                audio.play(audios.data_long.clone());
                // button.hovering = true;
            },
        ),
    ));
    container_children.apply(entity.as_parent());

    // Don't forget to add to the entity as a
    // component!
    commands.entity(**entity).insert(container_children);
}

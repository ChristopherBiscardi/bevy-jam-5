use bevy_mod_picking::prelude::*;
use std::time::Duration;

use crate::{widgets::*, AppState};
use bevy::{color::palettes::tailwind::*, prelude::*};
use woodpecker_ui::prelude::*;

use crate::assets::FontVelloAssets;
// use woodpecker_ui_macros::Widget;

pub fn main_menu_interaction(
    mut interactions: Query<(
        Entity,
        &PickingInteraction,
        &mut WoodpeckerStyle,
    )>,
    children: Query<&Children>,
    transitions: Query<&TransitionTimer>,
) {
    for (entity, interaction, mut style) in
        &mut interactions
    {
        let all_timers_finished = children
            .iter_descendants(entity)
            .filter_map(|entity| {
                transitions.get(entity).ok()
            })
            // note: all will return true if filter_map filters
            // *all* items out, resulting in an empty iterator
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
            PickingInteraction::Pressed => RED_400.into(),
            PickingInteraction::Hovered => RED_400.into(),
            PickingInteraction::None => Color::WHITE,
        };
        let mut new_style = style.clone();
        new_style.background_color = interaction_color;
        *style = new_style;
    }
}
// We can derive widget here and pass in our
// systems passing in the widget_systems is
// optional and if we don't pass them in we need
// to call `app.add_widget_systems`!
#[derive(Component, Widget, Clone)]
#[widget_systems(update, render)]
pub struct MainMenuButtonWidget {
    pub content: String,
    // TODO: can we measure the inner text to calculate
    // this width?
    pub width: Units,
}

impl Default for MainMenuButtonWidget {
    fn default() -> Self {
        Self {
            content: "A button".to_string(),
            width: Units::Pixels(300.),
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
    // mut interaction: Query<&mut PickingInteraction>,
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

    // let Ok(mut state) = states.get_mut(state_entity) else {
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
    // interaction: Query<&PickingInteraction>,
) {
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
    // let Ok(mut state) = states.get_mut(state_entity) else {
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
            ElementBundle::default(),
            WidgetRender::Quad,
            TransitionTimer {
                easing: timer_transition::TransitionEasing::QuinticOut,
                start: Timer::new(
                    Duration::from_millis(300),
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
    //         easing: timer_transition::TransitionEasing::QuinticOut,
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
    //             background_color: Color::WHITE.into(),
    //             width: Units::Pixels(0.),
    //             height: Units::Pixels(60.),
    //             ..default()
    //         },
    //         WoodpeckerStyle {
    //             position: WidgetPosition::Absolute,
    //             background_color: Color::WHITE.into(),
    //             width: props.width,
    //             height: Units::Pixels(60.),
    //             ..default()
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
                Duration::from_millis(300),
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
                color: SLATE_950.into(),
                font: Some(fonts.outfit_extra_bold.id()),
                ..Default::default()
            },
            WoodpeckerStyle {
                margin: Edge::all(10.),
                font_size: 30.0,
                color: SLATE_950.into(),
                font: Some(fonts.outfit_extra_bold.id()),
                ..Default::default()
            }],
            ..default()
        }
    ));

    inner_container_children.add::<Element>((
        Name::new("Secondary Reveal"),
        ElementBundle::default(),
        WidgetRender::Quad,
        TransitionTimer {
            easing: timer_transition::TransitionEasing::QuinticOut,
            start: Timer::new(
                Duration::from_millis(0),
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
        Name::new("Primary Reveal"),
        ElementBundle::default(),
        WidgetRender::Quad,
        TransitionTimer {
            easing: timer_transition::TransitionEasing::QuinticOut,
            start: Timer::new(
                Duration::from_millis(200),
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
        On::<Pointer<Over>>::listener_component_mut::<
            PickingInteraction,
        >(|_, button| {
            // button.hovering = true;
            info!("over");
        }),
        On::<Pointer<Out>>::listener_component_mut::<
            PickingInteraction,
        >(|_, button| {
            info!("out");
            // button.hovering = false;
        }),
    ));
    container_children.apply(entity.as_parent());

    // Don't forget to add to the entity as a
    // component!
    commands.entity(**entity).insert(container_children);
}

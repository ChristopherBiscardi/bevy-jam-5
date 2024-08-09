use std::time::{Duration, Instant};

use bevy::{
    color::palettes::tailwind::*, prelude::*,
    render::view::RenderLayers,
};
use bevy_mod_picking::prelude::*;
use woodpecker_ui::prelude::*;

use crate::{
    assets::{FontAssets, FontVelloAssets}, navmesh::SpawnObstacle, spawn_2d_camera, widgets::{self, *}, AppState
};

// TODO: enter pause menu

pub fn spawn_game_menu(
    mut commands: Commands,
    mut ui_context: ResMut<WoodpeckerContext>,
    fonts: Res<FontVelloAssets>,
    mut font_manager: ResMut<FontManager>,
    asset_server: Res<AssetServer>,
) {
    let mut buttons = WidgetChildren::default();
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "Drop Box".to_string(),
                offset: 0,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                commands.trigger(SpawnObstacle);
            },
        ),
    ));

    let root = commands
        .spawn((
            StateScoped(AppState::InGame),
            WoodpeckerAppBundle {
                children: WidgetChildren::default().with_child::<Element>(ElementBundle {
                    styles: WoodpeckerStyle {
                        width: Units::Percentage(100.0),
                        height: Units::Percentage(100.0),
                        justify_content: Some(WidgetAlignContent::FlexStart),
                        align_content: Some(WidgetAlignContent::Center),
                        padding: Edge {
                            left: 0.0.into(),
                            right: 0.0.into(),
                            top: 25.0.into(),
                            bottom: 0.0.into(),
                        },
                        ..default()
                    },
   
                    children: WidgetChildren::default().with_child::<Element>((
                        ElementBundle {
                            styles: WoodpeckerStyle {
                                background_color: Srgba::hex("FF007F").unwrap().into(),
                                width: Units::Pixels(300.),
                                height: Units::Pixels(300.),
                                gap: (Units::Pixels(10.), Units::Pixels(5.)),
                                justify_content: Some(WidgetAlignContent::Center),
                                align_content: Some(WidgetAlignContent::Center),
                                display: WidgetDisplay::Flex,
                                flex_direction: WidgetFlexDirection::Column,
                                position: WidgetPosition::Relative,
                                // flex_wrap: WidgetFlexWrap::Wrap,
                                ..default()
                            },
                            children: buttons,
                            ..default()
                        },
                    ))
                    .with_child::<OptionsModal>(
                        OptionsModalBundle {
                            styles: WoodpeckerStyle {
                                width: Units::Percentage(100.0),
                                justify_content: Some(
                                    WidgetAlignContent::Center,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                    ),
                    ..default()
                }),
                ..default()
            },
        ))
        .id();

    // let mut root_children = WidgetChildren::default();

    ui_context.set_root_widget(root);
}

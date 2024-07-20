use bevy::{
    color::palettes::tailwind::{
        SLATE_400, SLATE_50, SLATE_500,
    },
    prelude::*,
};
use bevy_mod_picking::prelude::*;
use style_helpers::FromLength;
use woodpecker_ui::prelude::*;

use crate::{
    assets::{FontAssets, FontVelloAssets},
    AppState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            spawn_main_menu,
        );
    }
}

const BUTTON_SIZE: f32 = 300.;

fn spawn_main_menu(
    mut commands: Commands,
    mut ui_context: ResMut<WoodpeckerContext>,
    fonts: Res<FontVelloAssets>,
) {
    // let root = commands
    //     .spawn((
    //         WoodpeckerAppBundle {
    //             ..Default::default()
    //         },
    //         WidgetRender::Text {
    //             font: fonts.outfit_extra_bold.clone(),
    //             size: 40.0,
    //             color: Color::WHITE,
    //             alignment: VelloTextAlignment::TopLeft,
    //             content: "Hello World! I am Woodpecker UI!"
    //                 .into(),
    //             word_wrap: false,
    //         },
    //     ))
    //     .id();

    let root = commands
        .spawn((
            WoodpeckerAppBundle {
                ..Default::default()
            },
            StateScoped(AppState::MainMenu),
        ))
        .id();

    let mut root_children = WidgetChildren::default();

    // root_children.add::<Element>(WidgetRender::Text {
    //     font: fonts.outfit_extra_bold.clone(),
    //     size: 40.0,
    //     color: Color::WHITE,
    //     alignment: VelloTextAlignment::TopLeft,
    //     content: "Hello World! I am Woodpecker UI!".into(),
    //     word_wrap: false,
    // });

    let font_size = 20.;
    let button = WButtonBundle {
        button_styles: ButtonStyles {
            normal: (
                SLATE_500.into(),
                WoodpeckerStyle::new()
                    .with_size(Size::from_lengths(
                       200.,50.
                    ))
                    .with_justify_content(Some(
                        taffy::AlignContent::Center,
                    ))
                    .with_align_content(Some(
                        taffy::AlignContent::Center,
                    )),
            ),
            hovered: (
                SLATE_400.into(),
                WoodpeckerStyle::new()
                    .with_size(Size::from_lengths(
                       200.,50.
                    ))
                    .with_justify_content(Some(
                        taffy::AlignContent::Center,
                    ))
                    .with_align_content(Some(
                        taffy::AlignContent::Center,
                    )),
            ),
        },
        children: WidgetChildren::default()
            .with_child::<Element>((
                ElementBundle {
                    styles: WoodpeckerStyle::new()
                        .with_size(Size::from_lengths(
                            180.,17.
                        ))
                        .with_margin(taffy::Rect {
                            left: LengthPercentageAuto::from_length(10.),
                            right: LengthPercentageAuto::from_length(10.),
                            top: LengthPercentageAuto::Auto,
                            bottom: LengthPercentageAuto::Auto,
                        }),
                    ..Default::default()
                },
                WidgetRender::Text {
                    font: fonts
                        .outfit_extra_bold
                        .clone(),
                    size: font_size,
                    color: Color::from(SLATE_50),
                    alignment:
                        VelloTextAlignment::TopLeft,
                    content: "New Game".into(),
                    word_wrap: false,
                },
            )),
        ..Default::default()
    };
    root_children.add::<WButton>((
        button.clone(),
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                info!("clicked");
                // commands.trigger()
            },
        ),
    ));
    root_children.add::<WButton>((
        button.clone(),
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                info!("clicked");
                // commands.trigger()
            },
        ),
    ));
    root_children.add::<WButton>((
        button.clone(),
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                info!("clicked");
                // commands.trigger()
            },
        ),
    ));

    commands.entity(root).insert(root_children);

    ui_context.set_root_widget(root);
}

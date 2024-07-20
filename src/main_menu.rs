use bevy::{
    color::palettes::tailwind::{
        SLATE_400, SLATE_50, SLATE_500,
    },
    prelude::*,
};
use bevy_mod_picking::prelude::*;
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

pub const BUTTON_STYLES: WoodpeckerStyle =
    WoodpeckerStyle {
        background_color: Color::Srgba(SLATE_500),
        width: Units::Pixels(200.),
        height: Units::Pixels(50.),
        justify_content: Some(WidgetAlignContent::Center),
        align_content: Some(WidgetAlignContent::Center),
        ..WoodpeckerStyle::DEFAULT
    };

pub const BUTTON_STYLES_HOVER: WoodpeckerStyle =
    WoodpeckerStyle {
        background_color: Color::Srgba(SLATE_400),
        width: Units::Pixels(200.),
        height: Units::Pixels(50.),
        justify_content: Some(WidgetAlignContent::Center),
        align_content: Some(WidgetAlignContent::Center),
        ..WoodpeckerStyle::DEFAULT
    };

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

    let mut buttons = WidgetChildren::default();

    let button = WButtonBundle {
        button_styles: ButtonStyles {
            normal: BUTTON_STYLES,
            hovered: BUTTON_STYLES_HOVER,
        },
        children: WidgetChildren::default()
            .with_child::<Element>((
                ElementBundle {
                    styles: WoodpeckerStyle {
                        width: Units::Pixels(20.),
                        height: Units::Pixels(20.),
                        margin: Edge {
                            left: (20. / 2.0).into(),
                            top: (20. / 2.0).into(),
                            ..Default::default()
                        },
                        font_size: 20.,
                        color: Color::Srgba(SLATE_50),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                WidgetRender::Text {
                    font: fonts.outfit_extra_bold.clone(),
                    alignment: VelloTextAlignment::TopLeft,
                    content: "New Game".into(),
                    word_wrap: false,
                },
            )),
        ..Default::default()
    };
    buttons.add::<WButton>((
        button.clone(),
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                info!("clicked");
                // commands.trigger()
            },
        ),
        // On::<Pointer<Over>>::listener_component_mut::<
        //     WButton,
        // >(|_, vello_widget| {
        //         vello_widget.hovering = true;
        // }),
        // On::<Pointer<Out>>::listener_component_mut::<
        //     WButton,
        // >(|_, vello_widget| {
        //         vello_widget.hovering = false;
        // }),
    ));
    buttons.add::<WButton>((
        button.clone(),
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                info!("clicked");
                // commands.trigger()
            },
        ),
    ));
    buttons.add::<WButton>((
        button.clone(),
        On::<Pointer<Click>>::run(
            |mut commands: Commands| {
                info!("clicked");
                // commands.trigger()
            },
        ),
    ));

    let root = commands
        .spawn((
            StateScoped(AppState::MainMenu),
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
                        ..Default::default()
                    },
   
                    children: WidgetChildren::default().with_child::<Element>((
                        ElementBundle {
                            styles: WoodpeckerStyle {
                                background_color: Srgba::hex("FF007F").unwrap().into(),
                                border_radius: Corner::all(Units::Pixels(5.0)),
                                width: Units::Pixels(300.),
                                height: Units::Pixels(300.),
                                gap: (Units::Pixels(10.), Units::Pixels(5.)),
                                justify_content: Some(WidgetAlignContent::Center),
                                align_content: Some(WidgetAlignContent::Center),
                                flex_wrap: WidgetFlexWrap::Wrap,
                                ..Default::default()
                            },
                            children: buttons,
                            ..Default::default()
                        },
                        // WidgetRender::Quad {
                        //     // color: Srgba::hex("FF007F").unwrap().into(),
                        //     color: Srgba::lcha(0.,0.,0.,0.),
                        //     border_radius: kurbo::RoundedRectRadii::from_single_radius(5.0),
                        // },
                    )),
                    ..Default::default()
                }),
                ..Default::default()
            },
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

    // commands.entity(root).insert(root_children);

    ui_context.set_root_widget(root);
}

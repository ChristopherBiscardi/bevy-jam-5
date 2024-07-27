use bevy::prelude::*;
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::On,
    DefaultPickingPlugins,
};
use woodpecker_ui::prelude::*;

#[derive(
    Debug,
    Component,
    Widget,
    Clone,
    Default,
    Copy,
    PartialEq,
)]
#[widget_systems(update, render)]
pub struct OptionsModal {
    pub show_modal: bool,
}

pub struct OptionsState {
    pub show_modal: bool,
}

#[derive(Bundle, Default, Clone)]
pub struct OptionsModalBundle {
    pub count: OptionsModal,
    pub styles: WoodpeckerStyle,
    pub children: WidgetChildren,
}

fn update(
    current_widget: Res<CurrentWidget>,
    query: Query<Entity, Changed<OptionsModal>>,
) -> bool {
    query.contains(**current_widget)
}

fn render(
    current_widget: Res<CurrentWidget>,
    mut query: Query<(&OptionsModal, &mut WidgetChildren)>,
) {
    let Ok((my_widget, mut widget_children)) =
        query.get_mut(**current_widget)
    else {
        return;
    };

    info!("made it {}", my_widget.show_modal);

    let my_widget_entity = **current_widget;
    // widget_children.add::<WButton>((
    //     WButtonBundle {
    //         children: WidgetChildren::default()
    //             .with_child::<Element>((
    //                 ElementBundle {
    //                     styles: WoodpeckerStyle {
    //                         font_size: 20.0,
    //                         ..Default::default()
    //                     },
    //                     ..Default::default()
    //                 },
    //                 WidgetRender::Text {
    //                     content: "Open Modal".into(),
    //                     word_wrap: false,
    //                 },
    //             )),
    //         ..Default::default()
    //     },
    //     On::<Pointer<Click>>::run(
    //         move |mut query: Query<&mut OptionsModal>| {
    //             if let Ok(mut my_widget) =
    //                 query.get_mut(my_widget_entity)
    //             {
    //                 my_widget.show_modal = true;
    //             }
    //         },
    //     ),
    // ));

    widget_children.add::<Modal>(ModalBundle {
        modal: Modal {
            visible: my_widget.show_modal,
            title: "I am a modal".into(),
            overlay_alpha: 0.85,
            ..Default::default()
        },
        children: PassedChildren(
            WidgetChildren::default()
                .with_child::<Element>(ElementBundle {
                    styles: WoodpeckerStyle {
                        align_items: Some(WidgetAlignItems::Center),
                        flex_direction: WidgetFlexDirection::Column,
                        padding: Edge::all(10.0),
                        width: Units::Percentage(100.0),
                        ..Default::default()
                    },
                    children: WidgetChildren::default().with_child::<Element>((
                        ElementBundle {
                            styles: WoodpeckerStyle {
                                font_size: 20.0,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        WidgetRender::Text {
                            content:
                                "Hello World! I am Woodpecker UI! This is an example of a modal window!"
                                    .into(),
                            word_wrap: true,
                        },
                    ))
                    .with_child::<WButton>((
                        WButtonBundle {
                            children: WidgetChildren::default().with_child::<Element>((
                                ElementBundle {
                                    styles: WoodpeckerStyle {
                                        width: Units::Percentage(100.0),
                                        font_size: 20.0,
                                        text_alignment: Some(TextAlign::Center),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                WidgetRender::Text {
                                    content: "Close Modal".into(),
                                    word_wrap: true,
                                },
                            )),
                            ..Default::default()
                        },
                        On::<Pointer<Click>>::run(move |mut query: Query<&mut OptionsModal>| {
                            if let Ok(mut my_widget) = query.get_mut(my_widget_entity) {
                                my_widget.show_modal = false;
                            }
                        }),
                    )),
                    ..Default::default()
                })
        ),
        ..Default::default()
    });

    widget_children.apply(current_widget.as_parent());
}

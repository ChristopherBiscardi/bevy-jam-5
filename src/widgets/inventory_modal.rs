use bevy::{color::palettes::tailwind::*, prelude::*};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::On,
    DefaultPickingPlugins,
};
use woodpecker_ui::prelude::*;

use crate::{game_scene::Player, inventory::Inventory};

use super::{InventoryBaseModal, InventoryBaseModalBundle};

#[derive(
    Debug,
    Component,
    Widget,
    Clone,
    Default,
    Copy,
    PartialEq,
    Reflect,
)]
#[widget_systems(update, render)]
pub struct InventoryModal {
    pub show_modal: bool,
}

pub struct InventoryState {
    pub show_modal: bool,
}

#[derive(Bundle, Default, Clone)]
pub struct InventoryModalBundle {
    pub count: InventoryModal,
    pub styles: WoodpeckerStyle,
    pub children: WidgetChildren,
}

// TODO: update when Player inventory updates
fn update(
    current_widget: Res<CurrentWidget>,
    query: Query<Entity, Changed<InventoryModal>>,
) -> bool {
    query.contains(**current_widget)
}

fn render(
    current_widget: Res<CurrentWidget>,
    mut query: Query<(
        &InventoryModal,
        &mut WidgetChildren,
    )>,
    inventory_query: Query<&Inventory, With<Player>>,
) {
    let Ok((my_widget, mut widget_children)) =
        query.get_mut(**current_widget)
    else {
        return;
    };

    let my_widget_entity = **current_widget;
    // widget_children.add::<WButton>((
    //     WButtonBundle {
    //         children: WidgetChildren::default()
    //             .with_child::<Element>((
    //                 ElementBundle {
    //                     styles: WoodpeckerStyle {
    //                         font_size: 20.0,
    //                         ..default()
    //                     },
    //                     ..default()
    //                 },
    //                 WidgetRender::Text {
    //                     content: "Open
    // Modal".into(),
    // word_wrap: false,                 },
    //             )),
    //         ..default()
    //     },
    //     On::<Pointer<Click>>::run(
    //         move |mut query: Query<&mut
    // InventoryModal>| {             if let Ok(mut
    // my_widget) =
    // query.get_mut(my_widget_entity)
    // {                 my_widget.show_modal =
    // true;             }
    //         },
    //     ),
    // ));

    let mut items = WidgetChildren::default();
    let Ok(inventory) = inventory_query.get_single() else {
        warn!("no player inventory");
        return;
    };
    for item in &inventory.items {
        items.add::<Element>((
            ElementBundle {
                styles: WoodpeckerStyle {
                    font_size: 20.0,
                    background_color: SKY_400.into(),
                    width: Units::Pixels(20.),
                    height: Units::Pixels(20.),
                    ..default()
                },
                ..default()
            },
            WidgetRender::Quad,
        ));
    }

    widget_children.add::<InventoryBaseModal>(
        InventoryBaseModalBundle {
            modal: InventoryBaseModal {
                visible: my_widget.show_modal,
                title: "Inventory".into(),
                overlay_color: SLATE_50
                    .with_alpha(0.35)
                    .into(),
                children_styles: WoodpeckerStyle {
                    background_color: GREEN_400.into(),
                    border_color: GREEN_400.into(),
                    ..default()
                },
                ..default()
            },
            children: PassedChildren(
                WidgetChildren::default()
                    .with_child::<Element>(ElementBundle {
                        styles: WoodpeckerStyle {
                            // TODO: display: Grid
                            display: WidgetDisplay::Flex,
                            flex_wrap: WidgetFlexWrap::Wrap,
                            gap: (
                                Units::Pixels(5.),
                                Units::Pixels(5.),
                            ),
                            align_items: Some(
                                WidgetAlignItems::Center,
                            ),
                            justify_content: Some(
                                WidgetAlignContent::Center,
                            ),
                            padding: Edge::all(10.0),
                            width: Units::Percentage(100.0),
                            background_color: RED_400
                                .into(),
                            ..default()
                        },
                        children: items,
                        ..default()
                    }),
            ),
            ..default()
        },
    );

    widget_children.apply(current_widget.as_parent());
}

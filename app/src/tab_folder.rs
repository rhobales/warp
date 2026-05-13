use std::sync::Arc;

use pathfinder_color::ColorU;
use serde::{Deserialize, Serialize};
use warp_core::ui::theme::AnsiColors;
use warpui::elements::{
    CrossAxisAlignment, DraggableState, Element, Flex, MainAxisAlignment, MainAxisSize,
    MouseStateHandle, ParentElement,
};

use crate::menu::{MenuAction, MenuItem, MenuItemFields};
use crate::tab::{LocalTabId, SelectedTabColor};
use crate::ui_components::color_dot::{render_color_dot, TAB_COLOR_OPTIONS};
use crate::workspace::WorkspaceAction;

/// Stable per-window identifier for a tab folder. Survives reorders, drag&drop
/// and persistence, just like `LocalTabId`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalFolderId(pub u32);

#[derive(Clone, Debug, PartialEq)]
pub enum SidebarItem {
    Tab(LocalTabId),
    Folder {
        id: LocalFolderId,
        children: Vec<LocalTabId>,
    },
}

#[derive(Clone)]
pub struct TabFolderData {
    pub id: LocalFolderId,
    pub name: String,
    pub color: SelectedTabColor,
    pub is_open: bool,
    pub header_mouse_state: MouseStateHandle,
    #[allow(dead_code, reason = "Wired up in a follow-up DnD pass for folder reorder")]
    pub draggable_state: DraggableState,
}

impl TabFolderData {
    pub fn new(id: LocalFolderId, name: String) -> Self {
        Self {
            id,
            name,
            color: SelectedTabColor::Unset,
            is_open: true,
            header_mouse_state: Default::default(),
            draggable_state: Default::default(),
        }
    }
}

pub fn folder_menu_items(
    folder: &TabFolderData,
    terminal_colors: AnsiColors,
) -> Vec<MenuItem<WorkspaceAction>> {
    let folder_id = folder.id;
    let is_open = folder.is_open;
    let current_color = folder.color;

    let mut items: Vec<MenuItem<WorkspaceAction>> = vec![
        MenuItemFields::new("Rename folder")
            .with_on_select_action(WorkspaceAction::RenameTabFolder { folder_id })
            .into_item(),
        MenuItemFields::new(if is_open { "Close folder" } else { "Open folder" })
            .with_on_select_action(WorkspaceAction::ToggleTabFolderOpen { folder_id })
            .into_item(),
        MenuItemFields::new("Delete folder")
            .with_on_select_action(WorkspaceAction::DeleteTabFolder { folder_id })
            .into_item(),
        MenuItem::Separator,
        folder_color_picker_item(folder_id, current_color, terminal_colors),
    ];
    items.shrink_to_fit();
    items
}

fn folder_color_picker_item(
    folder_id: LocalFolderId,
    current_color: SelectedTabColor,
    terminal_colors: AnsiColors,
) -> MenuItem<WorkspaceAction> {
    let mouse_states: Vec<MouseStateHandle> = (0..TAB_COLOR_OPTIONS.len() + 1)
        .map(|_| MouseStateHandle::default())
        .collect();
    let resolved = match current_color {
        SelectedTabColor::Color(id) => Some(id),
        _ => None,
    };

    MenuItem::Item(
        MenuItemFields::new_with_custom_label(
            Arc::new(move |_is_selected, _is_hovered, appearance, _app| {
                let theme = appearance.theme();
                let ring_color: ColorU = theme.accent().into();

                let mut row = Flex::row()
                    .with_main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_main_axis_size(MainAxisSize::Max);

                for (ansi_id, mouse_state) in std::iter::once(None)
                    .chain(TAB_COLOR_OPTIONS.iter().copied().map(Some))
                    .zip(mouse_states.iter().cloned())
                {
                    let is_selected = ansi_id == resolved;
                    let dot_color: ColorU = match ansi_id {
                        None => ColorU::transparent_black(),
                        Some(id) => id.to_ansi_color(&terminal_colors).into(),
                    };
                    let tooltip = match ansi_id {
                        None => "Default (no color)".to_string(),
                        Some(id) => id.to_string(),
                    };

                    let dot = render_color_dot(
                        mouse_state,
                        dot_color,
                        is_selected,
                        ring_color,
                        ansi_id.is_none(),
                        theme.foreground(),
                        tooltip,
                        appearance,
                    )
                    .on_click(move |ctx, _, _| {
                        let new_color = match ansi_id {
                            Some(c) => SelectedTabColor::Color(c),
                            None => SelectedTabColor::Unset,
                        };
                        ctx.dispatch_typed_action(WorkspaceAction::SetTabFolderColor {
                            folder_id,
                            color: new_color,
                        });
                        ctx.dispatch_typed_action(MenuAction::Close(true));
                    });

                    row.add_child(dot.finish());
                }

                row.finish()
            }),
            None,
        )
        .no_highlight_on_hover()
        .with_no_interaction_on_hover(),
    )
}

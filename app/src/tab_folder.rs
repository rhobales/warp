use serde::{Deserialize, Serialize};
use warpui::elements::{DraggableState, MouseStateHandle};

use crate::tab::{LocalTabId, SelectedTabColor};

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

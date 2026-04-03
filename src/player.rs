// SPDX-License-Identifier: Apache-2.0

use crate::types::{Axis, PlayerId, Selection};

pub struct PlayerState {
    pub id: PlayerId,
    pub last_selection: Option<Selection>,
}

impl PlayerState {
    pub fn new(id: PlayerId) -> Self {
        PlayerState {
            id,
            last_selection: None,
        }
    }

    pub fn required_axis(&self) -> Option<Axis> {
        self.last_selection.map(|s| s.axis.opposite())
    }

    pub fn clear_selection(&mut self) {
        self.last_selection = None;
    }
}

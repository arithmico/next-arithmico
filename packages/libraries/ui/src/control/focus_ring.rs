#[derive(Clone, Debug, PartialEq)]
pub enum FocusRingOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub struct FocusRing {
    orientation: FocusRingOrientation,
    position: usize,
    reset_position: usize,
    length: usize,
}

impl FocusRing {
    pub fn new(
        position: usize,
        length: usize,
        orientation: FocusRingOrientation,
    ) -> Self {
        Self {
            orientation,
            position,
            length,
            reset_position: position,
        }
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
        self.reset_position = position;
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    pub fn reset_position(&mut self) {
        self.position = self.reset_position;
    }

    pub fn handle_keydown(&mut self, key: &str) -> bool {
        match key {
            "ArrowUp"
                if self.orientation == FocusRingOrientation::Vertical
                    && self.position > 0 =>
            {
                self.position -= 1;
            }

            "ArrowDown"
                if self.orientation == FocusRingOrientation::Vertical
                    && self.position + 1 < self.length =>
            {
                self.position += 1;
            }

            "ArrowLeft"
                if self.orientation == FocusRingOrientation::Horizontal
                    && self.position > 0 =>
            {
                self.position -= 1;
            }

            "ArrowRight"
                if self.orientation == FocusRingOrientation::Horizontal
                    && self.position + 1 < self.length =>
            {
                self.position += 1;
            }

            _ => return false,
        }
        true
    }
}

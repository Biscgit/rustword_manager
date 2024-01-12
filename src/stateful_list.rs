use ratatui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };
        list.default_selected();
        list
    }

    pub fn default_selected(&mut self) {
        if !self.items.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn reset_selected(&mut self) {
        self.state.select(None);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn current_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn current_item(&self) -> Option<&T> {
        self.items.get(self.current_index().unwrap_or(self.items.len() + 1))
    }
}
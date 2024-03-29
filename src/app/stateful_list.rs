use ratatui::widgets::ListState;

pub struct StatefulList<T> {
    // a stateful list that stores the currently selected index
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        // create a new List from a Vec<T>
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };
        list.default_selected();
        list
    }

    pub fn len(&self) -> usize {
        // returns the length
        self.items.len()
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        // set items to a new vector
        self.items = items;
        self.default_selected();
    }

    pub fn set_index(&mut self, index: usize) {
        // sets index of currently selected
        if index < self.len() {
            self.state.select(Some(index));
        } else {
            self.default_selected();
        }
    }

    pub fn default_selected(&mut self) {
        // select first element if possible
        if !self.items.is_empty() {
            self.state.select(Some(0));
        } else {
            self.reset_selected();
        }
    }

    pub fn reset_selected(&mut self) {
        // reset selection
        self.state.select(None);
    }

    pub fn next(&mut self) {
        // increases index if possible
        if !self.items.is_empty() {
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
    }

    pub fn previous(&mut self) {
        // decreases index if possible
        if !self.items.is_empty() {
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
    }

    pub fn get_ref(&mut self, index: usize) -> Option<&T> {
        // get a reference to an item from index
        self.items.get(index)
    }

    pub fn _get_mut(&mut self, index: usize) -> Option<&mut T> {
        // get a mutable reference to an item from index
        self.items.get_mut(index)
    }

    pub fn current_index(&self) -> Option<usize> {
        // returns currently selected if possible
        self.state.selected()
    }

    pub fn current_item(&self) -> Option<&T> {
        // returns the currently selected item
        self.items.get(self.current_index().unwrap_or(self.items.len() + 1))
    }

    pub fn current_item_mut(&mut self) -> Option<&mut T> {
        // returns a mutable reference to the currently selected item
        let index = self.current_index().unwrap_or(self.items.len() + 1);
        self.items.get_mut(index)
    }
}

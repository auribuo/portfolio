use crate::LocalStorageSettings;

#[derive(Debug, Clone)]
pub(crate) struct History {
    commands: Vec<String>,
    nav_pointer: usize,
}

impl From<&LocalStorageSettings> for History {
    fn from(value: &LocalStorageSettings) -> Self {
        let len = value.history.len();
        Self {
            commands: value.history.clone(),
            nav_pointer: len,
        }
    }
}

impl History {
    pub(crate) fn nav_back(&mut self) -> Option<&String> {
        if self.nav_pointer == 0 {
            None
        } else {
            self.nav_pointer -= 1;
            self.commands.get(self.nav_pointer)
        }
    }

    pub(crate) fn nav_front(&mut self) -> Option<&String> {
        if self.nav_pointer == self.commands.len() {
            None
        } else {
            self.nav_pointer += 1;
            self.commands.get(self.nav_pointer)
        }
    }

    pub(crate) fn push(&mut self, cmd: String) {
        self.commands.push(cmd);
        self.nav_pointer = self.commands.len();
    }

    pub(crate) fn entries(&self) -> &Vec<String> {
        &self.commands
    }

    pub(crate) fn clear(&mut self) {
        self.commands.clear();
        self.nav_pointer = 0;
    }
}

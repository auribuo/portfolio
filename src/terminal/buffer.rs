use super::commands::CommandResult;

#[derive(Debug, Clone)]
pub(crate) enum EntryContent {
    Text(String),
    Empty,
}

impl EntryContent {
    pub(crate) fn render(&self) -> String {
        match self {
            EntryContent::Empty => "".to_string(),
            EntryContent::Text(text) => text.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CommandEntry {
    pub(crate) cmd: String,
    pub(crate) output: EntryContent,
    pub(crate) failed: bool,
}

impl CommandEntry {
    pub(crate) fn new(cmd: String, output: EntryContent, failed: bool) -> Self {
        Self {
            cmd,
            output,
            failed,
        }
    }

    pub(crate) fn ok(cmd: &str, output: EntryContent) -> Self {
        Self::new(cmd.to_string(), output, false)
    }

    pub(crate) fn err(cmd: &str, output: EntryContent) -> Self {
        Self::new(cmd.to_string(), output, true)
    }

    pub(crate) fn empty() -> Self {
        Self::new("".to_string(), EntryContent::Empty, false)
    }

    pub(crate) fn no_output(cmd: &str) -> Self {
        Self::new(cmd.to_string(), EntryContent::Empty, false)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TerminalBuffer {
    commands: Vec<CommandEntry>,
}

impl TerminalBuffer {
    pub(crate) fn commands(&self) -> &Vec<CommandEntry> {
        &self.commands
    }

    pub(crate) fn new() -> Self {
        Self { commands: vec![] }
    }

    pub(crate) fn process(&mut self, res: CommandResult) {
        info!("Received command: {:?}", res);
        match res {
            CommandResult::None => self.commands.push(CommandEntry::empty()),
            CommandResult::Help(help) => self
                .commands
                .push(CommandEntry::ok("help", EntryContent::Text(help))),
            CommandResult::Clear => {
                self.commands.push(CommandEntry::no_output("clear"));
                self.clear();
            }
            CommandResult::Pwd(pwd) => self
                .commands
                .push(CommandEntry::ok("pwd", EntryContent::Text(pwd.to_string()))),
            CommandResult::Cd(cmd) => self.commands.push(CommandEntry::no_output(&cmd)),
            CommandResult::Ls(ls) => self
                .commands
                .push(CommandEntry::ok("ls", EntryContent::Text(ls))),
            CommandResult::Theme(cmd) => self.commands.push(CommandEntry::no_output(&cmd)),
            CommandResult::History(cmd, history) => self
                .commands
                .push(CommandEntry::ok(&cmd, EntryContent::Text(history))),
            CommandResult::Failed(cmd, output) => self
                .commands
                .push(CommandEntry::err(&cmd, EntryContent::Text(output))),
            CommandResult::Unknown(cmd) => self.commands.push(CommandEntry::err(
                &cmd,
                EntryContent::Text(format!("command not found: {}", cmd)),
            )),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.commands.clear()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

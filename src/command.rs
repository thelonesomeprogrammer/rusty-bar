use anyhow::Result;
use std::process::Command as Process;
use std::time::Duration;

pub struct Command {
    attr: Attributes,
    command: String,
    update_interval: Duration,
}

impl Command {
    /// Creates a new [`Command`] widget.
    ///
    /// Arguments
    ///
    /// * `attr` - Represents `Attributes` which controls properties like
    /// `Font`, foreground and background color etc.
    ///
    /// * `command` - Command to be executed.
    ///
    /// * `update_interval` - Time interval between updates.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate cnx;
    ///
    /// use cnx::*;
    /// use cnx::text::*;
    /// use cnx_contrib::widgets::command::*;
    /// use anyhow::Result;
    /// use std::time::Duration;
    ///
    /// fn run() -> Result<()> {
    /// let attr = Attributes {
    ///     font: Font::new("SourceCodePro 16"),
    ///     fg_color: Color::white(),
    ///     bg_color: None,
    ///     padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    /// };
    ///
    /// let mut cnx = Cnx::new(Position::Top);
    /// cnx.add_widget(Command::new(attr, "echo foo".into(), Duration::from_secs(10)));
    /// Ok(())
    /// }
    /// fn main() { run().unwrap(); }
    /// ```
    pub fn new(attr: Attributes, command: String, update_interval: Duration) -> Self {
        Self {
            attr,
            command,
            update_interval,
        }
    }

    fn tick(&self) -> Vec<Text> {
        let output = Process::new("sh")
            .arg("-c")
            .arg(self.command.clone())
            .output()
            .expect("failed to execute process");

        let texts = vec![Text {
            attr: self.attr.clone(),
            text: String::from_utf8(output.stdout).unwrap_or_else(|_| "error".into()),
            stretch: false,
            markup: true,
        }];

        texts
    }
}
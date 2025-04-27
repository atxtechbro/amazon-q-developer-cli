use std::future::Future;
use std::io::Write;
use std::pin::Pin;

use crossterm::queue;
use crossterm::style::{
    self,
    Color,
};
use eyre::Result;

use crate::commands::CommandHandler;
use crate::{
    ChatContext, ChatState, QueuedTool
};

/// Handler for the profile rename command
pub struct RenameProfileCommand {
    old_name: String,
    new_name: String,
}

impl RenameProfileCommand {
    pub fn new(old_name: &str, new_name: &str) -> Self {
        Self {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
        }
    }
}

impl CommandHandler for RenameProfileCommand {
    fn name(&self) -> &'static str {
        "rename"
    }

    fn description(&self) -> &'static str {
        "Rename a profile"
    }

    fn usage(&self) -> &'static str {
        "/profile rename <old-name> <new-name>"
    }

    fn help(&self) -> String {
        "Rename a profile from <old-name> to <new-name>. You cannot rename the default profile.".to_string()
    }

    fn execute<'a>(
        &'a self,
        _args: Vec<&'a str>,
        ctx: &'a ChatContext,
        tool_uses: Option<Vec<QueuedTool>>,
        pending_tool_index: Option<usize>,
    ) -> Pin<Box<dyn Future<Output = Result<ChatState>> + Send + 'a>> {
        Box::pin(async move {
            // Get the context manager
            let Some(context_manager) = ctx.conversation_state.context_manager.as_mut() else {
                queue!(
                    ctx.output,
                    style::SetForegroundColor(Color::Red),
                    style::Print("Error: Context manager not initialized\n"),
                    style::ResetColor
                )?;
                ctx.output.flush()?;
                return Ok(ChatState::PromptUser {
                    tool_uses,
                    pending_tool_index,
                    skip_printing_tools: true,
                });
            };

            // Rename the profile
            match context_manager.rename_profile(&self.old_name, &self.new_name).await {
                Ok(_) => {
                    // Success message
                    queue!(
                        ctx.output,
                        style::SetForegroundColor(Color::Green),
                        style::Print(format!("\nRenamed profile: {} -> {}\n\n", self.old_name, self.new_name)),
                        style::ResetColor
                    )?;
                },
                Err(e) => {
                    // Error message
                    queue!(
                        ctx.output,
                        style::SetForegroundColor(Color::Red),
                        style::Print(format!("\nError renaming profile: {}\n\n", e)),
                        style::ResetColor
                    )?;
                },
            }

            ctx.output.flush()?;

            Ok(ChatState::PromptUser {
                tool_uses,
                pending_tool_index,
                skip_printing_tools: true,
            })
        })
    }

    fn requires_confirmation(&self, _args: &[&str]) -> bool {
        false // Rename command doesn't require confirmation
    }

    fn parse_args<'a>(&self, args: Vec<&'a str>) -> Result<Vec<&'a str>> {
        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rename_profile_command() {
        let command = RenameProfileCommand::new("old", "new");
        assert_eq!(command.name(), "rename");
        assert_eq!(command.description(), "Rename a profile");
        assert_eq!(command.usage(), "/profile rename <old-name> <new-name>");

        // Note: Full testing would require mocking the context manager
    }
}

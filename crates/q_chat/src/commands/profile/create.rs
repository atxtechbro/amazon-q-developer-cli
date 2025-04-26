use std::future::Future;
use std::io::Write;
use std::pin::Pin;

use crossterm::queue;
use crossterm::style::{
    self,
    Color,
};
use eyre::Result;
use fig_os_shim::Context;

use crate::commands::CommandHandler;
use crate::{
    ChatState,
    QueuedTool,
};

/// Handler for the profile create command
pub struct CreateProfileCommand {
    name: String,
}

impl CreateProfileCommand {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl CommandHandler for CreateProfileCommand {
    fn name(&self) -> &'static str {
        "create"
    }

    fn description(&self) -> &'static str {
        "Create a new profile"
    }

    fn usage(&self) -> &'static str {
        "/profile create <n>"
    }

    fn help(&self) -> String {
        "Create a new profile with the specified name. Profile names can only contain alphanumeric characters, hyphens, and underscores.".to_string()
    }

    fn execute<'a>(
        &'a self,
        _args: Vec<&'a str>,
        ctx: &'a Context,
        tool_uses: Option<Vec<QueuedTool>>,
        pending_tool_index: Option<usize>,
    ) -> Pin<Box<dyn Future<Output = Result<ChatState>> + Send + 'a>> {
        Box::pin(async move {
            // Get the conversation state from the context
            let mut stdout = ctx.stdout();
            let conversation_state = ctx.get_conversation_state()?;

            // Get the context manager
            let Some(context_manager) = &mut conversation_state.context_manager else {
                queue!(
                    stdout,
                    style::SetForegroundColor(Color::Red),
                    style::Print("Error: Context manager not initialized\n"),
                    style::ResetColor
                )?;
                stdout.flush()?;
                return Ok(ChatState::PromptUser {
                    tool_uses,
                    pending_tool_index,
                    skip_printing_tools: true,
                });
            };

            // Create the profile
            match context_manager.create_profile(&self.name).await {
                Ok(_) => {
                    // Success message
                    queue!(
                        stdout,
                        style::SetForegroundColor(Color::Green),
                        style::Print(format!("\nCreated profile: {}\n\n", self.name)),
                        style::ResetColor
                    )?;

                    // Switch to the newly created profile
                    if let Err(e) = context_manager.switch_profile(&self.name).await {
                        queue!(
                            stdout,
                            style::SetForegroundColor(Color::Yellow),
                            style::Print(format!("Warning: Failed to switch to the new profile: {}\n\n", e)),
                            style::ResetColor
                        )?;
                    } else {
                        queue!(
                            stdout,
                            style::SetForegroundColor(Color::Green),
                            style::Print(format!("Switched to profile: {}\n\n", self.name)),
                            style::ResetColor
                        )?;
                    }
                },
                Err(e) => {
                    // Error message
                    queue!(
                        stdout,
                        style::SetForegroundColor(Color::Red),
                        style::Print(format!("\nError creating profile: {}\n\n", e)),
                        style::ResetColor
                    )?;
                },
            }

            stdout.flush()?;

            Ok(ChatState::PromptUser {
                tool_uses,
                pending_tool_index,
                skip_printing_tools: true,
            })
        })
    }

    fn requires_confirmation(&self, _args: &[&str]) -> bool {
        false // Create command doesn't require confirmation
    }

    fn parse_args<'a>(&self, args: Vec<&'a str>) -> Result<Vec<&'a str>> {
        Ok(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_profile_command() {
        let command = CreateProfileCommand::new("test");
        assert_eq!(command.name(), "create");
        assert_eq!(command.description(), "Create a new profile");
        assert_eq!(command.usage(), "/profile create <n>");

        // Note: Full testing would require mocking the context manager
    }
}

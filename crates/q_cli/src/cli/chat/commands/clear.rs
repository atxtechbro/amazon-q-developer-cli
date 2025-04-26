use std::future::Future;
use std::pin::Pin;

use eyre::Result;
use fig_os_shim::Context;

use crate::cli::chat::commands::CommandHandler;
use crate::cli::chat::{
    ChatState,
    QueuedTool,
};

/// Handler for the clear command
pub struct ClearCommand;

impl ClearCommand {
    pub fn new() -> Self {
        Self
    }
}

impl CommandHandler for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn description(&self) -> &'static str {
        "Clear the conversation history"
    }

    fn usage(&self) -> &'static str {
        "/clear"
    }

    fn help(&self) -> String {
        "Clears the conversation history in the current session.".to_string()
    }

    fn execute<'a>(
        &'a self,
        _args: Vec<&'a str>,
        _ctx: &'a Context,
        tool_uses: Option<Vec<QueuedTool>>,
        pending_tool_index: Option<usize>,
    ) -> Pin<Box<dyn Future<Output = Result<ChatState>> + Send + 'a>> {
        Box::pin(async move {
            // Return PromptUser state with skip_printing_tools set to true
            Ok(ChatState::PromptUser {
                tool_uses,
                pending_tool_index,
                skip_printing_tools: true,
            })
        })
    }

    fn requires_confirmation(&self, _args: &[&str]) -> bool {
        false // Clearing doesn't require confirmation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_command() {
        let command = ClearCommand::new();
        assert_eq!(command.name(), "clear");
        assert_eq!(command.description(), "Clear the conversation history");
        assert_eq!(command.usage(), "/clear");
        assert!(!command.requires_confirmation(&[]));
    }
}

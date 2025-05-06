# Amazon Q Development Guidelines

This document stores learning notes about MCP implementation in this application. It's more about learning and note-taking than style and guidance.

Whenever new MCP knowledge is obtained or "aha moments" about its implementation are discovered, add those insights here. Any MCP-related findings should trigger notes and refinement of this document. Keep the total file under 300 lines to maintain focus on the most valuable information.

Always follow these guidelines when assisting in development for the Amazon Q CLI.

## MCP (Model Context Protocol) Usage

The MCP functionality in Amazon Q CLI has the following characteristics:

1. The `mcp_client` crate exists and has functionality for interacting with MCP servers, but there's no dedicated CLI command to manage them.

2. MCP servers load correctly during chat initialization:
   ```
   ✓ test loaded in 0.02 s
   ✓ github loaded in 0.02 s
   ✓ 2 of 2 mcp servers initialized
   ```

3. The command `q mcp list` doesn't exist in the CLI interface.

4. To use MCP tools, you need to:
   - Use them within `q chat` sessions
   - Use the `/tools` command within chat to see available tools
   - Use the `/prompts` command within chat to see available prompts from MCP servers

5. The MCP functionality is designed to be used within the chat interface rather than through a separate CLI command.

Example output from `/prompts` and `/tools` commands:
```
> /prompts

Prompt                                         Arguments (* = required)
▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔

> /tools

Tool                      Permission
▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔Built-in:
- fs_write                * not trusted
- use_aws                 * trust read-only commands
- report_issue            * trusted
- fs_read                 * trusted
- execute_bash            * trust read-only commands

test (MCP):
- test___test_hello       * not trusted

github (MCP):
- github___github_info    * not trusted
```

## Rust Best Practices

### File Operations

When working with file operations in Rust:

1. Prefer using the simpler `fs::read_to_string()` and `fs::write()` functions over verbose `File::open()` + `read_to_string()` or `File::create()` + `write_all()` combinations
2. Avoid the `#[allow(clippy::verbose_file_reads)]` annotation by using the recommended methods
3. Use `serde_json::to_string_pretty()` + `fs::write()` instead of creating a file and then writing to it with `serde_json::to_writer_pretty()`
4. Keep imports organized by functionality (e.g., group path-related imports together)

## Git

### Committing Changes

Follow the git best practice of committing early and often. Run `git commit` often, but DO NOT ever run `git push`

BEFORE committing a change, ALWAYS do the following steps:

1. Run `cargo build` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
2. Run `cargo test` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
3. Run `cargo +nightly fmt` to auto-format the code
4. Commit the changes

### Commit Messages

All commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/) specification and include best practices:


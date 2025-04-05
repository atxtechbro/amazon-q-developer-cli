# Amazon Q on Windows

## Current Support Status

> **Note:** Native Windows support is currently in development. For now, Amazon Q CLI is available on Windows through Windows Subsystem for Linux (WSL 2).

While we're working on full native Windows support, please note:
- Autocomplete functionality is not yet available on native Windows
- The chat functionality works in WSL 2 environments

## Installation with WSL 2

For the best experience on Windows, we strongly recommend using Windows Subsystem for Linux (WSL 2):

1. **Install WSL 2** by following [Microsoft's official guide](https://learn.microsoft.com/en-us/windows/wsl/install)

2. **Launch your WSL 2 distribution** (Ubuntu is recommended)

3. **Follow the Linux installation instructions**:
   ```bash
   # Download the latest .deb package
   curl -LO https://d3op2l77j7wnti.cloudfront.net/amazon-q/latest/amazon-q-latest-amd64.deb

   # Install the package
   sudo dpkg -i amazon-q-latest-amd64.deb
   sudo apt-get install -f
   ```

4. **Start using Amazon Q**:
   ```bash
   q login
   ```

## Future Windows Support

We're actively working on bringing full native Windows support, including autocomplete functionality. For updates and discussions about Windows support, please follow:
- [Windows Support Discussion](https://github.com/aws/q-command-line-discussions/discussions/15)

## Support and Uninstall

If you're having issues with your installation, first run:

```shell
q doctor
```

If that fails to resolve your issue, see our [support guide](../support.md). To uninstall Amazon Q:

```bash
q uninstall
```

# Webflow MCP Server Extension for Zed

This extension integrates [Webflow MCP Server](https://github.com/webflow/mcp-server) as a context server for [Zed's](https://zed.dev) [Assistant.](https://zed.dev/docs/assistant/assistant)

To install navigate to: **Zed** > **Extensions**. Or use the command palette ([macOS](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json#L581), [Linux](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-linux.json#L459)) to search `extensions`.

```
"context_servers": {
    "mcp-server-webflow": {
      "settings": {
        "webflow_token": "<WEBFLOW_TOKEN>"
      }
    }
  },
```

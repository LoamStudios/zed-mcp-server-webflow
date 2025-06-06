# Webflow MCP Server Extension for Zed

This extension integrates [Webflow MCP
Server](https://github.com/webflow/mcp-server) as a context server for
[Zed's](https://zed.dev) [Agent Panel.](https://zed.dev/docs/ai/overview)

To install navigate to: **Zed** > **Extensions**. Or use the command palette
([macOS](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-macos.json#L581),
[Linux](https://github.com/zed-industries/zed/blob/main/assets/keymaps/default-linux.json#L459))
to search `extensions`.

```json
"context_servers": {
  "mcp-server-webflow": {
    "settings": {
      "webflow_token": "<WEBFLOW_TOKEN>"
    }
  }
}
```

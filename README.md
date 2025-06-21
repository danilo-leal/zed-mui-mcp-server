# MUI MCP Server Extension for Zed

This extension provides a MCP server for retrieving [Material UI from MUI](https://mui.com/material-ui/getting-started/) documentation data directly from [Zed](https://zed.dev).

## Configuration

You don't need to configure anything to use this MCP server, but it does provide two optional fields if you'd to specify your usage a bit more:

### Preferred Theme (optional)

Add which theme you want to have documentation information focused on:

```json
{
  "context_servers": {
    "mcp-mui-server": {
      "settings": {
        "preferred_theme": "dark"
      }
    }
  }
}
```

### Component Filter (Optional)

Add which components you want to have documentation information focused on:

```json
{
  "context_servers": {
    "mcp-mui-server": {
      "settings": {
        "component_filter": ["Button", "Card", "TextField"]
      }
    }
  }
}
```

## Further Information

- [Material UI MCP Docs](https://mui.com/material-ui/getting-started/mcp)
- [Zed MCP Docs](https://zed.dev/docs/assistant/model-context-protocol)

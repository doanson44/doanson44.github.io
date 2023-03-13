# VS Code Debug Configuration

This folder contains VS Code configuration files for debugging the Blazor WebAssembly Todo application.

## Files

- `launch.json` - Debug configurations
- `tasks.json` - Build tasks
- `settings.json` - Workspace settings
- `extensions.json` - Recommended extensions

## How to Debug

1. **Install recommended extensions** when prompted or manually install them
2. **Set breakpoints** in your C# code
3. **Run the application** using one of these methods:

### Option 1: Use VS Code Debug Panel

1. Open Debug panel (Ctrl+Shift+D)
2. Select "Launch and Debug Blazor WebAssembly"
3. Click the green play button

### Option 2: Use Command Palette

1. Press F1 or Ctrl+Shift+P
2. Type "Debug: Start Debugging"
3. Select "Launch and Debug Blazor WebAssembly"

### Option 3: Use Terminal

```bash
dotnet run
```

Then attach debugger using "Debug in Edge" configuration.

## Debug Configurations

- **Launch and Debug Blazor WebAssembly**: Full debug experience with browser launch
- **.NET Core Launch (web)**: Debug the .NET part with Chrome debugging
- **Debug in Edge**: Attach to running application in Edge browser

## Troubleshooting

If you encounter source map errors (404 on GitHub URLs):

1. The application will still work normally
2. Source maps are only needed for detailed debugging of .NET runtime code
3. Your application code debugging should work fine

## Build Tasks

- `build`: Build the project
- `publish`: Publish for deployment
- `watch`: Run with hot reload

# Quick Start Guide

## Prerequisites

- .NET 10 SDK (10.0.100 or later)
- Git
- A code editor (VS Code, Visual Studio, or Rider)

## 🚀 Running Locally

### Step 1: Clone Repository (if not already done)
```powershell
git clone https://github.com/doanson44/doanson44.github.io.git
cd doanson44.github.io
```

### Step 2: Restore Dependencies
```powershell
dotnet restore
```

### Step 3: Run the Application
```powershell
dotnet watch run
```

The application will start and open in your default browser at `https://localhost:5001` (or similar).

## 🎨 Exploring Features

### Dashboard
1. Navigate to the home page (`/`)
2. See metrics: Total Tasks, Completed, Pending, Completion Rate
3. View recent activity
4. Click "Add New Todo" or "View All Todos"

### Todo List
1. Navigate to `/todos`
2. View all todos in a data grid
3. Filter by title, description, priority, or status
4. Sort by any column
5. Use pagination (10 items per page)
6. Actions:
   - ✓ Mark as complete
   - ✏️ Edit
   - 🗑️ Delete (with confirmation)

### Add/Edit Todo
1. Click "Add New" button
2. Fill in the form:
   - **Title** (required, max 100 chars)
   - **Description** (optional, max 500 chars)
   - **Priority** (Low, Medium, High)
3. Click "Create" or "Update"
4. See validation messages if fields are invalid

### Responsive Layout
1. Resize your browser window
2. Mobile (< 768px):
   - Sidebar hidden by default
   - Click hamburger menu to open
   - Click overlay to close
3. Tablet (768-991px):
   - 2-column grid on dashboard
4. Desktop (≥ 992px):
   - Sidebar always visible
   - 3-4 column grid on dashboard

## 🏗️ Building for Production

```powershell
# Clean build
dotnet clean

# Build in Release mode
dotnet build --configuration Release

# Publish (creates wwwroot folder)
dotnet publish --configuration Release --output publish
```

The published files will be in `publish/wwwroot/` - ready for deployment!

## 🌐 Deploying to GitHub Pages

### Option 1: Automatic (Recommended)
1. Commit your changes:
   ```powershell
   git add .
   git commit -m "Initial commit"
   ```

2. Push to main branch:
   ```powershell
   git push origin main
   ```
   (or `git push origin master` if using master branch)

3. GitHub Actions will automatically:
   - Build the project
   - Publish the app
   - Deploy to GitHub Pages

4. Check deployment:
   - Go to GitHub → Actions tab
   - Wait for workflow to complete (1-2 minutes)
   - Visit `https://doanson44.github.io`

### Option 2: Manual
If you need to deploy manually:

1. Build and publish:
   ```powershell
   dotnet publish --configuration Release --output publish
   ```

2. The `publish/wwwroot` folder contains the static site

3. Upload to GitHub Pages or any static hosting service

## 🔧 Common Commands

```powershell
# Run with hot reload
dotnet watch run

# Build without running
dotnet build

# Clean build artifacts
dotnet clean

# Restore NuGet packages
dotnet restore

# Check .NET version
dotnet --version

# List installed SDKs
dotnet --list-sdks
```

## 🐛 Troubleshooting

### "SDK not found" error
- Install .NET 10 SDK from https://dotnet.microsoft.com/download
- Or update `global.json` to a version you have installed

### Port already in use
```powershell
# Kill the process on port 5001
netstat -ano | findstr :5001
taskkill /F /PID <PID>
```

### Build errors after package changes
```powershell
dotnet clean
dotnet restore
dotnet build
```

### Hot reload not working
- Press `Ctrl+C` to stop
- Run `dotnet watch run` again

## 📱 Testing Responsive Design

### Using Browser DevTools
1. Press `F12` to open DevTools
2. Click "Toggle device toolbar" (or press `Ctrl+Shift+M`)
3. Test different devices:
   - iPhone 12 Pro (390x844)
   - iPad Air (820x1180)
   - Desktop (1920x1080)

### Breakpoints to Test
- **Mobile**: < 768px (sidebar hidden, 1 column)
- **Tablet**: 768-991px (sidebar toggle, 2 columns)
- **Desktop**: ≥ 992px (sidebar open, 3-4 columns)

## 🎯 What to Look For

### CQRS in Action
- Open browser DevTools → Network tab
- Create a new todo
- See the cache invalidation
- Reload the page
- First load: query executed
- Refresh within 5 min: served from cache

### AutoMapper
- Check `Features/Todos/Commands/CreateTodoCommandHandler.cs`
- Line: `var todoItem = _mapper.Map<TodoItem>(command);`
- This automatically maps CreateTodoCommand → TodoItem

### Cache Behavior
- Open `Features/Todos/Queries/GetTodoListQueryHandler.cs`
- See `_cacheService.GetOrAddAsync("todos:all", ...)`
- Results cached for 5 minutes
- Cache cleared on Create/Update/Delete

## 📚 Learning Resources

- **Blazor**: https://blazor.net
- **CQRS Pattern**: https://martinfowler.com/bliki/CQRS.html
- **AutoMapper**: https://docs.automapper.org
- **Radzen Blazor**: https://blazor.radzen.com/docs/guides/getting-started.html
- **Bootstrap 5**: https://getbootstrap.com/docs/5.3

## 🎉 Success!

If you can:
1. ✅ See the dashboard with metrics
2. ✅ Add a new todo
3. ✅ Edit an existing todo
4. ✅ Delete a todo
5. ✅ Toggle the sidebar on mobile

**Congratulations!** The app is working perfectly! 🚀

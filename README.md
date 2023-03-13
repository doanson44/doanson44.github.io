# Blazor WebAssembly App - GitHub Pages

A modern, responsive Blazor WebAssembly standalone application demonstrating best practices with CQRS architecture, caching, AutoMapper, Bootstrap 5.3.8, and Radzen Blazor components.

## 🚀 Live Demo

**URL:** https://doanson44.github.io

## 🏗️ Project Structure

```
doanson44.github.io/
├── .github/
│   └── workflows/
│       └── deploy.yml          # GitHub Actions CI/CD workflow
├── CQRS/                        # CQRS pattern abstractions
│   ├── ICommand.cs
│   ├── ICommandHandler.cs
│   ├── IQuery.cs
│   └── IQueryHandler.cs
├── Features/
│   └── Todos/
│       ├── Commands/            # Todo command handlers
│       │   ├── CreateTodoCommand.cs
│       │   ├── CreateTodoCommandHandler.cs
│       │   ├── UpdateTodoCommand.cs
│       │   ├── UpdateTodoCommandHandler.cs
│       │   ├── DeleteTodoCommand.cs
│       │   └── DeleteTodoCommandHandler.cs
│       └── Queries/             # Todo query handlers
│           ├── GetTodoListQuery.cs
│           ├── GetTodoListQueryHandler.cs
│           ├── GetTodoByIdQuery.cs
│           └── GetTodoByIdQueryHandler.cs
├── Layout/
│   └── MainLayout.razor         # Responsive layout with navbar & sidebar
├── Mappings/
│   └── TodoProfile.cs           # AutoMapper profiles
├── Models/
│   ├── TodoItem.cs              # Domain entity
│   └── TodoDto.cs               # Data Transfer Object
├── Pages/
│   ├── Home.razor               # Dashboard page
│   ├── TodoList.razor           # Todo list with DataGrid
│   └── TodoForm.razor           # Add/Edit todo form
├── Services/
│   ├── ICacheService.cs         # Cache service interface
│   ├── InMemoryCacheService.cs  # In-memory cache implementation
│   ├── ITodoRepository.cs
│   └── InMemoryTodoRepository.cs
├── wwwroot/
│   ├── css/
│   │   └── site.css             # Custom CSS styles
│   └── index.html               # Main HTML file
├── global.json                  # .NET 10 SDK version
├── Program.cs                   # DI configuration
└── Client.csproj
```

## 🛠️ Technologies & Features

### Core Technologies
- **.NET 10** (pinned via `global.json`)
- **Blazor WebAssembly** (standalone, no ASP.NET Core backend)
- **Bootstrap 5.3.8** (via CDN)
- **Radzen Blazor Components** (latest version)

### Architecture Patterns
- **CQRS (Command Query Responsibility Segregation)**
  - Separate command and query handlers
  - Clear separation of read and write operations
  - Example: `CreateTodoCommand`, `GetTodoListQuery`

- **Caching Strategy**
  - `ICacheService` abstraction
  - In-memory cache with configurable expiration
  - Cache invalidation on data changes
  - Example: Todo list cached for 5 minutes

- **Object Mapping**
  - AutoMapper for entity ↔ DTO mapping
  - Command → Entity mapping
  - Reduces boilerplate code

### UI Features
- **Responsive Design**
  - Mobile-first approach
  - 1 column (phone) → 2 columns (tablet) → 3+ columns (desktop)
  - Touch-friendly buttons (44px minimum)

- **Sidebar Navigation**
  - Desktop: Always visible, pinned
  - Tablet/Mobile: Collapsible with overlay
  - Radzen `RadzenPanelMenu` component

- **Pages**
  1. **Dashboard** - Metrics cards, recent activity, quick actions
  2. **Todo List** - Radzen DataGrid with filtering, sorting, paging
  3. **Todo Form** - Add/Edit with validation

## 📦 NuGet Packages

```xml
<PackageReference Include="Radzen.Blazor" Version="8.3.2" />
<PackageReference Include="AutoMapper" Version="15.1.0" />
<PackageReference Include="AutoMapper.Extensions.Microsoft.DependencyInjection" Version="12.0.1" />
<PackageReference Include="Microsoft.Extensions.Caching.Memory" Version="10.0.0" />
```

## 🚀 Local Development

### Prerequisites
- .NET 10 SDK (10.0.100 or later)

### Build & Run

```powershell
# Restore dependencies
dotnet restore

# Run the application
dotnet watch run

# Build for production
dotnet publish --configuration Release --output publish
```

The application will be available at `https://localhost:5001` (or the port shown in the terminal).

### Project Commands

```powershell
# Clean build artifacts
dotnet clean

# Build without running
dotnet build --configuration Release

# Run tests (if any)
dotnet test
```

## 🌐 Deployment to GitHub Pages

### Automatic Deployment (via GitHub Actions)

The project includes a CI/CD workflow (`.github/workflows/deploy.yml`) that:

1. **Triggers on push to `main` or `master` branch**
2. **Builds the project**
   - Checkout code
   - Setup .NET 10 SDK (using `global.json`)
   - Restore dependencies
   - Build in Release configuration
   - Publish to `publish/wwwroot`
3. **Prepares for GitHub Pages**
   - Adds `.nojekyll` file
   - Uploads artifact
4. **Deploys to GitHub Pages**
   - Uses `actions/deploy-pages@v4`
   - Site available at `https://doanson44.github.io`

### Manual Verification

After pushing to `main`/`master`:
1. Go to **Actions** tab in GitHub
2. Check the workflow run status
3. Once complete, visit `https://doanson44.github.io`

### GitHub Pages Settings

Ensure GitHub Pages is configured:
1. Repository → **Settings** → **Pages**
2. **Source**: GitHub Actions
3. **Branch**: Deployed via workflow (no manual branch selection needed)

## 🎨 Customization

### Update Brand Name
Edit `Layout/MainLayout.razor`:
```razor
<a class="navbar-brand ms-2" href="/">Your Brand Name</a>
```

### Change Color Scheme
Edit `wwwroot/css/site.css` to modify dashboard card gradients:
```css
.dashboard-card.card-primary {
    background: linear-gradient(135deg, #your-color-1 0%, #your-color-2 100%);
}
```

### Add New Pages
1. Create `Pages/YourPage.razor`
2. Add route: `@page "/your-route"`
3. Update sidebar in `Layout/MainLayout.razor`:
```razor
<RadzenPanelMenuItem Text="Your Page" Icon="icon_name" Path="/your-route" />
```

## 📝 CQRS Examples

### Query Example (with Cache)
```csharp
// Query
var query = new GetTodoListQuery();

// Execute via handler (uses cache internally)
var todos = await queryHandler.HandleAsync(query);
```

### Command Example
```csharp
// Command
var command = new CreateTodoCommand
{
    Title = "New Task",
    Description = "Task description",
    Priority = "High"
};

// Execute via handler (AutoMapper used internally)
await commandHandler.HandleAsync(command);
```

## 🧪 Testing Locally

```powershell
# Run the app
dotnet watch run

# Open browser to https://localhost:5001

# Test features:
# - Dashboard metrics update
# - Add new todo
# - Edit existing todo
# - Delete todo with confirmation
# - Filter/sort in DataGrid
# - Responsive layout (resize browser)
# - Sidebar toggle on mobile
```

## 📚 Code Highlights

### CQRS Pattern
- **Commands** modify state (Create, Update, Delete)
- **Queries** read data (GetAll, GetById)
- Handlers are registered in DI container

### Caching
- `GetTodoListQueryHandler` caches results for 5 minutes
- Cache invalidated on Create/Update/Delete operations
- Cache key: `"todos:all"`

### AutoMapper
- `TodoProfile` defines mappings
- Used in command handlers to map Command → Entity
- Used in query handlers to map Entity → DTO

### Responsive Layout
- CSS Grid for dashboard cards
- Bootstrap responsive utilities (`d-none d-md-block`)
- Sidebar with overlay on mobile
- Touch-friendly buttons (min 44px)

## 🔧 Troubleshooting

### Build Errors
```powershell
# Clear obj/bin folders
dotnet clean

# Restore packages
dotnet restore

# Build again
dotnet build
```

### GitHub Actions Deployment Fails
1. Check workflow logs in **Actions** tab
2. Ensure `.NET 10 SDK` is available
3. Verify `global.json` has correct version
4. Check GitHub Pages is enabled in Settings

### App Doesn't Load on GitHub Pages
1. Verify `<base href="/" />` in `index.html`
2. Check `.nojekyll` file exists
3. Clear browser cache
4. Check browser console for errors

## 📄 License

This project is open source and available under the MIT License.

## 👤 Author

**doanson44**
- GitHub: [@doanson44](https://github.com/doanson44)
- Website: https://doanson44.github.io

## 🙏 Acknowledgments

- [Blazor](https://blazor.net) - Microsoft's web framework
- [Radzen Blazor](https://blazor.radzen.com) - Blazor component library
- [AutoMapper](https://automapper.org) - Object-to-object mapping
- [Bootstrap](https://getbootstrap.com) - CSS framework

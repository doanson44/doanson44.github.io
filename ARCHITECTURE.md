# Architecture Documentation

## Overview

This Blazor WebAssembly application implements a clean, maintainable architecture following industry best practices with CQRS pattern, caching, and object mapping.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      Blazor WebAssembly                     │
│                        (Browser)                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐         ┌──────────────┐                │
│  │   Pages/     │────────▶│   Layout/    │                │
│  │  Components  │         │  MainLayout  │                │
│  └──────────────┘         └──────────────┘                │
│         │                                                   │
│         ▼                                                   │
│  ┌──────────────────────────────────────┐                 │
│  │      CQRS Command/Query Handlers     │                 │
│  │  (Features/Todos/Commands & Queries) │                 │
│  └──────────────────────────────────────┘                 │
│         │                      │                           │
│         ▼                      ▼                           │
│  ┌─────────────┐        ┌─────────────┐                  │
│  │ AutoMapper  │        │ Cache Svc   │                  │
│  └─────────────┘        └─────────────┘                  │
│         │                      │                           │
│         ▼                      ▼                           │
│  ┌──────────────────────────────────────┐                 │
│  │         Repository Layer              │                 │
│  │    (Services/ITodoRepository)         │                 │
│  └──────────────────────────────────────┘                 │
│         │                                                   │
│         ▼                                                   │
│  ┌──────────────────────────────────────┐                 │
│  │       In-Memory Data Store            │                 │
│  │      (Domain Models: TodoItem)        │                 │
│  └──────────────────────────────────────┘                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Layers & Responsibilities

### 1. Presentation Layer (`Pages/`, `Layout/`)
- **Responsibility**: UI components and user interaction
- **Components**:
  - `MainLayout.razor` - Application shell with navbar and sidebar
  - `Home.razor` - Dashboard with metrics
  - `TodoList.razor` - Data grid for todos
  - `TodoForm.razor` - Create/Edit forms
- **Dependencies**: Injects CQRS handlers, Radzen services

### 2. CQRS Layer (`Features/Todos/`)
- **Responsibility**: Command and Query handling
- **Pattern**: Command Query Responsibility Segregation
- **Structure**:
  ```
  Features/
  └── Todos/
      ├── Commands/
      │   ├── CreateTodoCommand.cs
      │   ├── CreateTodoCommandHandler.cs
      │   ├── UpdateTodoCommand.cs
      │   ├── UpdateTodoCommandHandler.cs
      │   ├── DeleteTodoCommand.cs
      │   └── DeleteTodoCommandHandler.cs
      └── Queries/
          ├── GetTodoListQuery.cs
          ├── GetTodoListQueryHandler.cs
          ├── GetTodoByIdQuery.cs
          └── GetTodoByIdQueryHandler.cs
  ```

#### Commands (Write Operations)
- **Purpose**: Modify system state
- **Flow**:
  1. UI creates command object
  2. Command handler receives command
  3. AutoMapper maps command → entity
  4. Repository persists changes
  5. Cache invalidated

#### Queries (Read Operations)
- **Purpose**: Retrieve data without side effects
- **Flow**:
  1. UI creates query object
  2. Query handler receives query
  3. Check cache first
  4. If cache miss, query repository
  5. AutoMapper maps entity → DTO
  6. Store in cache
  7. Return result

### 3. Service Layer (`Services/`)
- **ICacheService / InMemoryCacheService**
  - In-memory caching with TTL
  - Used by query handlers
  - Invalidated by command handlers

- **ITodoRepository / InMemoryTodoRepository**
  - Data access abstraction
  - In-memory implementation (easily replaceable)
  - CRUD operations

### 4. Domain Layer (`Models/`)
- **TodoItem** (Entity)
  - Domain model
  - Business logic container
  - Persisted by repository

- **TodoDto** (Data Transfer Object)
  - Data transfer between layers
  - Used in queries
  - No business logic

### 5. Mapping Layer (`Mappings/`)
- **TodoProfile** (AutoMapper Profile)
  - Defines mappings:
    - TodoItem ↔ TodoDto
    - CreateTodoCommand → TodoItem
    - UpdateTodoCommand → TodoItem
  - Eliminates manual mapping code

## Design Patterns

### 1. CQRS (Command Query Responsibility Segregation)
**Why?**
- Separates read and write concerns
- Easier to scale (can optimize queries separately)
- Clear intent (commands change state, queries don't)

**Implementation:**
```csharp
// Command
public class CreateTodoCommand : ICommand
{
    public string Title { get; set; }
    public string Description { get; set; }
}

// Command Handler
public class CreateTodoCommandHandler : ICommandHandler<CreateTodoCommand>
{
    public async Task HandleAsync(CreateTodoCommand command, CancellationToken ct)
    {
        // Business logic here
    }
}

// Query
public class GetTodoListQuery : IQuery<List<TodoDto>> { }

// Query Handler
public class GetTodoListQueryHandler : IQueryHandler<GetTodoListQuery, List<TodoDto>>
{
    public async Task<List<TodoDto>> HandleAsync(GetTodoListQuery query, CancellationToken ct)
    {
        // Retrieval logic here
    }
}
```

### 2. Repository Pattern
**Why?**
- Abstracts data access
- Easy to swap implementations (in-memory → SQL → API)
- Testability

**Implementation:**
```csharp
public interface ITodoRepository
{
    Task<List<TodoItem>> GetAllAsync();
    Task<TodoItem?> GetByIdAsync(int id);
    Task<TodoItem> AddAsync(TodoItem item);
    Task<TodoItem> UpdateAsync(TodoItem item);
    Task<bool> DeleteAsync(int id);
}
```

### 3. Dependency Injection
**Why?**
- Loose coupling
- Testability
- Flexibility

**Registration in `Program.cs`:**
```csharp
// Services
builder.Services.AddSingleton<ICacheService, InMemoryCacheService>();
builder.Services.AddSingleton<ITodoRepository, InMemoryTodoRepository>();

// CQRS Handlers
builder.Services.AddTransient<ICommandHandler<CreateTodoCommand>, CreateTodoCommandHandler>();
builder.Services.AddTransient<IQueryHandler<GetTodoListQuery, List<TodoDto>>, GetTodoListQueryHandler>();

// AutoMapper
builder.Services.AddAutoMapper(typeof(Program).Assembly);
```

### 4. DTO Pattern
**Why?**
- Separates internal models from API contracts
- Security (don't expose entire entity)
- Flexibility (DTO can combine multiple entities)

**Example:**
```csharp
// Domain Entity
public class TodoItem
{
    public int Id { get; set; }
    public string Title { get; set; }
    // ... internal fields
}

// DTO for external use
public class TodoDto
{
    public int Id { get; set; }
    public string Title { get; set; }
    // Only exposed fields
}
```

## Data Flow Examples

### Creating a Todo (Write)
```
1. User fills form in TodoForm.razor
   ↓
2. Form submits CreateTodoCommand
   ↓
3. CreateTodoCommandHandler receives command
   ↓
4. AutoMapper maps CreateTodoCommand → TodoItem
   ↓
5. Repository.AddAsync(todoItem) saves to store
   ↓
6. Cache invalidated (Remove "todos:all")
   ↓
7. Success notification shown
   ↓
8. Navigate back to list
```

### Loading Todo List (Read)
```
1. User navigates to TodoList.razor
   ↓
2. Page creates GetTodoListQuery
   ↓
3. GetTodoListQueryHandler receives query
   ↓
4. Check cache for "todos:all" key
   ↓
   ├─ Cache HIT ──▶ Return cached data
   │
   └─ Cache MISS:
      ↓
      Repository.GetAllAsync() fetches data
      ↓
      AutoMapper maps List<TodoItem> → List<TodoDto>
      ↓
      Store in cache (5 min TTL)
      ↓
      Return data
   ↓
5. DataGrid displays todos
```

## Caching Strategy

### Cache Keys
- `"todos:all"` - All todo items

### Cache Configuration
- **Provider**: IMemoryCache (in-memory)
- **TTL**: 5 minutes (default)
- **Eviction**: LRU (Least Recently Used)

### Cache Invalidation
- **On Create**: Remove `"todos:all"`
- **On Update**: Remove `"todos:all"`
- **On Delete**: Remove `"todos:all"`

### Future Enhancements
- Per-item caching: `"todos:{id}"`
- Sliding expiration
- Distributed cache (Redis)

## AutoMapper Profiles

### TodoProfile Mappings

```csharp
CreateMap<TodoItem, TodoDto>().ReverseMap();

CreateMap<CreateTodoCommand, TodoItem>()
    .ForMember(dest => dest.Id, opt => opt.Ignore())
    .ForMember(dest => dest.CreatedAt, opt => opt.MapFrom(_ => DateTime.UtcNow))
    .ForMember(dest => dest.IsCompleted, opt => opt.MapFrom(_ => false));

CreateMap<UpdateTodoCommand, TodoItem>()
    .ForMember(dest => dest.CreatedAt, opt => opt.Ignore());
```

## Dependency Graph

```
Pages (UI Layer)
  ├─ Depends on: CQRS Handlers, Radzen Services, Navigation
  └─ Injects: ICommandHandler<T>, IQueryHandler<T, R>

CQRS Handlers (Application Layer)
  ├─ Depends on: Repository, AutoMapper, Cache Service
  └─ Injects: ITodoRepository, IMapper, ICacheService

Services (Infrastructure Layer)
  ├─ No dependencies (except .NET base classes)
  └─ Implements: ICacheService, ITodoRepository

Models (Domain Layer)
  └─ No dependencies (pure POCOs)
```

## Testing Strategy (Future)

### Unit Tests
- **Command Handlers**: Mock repository, verify calls
- **Query Handlers**: Mock repository, verify cache usage
- **AutoMapper**: Verify mappings are correct
- **Cache Service**: Verify TTL, eviction

### Integration Tests
- **End-to-End**: Create → Read → Update → Delete flow
- **Cache**: Verify cache invalidation

### UI Tests
- **Blazor Components**: bUnit testing
- **Responsive**: Test different breakpoints

## Performance Considerations

### Current Optimizations
1. **Caching**: Reduces redundant data fetching
2. **AutoMapper**: Compiled expressions (fast mapping)
3. **Lazy Loading**: Components load on demand
4. **Blazor WASM**: Runs in browser (no server round-trips)

### Future Optimizations
1. **Virtualization**: For large lists (Radzen supports this)
2. **Pagination**: Already implemented in DataGrid
3. **Debouncing**: For search/filter inputs
4. **Code Splitting**: Lazy load assemblies

## Security Considerations

### Current
- **Client-side only**: No sensitive data
- **Validation**: DataAnnotations on commands
- **XSS Protection**: Blazor auto-escapes HTML

### Future (when adding backend)
- **Authentication**: JWT tokens, OAuth
- **Authorization**: Role-based access
- **HTTPS**: Always use HTTPS
- **CSRF Protection**: Anti-forgery tokens

## Scalability Path

### Phase 1: Current (In-Memory)
- Single-user, client-side
- No persistence between sessions

### Phase 2: API Backend
- Replace `InMemoryTodoRepository` with `ApiTodoRepository`
- Implement ASP.NET Core Web API
- Same CQRS handlers, just swap repository

### Phase 3: Database
- Add Entity Framework Core
- SQL Server / PostgreSQL
- Keep repository abstraction

### Phase 4: Distributed Cache
- Replace `InMemoryCacheService` with `RedisCacheService`
- Shared cache across users

### Phase 5: Event Sourcing (Advanced)
- Store commands as events
- Rebuild state from events
- Full audit trail

## Deployment Architecture

```
Developer Machine
  │
  │ git push
  ▼
GitHub Repository (main branch)
  │
  │ triggers
  ▼
GitHub Actions Workflow
  │
  ├─ Checkout code
  ├─ Setup .NET 10
  ├─ dotnet restore
  ├─ dotnet build
  ├─ dotnet publish
  └─ Upload to GitHub Pages
       │
       ▼
GitHub Pages CDN
  │
  │ serves static files
  ▼
End Users (Browser)
  │
  │ downloads Blazor WASM
  ▼
Blazor Runtime in Browser
  │
  │ executes C# via WebAssembly
  ▼
User sees app
```

## Conclusion

This architecture provides:
✅ **Separation of Concerns** - Clear layer boundaries  
✅ **Testability** - Easy to mock and test  
✅ **Maintainability** - Easy to understand and modify  
✅ **Scalability** - Can grow from in-memory to distributed  
✅ **Flexibility** - Easy to swap implementations  

The CQRS + Repository + Caching pattern is production-ready and follows industry best practices.

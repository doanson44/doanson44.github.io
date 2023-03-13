using Client.Common.Caching;
using Client.Common.CQRS;
using Client.Models;

namespace Client.Modules.TodoModule.Queries;

/// <summary>
/// Query to get all Todos list
/// </summary>
public class GetAllTodosQuery : IQuery<List<TodoDto>>, ICacheableQuery<List<TodoDto>>
{
    public bool IncludeCompleted { get; set; } = true;
    public string? PriorityFilter { get; set; }

    /// <summary>
    /// Cache key for all todos list with filters
    /// </summary>
    public string CacheKey => $"todos:all:{IncludeCompleted}:{PriorityFilter ?? "all"}";

    /// <summary>
    /// Cache for 5 minutes - todos list changes frequently
    /// </summary>
    public TimeSpan? AbsoluteExpirationRelativeToNow => TimeSpan.FromMinutes(5);
}

/// <summary>
/// Query to get Todo by ID
/// </summary>
public class GetTodoByIdQuery : IQuery<TodoDto?>, ICacheableQuery<TodoDto?>
{
    public Guid Id { get; set; }

    public GetTodoByIdQuery(Guid id)
    {
        Id = id;
    }

    /// <summary>
    /// Cache key for individual todo item
    /// </summary>
    public string CacheKey => $"todos:{Id}";

    /// <summary>
    /// Cache for 10 minutes - individual items change less frequently
    /// </summary>
    public TimeSpan? AbsoluteExpirationRelativeToNow => TimeSpan.FromMinutes(10);
}

/// <summary>
/// Query to get Todos by priority
/// </summary>
public class GetTodosByPriorityQuery : IQuery<List<TodoDto>>, ICacheableQuery<List<TodoDto>>
{
    public string Priority { get; set; } = string.Empty;

    public GetTodosByPriorityQuery(string priority)
    {
        Priority = priority;
    }

    /// <summary>
    /// Cache key for todos filtered by priority
    /// </summary>
    public string CacheKey => $"todos:priority:{Priority}";

    /// <summary>
    /// Cache for 5 minutes - priority filtered lists change with todo updates
    /// </summary>
    public TimeSpan? AbsoluteExpirationRelativeToNow => TimeSpan.FromMinutes(5);
}

/// <summary>
/// Query to get Todo statistics
/// </summary>
public class GetTodoStatisticsQuery : IQuery<TodoStatisticsDto>, ICacheableQuery<TodoStatisticsDto>
{
    /// <summary>
    /// Cache key for todo statistics
    /// </summary>
    public string CacheKey => "todos:stats";

    /// <summary>
    /// Cache for 2 minutes - statistics change frequently but don't need real-time updates
    /// </summary>
    public TimeSpan? AbsoluteExpirationRelativeToNow => TimeSpan.FromMinutes(2);
}

/// <summary>
/// DTO for Todo statistics
/// </summary>
public class TodoStatisticsDto
{
    public int TotalTodos { get; set; }
    public int CompletedTodos { get; set; }
    public int PendingTodos { get; set; }
    public double CompletionRate { get; set; }
    public Dictionary<string, int> TodosByPriority { get; set; } = new();
}

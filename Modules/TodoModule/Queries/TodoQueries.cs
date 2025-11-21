using Client.Common.CQRS;
using Client.Models;

namespace Client.Modules.TodoModule.Queries;

/// <summary>
/// Query to get all Todos list
/// </summary>
public class GetAllTodosQuery : IQuery<List<TodoDto>>
{
    public bool IncludeCompleted { get; set; } = true;
    public string? PriorityFilter { get; set; }
}

/// <summary>
/// Query to get Todo by ID
/// </summary>
public class GetTodoByIdQuery : IQuery<TodoDto?>
{
    public Guid Id { get; set; }

    public GetTodoByIdQuery(Guid id)
    {
        Id = id;
    }
}

/// <summary>
/// Query to get Todos by priority
/// </summary>
public class GetTodosByPriorityQuery : IQuery<List<TodoDto>>
{
    public string Priority { get; set; } = string.Empty;

    public GetTodosByPriorityQuery(string priority)
    {
        Priority = priority;
    }
}

/// <summary>
/// Query to get Todo statistics
/// </summary>
public class GetTodoStatisticsQuery : IQuery<TodoStatisticsDto>
{
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

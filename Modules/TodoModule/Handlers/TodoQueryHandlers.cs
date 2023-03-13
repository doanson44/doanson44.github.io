using AutoMapper;
using Client.Common.CQRS;
using Client.Models;
using Client.Modules.TodoModule.Queries;
using Client.Services;

namespace Client.Modules.TodoModule.Handlers;

/// <summary>
/// Handler for GetAllTodosQuery
/// Caching is handled by QueryCachingBehavior pipeline behavior.
/// </summary>
public class GetAllTodosQueryHandler : IQueryHandler<GetAllTodosQuery, List<TodoDto>>
{
    private readonly ILocalRepository<TodoItem> _repository;
    private readonly IMapper _mapper;

    public GetAllTodosQueryHandler(ILocalRepository<TodoItem> repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }

    public async Task<List<TodoDto>> HandleAsync(GetAllTodosQuery query, CancellationToken cancellationToken = default)
    {
        var items = await _repository.GetAllAsync(cancellationToken);

        // Apply filters
        var filteredItems = items.AsEnumerable();
        if (!query.IncludeCompleted)
        {
            filteredItems = filteredItems.Where(t => !t.IsCompleted);
        }

        if (!string.IsNullOrEmpty(query.PriorityFilter))
        {
            filteredItems = filteredItems.Where(t => t.Priority == query.PriorityFilter);
        }

        // Map to DTOs
        return _mapper.Map<List<TodoDto>>(filteredItems);
    }
}

/// <summary>
/// Handler for GetTodoByIdQuery
/// Caching is handled by QueryCachingBehavior pipeline behavior.
/// </summary>
public class GetTodoByIdQueryHandler : IQueryHandler<GetTodoByIdQuery, TodoDto?>
{
    private readonly ILocalRepository<TodoItem> _repository;
    private readonly IMapper _mapper;

    public GetTodoByIdQueryHandler(ILocalRepository<TodoItem> repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }

    public async Task<TodoDto?> HandleAsync(GetTodoByIdQuery query, CancellationToken cancellationToken = default)
    {
        var item = await _repository.GetByIdAsync(query.Id, cancellationToken);
        return item != null ? _mapper.Map<TodoDto>(item) : null;
    }
}

/// <summary>
/// Handler for GetTodosByPriorityQuery
/// Caching is handled by QueryCachingBehavior pipeline behavior.
/// </summary>
public class GetTodosByPriorityQueryHandler : IQueryHandler<GetTodosByPriorityQuery, List<TodoDto>>
{
    private readonly ILocalRepository<TodoItem> _repository;
    private readonly IMapper _mapper;

    public GetTodosByPriorityQueryHandler(ILocalRepository<TodoItem> repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }

    public async Task<List<TodoDto>> HandleAsync(GetTodosByPriorityQuery query, CancellationToken cancellationToken = default)
    {
        var items = await _repository.GetAllAsync(cancellationToken);
        var filteredItems = items.Where(t => t.Priority == query.Priority).ToList();
        return _mapper.Map<List<TodoDto>>(filteredItems);
    }
}

/// <summary>
/// Handler for GetTodoStatisticsQuery
/// Caching is handled by QueryCachingBehavior pipeline behavior.
/// </summary>
public class GetTodoStatisticsQueryHandler : IQueryHandler<GetTodoStatisticsQuery, TodoStatisticsDto>
{
    private readonly ILocalRepository<TodoItem> _repository;

    public GetTodoStatisticsQueryHandler(ILocalRepository<TodoItem> repository)
    {
        _repository = repository;
    }

    public async Task<TodoStatisticsDto> HandleAsync(GetTodoStatisticsQuery query, CancellationToken cancellationToken = default)
    {
        var allTodos = await _repository.GetAllAsync(cancellationToken);

        var stats = new TodoStatisticsDto
        {
            TotalTodos = allTodos.Count,
            CompletedTodos = allTodos.Count(t => t.IsCompleted),
            PendingTodos = allTodos.Count(t => !t.IsCompleted),
            CompletionRate = allTodos.Count > 0
                ? (double)allTodos.Count(t => t.IsCompleted) / allTodos.Count * 100
                : 0,
            TodosByPriority = allTodos
                .GroupBy(t => t.Priority)
                .ToDictionary(g => g.Key, g => g.Count())
        };

        return stats;
    }
}

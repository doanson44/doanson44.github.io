using AutoMapper;
using Client.Common.CQRS;
using Client.Models;
using Client.Modules.TodoModule.Commands;
using Client.Services;

namespace Client.Modules.TodoModule.Handlers;

/// <summary>
/// Handler for CreateTodoCommand
/// Cache invalidation is handled by CommandCachingBehavior pipeline behavior.
/// Sync to server is handled by CommandSyncBehavior pipeline behavior.
/// </summary>
public class CreateTodoCommandHandler : ICommandHandler<CreateTodoCommand>
{
    private readonly ILocalRepository<TodoItem> _repository;
    private readonly IMapper _mapper;

    public CreateTodoCommandHandler(ILocalRepository<TodoItem> repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }

    public async Task HandleAsync(CreateTodoCommand command, CancellationToken cancellationToken = default)
    {
        // Map command to entity
        var todoItem = _mapper.Map<TodoItem>(command);
        todoItem.CreatedAt = DateTime.UtcNow;

        // Save to local repository
        await _repository.AddAsync(todoItem, cancellationToken);
    }
}

/// <summary>
/// Handler for UpdateTodoCommand
/// Cache invalidation is handled by CommandCachingBehavior pipeline behavior.
/// Sync to server is handled by CommandSyncBehavior pipeline behavior.
/// </summary>
public class UpdateTodoCommandHandler : ICommandHandler<UpdateTodoCommand>
{
    private readonly ILocalRepository<TodoItem> _repository;
    private readonly IMapper _mapper;

    public UpdateTodoCommandHandler(ILocalRepository<TodoItem> repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }

    public async Task HandleAsync(UpdateTodoCommand command, CancellationToken cancellationToken = default)
    {
        // Map command to entity
        var todoItem = _mapper.Map<TodoItem>(command);

        // Update in local repository
        await _repository.UpdateAsync(todoItem, cancellationToken);
    }
}

/// <summary>
/// Handler for DeleteTodoCommand
/// Cache invalidation is handled by CommandCachingBehavior pipeline behavior.
/// Sync to server is handled by CommandSyncBehavior pipeline behavior.
/// </summary>
public class DeleteTodoCommandHandler : ICommandHandler<DeleteTodoCommand>
{
    private readonly ILocalRepository<TodoItem> _repository;

    public DeleteTodoCommandHandler(ILocalRepository<TodoItem> repository)
    {
        _repository = repository;
    }

    public async Task HandleAsync(DeleteTodoCommand command, CancellationToken cancellationToken = default)
    {
        // Delete from local repository
        await _repository.DeleteAsync(command.Id, cancellationToken);
    }
}

/// <summary>
/// Handler for MarkTodoCompletedCommand
/// Cache invalidation is handled by CommandCachingBehavior pipeline behavior.
/// Sync to server is handled by CommandSyncBehavior pipeline behavior.
/// </summary>
public class MarkTodoCompletedCommandHandler : ICommandHandler<MarkTodoCompletedCommand>
{
    private readonly ILocalRepository<TodoItem> _repository;

    public MarkTodoCompletedCommandHandler(ILocalRepository<TodoItem> repository)
    {
        _repository = repository;
    }

    public async Task HandleAsync(MarkTodoCompletedCommand command, CancellationToken cancellationToken = default)
    {
        var todo = await _repository.GetByIdAsync(command.Id, cancellationToken);
        if (todo != null)
        {
            todo.IsCompleted = true;
            todo.CompletedAt = DateTime.UtcNow;
            await _repository.UpdateAsync(todo, cancellationToken);
        }
    }
}

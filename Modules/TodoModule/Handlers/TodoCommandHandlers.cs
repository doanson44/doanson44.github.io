using AutoMapper;
using Client.Common.CQRS;
using Client.Models;
using Client.Modules.TodoModule.Commands;
using Client.Services;

namespace Client.Modules.TodoModule.Handlers;

/// <summary>
/// Handler for CreateTodoCommand
/// </summary>
public class CreateTodoCommandHandler : ICommandHandler<CreateTodoCommand>
{
    private readonly ITodoRepository _repository;
    private readonly IMapper _mapper;

    public CreateTodoCommandHandler(ITodoRepository repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }

    public async Task HandleAsync(CreateTodoCommand command, CancellationToken cancellationToken = default)
    {
        // Map command to entity
        var todoItem = _mapper.Map<TodoItem>(command);
        todoItem.CreatedAt = DateTime.UtcNow;

        // Save to repository
        await _repository.AddAsync(todoItem);
    }
}

/// <summary>
/// Handler for UpdateTodoCommand
/// </summary>
public class UpdateTodoCommandHandler : ICommandHandler<UpdateTodoCommand>
{
    private readonly ITodoRepository _repository;
    private readonly IMapper _mapper;

    public UpdateTodoCommandHandler(ITodoRepository repository, IMapper mapper)
    {
        _repository = repository;
        _mapper = mapper;
    }

    public async Task HandleAsync(UpdateTodoCommand command, CancellationToken cancellationToken = default)
    {
        // Map command to entity
        var todoItem = _mapper.Map<TodoItem>(command);

        // Update in repository
        await _repository.UpdateAsync(todoItem);
    }
}

/// <summary>
/// Handler for DeleteTodoCommand
/// </summary>
public class DeleteTodoCommandHandler : ICommandHandler<DeleteTodoCommand>
{
    private readonly ITodoRepository _repository;

    public DeleteTodoCommandHandler(ITodoRepository repository)
    {
        _repository = repository;
    }

    public async Task HandleAsync(DeleteTodoCommand command, CancellationToken cancellationToken = default)
    {
        // Delete from repository
        await _repository.DeleteAsync(command.Id);
    }
}

/// <summary>
/// Handler for MarkTodoCompletedCommand
/// </summary>
public class MarkTodoCompletedCommandHandler : ICommandHandler<MarkTodoCompletedCommand>
{
    private readonly ITodoRepository _repository;

    public MarkTodoCompletedCommandHandler(ITodoRepository repository)
    {
        _repository = repository;
    }

    public async Task HandleAsync(MarkTodoCompletedCommand command, CancellationToken cancellationToken = default)
    {
        var todo = await _repository.GetByIdAsync(command.Id);
        if (todo != null)
        {
            todo.IsCompleted = true;
            todo.CompletedAt = DateTime.UtcNow;
            await _repository.UpdateAsync(todo);
        }
    }
}

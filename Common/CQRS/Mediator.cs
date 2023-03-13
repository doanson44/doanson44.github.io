namespace Client.Common.CQRS;

/// <summary>
/// Mediator pattern to dispatch commands and queries
/// Replaces direct handler injection
/// </summary>
public interface IMediator
{
    Task SendAsync<TCommand>(TCommand command, CancellationToken cancellationToken = default)
        where TCommand : ICommand;

    Task<TResult> SendAsync<TCommand, TResult>(TCommand command, CancellationToken cancellationToken = default)
        where TCommand : ICommand;

    Task<TResult> QueryAsync<TQuery, TResult>(TQuery query, CancellationToken cancellationToken = default)
        where TQuery : IQuery<TResult>;
}

/// <summary>
/// Simple implementation of Mediator with pipeline behavior support
/// In production, consider using a library like MediatR
/// </summary>
public class Mediator : IMediator
{
    private readonly IServiceProvider _serviceProvider;
    private readonly IServiceScopeFactory _scopeFactory;

    public Mediator(IServiceProvider serviceProvider, IServiceScopeFactory scopeFactory)
    {
        _serviceProvider = serviceProvider;
        _scopeFactory = scopeFactory;
    }

    private async Task ExecuteWithScopeAsync<TCommand>(TCommand command, Func<IServiceProvider, Task> action, CancellationToken cancellationToken)
        where TCommand : ICommand
    {
        using var scope = _scopeFactory.CreateScope();
        var scopedProvider = scope.ServiceProvider;

        var handlerType = typeof(ICommandHandler<TCommand>);
        var handler = scopedProvider.GetService(handlerType) as ICommandHandler<TCommand>
            ?? throw new InvalidOperationException($"No handler found for {typeof(TCommand).Name}");

        // Check for pipeline behavior - behaviors are resolved from root provider as they are singletons
        var behaviorType = typeof(ICommandPipelineBehavior<TCommand>);
        var behavior = _serviceProvider.GetService(behaviorType) as ICommandPipelineBehavior<TCommand>;

        if (behavior != null)
        {
            // Execute with behavior
            await behavior.HandleAsync(command, () => action(scopedProvider), cancellationToken);
        }
        else
        {
            // Execute without behavior
            await action(scopedProvider);
        }
    }

    private async Task<TResult> ExecuteQueryWithScopeAsync<TQuery, TResult>(TQuery query, Func<IServiceProvider, Task<TResult>> action, CancellationToken cancellationToken)
        where TQuery : IQuery<TResult>
    {
        using var scope = _scopeFactory.CreateScope();
        var scopedProvider = scope.ServiceProvider;

        var handlerType = typeof(IQueryHandler<TQuery, TResult>);
        var handler = scopedProvider.GetService(handlerType) as IQueryHandler<TQuery, TResult>
            ?? throw new InvalidOperationException($"No handler found for {typeof(TQuery).Name}");

        // Check for pipeline behavior - behaviors are resolved from root provider as they are singletons
        var behaviorType = typeof(IQueryPipelineBehavior<TQuery, TResult>);
        var behavior = _serviceProvider.GetService(behaviorType) as IQueryPipelineBehavior<TQuery, TResult>;

        if (behavior != null)
        {
            // Execute with behavior
            return await behavior.HandleAsync(query, () => action(scopedProvider), cancellationToken);
        }
        else
        {
            // Execute without behavior
            return await action(scopedProvider);
        }
    }

    public async Task SendAsync<TCommand>(TCommand command, CancellationToken cancellationToken = default)
        where TCommand : ICommand
    {
        await ExecuteWithScopeAsync(command, async (scopedProvider) =>
        {
            var handlerType = typeof(ICommandHandler<TCommand>);
            var handler = scopedProvider.GetService(handlerType) as ICommandHandler<TCommand>
                ?? throw new InvalidOperationException($"No handler found for {typeof(TCommand).Name}");
            await handler.HandleAsync(command, cancellationToken);
        }, cancellationToken);
    }

    public async Task<TResult> SendAsync<TCommand, TResult>(TCommand command, CancellationToken cancellationToken = default)
        where TCommand : ICommand
    {
        using var scope = _scopeFactory.CreateScope();
        var scopedProvider = scope.ServiceProvider;

        var handlerType = typeof(ICommandHandler<TCommand, TResult>);
        var handler = scopedProvider.GetService(handlerType) as ICommandHandler<TCommand, TResult>
            ?? throw new InvalidOperationException($"No handler found for {typeof(TCommand).Name}");

        // Commands with results typically don't use behaviors, so just execute
        return await handler.HandleAsync(command, cancellationToken);
    }

    public async Task<TResult> QueryAsync<TQuery, TResult>(TQuery query, CancellationToken cancellationToken = default)
        where TQuery : IQuery<TResult>
    {
        return await ExecuteQueryWithScopeAsync(query, async (scopedProvider) =>
        {
            var handlerType = typeof(IQueryHandler<TQuery, TResult>);
            var handler = scopedProvider.GetService(handlerType) as IQueryHandler<TQuery, TResult>
                ?? throw new InvalidOperationException($"No handler found for {typeof(TQuery).Name}");
            return await handler.HandleAsync(query, cancellationToken);
        }, cancellationToken);
    }
}

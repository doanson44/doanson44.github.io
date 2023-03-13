namespace Client.Common.CQRS;

/// <summary>
/// Pipeline behavior for commands
/// </summary>
public interface ICommandPipelineBehavior<TCommand>
    where TCommand : ICommand
{
    Task HandleAsync(TCommand command, Func<Task> next, CancellationToken cancellationToken = default);
}

/// <summary>
/// Pipeline behavior for queries
/// </summary>
public interface IQueryPipelineBehavior<TQuery, TResult>
    where TQuery : IQuery<TResult>
{
    Task<TResult> HandleAsync(TQuery query, Func<Task<TResult>> next, CancellationToken cancellationToken = default);
}
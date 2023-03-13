using Client.Common.Caching;
using Client.Services;
using System.Text.Json;

namespace Client.Common.CQRS;

/// <summary>
/// Caching behavior for queries that implement ICacheableQuery
/// </summary>
public class QueryCachingBehavior<TQuery, TResult> : IQueryPipelineBehavior<TQuery, TResult>
    where TQuery : IQuery<TResult>
{
    private readonly ICache _cache;

    public QueryCachingBehavior(ICache cache)
    {
        _cache = cache;
    }

    public async Task<TResult> HandleAsync(TQuery query, Func<Task<TResult>> next, CancellationToken cancellationToken = default)
    {
        // Only apply caching if query implements ICacheableQuery<TResult>
        if (query is ICacheableQuery<TResult> cacheableQuery)
        {
            return await _cache.GetOrAddAsync(
                cacheableQuery.CacheKey,
                async () => await next(),
                cacheableQuery.AbsoluteExpirationRelativeToNow,
                cancellationToken
            ) ?? default(TResult)!;
        }

        // If not cacheable, just call the handler
        return await next();
    }
}

/// <summary>
/// Cache invalidation behavior for commands that implement IInvalidateCacheCommand
/// </summary>
public class CommandCachingBehavior<TCommand> : ICommandPipelineBehavior<TCommand>
    where TCommand : ICommand
{
    private readonly ICache _cache;

    public CommandCachingBehavior(ICache cache)
    {
        _cache = cache;
    }

    public async Task HandleAsync(TCommand command, Func<Task> next, CancellationToken cancellationToken = default)
    {
        // Execute the command first
        await next();

        // Then invalidate cache if command implements IInvalidateCacheCommand
        if (command is IInvalidateCacheCommand invalidateCommand)
        {
            foreach (var cacheKey in invalidateCommand.CacheKeysToInvalidate)
            {
                await _cache.RemoveAsync(cacheKey, cancellationToken);
            }
        }
    }
}

/// <summary>
/// Sync behavior for commands to enqueue them for server synchronization
/// </summary>
public class CommandSyncBehavior<TCommand> : ICommandPipelineBehavior<TCommand>
    where TCommand : ICommand
{
    private readonly IServiceScopeFactory _scopeFactory;

    public CommandSyncBehavior(IServiceScopeFactory scopeFactory)
    {
        _scopeFactory = scopeFactory;
    }

    public async Task HandleAsync(TCommand command, Func<Task> next, CancellationToken cancellationToken = default)
    {
        // Execute the command first
        await next();

        // Then enqueue command for server sync using a scoped service
        using var scope = _scopeFactory.CreateScope();
        var scopedProvider = scope.ServiceProvider;
        var commandOutbox = scopedProvider.GetRequiredService<ICommandOutbox>();

        var pendingCommand = new PendingCommand
        {
            CommandType = typeof(TCommand).Name,
            PayloadJson = JsonSerializer.Serialize(command)
        };
        await commandOutbox.EnqueueAsync(pendingCommand, cancellationToken);
    }
}
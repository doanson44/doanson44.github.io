namespace Client.Common.Caching;

/// <summary>
/// Interface for queries that support caching.
/// Queries implementing this interface will be cached automatically.
/// </summary>
/// <typeparam name="TResult">The result type of the query</typeparam>
public interface ICacheableQuery<TResult>
{
    /// <summary>
    /// Gets the cache key for this query.
    /// Should be unique and descriptive, e.g., "TodoList_All" or $"TodoItem_{Id}".
    /// </summary>
    string CacheKey { get; }

    /// <summary>
    /// Gets the absolute expiration time relative to now.
    /// Default recommendation: TimeSpan.FromMinutes(5).
    /// </summary>
    TimeSpan? AbsoluteExpirationRelativeToNow { get; }
}
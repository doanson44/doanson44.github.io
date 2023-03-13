namespace Client.Common.Caching;

/// <summary>
/// Centralized cache interface for the application.
/// Provides async caching operations with expiration support.
/// </summary>
public interface ICache
{
    /// <summary>
    /// Gets a cached value or adds it using the factory function if not found.
    /// </summary>
    /// <typeparam name="T">Type of value to cache</typeparam>
    /// <param name="key">Cache key</param>
    /// <param name="factory">Function to create the value if not cached</param>
    /// <param name="absoluteExpiration">Optional expiration time</param>
    /// <param name="cancellationToken">Cancellation token</param>
    /// <returns>The cached or newly created value</returns>
    Task<T?> GetOrAddAsync<T>(
        string key,
        Func<Task<T>> factory,
        TimeSpan? absoluteExpiration = null,
        CancellationToken cancellationToken = default);

    /// <summary>
    /// Removes an item from the cache.
    /// </summary>
    /// <param name="key">Cache key to remove</param>
    /// <param name="cancellationToken">Cancellation token</param>
    Task RemoveAsync(string key, CancellationToken cancellationToken = default);
}
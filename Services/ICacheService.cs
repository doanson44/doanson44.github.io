namespace Client.Services;

/// <summary>
/// Generic cache service interface for storing and retrieving data.
/// Supports automatic cache population and expiration.
/// </summary>
public interface ICacheService
{
    /// <summary>
    /// Gets a cached value or adds it using the factory function if not found.
    /// </summary>
    /// <typeparam name="T">Type of value to cache</typeparam>
    /// <param name="key">Cache key</param>
    /// <param name="factory">Function to create the value if not cached</param>
    /// <param name="absoluteExpiration">Optional expiration time</param>
    /// <returns>The cached or newly created value</returns>
    Task<T?> GetOrAddAsync<T>(string key, Func<Task<T>> factory, TimeSpan? absoluteExpiration = null);

    /// <summary>
    /// Removes an item from the cache.
    /// </summary>
    /// <param name="key">Cache key to remove</param>
    void Remove(string key);

    /// <summary>
    /// Clears all items from the cache.
    /// </summary>
    void Clear();
}

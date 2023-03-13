using Microsoft.Extensions.Caching.Memory;

namespace Client.Common.Caching;

/// <summary>
/// In-memory implementation of ICache using Microsoft.Extensions.Caching.Memory.
/// Provides thread-safe caching with expiration support.
/// </summary>
public class InMemoryCache : ICache
{
    private readonly IMemoryCache _memoryCache;

    public InMemoryCache(IMemoryCache memoryCache)
    {
        _memoryCache = memoryCache;
    }

    /// <inheritdoc />
    public async Task<T?> GetOrAddAsync<T>(
        string key,
        Func<Task<T>> factory,
        TimeSpan? absoluteExpiration = null,
        CancellationToken cancellationToken = default)
    {
        // Check if already cached
        if (_memoryCache.TryGetValue(key, out T? cachedValue))
        {
            return cachedValue;
        }

        // Not cached, execute factory
        var value = await factory();

        // Cache the result
        var cacheEntryOptions = new MemoryCacheEntryOptions();

        if (absoluteExpiration.HasValue)
        {
            cacheEntryOptions.AbsoluteExpirationRelativeToNow = absoluteExpiration.Value;
        }
        else
        {
            // Default expiration: 5 minutes
            cacheEntryOptions.AbsoluteExpirationRelativeToNow = TimeSpan.FromMinutes(5);
        }

        _memoryCache.Set(key, value, cacheEntryOptions);

        return value;
    }

    /// <inheritdoc />
    public Task RemoveAsync(string key, CancellationToken cancellationToken = default)
    {
        _memoryCache.Remove(key);
        return Task.CompletedTask;
    }
}
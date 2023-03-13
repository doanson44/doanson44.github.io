using Microsoft.Extensions.Caching.Memory;

namespace Client.Services;

/// <summary>
/// In-memory implementation of ICacheService using IMemoryCache.
/// Stores cached data in memory for fast access.
/// </summary>
public class InMemoryCacheService : ICacheService
{
    private readonly IMemoryCache _cache;

    public InMemoryCacheService(IMemoryCache cache)
    {
        _cache = cache;
    }

    /// <inheritdoc/>
    public async Task<T?> GetOrAddAsync<T>(string key, Func<Task<T>> factory, TimeSpan? absoluteExpiration = null)
    {
        // Try to get from cache
        if (_cache.TryGetValue(key, out T? cachedValue))
        {
            return cachedValue;
        }

        // Not in cache, create new value
        var value = await factory();

        // Set cache options
        var cacheEntryOptions = new MemoryCacheEntryOptions();
        if (absoluteExpiration.HasValue)
        {
            cacheEntryOptions.SetAbsoluteExpiration(absoluteExpiration.Value);
        }
        else
        {
            // Default: cache for 5 minutes
            cacheEntryOptions.SetAbsoluteExpiration(TimeSpan.FromMinutes(5));
        }

        // Store in cache
        _cache.Set(key, value, cacheEntryOptions);

        return value;
    }

    /// <inheritdoc/>
    public void Remove(string key)
    {
        _cache.Remove(key);
    }

    /// <inheritdoc/>
    public void Clear()
    {
        // IMemoryCache doesn't have a Clear method, so we need to track keys or use a wrapper
        // For simplicity, we'll dispose and recreate if needed
        // In production, consider using a different approach or MemoryCache with tracking
        if (_cache is MemoryCache memCache)
        {
            memCache.Compact(1.0); // Remove all entries
        }
    }
}

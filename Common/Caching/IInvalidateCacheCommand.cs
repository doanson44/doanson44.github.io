namespace Client.Common.Caching;

/// <summary>
/// Interface for commands that invalidate cache entries.
/// Commands implementing this interface will automatically invalidate specified cache keys after successful execution.
/// </summary>
public interface IInvalidateCacheCommand
{
    /// <summary>
    /// Gets the cache keys that should be invalidated when this command executes successfully.
    /// Examples: "TodoList_All", $"TodoItem_{Id}".
    /// </summary>
    IEnumerable<string> CacheKeysToInvalidate { get; }
}
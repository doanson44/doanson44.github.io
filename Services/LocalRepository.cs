using System.Collections.ObjectModel;

namespace Client.Services;

/// <summary>
/// Interface for local repository operations.
/// </summary>
public interface ILocalRepository<T>
{
    Task<IReadOnlyList<T>> GetAllAsync(CancellationToken cancellationToken = default);
    Task<T?> GetByIdAsync(Guid id, CancellationToken cancellationToken = default);
    Task AddAsync(T entity, CancellationToken cancellationToken = default);
    Task UpdateAsync(T entity, CancellationToken cancellationToken = default);
    Task DeleteAsync(Guid id, CancellationToken cancellationToken = default);
}

/// <summary>
/// Implementation of ILocalRepository using ILocalStore.
/// </summary>
public class LocalRepository<T> : ILocalRepository<T> where T : class, new()
{
    private readonly ILocalStore _localStore;
    private readonly string _storageKey;
    private readonly Func<T, Guid> _getId;
    private readonly Action<T, Guid> _setId;

    public LocalRepository(ILocalStore localStore, string storageKey, Func<T, Guid> getId, Action<T, Guid> setId)
    {
        _localStore = localStore;
        _storageKey = storageKey;
        _getId = getId;
        _setId = setId;
    }

    public async Task<IReadOnlyList<T>> GetAllAsync(CancellationToken cancellationToken = default)
    {
        var items = await _localStore.LoadAsync<List<T>>(_storageKey, cancellationToken) ?? new List<T>();
        return new ReadOnlyCollection<T>(items);
    }

    public async Task<T?> GetByIdAsync(Guid id, CancellationToken cancellationToken = default)
    {
        var items = await GetAllAsync(cancellationToken);
        return items.FirstOrDefault(item => _getId(item) == id);
    }

    public async Task AddAsync(T entity, CancellationToken cancellationToken = default)
    {
        var items = (await GetAllAsync(cancellationToken)).ToList();
        if (_getId(entity) == Guid.Empty)
        {
            _setId(entity, Guid.NewGuid());
        }
        items.Add(entity);
        await _localStore.SaveAsync(_storageKey, items, cancellationToken);
    }

    public async Task UpdateAsync(T entity, CancellationToken cancellationToken = default)
    {
        var items = (await GetAllAsync(cancellationToken)).ToList();
        var index = items.FindIndex(item => _getId(item) == _getId(entity));
        if (index >= 0)
        {
            items[index] = entity;
            await _localStore.SaveAsync(_storageKey, items, cancellationToken);
        }
    }

    public async Task DeleteAsync(Guid id, CancellationToken cancellationToken = default)
    {
        var items = (await GetAllAsync(cancellationToken)).ToList();
        items.RemoveAll(item => _getId(item) == id);
        await _localStore.SaveAsync(_storageKey, items, cancellationToken);
    }
}
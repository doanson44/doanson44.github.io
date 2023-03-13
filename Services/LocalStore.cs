namespace Client.Services;

/// <summary>
/// Interface for local key-value storage.
/// </summary>
public interface ILocalStore
{
    Task SaveAsync<T>(string key, T value, CancellationToken cancellationToken = default);
    Task<T?> LoadAsync<T>(string key, CancellationToken cancellationToken = default);
    Task RemoveAsync(string key, CancellationToken cancellationToken = default);
}

/// <summary>
/// Implementation of ILocalStore using Blazored.LocalStorage.
/// </summary>
public class LocalStorageStore : ILocalStore
{
    private readonly Blazored.LocalStorage.ILocalStorageService _localStorage;

    public LocalStorageStore(Blazored.LocalStorage.ILocalStorageService localStorage)
    {
        _localStorage = localStorage;
    }

    public async Task SaveAsync<T>(string key, T value, CancellationToken cancellationToken = default)
    {
        await _localStorage.SetItemAsync(key, value, cancellationToken);
    }

    public async Task<T?> LoadAsync<T>(string key, CancellationToken cancellationToken = default)
    {
        return await _localStorage.GetItemAsync<T>(key, cancellationToken);
    }

    public async Task RemoveAsync(string key, CancellationToken cancellationToken = default)
    {
        await _localStorage.RemoveItemAsync(key, cancellationToken);
    }
}
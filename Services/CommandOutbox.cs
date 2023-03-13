namespace Client.Services;

/// <summary>
/// Represents a pending command to be synced with the server.
/// </summary>
public sealed class PendingCommand
{
    public Guid Id { get; set; } = Guid.NewGuid();
    public string CommandType { get; set; } = string.Empty;
    public string PayloadJson { get; set; } = string.Empty;
    public DateTimeOffset CreatedAt { get; set; } = DateTimeOffset.UtcNow;
}

/// <summary>
/// Interface for command outbox operations.
/// </summary>
public interface ICommandOutbox
{
    Task EnqueueAsync(PendingCommand command, CancellationToken cancellationToken = default);
    Task<IReadOnlyList<PendingCommand>> GetAllAsync(CancellationToken cancellationToken = default);
    Task RemoveAsync(Guid id, CancellationToken cancellationToken = default);
}

/// <summary>
/// Implementation of ICommandOutbox using ILocalStore.
/// </summary>
public class LocalCommandOutbox : ICommandOutbox
{
    private readonly ILocalStore _localStore;
    private const string StorageKey = "pending-commands";

    public LocalCommandOutbox(ILocalStore localStore)
    {
        _localStore = localStore;
    }

    public async Task EnqueueAsync(PendingCommand command, CancellationToken cancellationToken = default)
    {
        var commands = (await GetAllAsync(cancellationToken)).ToList();
        commands.Add(command);
        await _localStore.SaveAsync(StorageKey, commands, cancellationToken);
    }

    public async Task<IReadOnlyList<PendingCommand>> GetAllAsync(CancellationToken cancellationToken = default)
    {
        var commands = await _localStore.LoadAsync<List<PendingCommand>>(StorageKey, cancellationToken) ?? new List<PendingCommand>();
        return commands.AsReadOnly();
    }

    public async Task RemoveAsync(Guid id, CancellationToken cancellationToken = default)
    {
        var commands = (await GetAllAsync(cancellationToken)).ToList();
        commands.RemoveAll(c => c.Id == id);
        await _localStore.SaveAsync(StorageKey, commands, cancellationToken);
    }
}
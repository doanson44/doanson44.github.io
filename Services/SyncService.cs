namespace Client.Services;

/// <summary>
/// Interface for synchronization service.
/// </summary>
public interface ISyncService
{
    Task SyncAsync(CancellationToken cancellationToken = default);
}

/// <summary>
/// Implementation of ISyncService.
/// Processes pending commands and syncs with the server.
/// </summary>
public class SyncService : ISyncService
{
    private readonly ICommandOutbox _commandOutbox;
    private readonly HttpClient _httpClient;

    public SyncService(ICommandOutbox commandOutbox, HttpClient httpClient)
    {
        _commandOutbox = commandOutbox;
        _httpClient = httpClient;
    }

    public async Task SyncAsync(CancellationToken cancellationToken = default)
    {
        var pendingCommands = await _commandOutbox.GetAllAsync(cancellationToken);

        foreach (var command in pendingCommands)
        {
            try
            {
                // Send to server based on command type
                var success = await SendToServerAsync(command, cancellationToken);
                if (success)
                {
                    await _commandOutbox.RemoveAsync(command.Id, cancellationToken);
                }
            }
            catch
            {
                // If sync fails, keep the command in the outbox
                // Could add retry logic or exponential backoff here
            }
        }
    }

    private async Task<bool> SendToServerAsync(PendingCommand command, CancellationToken cancellationToken)
    {
        // Placeholder: Implement actual server sync logic based on command type
        // For now, assume success for demonstration
        // In real implementation, deserialize payload and call appropriate API

        // Example:
        // if (command.CommandType == "CreateTodo")
        // {
        //     var createCommand = JsonSerializer.Deserialize<CreateTodoCommand>(command.PayloadJson);
        //     var response = await _httpClient.PostAsJsonAsync("api/todos", createCommand, cancellationToken);
        //     return response.IsSuccessStatusCode;
        // }

        return true; // Placeholder
    }
}
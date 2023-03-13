using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Routing;

namespace Client.Services;

/// <summary>
/// Service to trigger synchronization on navigation and startup.
/// </summary>
public class SyncTriggerService : IDisposable
{
    private readonly NavigationManager _navigationManager;
    private readonly ISyncService _syncService;

    public SyncTriggerService(NavigationManager navigationManager, ISyncService syncService)
    {
        _navigationManager = navigationManager;
        _syncService = syncService;

        // Subscribe to location changed
        _navigationManager.LocationChanged += OnLocationChanged;
    }

    private void OnLocationChanged(object? sender, LocationChangedEventArgs e)
    {
        // Trigger sync on navigation
        _ = _syncService.SyncAsync();
    }

    public void Dispose()
    {
        _navigationManager.LocationChanged -= OnLocationChanged;
    }

    /// <summary>
    /// Trigger initial sync on app startup.
    /// </summary>
    public async Task InitializeAsync()
    {
        await _syncService.SyncAsync();
    }
}
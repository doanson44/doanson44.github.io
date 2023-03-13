using Blazored.LocalStorage;
using Client;
using Client.Common.Caching;
using Client.Common.CQRS;
using Client.Services;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Microsoft.Extensions.Localization;
using Microsoft.Extensions.Localization;
using Radzen;
using Serilog;
using Serilog.Core;
using System.Globalization;

// Configure Serilog
Log.Logger = new LoggerConfiguration()
    .MinimumLevel.Debug()
    .WriteTo.BrowserConsole(
        outputTemplate: "[{Timestamp:HH:mm:ss} {Level:u3}] {Message:lj}{NewLine}{Exception}")
    .Enrich.FromLogContext()
    .Enrich.WithProperty("Application", "TodoApp")
    .Enrich.WithProperty("Environment", "Client")
    .CreateLogger();

try
{
    Log.Information("Starting Blazor WebAssembly application");

    var builder = WebAssemblyHostBuilder.CreateDefault(args);
    
    // Configure for hash-based routing
    builder.RootComponents.Add<App>("#app");
    builder.RootComponents.Add<HeadOutlet>("head::after");

    // Add localization
    builder.Services.AddLocalization(options => options.ResourcesPath = "Resources");
    var supportedCultures = new[]
    {
        new CultureInfo("en-US"),
        new CultureInfo("vi-VN")
    };

    // Add Serilog
    builder.Logging.ClearProviders();
    builder.Logging.AddSerilog(dispose: true);

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });

// Register Radzen services
builder.Services.AddRadzenComponents();
builder.Services.AddScoped<DialogService>();
builder.Services.AddScoped<NotificationService>();

// Register Blazored LocalStorage
builder.Services.AddBlazoredLocalStorage();

// Register AutoMapper
builder.Services.AddAutoMapper(typeof(Program).Assembly);

// Register caching service
builder.Services.AddMemoryCache();
builder.Services.AddSingleton<ICacheService, InMemoryCacheService>();
builder.Services.AddSingleton<ICache, InMemoryCache>();

// Register caching behaviors
builder.Services.AddSingleton(typeof(IQueryPipelineBehavior<,>), typeof(QueryCachingBehavior<,>));
builder.Services.AddSingleton(typeof(ICommandPipelineBehavior<>), typeof(CommandCachingBehavior<>));
builder.Services.AddSingleton(typeof(ICommandPipelineBehavior<>), typeof(CommandSyncBehavior<>));

// Register local storage services
builder.Services.AddScoped<ILocalStore, LocalStorageStore>();
builder.Services.AddScoped<ICommandOutbox, LocalCommandOutbox>();
builder.Services.AddScoped<ISyncService, SyncService>();
builder.Services.AddScoped<SyncTriggerService>();

// Register local repository for TodoItem
builder.Services.AddScoped<ILocalRepository<Client.Models.TodoItem>>(sp =>
{
    var localStore = sp.GetRequiredService<ILocalStore>();
    return new LocalRepository<Client.Models.TodoItem>(
        localStore,
        "todos",
        t => t.Id,
        (t, id) => t.Id = id
    );
});

// Register localization service
builder.Services.AddScoped<LocalizationService>();
builder.Services.AddScoped<CultureService>();

// Register Mediator pattern
builder.Services.AddSingleton<IMediator, Mediator>();

// Register CQRS Command and Query Handlers automatically
builder.Services.AddCQRSHandlers();

    var app = builder.Build();

    // Initialize culture
    var cultureService = app.Services.GetRequiredService<CultureService>();
    await cultureService.InitializeCultureAsync();

    await app.RunAsync();
    
    Log.Information("Application stopped successfully");
}
catch (Exception ex)
{
    Log.Fatal(ex, "Application terminated unexpectedly");
}
finally
{
    await Log.CloseAndFlushAsync();
}

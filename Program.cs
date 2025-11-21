using Client;
using Client.Common.CQRS;
using Client.Services;
using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Radzen;

var builder = WebAssemblyHostBuilder.CreateDefault(args);

// Configure for hash-based routing
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });

// Register Radzen services
builder.Services.AddRadzenComponents();
builder.Services.AddScoped<DialogService>();
builder.Services.AddScoped<NotificationService>();

// Register AutoMapper
builder.Services.AddAutoMapper(typeof(Program).Assembly);

// Register in-memory repository for TodoItem
builder.Services.AddSingleton<ITodoRepository, InMemoryTodoRepository>();

// Register Mediator pattern
builder.Services.AddSingleton<IMediator, Mediator>();

// Register CQRS Command and Query Handlers automatically
builder.Services.AddCQRSHandlers();

await builder.Build().RunAsync();

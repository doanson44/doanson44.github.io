using System.Reflection;

namespace Client.Common.CQRS;

/// <summary>
/// Extension methods for registering CQRS handlers
/// </summary>
public static class CQRSServiceCollectionExtensions
{
    /// <summary>
    /// Registers all CQRS command and query handlers from the specified assemblies
    /// </summary>
    /// <param name="services">The service collection</param>
    /// <param name="assemblies">The assemblies to scan for handlers</param>
    /// <returns>The service collection for chaining</returns>
    public static IServiceCollection AddCQRSHandlers(this IServiceCollection services, params Assembly[] assemblies)
    {
        if (assemblies.Length == 0)
        {
            assemblies = new[] { Assembly.GetExecutingAssembly() };
        }

        foreach (var assembly in assemblies)
        {
            // Register command handlers
            var commandHandlerTypes = assembly.GetTypes()
                .Where(t => t.IsClass && !t.IsAbstract)
                .SelectMany(t => t.GetInterfaces()
                    .Where(i => i.IsGenericType && i.GetGenericTypeDefinition() == typeof(ICommandHandler<>))
                    .Select(i => new { HandlerType = t, InterfaceType = i }));

            foreach (var handler in commandHandlerTypes)
            {
                services.AddTransient(handler.InterfaceType, handler.HandlerType);
            }

            // Register query handlers
            var queryHandlerTypes = assembly.GetTypes()
                .Where(t => t.IsClass && !t.IsAbstract)
                .SelectMany(t => t.GetInterfaces()
                    .Where(i => i.IsGenericType && i.GetGenericTypeDefinition() == typeof(IQueryHandler<,>))
                    .Select(i => new { HandlerType = t, InterfaceType = i }));

            foreach (var handler in queryHandlerTypes)
            {
                services.AddTransient(handler.InterfaceType, handler.HandlerType);
            }
        }

        return services;
    }
}
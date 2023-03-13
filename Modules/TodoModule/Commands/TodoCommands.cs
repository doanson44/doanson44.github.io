using Client.Common.Caching;
using Client.Common.CQRS;
using System.ComponentModel.DataAnnotations;

namespace Client.Modules.TodoModule.Commands;

/// <summary>
/// Command to create a new Todo
/// Invalidates: todos:all:* (all todo lists), todos:stats (statistics)
/// </summary>
public class CreateTodoCommand : ICommand, IInvalidateCacheCommand
{
    [Required(ErrorMessage = "Title is required")]
    [StringLength(100, ErrorMessage = "Title cannot exceed 100 characters")]
    public string Title { get; set; } = string.Empty;

    [StringLength(500, ErrorMessage = "Description cannot exceed 500 characters")]
    public string Description { get; set; } = string.Empty;

    [Required(ErrorMessage = "Priority is required")]
    public string Priority { get; set; } = "Medium";

    public bool IsCompleted { get; set; } = false;

    /// <summary>
    /// Cache keys to invalidate after creating a todo
    /// </summary>
    public IEnumerable<string> CacheKeysToInvalidate => new[]
    {
        "todos:all:true:all",    // All todos list
        "todos:all:false:all",   // Pending todos only
        "todos:stats"            // Statistics
    };
}

/// <summary>
/// Command to update Todo
/// Invalidates: todos:all:* (all todo lists), todos:{Id} (specific item), todos:stats (statistics)
/// </summary>
public class UpdateTodoCommand : ICommand, IInvalidateCacheCommand
{
    public Guid Id { get; set; }

    [Required(ErrorMessage = "Title is required")]
    [StringLength(100, ErrorMessage = "Title cannot exceed 100 characters")]
    public string Title { get; set; } = string.Empty;

    [StringLength(500, ErrorMessage = "Description cannot exceed 500 characters")]
    public string Description { get; set; } = string.Empty;

    [Required(ErrorMessage = "Priority is required")]
    public string Priority { get; set; } = "Medium";

    public bool IsCompleted { get; set; }

    /// <summary>
    /// Cache keys to invalidate after updating a todo
    /// </summary>
    public IEnumerable<string> CacheKeysToInvalidate => new[]
    {
        "todos:all:true:all",        // All todos list
        "todos:all:false:all",       // Pending todos only
        $"todos:{Id}",               // Specific todo item
        "todos:stats"                // Statistics
    };
}

/// <summary>
/// Command to delete Todo
/// Invalidates: todos:all:* (all todo lists), todos:{Id} (specific item), todos:stats (statistics)
/// </summary>
public class DeleteTodoCommand : ICommand, IInvalidateCacheCommand
{
    public Guid Id { get; set; }

    public DeleteTodoCommand(Guid id)
    {
        Id = id;
    }

    /// <summary>
    /// Cache keys to invalidate after deleting a todo
    /// </summary>
    public IEnumerable<string> CacheKeysToInvalidate => new[]
    {
        "todos:all:true:all",        // All todos list
        "todos:all:false:all",       // Pending todos only
        $"todos:{Id}",               // Specific todo item
        "todos:stats"                // Statistics
    };
}

/// <summary>
/// Command to mark Todo as completed
/// Invalidates: todos:all:* (all todo lists), todos:{Id} (specific item), todos:stats (statistics)
/// </summary>
public class MarkTodoCompletedCommand : ICommand, IInvalidateCacheCommand
{
    public Guid Id { get; set; }

    public MarkTodoCompletedCommand(Guid id)
    {
        Id = id;
    }

    /// <summary>
    /// Cache keys to invalidate after marking todo as completed
    /// </summary>
    public IEnumerable<string> CacheKeysToInvalidate => new[]
    {
        "todos:all:true:all",        // All todos list
        "todos:all:false:all",       // Pending todos only
        $"todos:{Id}",               // Specific todo item
        "todos:stats"                // Statistics
    };
}

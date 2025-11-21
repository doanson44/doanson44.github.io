using Client.Common.CQRS;
using System.ComponentModel.DataAnnotations;

namespace Client.Modules.TodoModule.Commands;

/// <summary>
/// Command to create a new Todo
/// </summary>
public class CreateTodoCommand : ICommand
{
    [Required(ErrorMessage = "Title is required")]
    [StringLength(100, ErrorMessage = "Title cannot exceed 100 characters")]
    public string Title { get; set; } = string.Empty;

    [StringLength(500, ErrorMessage = "Description cannot exceed 500 characters")]
    public string Description { get; set; } = string.Empty;

    [Required(ErrorMessage = "Priority is required")]
    public string Priority { get; set; } = "Medium";

    public bool IsCompleted { get; set; } = false;
}

/// <summary>
/// Command to update Todo
/// </summary>
public class UpdateTodoCommand : ICommand
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
}

/// <summary>
/// Command to delete Todo
/// </summary>
public class DeleteTodoCommand : ICommand
{
    public Guid Id { get; set; }

    public DeleteTodoCommand(Guid id)
    {
        Id = id;
    }
}

/// <summary>
/// Command to mark Todo as completed
/// </summary>
public class MarkTodoCompletedCommand : ICommand
{
    public Guid Id { get; set; }

    public MarkTodoCompletedCommand(Guid id)
    {
        Id = id;
    }
}

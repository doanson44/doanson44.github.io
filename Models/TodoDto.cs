namespace Client.Models;

/// <summary>
/// Data Transfer Object for Todo items.
/// Used for passing data between layers.
/// </summary>
public class TodoDto
{
    public Guid Id { get; set; }
    public string Title { get; set; } = string.Empty;
    public string Description { get; set; } = string.Empty;
    public bool IsCompleted { get; set; }
    public DateTime CreatedAt { get; set; }
    public DateTime? CompletedAt { get; set; }
    public string Priority { get; set; } = "Medium";
}

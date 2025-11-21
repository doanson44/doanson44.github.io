using Client.Models;

namespace Client.Services;

/// <summary>
/// Repository interface for Todo items.
/// Defines data access operations.
/// </summary>
public interface ITodoRepository
{
    Task<List<TodoItem>> GetAllAsync();
    Task<TodoItem?> GetByIdAsync(Guid id);
    Task<TodoItem> AddAsync(TodoItem item);
    Task<TodoItem> UpdateAsync(TodoItem item);
    Task<bool> DeleteAsync(Guid id);
}

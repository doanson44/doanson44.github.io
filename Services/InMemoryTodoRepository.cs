using Client.Models;
using System.Collections.Concurrent;

namespace Client.Services;

/// <summary>
/// In-memory implementation of ITodoRepository for TodoItem.
/// Data is stored only during the session.
/// </summary>
public class InMemoryTodoRepository : ITodoRepository
{
    private readonly ConcurrentDictionary<Guid, TodoItem> _items = new();

    public Task<List<TodoItem>> GetAllAsync()
    {
        var items = _items.Values.ToList();
        return Task.FromResult(items);
    }

    public Task<TodoItem?> GetByIdAsync(Guid id)
    {
        _items.TryGetValue(id, out var item);
        return Task.FromResult(item);
    }

    public Task<TodoItem> AddAsync(TodoItem item)
    {
        if (item.Id == Guid.Empty)
        {
            item.Id = Guid.NewGuid();
        }
        _items[item.Id] = item;
        return Task.FromResult(item);
    }

    public Task<TodoItem> UpdateAsync(TodoItem item)
    {
        _items[item.Id] = item;
        return Task.FromResult(item);
    }

    public Task<bool> DeleteAsync(Guid id)
    {
        var removed = _items.TryRemove(id, out _);
        return Task.FromResult(removed);
    }
}
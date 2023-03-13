using AutoMapper;
using Client.Models;
using Client.Modules.TodoModule.Commands;

namespace Client.Mappings;

/// <summary>
/// AutoMapper profile for Todo entity mappings.
/// Defines how to map between TodoItem, TodoDto, and command objects.
/// </summary>
public class TodoProfile : Profile
{
    public TodoProfile()
    {
        // Entity <-> DTO mappings
        CreateMap<TodoItem, TodoDto>().ReverseMap();

        // Command -> Entity mappings
        CreateMap<CreateTodoCommand, TodoItem>()
            .ForMember(dest => dest.Id, opt => opt.Ignore())
            .ForMember(dest => dest.CreatedAt, opt => opt.MapFrom(_ => DateTime.UtcNow))
            .ForMember(dest => dest.IsCompleted, opt => opt.MapFrom(_ => false))
            .ForMember(dest => dest.CompletedAt, opt => opt.Ignore());

        CreateMap<UpdateTodoCommand, TodoItem>()
            .ForMember(dest => dest.CreatedAt, opt => opt.Ignore());
    }
}

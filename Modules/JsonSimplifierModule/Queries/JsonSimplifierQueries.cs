using Client.Common.CQRS;
using System.ComponentModel.DataAnnotations;

namespace Client.Modules.JsonSimplifierModule.Queries;

/// <summary>
/// Query to get simplified JSON
/// </summary>
public class GetSimplifiedJsonQuery : IQuery<string>
{
    [Required(ErrorMessage = "JsonString is required")]
    public string JsonString { get; set; } = string.Empty;

    [Required(ErrorMessage = "MaxItems is required")]
    [Range(1, int.MaxValue, ErrorMessage = "MaxItems must be greater than 0")]
    public int MaxItems { get; set; } = 5;
}
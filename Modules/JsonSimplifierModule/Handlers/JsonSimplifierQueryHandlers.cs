using Client.Common.CQRS;
using Client.Modules.JsonSimplifierModule.Queries;
using System.Text.Json;
using System.Text.Json.Nodes;

namespace Client.Modules.JsonSimplifierModule.Handlers;

/// <summary>
/// Handler for GetSimplifiedJsonQuery
/// </summary>
public class GetSimplifiedJsonQueryHandler : IQueryHandler<GetSimplifiedJsonQuery, string>
{
    public async Task<string> HandleAsync(GetSimplifiedJsonQuery query, CancellationToken cancellationToken = default)
    {
        try
        {
            using var doc = JsonDocument.Parse(query.JsonString);
            var root = doc.RootElement;

            var simplified = SimplifyJson(root, query.MaxItems);
            return simplified.ToJsonString(new JsonSerializerOptions { WriteIndented = true });
        }
        catch (JsonException)
        {
            // If invalid JSON, return original
            return query.JsonString;
        }
    }

    private static JsonNode SimplifyJson(JsonElement element, int maxItems)
    {
        if (element.ValueKind == JsonValueKind.Array)
        {
            var array = new JsonArray();
            foreach (var item in element.EnumerateArray().Take(maxItems))
            {
                array.Add(SimplifyJson(item, maxItems));
            }
            return array;
        }
        else if (element.ValueKind == JsonValueKind.Object)
        {
            var obj = new JsonObject();
            foreach (var prop in element.EnumerateObject())
            {
                obj[prop.Name] = SimplifyJson(prop.Value, maxItems);
            }
            return obj;
        }
        else
        {
            return JsonNode.Parse(element.GetRawText())!;
        }
    }
}
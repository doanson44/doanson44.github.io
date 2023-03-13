using System.Globalization;
using Microsoft.Extensions.Localization;

namespace Client.Services;

public class LocalizationService
{
    private readonly IStringLocalizer<Resources> _localizer;

    public LocalizationService(IStringLocalizer<Resources> localizer)
    {
        _localizer = localizer;
    }

    public string this[string key] => _localizer[key];

    public string GetString(string key) => _localizer[key];

    public string GetString(string key, params object[] arguments) => _localizer[key, arguments];

    public CultureInfo GetCurrentCulture() => CultureInfo.CurrentCulture;

    public CultureInfo GetCurrentUICulture() => CultureInfo.CurrentUICulture;
}

// This is a marker class for the resource files
public class Resources
{
}
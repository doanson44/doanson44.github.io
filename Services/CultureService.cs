using System.Globalization;
using Microsoft.JSInterop;

namespace Client.Services;

public class CultureService
{
    private readonly IJSRuntime _jsRuntime;

    public CultureService(IJSRuntime jsRuntime)
    {
        _jsRuntime = jsRuntime;
    }

    public async Task SetCultureAsync(string culture)
    {
        var cultureInfo = new CultureInfo(culture);
        CultureInfo.DefaultThreadCurrentCulture = cultureInfo;
        CultureInfo.DefaultThreadCurrentUICulture = cultureInfo;

        // Store in localStorage
        await _jsRuntime.InvokeVoidAsync("localStorage.setItem", "culture", culture);
    }

    public async Task<string> GetStoredCultureAsync()
    {
        var culture = await _jsRuntime.InvokeAsync<string>("localStorage.getItem", "culture");
        return culture ?? "en-US";
    }

    public async Task InitializeCultureAsync()
    {
        var culture = await GetStoredCultureAsync();
        await SetCultureAsync(culture);
    }
}
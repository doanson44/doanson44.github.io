using Radzen;

namespace Client.Common;

/// <summary>
/// Extension methods for NotificationService to provide convenient notification methods
/// with consistent global styling
/// </summary>
public static class NotificationExtensions
{
    /// <summary>
    /// Shows a success notification with global styling at bottom right
    /// </summary>
    public static void NotifySuccess(this NotificationService service, string summary, string detail, int duration = 3000)
    {
        service.Notify(new NotificationMessage
        {
            Style = NotificationConstants.GlobalStyle,
            Severity = NotificationSeverity.Success,
            Summary = summary,
            Detail = detail,
            Duration = duration
        });
    }

    /// <summary>
    /// Shows an error notification with global styling at bottom right
    /// </summary>
    public static void NotifyError(this NotificationService service, string summary, string detail, int duration = 5000)
    {
        service.Notify(new NotificationMessage
        {
            Style = NotificationConstants.GlobalStyle,
            Severity = NotificationSeverity.Error,
            Summary = summary,
            Detail = detail,
            Duration = duration
        });
    }

    /// <summary>
    /// Shows an info notification with global styling at bottom right
    /// </summary>
    public static void NotifyInfo(this NotificationService service, string summary, string detail, int duration = 4000)
    {
        service.Notify(new NotificationMessage
        {
            Style = NotificationConstants.GlobalStyle,
            Severity = NotificationSeverity.Info,
            Summary = summary,
            Detail = detail,
            Duration = duration
        });
    }

    /// <summary>
    /// Shows a warning notification with global styling at bottom right
    /// </summary>
    public static void NotifyWarning(this NotificationService service, string summary, string detail, int duration = 4000)
    {
        service.Notify(new NotificationMessage
        {
            Style = NotificationConstants.GlobalStyle,
            Severity = NotificationSeverity.Warning,
            Summary = summary,
            Detail = detail,
            Duration = duration
        });
    }
}
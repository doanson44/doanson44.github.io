namespace Client.Common.CQRS;

/// <summary>
/// Common interface for all Commands
/// </summary>
public interface ICommand
{
}

/// <summary>
/// Common interface for all Queries
/// </summary>
/// <typeparam name="TResult">The result type to return</typeparam>
public interface IQuery<TResult>
{
}

/// <summary>
/// Common interface for all Command Handlers
/// </summary>
public interface ICommandHandler<in TCommand> where TCommand : ICommand
{
    Task HandleAsync(TCommand command, CancellationToken cancellationToken = default);
}

/// <summary>
/// Common interface for all Query Handlers
/// </summary>
public interface IQueryHandler<in TQuery, TResult> where TQuery : IQuery<TResult>
{
    Task<TResult> HandleAsync(TQuery query, CancellationToken cancellationToken = default);
}

/// <summary>
/// Interface for Command Handlers that return a result
/// </summary>
public interface ICommandHandler<in TCommand, TResult> where TCommand : ICommand
{
    Task<TResult> HandleAsync(TCommand command, CancellationToken cancellationToken = default);
}

/// <summary>
/// Base class for Command Result
/// </summary>
public abstract class CommandResult
{
    public bool IsSuccess { get; protected set; }
    public string? ErrorMessage { get; protected set; }

    protected CommandResult(bool isSuccess, string? errorMessage = null)
    {
        IsSuccess = isSuccess;
        ErrorMessage = errorMessage;
    }

    public static CommandResult Success() => new SuccessResult();
    public static CommandResult Failure(string errorMessage) => new FailureResult(errorMessage);

    private class SuccessResult : CommandResult
    {
        public SuccessResult() : base(true) { }
    }

    private class FailureResult : CommandResult
    {
        public FailureResult(string errorMessage) : base(false, errorMessage) { }
    }
}

/// <summary>
/// Base class for Query Result
/// </summary>
public abstract class QueryResult<T>
{
    public bool IsSuccess { get; protected set; }
    public T? Data { get; protected set; }
    public string? ErrorMessage { get; protected set; }

    protected QueryResult(bool isSuccess, T? data = default, string? errorMessage = null)
    {
        IsSuccess = isSuccess;
        Data = data;
        ErrorMessage = errorMessage;
    }

    public static QueryResult<T> Success(T data) => new SuccessResult<T>(data);
    public static QueryResult<T> Failure(string errorMessage) => new FailureResult<T>(errorMessage);

    private class SuccessResult<TData> : QueryResult<TData>
    {
        public SuccessResult(TData data) : base(true, data) { }
    }

    private class FailureResult<TData> : QueryResult<TData>
    {
        public FailureResult(string errorMessage) : base(false, default, errorMessage) { }
    }
}

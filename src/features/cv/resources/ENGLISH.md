# Interview full knowledge (English)

Self-contained. Read top to bottom. No other files required.

How to study: after each heading, say one example out loud. Parts 1–2 = senior backend. Parts 3–4 = technical architect.

Voice in the room: numbers → constraint → option A/B → cost → rollback.

Running example: a field-force / trade-marketing platform — mobile + admin web, BFF, ASP.NET Core APIs, SQL Server, EF Core, MediatR, Hangfire, Redis, Azure AD.

---

# Part 1 — Foundation

## OOP

An object has **attributes** (facts) and **methods** (behaviour).

- **Abstraction:** keep the core (employee: name, date of birth) — drop height and hobbies.
- **Encapsulation:** hide state; change it only through public methods.
- **Inheritance:** a child reuses a parent (`Smartphone` → iPhone / Samsung). C# allows one class parent.
- **Polymorphism:** the same call, different behaviour (iOS vs Android; dog vs cat).

### Virtual vs abstract vs interface

| | Virtual method | Abstract method | Interface |
|---|----------------|-----------------|-----------|
| Body | Default yes | No | C# 8+ default optional |
| Override | Optional | Required | Implement all (unless default) |
| Where | Any class | Abstract class only | Contract type |
| Multiple | — | One base class | Many interfaces |

Interface = “what it can do” (`IDisposable`). Abstract class = “what kind it is” plus shared code.

```csharp
public abstract class Parent
{
    public abstract void Must();
    public virtual void Maybe() { }
}

public class Child : Parent
{
    public override void Must() { }
    public override void Maybe() { base.Maybe(); }
}
```

### Generics

Write once; the caller supplies the type: `List<T>`, `Dictionary<TKey,TValue>`. Avoids `ArrayList` boxing and runtime cast errors.

### String vs StringBuilder

`string` is immutable — each change allocates a new object. `StringBuilder` grows a buffer. Use a builder inside loops.

### List vs ArrayList

Prefer `List<T>`. `ArrayList` stores `object`: compiles, can throw at runtime.

### Value vs reference

Default: copy the value (or copy the reference for objects). `ref`: the caller’s variable changes; it must already be assigned. `out`: must assign inside the method; the caller need not assign first. The call site must write `ref` / `out`.

### Stack vs heap

Both live in RAM.

- **Stack:** locals, parameters, fast, fixed size (~1 MB per thread on Windows), freed on return, overflow on infinite recursion. Each thread has its own stack.
- **Heap:** objects from `new`, garbage-collected in .NET, shared across threads (needs synchronisation).

### `static`

Belongs to the type, not an instance. One field per AppDomain, on the Managed Heap. Mutable statics are shared state: hard to test and racy.

### Singleton

One instance for the app: `services.AddSingleton<T>()`, or a private constructor plus `Instance` / `Lazy<T>`. Never register `DbContext` as singleton. **Captive dependency:** a Singleton that holds a Scoped service (for example `DbContext`) — a production bug.

### `volatile`

A hint against stale reads across threads. Usually the wrong tool: not atomic beyond assignment, does not prevent races. Prefer `Interlocked`, `lock`, concurrent collections. Do not mark `long` or `double`. Narrow valid use: a `_shouldStop` flag so the compiler does not cache it in a register.

### Provider (two meanings)

1. Logging / configuration: `ILoggerProvider`, `IConfigurationProvider` — who supplies the implementation.
2. Observer: `IObservable<T>` — `Subscribe` returns `IDisposable`; push `OnNext` / `OnError` / `OnCompleted`.

## SOLID

- **S — Single responsibility:** one reason to change. Split “read the database” and “print the report”.
- **O — Open/closed:** extend; do not edit the old class for every feature.
- **L — Liskov:** a child must be substitutable for the parent. A penguin is not a `FlyingBird`.
- **I — Interface segregation:** many small interfaces, not one fat interface.
- **D — Dependency inversion:** depend on abstractions. The socket is the interface; the bulbs are implementations.

## DIP vs IoC vs DI

- **DIP:** principle — high-level modules depend on abstractions.
- **IoC:** invert `new`; construction happens outside the consumer.
- **DI:** how you inject — constructor (default), setter, or method. The container registers, resolves, and applies lifetime.

Three DI styles: constructor, property/setter, method (`SetDependency`). Constructor is the ASP.NET Core default.

Lifetimes: **Singleton** (config, Redis multiplexer) / **Scoped** (`DbContext`, one per HTTP request) / **Transient** (new every resolve).

## Design patterns

**Creational** (hide `new`): Factory, Singleton, Builder, Prototype.

**Structural** (relationships): Adapter, Decorator, Facade, Proxy.

**Behavioral** (algorithms / flow): Strategy, Observer, Mediator, Command, Template Method.

Factory example: switch on image type → `GifReader` / `JpegReader`.

**CQRS:** commands write; queries read (`AsNoTracking`, DTOs). **MediatR:** the controller sends a `Request`; one handler per use case; validation in the pipeline. Do not put business rules in controllers.

## Frontend

**React (class):** mount `constructor → render → componentDidMount`; update `render → componentDidUpdate`; unmount `componentWillUnmount`. Function components: `useEffect`. `PureComponent` / `React.memo` = shallow compare of props/state.

**Angular component:** class + `@Component({ selector, template, styles })` + `@Input` / `@Output` + services injected in the constructor.

Lifecycle: `constructor` (DI, inputs not set) → `ngOnChanges` → `ngOnInit` (fetch data) → `ngDoCheck` → content hooks → view hooks (`ViewChild` ready) → `ngOnDestroy` (unsubscribe). Never call HTTP in the constructor.

**Webpack:** entry → dependency graph → bundle. Loaders transpile; plugins minify; dynamic `import()` splits chunks. Angular CLI / Vite hide the bundler.

**N-layer vs MVC:** n-layer is the whole system (UI | business | data). MVC is a UI pattern (Controller, Model, View). They compose. **Clean Architecture** inverts dependencies so the domain does not reference EF.

## Scrum

Ship a usable increment every 1–4 weeks.

**Artefacts:** product backlog (Product Owner); sprint backlog (team selects in planning); increment; burndown chart.

**Roles:** Product Owner (priority, market); Scrum Master (coaches the process); development team (builds).

**Events:** sprint planning; daily stand-up (yesterday / today / blockers); sprint review (demo finished work); retrospective (process).

---

# Part 2 — Backend

## MVC and .NET

**Model** = data and rules. **View** = UI. **Controller** = HTTP in, model work, view or JSON out.

**.NET Core** (now just “.NET”): cross-platform, containers, microservices, side-by-side versions.

**.NET Framework:** Windows, WPF / WinForms, existing apps you extend rather than rewrite.

**Kestrel** is the asynchronous web server. In Azure App Service it sits behind IIS/ANCM.

## HTTP is stateless

Each request must carry enough information. You approximate state with URL, form fields, cookies, or a server session (id in a cookie).

Controller → view:

| | ViewBag | ViewData | TempData |
|---|---------|----------|----------|
| Type | `dynamic` | dictionary | dictionary |
| Lifetime | one request; lost on redirect | one request | until read; `TempData.Keep()` to keep |
| Cast | no | yes | yes |

**Model binding order** when names collide: form body > route > query string.

**Web API vs MVC:** API returns data, supports content negotiation (`Accept`), maps by HTTP method (GET/POST/…). MVC maps by action name and can return views. If both live in one project you have two filter pipelines.

## Routing

Parse the URL → match a template in the route collection → handler. Example: `{controller=Home}/{action=Index}/{id?}`. MVC’s route handler creates the controller and runs the action.

## Middleware vs filters

**Middleware** runs for every request. Recommended order:

1. Exception handler
2. HTTPS / static files
3. `UseRouting`
4. CORS
5. `UseAuthentication` (sets `HttpContext.User`)
6. `UseAuthorization`
7. `UseSession`
8. Endpoints (`MapControllers`)

Each middleware calls `next` or short-circuits. Wrong order → empty `User`, session not ready, 401/403.

**Filters** run after routing has chosen an action:

| Filter | When |
|--------|------|
| Authorization | First; no “after”; do not throw (exception filters will not catch it) |
| Resource | Wraps model binding; cache hit can short-circuit |
| Action | Immediately around the action method; can set `Result` |
| Exception | Unhandled in the action; prefer exception middleware unless API vs HTML need different bodies |
| Result | Around executing the result (view / JSON) |

Scope: global → controller → action on the way in; reverse on the way out. `IOrderedFilter.Order` overrides scope (lower runs earlier on the way in).

DI: `[ServiceFilter<T>]` requires `T` registered in the container; `[TypeFilter(typeof(T))]` does not. Implement the sync **or** the async interface, not both (runtime prefers async).

Short-circuit: assign `context.Result` and do not call `next`.

Permission example: after JWT, an action filter loads the user by id from Redis/CMS and checks a feature/role.

## Authentication vs authorization

**Authentication (AuthN)** = who you are. Result: `ClaimsPrincipal` on `HttpContext.User`. Schemes: cookie (web / BFF), JWT Bearer (API), OpenID Connect (Azure AD). Cookie `SlidingExpiration`: the cookie is re-issued after more than half of `ExpireTimeSpan`.

**Authorization (AuthZ)** = what you may do. `[Authorize]`, roles, policies, **resource-based** checks (IDOR: the user may be logged in and still must own that outlet/plan). 401 = not authenticated or expired token. 403 = known user, not allowed.

Web: BFF holds a session in Redis and sets an httpOnly cookie. APIs receive JWT. Do not keep long-lived tokens in SPA `localStorage`.

## Session and cache

**Session:** data on the server; cookie holds only the session id. `IdleTimeout` default 20 minutes, **sliding** (each request through session middleware resets the clock). That is independent of cookie-auth lifetime. Scale-out: store session in Redis/SQL, not in process memory.

**IMemoryCache:** in-process. `AbsoluteExpiration` dies at a clock time. `SlidingExpiration` extends on each get (pair with a max absolute). Lost on restart; not shared across machines.

**IDistributedCache:** values are `byte[]` (JSON serialize). Redis or SQL. Cache-aside: miss → query DB → `Set`. Stampede (many misses at once): short lock. Key must include tenant (and locale/version if needed).

If server A cached in memory and server B handles the next request, B misses — that is expected. Use Redis.

## Async vs sync

**Async** for I/O: database, HTTP, files (`ToListAsync`, `HttpClient.GetAsync`). The thread returns to the pool while waiting.

**Sync** for in-process CPU work. Fake async on CPU work is not faster.

Never `.Result` or `.Wait()` on async (deadlock / thread-pool starvation). Push work the user should not wait for onto Hangfire or a queue.

## WPF (if asked)

`Binding` + `DataContext`. Modes: OneWay, TwoWay, OneTime, OneWayToSource. Source should implement `INotifyPropertyChanged`. Lists: `ObservableCollection<T>`.

## Bundling and minification

Bundling groups files so the browser makes one request. Minification strips whitespace and comments. Many JS/CSS files = many HTTP requests = slow.

## Login and registration

**Register:** validate unique email and password policy → hash (ASP.NET Identity / PBKDF2 / bcrypt / Argon2) → insert user → optional email confirm. Never store plaintext. Unique index on email.

**Login:** load user → verify hash → lock out after too many failures → issue cookie or JWT over HTTPS. Use the same error if you must not reveal whether the email exists.

## EF Core

ORM: C# classes ↔ tables; LINQ becomes SQL.

```csharp
builder.Services.AddDbContext<DatabaseContext>(options =>
    options.UseSqlServer(configuration.GetConnectionString("Database")));
```

`DbContext` is **Scoped** (one per HTTP request). **Code First** + migrations is the usual path. `SaveChanges` is the unit of work.

**Loading related data**

- **Eager:** `Include` / `ThenInclude` in the first query.
- **Lazy:** load when a navigation is touched — **turn off** in web APIs (N+1).
- **Explicit:** `Entry(entity).Collection(x => x.Items).Load()`.

**Tuning**

- Keep `IQueryable` until `ToListAsync` so filters run in SQL.
- Never `await` a database call inside a loop — load by key set once.
- Do not `ToList()` then `.Contains()` on a large key list — compose a join.
- Reads: `AsNoTracking()`. Project with `Select` to the columns you need.
- Several collections: `AsSplitQuery()` to avoid cartesian explosion.
- Bulk insert/update for thousands of rows; not `SaveChanges` per row.

**Measure.** A `let` in query syntax can allocate extra anonymous types and be *slower* even though it “computes once”. A trick that helps LINQ to Objects can hurt EF SQL.

**N+1:** lazy loading or a forgotten `Include` — N extra queries. **Cartesian explosion:** eager `Include` of two collections in one JOIN. Fix: DTO projection or split query.

**Optimistic concurrency:** `byte[]` `rowversion` / `[Timestamp]`. SQL: `UPDATE … WHERE Id=@id AND Version=@old`. Zero rows → `DbUpdateConcurrencyException` → tell the user to reload.

**Repository / unit of work:** `DbContext` already is a unit of work and identity map. Thin repositories are fine. Avoid a 20-method generic `IRepository<T>` nobody uses.

## SQL

**Index:** on-disk B+ tree (rowstore) that speeds lookup. Maintained on every write.

**Clustered:** the table itself is stored in key order. **One** per table. No clustered index = **heap**. A primary key usually creates a clustered index. Keep the clustered key narrow and increasing (`bigint` identity). Random GUIDs fragment the table.

**Nonclustered:** a separate structure. Leaf = key + locator (RID on a heap, clustered key on a clustered table). `INCLUDE` extra columns at the leaf for covering queries (no key lookup).

The optimizer may still scan if most rows match. Indexes slow `INSERT`/`UPDATE`/`DELETE`.

**View:** a named `SELECT`, queried like a table. Optional security layer. **Indexed view** materialises rows — extra write cost.

**Stored procedure:** T-SQL with parameters, control flow, transactions. Plan cache, fewer round-trips. Use for heavy set-based work; do not hide the whole domain in procedures.

**Trigger:** runs `AFTER`/`INSTEAD OF` insert/update/delete. Easy to hide side effects. Acceptable for audit; not for large business rules.

## Exercise: students and courses

Many-to-many needs a junction table:

```text
Student              Enrollment                 Course
--------             -----------                ------
Id PK                StudentId FK               Id PK
FullName             CourseId FK                Code unique
DateOfBirth          EnrolledAt                 Name
Email unique         Grade NULL                 Credits
                     PK (StudentId, CourseId)
```

Indexes: primary keys; `Email`; `Enrollment(CourseId)` (StudentId is already in the PK).

Sort by year of birth: `ORDER BY DateOfBirth` (can use an index). `YEAR(DateOfBirth)` usually cannot. LINQ: `students.OrderBy(s => s.DateOfBirth)`.

---

# Part 3 — Architecture

Architecture is **decisions under constraints**, written down (ADR: context, decision, consequences). A quality-attribute scenario has six parts: source, stimulus, environment, artifact, response, **measure**. If you cannot fill the measure, it is a wish.

## Quality attributes

| Attribute | How you talk about it |
|-----------|------------------------|
| Availability | Error budget. 99.9% ≈ 8.8 hours downtime / year. SLO cannot exceed the weakest dependency. |
| Latency | Budget p95/p99 for a **user journey**, not one endpoint. Little’s law: concurrency ≈ throughput × latency. |
| Scalability | Stateless API + shared Redis. Shard last. |
| Consistency | Strong inside one aggregate / one SQL transaction. Eventual across processes. |
| Security | IDOR tests, BFF, secrets in Key Vault. |
| Observability | One trace id from BFF through API through Hangfire. RED metrics (rate, errors, duration). |
| Recoverability | RPO / RTO from a restore you have **actually run**. |
| Cost | Cost per active staff per month. |

**PACELC:** if Partitioned, choose Availability or Consistency; Else choose Latency or Consistency.

Do not promise 99.99% if you have shared Azure SQL, Hangfire on SQL, and SharePoint on the hot path. Availability is the minimum of dependencies.

**ATAM-lite in a review:** list 3–5 scenarios → walk C4 + sequence → sensitivity (a change here swings quality) and trade-off (helps A, hurts B) → risks → ADR or spike.

**Fitness function:** an automated check of an architectural rule, for example: search endpoints must paginate; API projects must not new up data-access types in controllers; secrets must not be in git; p95 of login → list plans under 800 ms in staging.

## C4

Draw **L1 context** (who talks to the system) and **L2 containers** (BFF, API, worker, SQL, Redis, blob). One sequence for the riskiest write. Zoom to components only if asked. If production is one API process, draw **one container with module boxes** — do not pretend you have twelve independently deployed services.

```text
 Staff app ──┐
 Admin web ──┼──► BFF (cookie, Redis session) ──► Azure AD
             └──► (private JWT)
                    ├── Identity API
                    ├── Journey / Timesheet API
                    └── Campaign / KPI API
                           │
                    SQL Server + Redis + Blob
                    Hangfire worker
```

## DDD (the useful 20%)

A **bounded context** is a model with a language boundary. “Status” on a journey plan is not “status” on a timesheet.

An **aggregate** is the consistency boundary: invariants hold inside one transaction, one connection.

Context map: **anti-corruption layer** around SharePoint / loyalty / ERP so the domain does not speak their DTOs; **conformist** toward the identity provider (do not reinvent users).

## Modular monolith first

Extract a **service** only when at least two of these are true: independent scale, independent release train, independent data (compliance / RPO), different runtime.

Until then: module folders, schema prefixes, no project reference to another module’s internals, an architecture test that fails the build if that happens.

**Strangler fig:** put a façade (BFF/gateway) in front → extract one capability behind it → delete the old path when traffic is gone. Do not freeze the monolith for an 18-month rewrite.

**BFF vs API gateway**

- Gateway: TLS, WAF, rate limit, routing.
- BFF: per-client aggregation, cookie session, hide internal APIs.

**Sync vs async:** HTTP when the user needs the answer (validation, “may I check in here?”). Queue when the HTTP 200 does not need the side effect (email, push, search index).

## Data and consistency

No distributed 2PC between SQL and a bus.

**Transactional outbox**

```text
BEGIN TRAN
  UPDATE JourneyPlan ...
  INSERT Outbox (Id, Type, Payload, CreatedUtc)
COMMIT
-- dispatcher publishes, then marks processed
```

Delivery is **at-least-once**. Consumers must be **idempotent** (inbox table on `EventId`, or a unique business key).

**Saga:** only when one transaction is impossible (campaign publish → generate QR → notify agencies in another system). Orchestration = a process-manager row. Choreography = each service reacts to events. Compensation is a **forward** command (`CancelReservation`), not a rollback.

**CQRS levels**

0. Same model, `AsNoTracking` on reads — default.  
1. Separate read DTOs / SQL views.  
2. Separate read store updated by events.  
3. Event sourcing — rare.

Two folders named Command and Query with the same shape is theatre.

**SQL topology:** failover group for DR; read replica + `ApplicationIntent=ReadOnly` for heavy reports; increasing clustered keys; covering indexes for search screens.

## Messaging

Decision tree:

```text
Does the caller need the result for the UX?
  yes → HTTP/gRPC, tight timeout, retry only if idempotent
  no  → async
        One consumer inside this process?
          yes → Hangfire (outbox if it must survive a crash)
          no  → message bus
                Need replay / many consumers / very high throughput?
                  yes → Kafka / Event Hubs
                  no  → Azure? sessions / duplicate detection?
                        yes → Service Bus
                        no  → RabbitMQ (you operate it)
```

**Exactly-once** across HTTP + SQL + bus does not exist. Design for at-least-once plus idempotency.

### AMQP 0-9-1 (RabbitMQ)

Producer publishes to an **exchange**. **Bindings** route copies to **queues**. Consumers push-subscribe (preferred) or pull.

| Exchange | Routing |
|----------|---------|
| Default (nameless direct) | Binding key = queue name — looks like “send to a queue” |
| Direct | Routing key equals binding key |
| Fanout | Every bound queue; routing key ignored |
| Topic | `*` = one word, `#` = zero or more words |
| Headers | Match message headers (`x-match` any/all) |

Queue flags: durable (metadata on disk), exclusive (one connection, deleted when it closes), auto-delete. Names starting `amq.` are reserved. Redeclare with different attributes → 406 PRECONDITION_FAILED.

**Durable queue does not make messages survive a restart.** Mark the message persistent (`delivery_mode=2` / `persistent: true`) as well.

**Ack:** automatic (broker deletes on deliver — can lose work) vs explicit (`basic.ack` after you finished). A dead consumer without ack → redeliver. Prefetch (`basic.qos`) limits unacked messages per consumer (load balance).

**Connection** is long-lived TCP. **Channel** is a lightweight multiplex; one channel per thread, do not share. **Vhost** isolates environments.

Unroutable messages: drop or return to the publisher. Dead-letter after max retries. Alert on DLQ depth.

### Azure Service Bus

Managed broker, consumers pull.

- **Queue:** one message, one competing consumer (work distribution, load leveling).
- **Topic + subscription:** each subscription gets a copy; SQL filters on properties.

PeekLock (lock, complete, abandon, dead-letter) vs ReceiveAndDelete. Duplicate detection, sessions (ordering / request-reply), scheduled messages. Basic tier = queues only; Standard/Premium add topics.

### Kafka / Event Hubs

Append-only log, partitions, consumer **offsets**, replay. High throughput. Not an AMQP work queue. Use for streams and audit, not “place order” commands unless you have a real stream platform.

### Hangfire

In-process (or a dedicated worker) scheduler: fire-and-forget, delayed, recurring cron. Dashboard for succeeded/failed. Examples: nightly KPI, staff day-off, SKU labels, email.

When jobs starve HTTP threads, run Hangfire in a **second process** (API only enqueues). Hangfire is not the integration contract for another team — that is a bus plus outbox.

### Flash sale (5,000 req/s vs database 2,000 req/s)

Do not hit SQL at 5k/s. Put 5k into a durable queue; drain at 2k/s. Remaining work is processed after the peak. Consumers must be idempotent (limited stock). Add a dead-letter queue. Sync path that also sends mail, points, and rank in the same request will time out — return 200 and queue those side effects.

```text
Sync (bad):  Client → API ──mail──points──rank──► 200 after ~1000 ms

Async (good): Client → API → 200
                     └── queue → workers (mail, points, rank)

Spike: Client × 5000/s → Queue → API/DB at 2000/s
```

## Security

**STRIDE** on login and file upload: spoofing (stolen cookie), tampering (IDOR on plan id), repudiation (no audit), disclosure (token in logs, public blob), denial of service (huge upload), elevation (missing `[Authorize]`).

**IDOR** is the number-one API risk in field-force apps. `GET /plans/{id}` must check tenant + role + assignment, not only authentication.

**OAuth / OIDC flows**

| Flow | Client | Use |
|------|--------|-----|
| Authorization code + client secret | BFF | Web (preferred) |
| Authorization code + PKCE | Mobile / public | Native apps |
| Client credentials | Worker | Daemon → Graph |
| On-behalf-of | Middle tier | Call downstream as the user |
| Resource Owner Password / Implicit | — | Do not use |

**Tokens:** access JWT 15–60 minutes, `aud` = API, claims = sub/tenant/roles — not the whole profile. Refresh longer, **rotating**, stored on the server (Redis); reuse of an old refresh revokes the family (theft). ID token tells the client who logged in; APIs do not authorise on it. JWT cannot be un-issued: short TTL or a `jti` block-list.

**Cookie vs bearer:** httpOnly Secure SameSite cookie on the BFF (better vs XSS; need CSRF care). Bearer in JS is stealable. Mobile: PKCE, tokens in the OS secure store.

**AuthZ models:** RBAC (roles) then resource checks. Redis may **cache** permissions; the database/CMS is the source of truth. Invalidate on role change.

**Secrets and network:** Key Vault + managed identity; never commit connection strings or signing keys. APIs private (no public IP). WAF on the BFF edge. Hangfire dashboard admin-only.

## Reliability and Azure

**SLI** = what you measure (p95 of submit timesheet, % 5xx). **SLO** = internal target. **SLA** = contract. **Error budget** = 1 − SLO; spend it on change, freeze releases if it burns too fast. Alert on burn rate, not “CPU > 80%”.

Golden journeys (pick about five): login, list plans, check-in, submit timesheet, admin search.

**Polly:** timeout on every egress; retry with jitter **only if idempotent**; circuit breaker; bulkhead so SharePoint cannot exhaust the API thread pool. EF `EnableRetryOnFailure` for transient SQL, not for unique-key violations.

**Health:** `/live` = process up. `/ready` = SQL + Redis. Do **not** fail ready because a non-critical dependency is down if you can degrade (hide images, queue notifications).

**Tracing:** W3C `traceparent` through BFF → API → Hangfire job. Structured logs with `code` + `traceId`, no phone/email. Histograms, not averages.

**DR:** system of record = SQL. Redis session loss may mean everyone logs in again — say so. Restore drill quarterly.

**Compute:** App Service for the API + a worker by default. Container Apps if you want containers without Kubernetes. AKS only with a platform team. Functions for blob/event glue, not the OLTP API.

**Edge:** Internet → Front Door / App Gateway (WAF) → public BFF → private APIs → private SQL/Redis/Service Bus.

**Autoscale** on HTTP queue length **and** Hangfire/bus depth, not CPU alone (I/O-bound apps lie).

**.NET 6 is out of support.** Move to 8 LTS: change TFM, audit packages, run integration tests — do not combine with a rewrite.

## Evolution (two to three quarters)

Technical debt is a loan with interest. Pay reckless N+1 on a hot path now. Schedule the framework upgrade. Do not take “no tests, ship Friday” loans.

1. Golden-journey tests, architecture tests, p95 dashboard.  
2. .NET 8.  
3. Hangfire in a worker process.  
4. Outbox for fire-and-forget after `SaveChanges`.  
5. Module walls (JourneyPlan must not reference Timesheet internals).  
6. SharePoint behind a named `HttpClient`, timeout, breaker; off the synchronous user path.  
7. BFF if tokens still live in JavaScript.  
8. Read replica if reports hurt OLTP.

Do not: Kafka for three events a day; a shared database with another product; fake Clean Architecture folders while Application still references DataAccess — stop the bleeding on **new** use cases only.

**Saying no** with an ADR and an alternative is the job: no second ORM, no 99.99% SLO without budget, no “temporary” public API IP.

**The sentence to close a design:** “Modular monolith on App Service + SQL + Redis, BFF for web, JWT internally, Hangfire for jobs, outbox when a second consumer appears, extract Notification if fan-out or SLA diverges. ADRs for identity, data, and messaging.”

## TA depth the panel will still ask

A senior who recites SOLID and “we should use Redis” still fails a TA round. The panel wants **numbers, failure modes, and how you would operate the choice**. This section is that layer.

### Back-of-envelope (do this on the board)

Assume 10,000 field staff, 30% active in the morning peak, 2 requests per active user per minute.

```text
Active ≈ 3,000
Peak ≈ 3,000 × 2 / 60 ≈ 100 req/s
p95 target 400 ms ⇒ in-flight ≈ 100 × 0.4 ≈ 40
SQL: 100 req/s × 3 queries ≈ 300 queries/s — one Azure SQL General Purpose box is fine
Photos: 3,000 × 8 × 2 MB / day ≈ 48 GB/day ingest → blob, not SQL
Connections: instances × (DbContext pool 128) must stay under SQL max (~150–300 on a small SKU)
```

If they 10× the users, I scale **out API** first, then Redis, then SQL (DTU/vCore, then read replica). I do not jump to Kafka because 100 req/s “feels small for Netflix”.

### Zero-downtime change

**App:** slot swap / blue-green; at least two instances; drain Hangfire or requeue on SIGTERM (`IHostApplicationLifetime`, 30 s grace).

**Schema — expand/contract:** add nullable column → deploy code that writes both → backfill → deploy code that reads new → drop old. Never expand and drop in one migration that locks a hot table. Online index create on large tables. Dual-write only with a measured window and a kill switch.

**Feature flags:** behaviour change behind a flag so rollback is a toggle, not a redeploy. Flags have an expiry; a flag that lives a year is a second configuration system.

### Pagination, keys, soft delete

**Offset pagination** (`SKIP/TAKE`) is simple and **wrong** for deep pages (the engine still walks skipped rows). **Keyset** (`WHERE (Date, Id) < (@d, @id) ORDER BY Date DESC, Id DESC`) is stable under inserts. Always a max `pageSize` (for example 50). Unbounded `ToList` is an incident.

**PK:** `bigint` identity clustered for OLTP. Random GUID fragments the clustered index. Sequential GUID only if you must expose a GUID.

**Soft delete:** `IsDeleted` plus a **filtered** index. Every query must remember the filter (global query filter in EF). Hard-delete PII on a retention job when the law requires it — soft delete is not erasure.

### Isolation, deadlock, replica lag

Default `READ COMMITTED`. `READ UNCOMMITTED` / `NOLOCK` for reports is a lie you will debug for days. `SNAPSHOT` / RCSI reduces reader-writer blocking; know it is on or off in your SQL.

Deadlock: two transactions lock A then B vs B then A. Fix: consistent lock order, shorter transactions, retry on deadlock (EF execution strategy). Do not retry unique-key violations as if they were deadlocks.

Read replica is **stale**. KPI dashboards: stale-OK. After a write, **read-your-writes** from the primary (or session sticky to primary for a few seconds). Never send the user to a replica for the row they just saved.

### Domain events vs integration events

**Domain event:** inside the bounded context, after commit, same process or outbox to itself (for example “TimesheetSubmitted” updates a local projection).

**Integration event:** contract with another context, versioned payload, stable `eventType`, no EF entities on the wire. Additive fields only. The outbox stores the integration event, not the domain object.

### Rate limit, versioning, CORS

Rate limit at the edge (Front Door / gateway) **and** per user/app on the API (`429` + `Retry-After`). Login and OTP tighter than GET search.

**Versioning:** additive JSON is free. Breaking change = new route or header (`v2`) plus a sunset date. Integer enums in JSON stay integers; do not “fix” them to strings.

CORS is not security. APIs stay private; BFF is the browser origin. Allowlist exact origins, not `*`, with credentials.

### Testing is architecture

- Unit: domain invariants, no I/O.
- Integration: golden journeys on real SQL (containers), assert `errors[].code`.
- Contract: BFF ↔ API OpenAPI compatibility.
- Load: one journey to SLO, then the next; soak for memory leaks; **never** the first load test in production.
- Architecture tests: forbidden references, pagination required.

If the only test is “the architect reviews PRs”, the architecture will rot.

### Incidents

Severity, commander, comms channel, timeline, “stop the bleeding” before root cause theatre. Rollback is a first-class option. Blameless postmortem: what SLO burned, what fitness function we add so it cannot recur. TA is in the rotation often enough that diagrams meet reality.

### Worked ADR (say this shape)

```text
Title: Transactional outbox for timesheet side effects
Status: Accepted
Context: Hangfire after SaveChanges is lost if the process dies; users still got 201.
Decision: Outbox row in the same SQL transaction as the timesheet. Dispatcher publishes. Consumers idempotent on EventId.
Consequences: Extra table, at-least-once, need a DLQ dashboard. We do not introduce Kafka for this.
```

### Time zones, clocks, multi-tenancy

Field-force across markets: store **UTC**, display in the user’s zone, business “today” is the **outlet’s** calendar date, not the server’s. Inject `TimeProvider` — never `DateTime.Now` in domain logic.

Multi-tenant: start **shared DB + TenantId** from the token (never from query string alone). Row-level filter on EF. Extract a database per market only for residency or noisy neighbour. Tenant in every cache key.

### Capacity of the data plane

`DbContext` pooling (for example 128) × instance count < SQL max sessions. Redis connections multiplexed (one multiplexer singleton). Blob for photos; SQL holds metadata and a hash. Lifecycle policy to cool storage.

### What I will not do in the first year

Rewrite the platform. Event-source the timesheet. Twelve microservices with one team. 99.99% SLO on a stack that includes a slow third party. A second ORM “for flexibility”.

---

---

# Part 4 — Interview

Panels listen for: you clarify constraints before drawing; you choose boring until numbers force otherwise; you name failure modes of your own design; you tie tactics to SLOs; you can go one level down (EF, tokens, indexes) without living there; you change the design when they change a constraint.

They fail buzzword salad, “Kubernetes and Kafka”, no order-of-magnitude load, blaming the team for SOLID.

First three minutes: “I’ll ask a few constraints, then draw C4 context and containers, then zoom into the riskiest flow — usually write path plus identity. I’ll mark what is synchronous, what is queued, and how it fails. Interrupt if the constraints are wrong.”

Ask them: clients (offline mobile?); peak writes (order of magnitude); region / data residency; existing stack; team size and who operates production; hardest incident last year. If they give no numbers, write assumptions on the board: `10k staff, 50 writes/s peak, photos 2 MB × 8`.

## HR English (say these)

**Tell me about yourself**

> My name is Thai Doan Son. I am a software developer. I have worked in software development for the past 6 years. My main job responsibilities include developing and testing. I love performance tuning. It’s challenging and you get a feeling of accomplishment when done.
>
> I’m from DakLak province. I studied at the University of Science Ho Chi Minh City. I graduated with a degree in information technology in 2015. I like programming because it allows me to think more logically. In the future I want to be a technical architect. I would like to live and work in Ho Chi Minh City.

**Strengths:** I learn fast. I read documentation and source code until I know how it works.

**Weaknesses:** I stall when the work is dull. I time-box it and ask for a clear outcome so I still ship.

**Why leave:** I am ready for more responsibility / learning / there is no leadership slot. Do not dump on the previous employer.

**Education:** University of Science, Ho Chi Minh City, Information Technology, 2015. Algorithms and databases still shape how I think; since then I have learned on the job — .NET, SQL, performance.

**Five years:** stay on the technical path; become a technical architect; keep improving the skill set.

**Salary:** `$1800 NET` — update to your number. Architects are priced on **risk they remove**, not years of naming patterns.

**Questions for them:** Which tools should I learn before day one? What is the culture of the team (ownership, speed, camaraderie)?

**Why this job**

> I’m interested because the stack matches what I already ship — ASP.NET Core, SQL Server, cache and messaging — and I can go deeper on performance and architecture. I want to own features end to end toward a technical architect path.

**Why hire you:** .NET experience, performance tuning, I learn from docs and source.

**Stress:** a signal that work is piling up. I break it down, time-box the risky parts, ask early if blocked, and keep a list so I do not hold everything in my head.

**Difficult situation:** first job, no documentation, several technologies stacked, a custom XML layer that broke MVC (business rules in XML next to the view), pages bound tightly to database fields. Hard because of technical debt, not algorithms. I mapped the system, isolated the custom framework, delivered incrementally.

**Architect flavour**

> I stay on the technical path. I want to own the shape of the system — identity, data, and how we fail — and leave a golden path so teams do not need me in every PR. I measure architecture with SLOs and fitness functions, not with slide count.

### Cover letter

```
Your Name
Your Address
Your City, State Zip Code
Your Phone Number
Your Email

Company Name
Address
City, State Zip Code

Dear HR Manager:

This letter is to express my interest in your posting for an experienced Software Developer. With a Bachelor’s degree in Information Technology, and hands-on experience using .Net languages to create and implement software applications, I am confident I will be an asset to your organization.

I enjoy being challenged and engaging with projects that require me to work outside my comfort and knowledge set, as continuing to learn new languages and development techniques are important to me and the success of your organization.

Your listed requirements closely match my background and skills. A few I would like to highlight that would enable me to contribute to your bottom line are:

- Highly skilled in designing, testing, and developing software
- Knowledgeable of back-end development best practices
- Hands-on software troubleshooting experience

I’ve attached a copy of my resume that details my projects and experience in software development. I can be reached anytime via my cell phone, 0376690868 or via email at doanson44@gmail.com.

Thank you for your time and consideration. I look forward to speaking with you about this opportunity.

Sincerely,
Son Thai
```

## Company questions

Each question: **Short** (15–30 seconds) then **Deep** (if they follow up). Stay in Short unless they ask “why / how / what if”.

### Round A — product backend

**Walk through projects and the tech stack.**

- **Short:** Two or three products: role, domain, stack — ASP.NET Core, EF Core, SQL Server, MediatR, Redis, Hangfire, Azure AD / BFF, RabbitMQ if we used it. One hard problem at the end (slow query, auth, or a queue).
- **Deep:** For each product: who the users were, write vs read load, what I owned. Stack by layer — HTTP (thin controllers, payload → MediatR), application (handlers, FluentValidation), SQL via EF, jobs on Hangfire, session/tokens on Redis, identity on Azure AD through a BFF. Close with one incident: for example a 7 s list of 5k rows; `AsNoTracking` plus projection brought it under 1 s; the lesson was measuring before guessing.

**Did you use Redis?**

- **Short:** Yes. Session and tokens live there; we also cache the user for permission filters. Memory cache is one process; Redis is for many instances.
- **Deep:** `IMemoryCache` dies on restart and is not shared behind a load balancer — server A hits, server B misses. Redis as `IDistributedCache` (`byte[]`, JSON) is cache-aside: miss → SQL → `Set` with absolute plus sliding expiry. Keys include tenant. Stampede: a short lock so 1,000 misses do not all hit SQL. Redis is a cache of permissions, not the source of truth — the database/CMS is; we invalidate on role change. Session in Redis means losing Redis logs everyone out; that is an accepted DR trade-off we say out loud.

**Kafka or RabbitMQ?**

- **Short:** RabbitMQ is queues and exchanges (direct / fanout / topic), durable plus persistent, prefetch, ack. Kafka is a partitioned log with offsets and replay — not an AMQP broker.
- **Deep:** Rabbit: producer → exchange → binding → queue → consumer; at-least-once with explicit ack; prefetch for competing consumers; durable queue **and** persistent message to survive a broker restart. Kafka: append-only log, consumer groups, replay by offset, high throughput. I pick Rabbit or Service Bus for commands and work queues; Kafka when we need replay, many independent consumers, or a real stream platform. Hangfire is for cron inside our process, not for another team to subscribe. Ops cost is a quality attribute — I will not stand up Kafka for three events a day.

**Volo ABP?**

- **Short:** Not used. It is a modular DDD framework on ASP.NET Core (Identity, multi-tenant, permissions included).
- **Deep:** ABP gives you a chassis: modules, permission system, multi-tenancy, AutoMapper, a conventional folder layout. We did not adopt it, so I will not pretend. What I *do* take from that shape: bounded modules, Identity as a product, permissions as data, not a custom framework. Introducing ABP into a living monolith is a rewrite risk; I would extract a module first, not wrap the whole solution.

**DistributedCache?**

- **Short:** Shared cache (Redis/SQL), `byte[]`, absolute or sliding expire. Different from in-process memory cache. Cache-aside: miss → DB → Set.
- **Deep:** `IDistributedCache` is the ASP.NET abstraction; implementations are memory (dev only), Redis, SQL Server. Every instance sees the same key. Values are bytes so you serialise. `Refresh` extends sliding expiry. Do not store huge graphs or secrets in plaintext. Invalidation is the hard part: prefer TTL plus a generation key per tenant over trying to remember every key you wrote.

**Optimize with SQL or EF?**

- **Short:** Both. Default is EF LINQ (`IQueryable`, filter in SQL). Stored procedures for heavy reports. No queries in loops; no materialize-then-Contains on large sets.
- **Deep:** Keep the query as `IQueryable` until `ToListAsync` so `Where`/`Select` become SQL. A `ToList` then `.Contains(ids)` on tens of thousands of keys is a giant IN clause — join instead. Loops with `await` per item are N+1. When the planner must nail a set-based write or a DBA-owned batch, I use a stored procedure or TVP. Views for a stable read contract. I look at the actual SQL (logging or profiler) before I “optimise”.

**How do you optimise EF Core?**

- **Short:** Keep `IQueryable` until `ToListAsync`; `AsNoTracking` on reads; `Select` needed columns; `AsSplitQuery` with several collections; bulk insert; indexes matching `WHERE`/`JOIN`.
- **Deep:** Reads should not use the change tracker (`AsNoTracking`). Project to a DTO instead of `Include` of a whole graph. Two collection Includes in one SQL JOIN cartesian-explode — `AsSplitQuery` or two queries. Compiled queries only if compile time shows up in traces. `SaveChanges` once per use case, never per row in a 5k insert. Turn lazy-loading **off** on web APIs. Concurrency: `rowversion` on human-edited aggregates. Measure with `Stopwatch` and the query log; a `let` in query syntax can allocate extra types and be slower.

**What is Hangfire for?**

- **Short:** Background and cron: email, reports, KPI, labels. The HTTP request does not wait.
- **Deep:** Fire-and-forget, delayed, recurring (cron), stored in SQL (or Redis). Dashboard for succeeded/failed, locked to admins. Examples: staff day-off, absence, SP KPI, SKU labels. If jobs starve HTTP threads, run Hangfire in a **second process** — API only enqueues. Hangfire after `SaveChanges` can lose the job on a crash; if the event must survive, write an outbox in the same transaction, then a Hangfire dispatcher publishes. Hangfire is not the contract for another team’s consumer — that is a bus.

**Async vs sync?**

- **Short:** Async for I/O; sync for CPU; the action returns `Task`; never `.Result`.
- **Deep:** `async`/`await` frees the thread pool while SQL or HTTP waits. CPU-bound work is not faster with fake async. `.Result` / `.Wait()` can deadlock or starve the pool. `Task.WhenAll` for independent I/O. Do not `Task.Run` inside a request to “make it async” — that is still burning a pool thread; use Hangfire or a queue. EF: `ToListAsync`, `SaveChangesAsync`. Named `HttpClient` with an explicit timeout.

### Round B — delivery and operations

**CI/CD — did you touch servers?**

- **Short:** Be honest. Pipeline is build → test → deploy; secrets in a variable group. If I did not have server access, I say so.
- **Deep:** A healthy pipeline: restore, build, unit/architecture tests, integration tests against real SQL (containers), deploy staging, smoke a few journeys, approve production. Secrets never in git — variable groups or Key Vault. I can write YAML and read a failing job log even if I never SSH’d the box. If the role needs people who operate IIS/K8s daily, I will not inflate that.

### Round C — architecture

**1. Sketch a microservices architecture.**

- **Short:** Client → BFF/gateway → auth, catalog, order, notification; each with its own database; Redis; worker; bus. Today we may still **deploy** one modular API — I say that.
- **Deep:** Independently deployable services, database per service, API gateway or BFF at the edge, messaging for side effects, Redis for session/cache. Dark energy (team autonomy, scale) pulls toward more services; dark matter (ACID, fewer network hops) pulls back. A distributed monolith (12 repos, one team, one database) is worse than a modular monolith. I draw modules inside one process if that is production, and name the first extract (Notification) if fan-out or SLA diverges.

**2. Besides REST, what else?**

- **Short:** gRPC internally, messages (Rabbit/Service Bus), Hangfire, SignalR; SOAP if legacy.
- **Deep:** REST for public/BFF (cacheable GET, wide tooling). gRPC for service-to-service (protobuf, low latency; not browsers without grpc-web). Messages when the caller must not wait. Hangfire for schedules inside our boundary. SignalR for push. GraphQL only if a BFF is drowning in chatty GETs — cost is N+1 in resolvers and field-level authZ. I do not add a style because it is fashionable.

**3. How do you handle API errors?**

- **Short:** Exception middleware → log + safe 500. Validation 400 with `errors[].code`. 404/401/403 as domain exceptions. No stack traces to the client.
- **Deep:** FluentValidation before the handler (and before a transaction). `NotFoundException` → 404. Unauthenticated 401, forbidden 403. Envelope `{ succeeded, result, errors: [{ code }] }` — clients key off `code`, not message text. Middleware logs with `traceId`; production body never includes the stack. Empty `catch` is forbidden. Validation failures are not 500.

**4. What authentication does the project use?**

- **Short:** Cookie on the BFF; JWT on APIs; OpenID Connect / Azure AD.
- **Deep:** Browser never sees a long-lived token. BFF is a confidential client (auth code + secret), stores access + refresh in Redis, sets httpOnly Secure SameSite cookie. APIs validate JWT `iss`/`aud`/`exp`/signature. Mobile: auth code + PKCE, tokens in the OS store. Workers: client credentials. Downstream as the user: on-behalf-of. No ROPC, no implicit, no JWT in `localStorage`.

**5. Token is near expiry — what then?**

- **Short:** Client reads `exp` or catches 401 → refresh → new access → retry. The BFF may refresh silently using Redis.
- **Deep:** Access JWT 15–60 minutes. Before expiry, or on 401, the BFF uses the rotating refresh token, gets a new access token, retries the original call. Resource servers reject expired JWT — they do not refresh. Reuse of an old refresh revokes the family (theft). Session TTL (~2 h) is independent; when it dies, the user logs in again.

**6. What is a refresh token for?**

- **Short:** A new access token without logging in again; longer TTL; rotate; revoke on logout.
- **Deep:** Access tokens are bearer and cannot be un-issued, so they stay short. Refresh lives on the server (Redis/httpOnly), is rotated on each use, and is revoked on logout or password change. It never goes to a SPA. APIs do not accept refresh tokens as access.

**7–8. N-layer vs Clean Architecture? Draw Clean.**

- **Short:** N-layer is Presentation → Business → Data, dependencies down. Clean is circles, dependencies inward: API → Application → Core, Infrastructure/EF pointing up at Core.
- **Deep:**

```text
        UI / API
            │
            ▼
       Application  ──► ports (interfaces)
            │
            ▼
          Core
            ▲
            │
   Infrastructure / EF
```

N-layer ships fast; UI often references EF. Clean keeps the domain free of EF so you can test and swap infrastructure. If Application still references DataAccess, I do not fake folders. New work: MediatR use cases; an architecture test so the API does not gain more EF. Vertical slices (one folder per use case) compose with either.

**9. DI, DIP, IoC — difference?**

- **Short:** DIP is the principle (depend on abstractions). IoC is the container inverting `new`. DI is constructor injection.
- **Deep:** DIP: high-level policy depends on `IDataAccess`, not `SqlDataAccess`. IoC: something else constructs the graph. DI: constructor (default), setter, or method. Captive dependency — Singleton holding Scoped `DbContext` — is the architect-level bug. Lifetimes: Singleton config/Redis; Scoped `DbContext`; Transient validators. `IHttpClientFactory` for outbound HTTP, never `new HttpClient()` in a loop.

**10. What is middleware?**

- **Short:** A `RequestDelegate` pipeline: log, auth, exception, session. Filters run after an action is selected.
- **Deep:** Each middleware sees every request (static files, health, MVC). Order: exception → HTTPS/static → routing → CORS → authentication → authorization → session → endpoints. Filters run later with `ActionContext`/`ModelState`: authorization, resource, action, exception, result. Cookie/JWT auth belongs in middleware. Feature/role checks that need the action belong in a filter. Wrong middleware order → empty `User` or 401.

**11. Singleton / Scoped / Transient?**

- **Short:** Singleton = whole app (config, Redis). Scoped = per request (`DbContext`). Transient = every resolve.
- **Deep:** Hangfire jobs are not HTTP requests — create a scope with `IServiceScopeFactory` or you reuse a disposed `DbContext`. Singleton `HttpClient` without `IHttpClientFactory` exhausts sockets. Transient for lightweight stateless services. Never Singleton `DbContext` (not thread-safe, holds a connection).

**12. AsNoTracking, AsSplitQuery?**

- **Short:** NoTracking = reads, no change tracker. SplitQuery = split SQL when including several collections, avoids cartesian product.
- **Deep:** Tracking snapshots every property for `SaveChanges` — wasteful on lists. `AsNoTracking` entities should not be updated unless you `Attach`. One JOIN of parent + two collections multiplies rows (cartesian explosion). `AsSplitQuery` runs one query per collection; data can drift if rows change between queries. Prefer a DTO `Select` when you do not need entities.

**13. Lazy vs eager loading?**

- **Short:** Eager = `Include` up front. Lazy = load on touch (off in APIs). Explicit = `Load()`.
- **Deep:** Eager when you know you need children and the graph is small. Lazy looks convenient and causes N+1 under a serializer that walks navigations. Web APIs: proxies off. Explicit load when only some parents need children. `ThenInclude` for deeper graphs; still project when the screen needs five columns.

**14. Is the 1-N problem from lazy or eager?**

- **Short:** N+1 comes from lazy or a forgotten `Include`. Cartesian explosion comes from eager of two collections.
- **Deep:** N parents, each touch of `parent.Children` = N extra SQL — lazy/forgotten Include. One SQL `JOIN` of two collections = parent rows × children × other children — eager cartesian. Fix N+1 with Include/projection/split. Fix cartesian with split query or two queries or a DTO. Turning lazy on “to make Include optional” is how list endpoints die in production.

**15. User A is viewing a row; user B already saved.**

- **Short:** Optimistic `rowversion` → 409 and reload. For check-in uniqueness a unique index is stronger than a version column.
- **Deep:** No lock on read. Column `rowversion`; `UPDATE … WHERE Id=@id AND Version=@old`; 0 rows → `DbUpdateConcurrencyException` → 409 with a message to reload (or merge). Last-write-wins only if losing a field is acceptable. Scarce inventory: short `UPDLOCK` transaction or Redis reservation plus async reconcile. Mobile offline: per-field merge or a conflict queue — last-write-wins is hostile. Duplicate check-in: unique `(StaffId, Day, OutletId)` plus an idempotency key from the client.

## Whiteboard (45 minutes)

**L1:** staff app, admin web, Azure AD, SAP/master data, FCM, SharePoint (legacy), your system.

**L2:** BFF, API and/or worker, SQL, Redis, blob, optional Service Bus, Hangfire store.

**Check-in sequence**

1. App → BFF cookie or token.  
2. BFF → API JWT.  
3. API authorises outlet assignment (IDOR).  
4. Transaction: timesheet row + outbox.  
5. 201 to the user.  
6. Worker: virus scan, FCM, search projection.

Call out: unique index against duplicate check-in; SharePoint not on this path; p95 budget; breaker on FCM — FCM failure must not fail check-in.

**“Microservices?”** Modular monolith today; extract Notification if fan-out or SLA diverges.

**“Scale 100×?”** Connection pool and SQL first; read replica for reports; queue photos; split the worker; then a read model. Not twenty repositories on day one.

### Follow-ups on the whiteboard

**N-layer vs Clean?**

- **Short:** N-layer ships fast, dependencies down. Clean inverts so the domain does not reference EF. I tighten modules and tests before a folder religion.
- **Deep:** N-layer: UI → BLL → DAL; the UI often knows EF. Clean: Core in the middle, Infrastructure implements ports. Both can be a modular monolith. I will not rename folders while Application still references DataAccess. New use cases: MediatR, no EF types in Core, an architecture test on the API project.

**Why not Kafka?**

- **Short:** No replay or multi-consumer stream yet. Service Bus or Rabbit for commands; Hangfire for schedules. Kafka when we have a platform and a replay story.
- **Deep:** Kafka shines at high-throughput logs, independent consumer groups, replay. A work queue with ack/prefetch/routing is Rabbit or Service Bus. Standing up ZooKeeper/KRaft, disk, and consumer lag for three events a day is an ops quality-attribute failure. If analytics later needs a year of events, I add Kafka (or Event Hubs) as a sink from the outbox, not as the command bus.

**Token expires while the user works?**

- **Short:** Short access; BFF refresh with rotating refresh in Redis; 401 interceptor; explicit session TTL. APIs reject expired JWT.
- **Deep:** Access 15–60 min. BFF sees `exp` or 401, uses server-side refresh, retries. Resource APIs never refresh. Rotating refresh: reuse of a stolen token kills the family. Cookie sliding expiration is not the same as session `IdleTimeout` — I configure both on purpose.

**User A is viewing; user B already saved.**

- **Short:** Optimistic `rowversion`; 409 + reload.
- **Deep:** Same as Round C question 15: version in the `WHERE`; 0 rows → concurrency exception. Unique constraint for “must not exist twice”. Pessimistic locks only for scarce seats, and keep the transaction tiny.

**Lazy vs eager 1-N?**

- **Short:** N+1 is lazy. Cartesian is two Includes. APIs: lazy off.
- **Deep:** Same as Round C question 14. Serializer walking navigations is a common silent N+1. I would show a log of 1+N SQL as evidence.

**How do you know production is healthy?**

- **Short:** SLOs on five journeys, burn-rate alerts, traces including Hangfire, live vs ready, error-budget policy for releases.
- **Deep:** SLI = p95 of check-in, % 5xx. SLO = 99.5% under 2 s over 30 days. Alert when the error budget burns too fast, not when CPU is 80%. `/live` vs `/ready` — ready must not fail because SharePoint is down if we can degrade. W3C `traceparent` on the Hangfire job. Restore drill for SQL quarterly.

**Biggest risk in this kind of codebase?**

- **Short:** Application coupled to DataAccess; Hangfire in-process; third-party latency on the request thread; .NET 6 out of support; missed IDOR; in-memory cache after scale-out.
- **Deep:** I pick the one that burns the SLO: SharePoint on the synchronous path, or N+1 on list endpoints. Then the EOL runtime (security patches). Then IDOR if `[Feature]` is forgotten — one shared authorization filter beats a check per action. In-memory cache is a latent incident the first time we add a second instance.

**Design for 5,000 req/s flash.**

- **Short:** Do not hit SQL at 5k if it does 2k. Queue, idempotent consumers, reserved stock, DLQ.
- **Deep:** Accept 200, persist an idempotency key, enqueue. Drain at what SQL can do. Stock: Redis reservation or `rowversion` on a counter; reconcile async. Side effects (mail, points) are workers, not the request. Poison messages → DLQ + alert. After the peak the queue drains. If stock is 1,000, extra requests fail fast on the reservation, not after a 30 s SQL timeout.

**DIP vs IoC vs DI?**

- **Short:** Principle / container / constructor injection.
- **Deep:** Same as Round C question 9. I mention captive dependencies without being asked — that is the follow-up they wanted.

## Round D — TA panel

Questions a technical architect panel asks after the whiteboard. **Short** then **Deep**.

**Estimate load for this field-force system.**

- **Short:** 10k staff, 30% at peak, 2 calls/min → ~100 req/s, ~40 in-flight at 400 ms. One SQL SKU is enough; photos go to blob (~tens of GB/day), not SQL.
- **Deep:** I write the formula on the board and mark every assumption. Connection budget: instances × pool size versus SQL max. 10× users → scale API and Redis, then SQL replica for reports. I will not propose Kafka or Kubernetes at 100 req/s unless they give a different constraint (fan-out, team topology, residency).

**How do you migrate a hot table without downtime?**

- **Short:** Expand/contract: add column, dual-write, backfill, switch read, drop old. Online index. Feature flag for the behaviour switch.
- **Deep:** One migration that rebuilds a clustered index on a 50 M row table in a change window is how you get an incident. Dual-write has a kill switch. Backfill is a Hangfire job with batch size and a progress row, not a blocking SQL script. Rollback is “read the old column again”, not restore the whole database.

**Offset pagination vs keyset?**

- **Short:** Offset is simple and slow on deep pages. Keyset on `(sort, id)` is what I use for infinite scroll and mobile. Always cap page size.
- **Deep:** `OFFSET 100000` still reads 100000 rows. Concurrent inserts make page 2 duplicate or skip. Keyset: `WHERE (UpdatedAt, Id) < (@t, @id)`. Total count is expensive — I return `hasMore` unless the product truly needs `total`.

**Read replica after a write?**

- **Short:** No. Replica is stale. The user who just saved reads the primary. Dashboards may be stale-OK.
- **Deep:** Async replica lag is seconds under load. Session “read-your-writes”: sticky to primary for N seconds after a POST, or a `Consistency: strong` header on that GET. KPI screens document “updated every 5 minutes”.

**Domain event vs integration event?**

- **Short:** Domain = inside one context. Integration = versioned contract to another context, no entities on the wire.
- **Deep:** After commit, the outbox holds the integration payload (`TimesheetSubmittedV1` with ids and totals), not `Timesheet` plus navigations. Additive fields; never reuse a field. Consumers are idempotent. If two contexts share a table, they are not two contexts.

**How do you test architecture?**

- **Short:** Unit invariants, integration golden journeys on real SQL, contract tests at the BFF, load against SLOs, architecture tests for forbidden references.
- **Deep:** I will not sign off a design whose only test is a slide. A fitness function in CI fails the build if a controller new’s a `DbContext`. Load: ramp to SLO, soak for leaks, stop if error budget on staging burns. Production is not the first load test.

**Walk an incident.**

- **Short:** Severity, commander, stop the bleeding (rollback/flag), then cause, then a fitness function so it cannot recur.
- **Deep:** Check-in p95 8 s. I look at traces, not CPU. SharePoint span is 6 s — open the breaker, degrade photos, restore SLO. Postmortem: SharePoint off the sync path, timeout 3 s, dedicated `HttpClient`. I do not restart the App Service as the “fix”.

**Rate limiting?**

- **Short:** Edge plus per-user on the API. Login tighter than GET. Return 429 with `Retry-After`.
- **Deep:** Anonymous vs authenticated buckets. Idempotent GET can be aggressive; POST check-in is per staff per minute. Gateway WAF for volumetric DDoS; API for fairness. Limits in config, not magic numbers in code.

**How do you version a public API?**

- **Short:** Additive fields, no rename. Breaking = `v2` plus a sunset date. Enums stay integers if that is the contract.
- **Deep:** Mobile binaries lag weeks. I add fields; old clients ignore them (tolerant reader). I do not ship `JsonStringEnumConverter` as a “fix”. Deprecation header, telemetry on `v1` traffic, then switch off.

**Multi-tenant?**

- **Short:** Tenant from the token, column on every table, cache key includes tenant. Separate DB only for residency or noisy neighbour.
- **Deep:** Query-string tenant is an IDOR. EF global filter on `TenantId`. Admin impersonation is an explicit, audited claim. A missed filter is a data-leak incident — integration tests for two tenants.

**GUID or int primary key?**

- **Short:** `bigint` identity clustered for OLTP. Random GUID fragments the index.
- **Deep:** External ids can be GUID in a unique nonclustered column. Sequential GUID if the client must create the id offline. I do not cluster on `NEWID()`.

**How do you work with a product owner who wants microservices next quarter?**

- **Short:** I ask which constraint (team, scale, data, runtime) we are solving. If none, I propose a modular monolith and an ADR.
- **Deep:** I translate to a quality-attribute scenario and a cost (ops, latency, 2PC). I offer a strangler of **one** capability with a success metric. “Netflix does it” is not a driver. If they still insist, I document the risk and the rollback — I do not sabotage, and I do not pretend it is free.

**When were you wrong as an architect?**

- **Short:** Pick a real case: caching without a stampede lock, or Hangfire after `SaveChanges`. What you changed, what fitness function you added.
- **Deep:** The panel is scoring honesty and a feedback loop. Structure: decision → production evidence → reversal → how we prevent a repeat. Do not invent a story where you were secretly right.

**App Service vs AKS?**

- **Short:** App Service until we have many services, sidecars, and a platform team. AKS is a product you now operate.
- **Deep:** App Service: TLS, scale-out, slots, enough for one API + worker. AKS: node pools, ingress, mesh, upgrades, on-call for the cluster. Container Apps as a middle. I choose the **least** platform that meets the SLO.

**How would you load-test check-in?**

- **Short:** Script the golden journey, ramp to 2× expected peak, assert p95 and error ratio against the SLO, soak for an hour.
- **Deep:** Production-like data volume (not 10 rows). Idempotency keys so retries do not double-insert. Watch SQL DTU, lock waits, Redis CPU, thread pool. Stop if we burn the staging error budget. Never “see what happens” on prod Friday.

**OWASP API — top risks you actually design for?**

- **Short:** IDOR (BOLA), broken auth, unconstrained payload, injection, security misconfig (public blob, open Hangfire).
- **Deep:** Every `GET /{id}` is an authorization test, not only authentication. BFF so tokens are not in JS. Max body size, virus scan async on photos. Parameterised SQL / EF. Hangfire and Swagger locked down in production. Rate limit login. No stack traces.

**Soft delete vs hard delete?**

- **Short:** Soft delete for undo and audit, filtered index, global EF filter. Hard delete PII on a retention job when required.
- **Deep:** Soft delete that every query forgets is worse than hard delete. Unique email + soft delete needs a filtered unique index. “Delete account” is a process: anonymise, revoke tokens, tombstone, legal hold exceptions.

**Where does UTC live?**

- **Short:** Store UTC. Display in the user zone. “Business today” is the outlet’s date.
- **Deep:** `DateTime.Now` in a handler is a bug in APAC. `TimeProvider` injected. Overnight jobs use a named time zone per market. `datetimeoffset` or UTC `datetime2` plus zone in the user profile — pick one and ADR it.

---

## 30 / 60 / 90 days

**30:** as-is C4, SLO draft, incident review, dependency diagram, .NET 6 risk, list of N+1 and chatty integrations.

**60:** ADRs for identity, data, messaging, compute; golden-path tests; worker-split spike; Key Vault gaps closed; SharePoint breaker.

**90:** .NET 8 in motion or done; outbox on one domain event; module-boundary tests; on-call dashboard; cost per journey.

## Night before

- [ ] C4 of the field-force system from memory
- [ ] Outbox versus 2PC in one minute
- [ ] BFF cookie flow in one minute
- [ ] PACELC and one saga example
- [ ] App Service versus AKS in two bullets
- [ ] Three fitness functions
- [ ] One ADR for week one (outbox shape)
- [ ] One thing you will **not** do (Kafka / microservices / rewrite)
- [ ] Load estimate: 10k staff → ~100 req/s on the board
- [ ] Expand/contract migration in four steps
- [ ] Keyset pagination vs offset
- [ ] Incident: stop bleeding before root cause
- [ ] HR answers out loud once
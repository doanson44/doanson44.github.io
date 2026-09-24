# Full knowledge phỏng vấn (Tiếng Việt)

Tài liệu **độc lập**. Đọc từ trên xuống. Không cần file nào khác.

Cách học: mỗi heading nói được một ví dụ. Phần 1–2 = senior backend. Phần 3–4 = technical architect.

Giọng trong phòng: số liệu → ràng buộc → phương án A/B → giá → rollback.

Câu trả lời **HR để tiếng Anh** (nói trong phòng).

Hệ thống mẫu: field-force / trade marketing — app mobile + admin web, BFF, ASP.NET Core, SQL Server, EF Core, MediatR, Hangfire, Redis, Azure AD.

---

# Phần 1 — Nền tảng

## OOP

Đối tượng: **thuộc tính** (thông tin) và **phương thức** (hành vi).

- **Trừu tượng:** giữ phần cốt (nhân viên: tên, ngày sinh) — bỏ chiều cao, sở thích.
- **Đóng gói:** ẩn state; chỉ đổi qua method public.
- **Kế thừa:** con dùng lại cha (`Smartphone` → iPhone / Samsung). C# chỉ một class cha.
- **Đa hình:** cùng lời gọi, hành vi khác (iOS vs Android; chó vs mèo).

### Virtual vs abstract vs interface

| | Virtual | Abstract | Interface |
|---|---------|----------|-----------|
| Thân | Có mặc định | Không | C# 8+ default tùy chọn |
| Override | Không bắt buộc | Bắt buộc | Implement hết (trừ default) |
| Nằm ở | Class bất kỳ | Chỉ abstract class | Kiểu hợp đồng |
| Nhiều | — | Một base class | Nhiều interface |

Interface = “làm được gì” (`IDisposable`). Abstract class = “là loại gì” + code chung.

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

### Generic

Viết một lần, kiểu do caller: `List<T>`, `Dictionary<TKey,TValue>`. Tránh `ArrayList` boxing và lỗi cast runtime.

### String vs StringBuilder

`string` immutable — mỗi lần đổi là object mới. `StringBuilder` nới buffer. Dùng builder trong vòng lặp.

### List vs ArrayList

Ưu tiên `List<T>`. `ArrayList` chứa `object`: compile được, runtime có thể vỡ.

### Tham trị / tham chiếu

Mặc định copy giá trị (hoặc copy reference với object). `ref`: biến gốc đổi; phải gán trước. `out`: phải gán trong hàm; caller không cần gán trước. Chỗ gọi cũng viết `ref` / `out`.

### Stack vs heap

Cả hai trên RAM.

- **Stack:** local, tham số, nhanh, cố định (~1 MB/thread Windows), tự giải phóng khi return, overflow nếu đệ quy vô hạn. Mỗi thread một stack.
- **Heap:** object `new`, GC trong .NET, dùng chung giữa thread (cần đồng bộ).

### `static`

Thuộc type, không thuộc instance. Một field / AppDomain trên Managed Heap. Static mutable = state dùng chung, khó test, race.

### Singleton

Một instance: `services.AddSingleton<T>()`, hoặc constructor private + `Instance` / `Lazy<T>`. Không singleton `DbContext`. **Captive dependency:** Singleton giữ service Scoped (ví dụ `DbContext`) — bug production.

### `volatile`

Hint chống đọc stale giữa thread. Thường **không** phải công cụ đúng: không atomic ngoài gán, không chống race. Ưu tiên `Interlocked`, `lock`, concurrent collection. Không gắn `long`/`double`. Hẹp: cờ `_shouldStop` để compiler không cache ở register.

### Provider (hai nghĩa)

1. Logging / configuration: `ILoggerProvider`, `IConfigurationProvider` — ai cung cấp implementation.
2. Observer: `IObservable<T>` — `Subscribe` trả `IDisposable`; đẩy `OnNext` / `OnError` / `OnCompleted`.

## SOLID

- **S:** một lý do để sửa. Tách “đọc DB” và “in báo cáo”.
- **O:** mở rộng; không sửa class cũ mỗi feature.
- **L:** con thay được cha. Cánh cụt không phải `FlyingBird`.
- **I:** nhiều interface nhỏ, không một interface béo.
- **D:** phụ thuộc abstraction. Ổ điện = interface; bóng đèn = implementation.

## DIP vs IoC vs DI

- **DIP:** nguyên lý — module cấp cao phụ thuộc abstraction.
- **IoC:** đảo chỗ `new`; việc tạo nằm ngoài class dùng.
- **DI:** cách inject — constructor (mặc định), setter, hoặc method. Container: register, resolve, lifetime.

Ba kiểu DI: constructor, property/setter, method (`SetDependency`). ASP.NET Core mặc định constructor.

Lifetime: **Singleton** (config, Redis multiplexer) / **Scoped** (`DbContext`, một / HTTP request) / **Transient** (mỗi lần resolve một instance).

## Design pattern

**Khởi tạo** (giấu `new`): Factory, Singleton, Builder, Prototype.

**Cấu trúc:** Adapter, Decorator, Facade, Proxy.

**Hành vi:** Strategy, Observer, Mediator, Command, Template Method.

Factory: switch định dạng ảnh → `GifReader` / `JpegReader`.

**CQRS:** command ghi; query đọc (`AsNoTracking`, DTO). **MediatR:** controller gửi `Request`; một handler một use case; validation trong pipeline. Không nhét business vào controller.

## Frontend

**React (class):** mount `constructor → render → componentDidMount`; update `render → componentDidUpdate`; unmount `componentWillUnmount`. Function: `useEffect`. `PureComponent` / `React.memo` = so sánh nông props/state.

**Angular:** class + `@Component({ selector, template, styles })` + `@Input` / `@Output` + service inject ở constructor.

Vòng đời: `constructor` (DI, chưa có input) → `ngOnChanges` → `ngOnInit` (fetch) → `ngDoCheck` → content hook → view hook (`ViewChild` sẵn) → `ngOnDestroy` (unsubscribe). Không HTTP trong constructor.

**Webpack:** entry → dependency graph → bundle. Loader transpile; plugin minify; `import()` động tách chunk. Angular CLI / Vite che bundler.

**N-layer vs MVC:** n-layer là cả hệ thống (UI | business | data). MVC là pattern UI (Controller, Model, View). Dùng chung được. **Clean Architecture** đảo dependency: domain không reference EF.

## Scrum

Chuyển giao increment dùng được mỗi 1–4 tuần.

**Artefact:** product backlog (Product Owner); sprint backlog (team chọn lúc planning); increment; burndown.

**Vai trò:** Product Owner (ưu tiên, thị trường); Scrum Master (coach quy trình); development team (làm).

**Meeting:** sprint planning; daily (hôm qua / hôm nay / blocker); sprint review (demo việc xong); retrospective (quy trình).

---

# Phần 2 — Backend

## MVC và .NET

**Model** = data và rule. **View** = UI. **Controller** = HTTP vào, xử lý model, trả view hoặc JSON.

**.NET Core** (nay gọi “.NET”): đa nền tảng, container, microservice, chạy song song nhiều version.

**.NET Framework:** Windows, WPF / WinForms, app sẵn thì extend hơn rewrite.

**Kestrel** là web server bất đồng bộ. Trên Azure App Service nằm sau IIS/ANCM.

## HTTP stateless

Mỗi request phải tự đủ thông tin. Giả lập state: URL, form, cookie, hoặc session server (id trên cookie).

Controller → view:

| | ViewBag | ViewData | TempData |
|---|---------|----------|----------|
| Kiểu | `dynamic` | dictionary | dictionary |
| Sống | một request; mất khi redirect | một request | đến khi đọc; `TempData.Keep()` để giữ |
| Ép kiểu | không | có | có |

**Thứ tự model binding** khi trùng tên: form > route > query string.

**Web API vs MVC:** API trả data, content negotiation (`Accept`), map theo HTTP method. MVC map theo tên action, có thể trả view. Cả hai trong một project = hai filter pipeline.

## Routing

Parse URL → khớp template trong route collection → handler. Ví dụ: `{controller=Home}/{action=Index}/{id?}`. Route handler MVC tạo controller và chạy action.

## Middleware vs filter

**Middleware** chạy mọi request. Thứ tự khuyến nghị:

1. Exception handler
2. HTTPS / static files
3. `UseRouting`
4. CORS
5. `UseAuthentication` (gắn `HttpContext.User`)
6. `UseAuthorization`
7. `UseSession`
8. Endpoint (`MapControllers`)

Mỗi middleware gọi `next` hoặc cắt pipeline. Sai thứ tự → `User` rỗng, session chưa sẵn, 401/403.

**Filter** chạy sau khi routing đã chọn action:

| Filter | Khi |
|--------|-----|
| Authorization | Đầu tiên; không “after”; đừng throw (exception filter không bắt) |
| Resource | Bọc cả model binding; cache hit có thể short-circuit |
| Action | Ngay quanh action method; có thể gán `Result` |
| Exception | Unhandled trong action; ưu tiên exception middleware trừ khi API vs HTML cần body khác |
| Result | Quanh lúc execute result (view / JSON) |

Scope: global → controller → action lúc vào; ngược lại lúc ra. `IOrderedFilter.Order` đè scope (số nhỏ chạy sooner lúc vào).

DI: `[ServiceFilter<T>]` cần đăng ký `T`; `[TypeFilter(typeof(T))]` không cần. Implement sync **hoặc** async, không cả hai (runtime ưu tiên async).

Short-circuit: gán `context.Result` và không gọi `next`.

Ví dụ quyền: sau JWT, action filter lấy user theo id từ Redis/CMS rồi check feature/role.

## Authentication vs authorization

**Authentication (AuthN)** = bạn là ai. Kết quả: `ClaimsPrincipal` trên `HttpContext.User`. Scheme: cookie (web / BFF), JWT Bearer (API), OpenID Connect (Azure AD). Cookie `SlidingExpiration`: renew khi đã qua hơn nửa `ExpireTimeSpan`.

**Authorization (AuthZ)** = được làm gì. `[Authorize]`, role, policy, check **theo resource** (IDOR: đã login vẫn phải đúng assignment outlet/plan). 401 = chưa auth hoặc token hết hạn. 403 = biết là ai, không đủ quyền.

Web: BFF giữ session trên Redis, set cookie httpOnly. API nhận JWT. Không để token dài trong `localStorage` SPA.

## Session và cache

**Session:** data trên server; cookie chỉ chứa session id. `IdleTimeout` mặc định 20 phút, **sliding** (mỗi request qua session middleware reset đồng hồ). Độc lập với lifetime cookie login. Scale-out: Redis/SQL, không memory trong process.

**IMemoryCache:** trong process. `AbsoluteExpiration` chết đúng giờ. `SlidingExpiration` gia hạn mỗi lần get (nên kèm absolute tối đa). Mất khi restart; không share máy.

**IDistributedCache:** value `byte[]` (serialize JSON). Redis hoặc SQL. Cache-aside: miss → query DB → `Set`. Stampede (nhiều miss cùng lúc): lock ngắn. Key phải gồm tenant (và locale/version nếu cần).

Server A cache memory, request sau vào server B → B miss — đúng kỳ vọng. Dùng Redis.

## Async vs sync

**Async** cho I/O: database, HTTP, file (`ToListAsync`, `HttpClient.GetAsync`). Thread trả về pool khi chờ.

**Sync** cho CPU trong process. Async giả trên CPU không nhanh hơn.

Không `.Result` / `.Wait()` trên async (deadlock / đói thread pool). Việc user không cần chờ: Hangfire hoặc queue.

## WPF (nếu hỏi)

`Binding` + `DataContext`. Mode: OneWay, TwoWay, OneTime, OneWayToSource. Source nên `INotifyPropertyChanged`. List: `ObservableCollection<T>`.

## Bundling và minification

Bundling gom file thành một HTTP request. Minification bỏ khoảng trắng, comment. Nhiều file JS/CSS = nhiều request = chậm.

## Login và registration

**Register:** validate email unique + password policy → hash (ASP.NET Identity / PBKDF2 / bcrypt / Argon2) → insert user → optional confirm email. Không lưu plaintext. Unique index trên email.

**Login:** load user → verify hash → lockout sau nhiều lần sai → phát cookie hoặc JWT trên HTTPS. Cùng một thông báo lỗi nếu không muốn lộ email có tồn tại.

## EF Core

ORM: class C# ↔ bảng; LINQ thành SQL.

```csharp
builder.Services.AddDbContext<DatabaseContext>(options =>
    options.UseSqlServer(configuration.GetConnectionString("Database")));
```

`DbContext` **Scoped** (một / HTTP request). **Code First** + migration là đường thường. `SaveChanges` là unit of work.

**Load dữ liệu liên quan**

- **Eager:** `Include` / `ThenInclude` ngay câu đầu.
- **Lazy:** load khi đụng navigation — **tắt** trên web API (N+1).
- **Explicit:** `Entry(entity).Collection(x => x.Items).Load()`.

**Tối ưu**

- Giữ `IQueryable` đến `ToListAsync` để filter chạy trên SQL.
- Không `await` database trong vòng lặp — load theo tập key một lần.
- Không `ToList()` rồi `.Contains()` trên tập key lớn — compose join.
- Đọc: `AsNoTracking()`. `Select` đúng cột cần.
- Nhiều collection: `AsSplitQuery()` tránh cartesian explosion.
- Bulk insert/update hàng nghìn dòng; không `SaveChanges` từng dòng.

**Đo.** `let` trong query syntax có thể tạo anonymous type phụ và *chậm hơn* dù “tính một lần”. Mẹo LINQ to Objects có thể hại SQL của EF.

**N+1:** lazy hoặc quên `Include` — N câu thêm. **Cartesian:** eager `Include` hai collection trong một JOIN. Fix: project DTO hoặc split query.

**Optimistic concurrency:** `byte[]` `rowversion` / `[Timestamp]`. SQL: `UPDATE … WHERE Id=@id AND Version=@old`. 0 row → `DbUpdateConcurrencyException` → bảo user reload.

**Repository / unit of work:** `DbContext` đã là UoW và identity map. Repo mỏng được. Tránh `IRepository<T>` 20 method không ai dùng.

## SQL

**Index:** B+ tree trên đĩa (rowstore) để tìm nhanh. Tự maintain mỗi lần ghi.

**Clustered:** chính bảng lưu theo thứ tự key. **Một** / bảng. Không clustered = **heap**. PK thường tạo clustered. Key hẹp, tăng dần (`bigint` identity). GUID ngẫu nhiên phân mảnh.

**Nonclustered:** cấu trúc riêng. Leaf = key + locator (RID trên heap, clustered key trên bảng clustered). `INCLUDE` cột thêm ở leaf để covering (không key lookup).

Optimizer vẫn có thể scan nếu khớp phần lớn bảng. Index làm chậm `INSERT`/`UPDATE`/`DELETE`.

**View:** `SELECT` đặt tên, query như bảng. Lớp bảo mật tùy chọn. **Indexed view** materialize — phạt ghi.

**Stored procedure:** T-SQL có tham số, rẽ nhánh, transaction. Plan cache, ít round-trip. Dùng set nặng; không giấu cả domain trong SP.

**Trigger:** `AFTER`/`INSTEAD OF` insert/update/delete. Dễ ẩn side effect. Audit thì được; business lớn thì không.

## Bài tập: sinh viên — khóa học

Nhiều-nhiều cần bảng trung gian:

```text
Student              Enrollment                 Course
--------             -----------                ------
Id PK                StudentId FK               Id PK
FullName             CourseId FK                Code unique
DateOfBirth          EnrolledAt                 Name
Email unique         Grade NULL                 Credits
                     PK (StudentId, CourseId)
```

Index: PK; `Email`; `Enrollment(CourseId)` (StudentId đã nằm trong PK).

Sắp theo năm sinh: `ORDER BY DateOfBirth` (dùng được index). `YEAR(DateOfBirth)` thường không. LINQ: `students.OrderBy(s => s.DateOfBirth)`.

---

# Phần 3 — Kiến trúc

Kiến trúc = **quyết định dưới ràng buộc**, ghi lại (ADR: context, decision, consequences). Kịch bản chất lượng sáu phần: nguồn, kích thích, môi trường, artefact, phản hồi, **số đo**. Không điền được số đo thì đó là ước muốn.

## Thuộc tính chất lượng

| Thuộc tính | Cách nói |
|------------|----------|
| Availability | Error budget. 99.9% ≈ 8.8 giờ downtime / năm. SLO không vượt mắt xích yếu nhất. |
| Latency | Budget p95/p99 cả **journey**, không một endpoint. Little: concurrency ≈ throughput × latency. |
| Scalability | API stateless + Redis dùng chung. Shard sau cùng. |
| Consistency | Mạnh trong một aggregate / một transaction SQL. Eventual giữa process. |
| Security | Test IDOR, BFF, secret trong Key Vault. |
| Observability | Một trace id từ BFF xuyên API xuyên Hangfire. Metric RED (rate, errors, duration). |
| Recoverability | RPO / RTO từ lần restore **đã chạy thật**. |
| Cost | Chi phí / staff active / tháng. |

**PACELC:** nếu Partition thì chọn Availability hoặc Consistency; không partition thì Latency hoặc Consistency.

Đừng hứa 99.99% nếu SQL dùng chung, Hangfire trên SQL, SharePoint trên đường nóng. Availability = min các dependency.

**ATAM-lite:** 3–5 kịch bản → đi C4 + sequence → điểm nhạy (đổi chỗ này chất lượng đảo) và đánh đổi (giúp A, hại B) → rủi ro → ADR hoặc spike.

**Fitness function:** kiểm tự động một luật kiến trúc, ví dụ: search phải phân trang; API không `new` kiểu data-access trong controller; không secret trong git; p95 login → list plan dưới 800 ms trên staging.

## C4

Vẽ **L1 context** (ai nói với hệ thống) và **L2 container** (BFF, API, worker, SQL, Redis, blob). Một sequence cho write rủi ro nhất. Zoom component chỉ khi được hỏi. Production là một process API thì vẽ **một container với ô module** — đừng giả 12 service deploy độc lập.

```text
 App staff ──┐
 Admin web ──┼──► BFF (cookie, session Redis) ──► Azure AD
             └──► (JWT nội bộ)
                    ├── Identity API
                    ├── Journey / Timesheet API
                    └── Campaign / KPI API
                           │
                    SQL Server + Redis + Blob
                    Hangfire worker
```

## DDD (20% hữu ích)

**Bounded context** = model có biên ngôn ngữ. “Status” journey plan không phải “status” timesheet.

**Aggregate** = biên nhất quán: invariant giữ trong một transaction, một connection.

Context map: **anti-corruption layer** bọc SharePoint / loyalty / ERP để domain không nói DTO của họ; **conformist** với identity provider (không tự invent user).

## Modular monolith trước

Tách **service** khi có ít nhất hai: scale độc lập, release train độc lập, data độc lập (tuân thủ / RPO), runtime khác.

Trước đó: folder module, prefix schema, không reference nội bộ module khác, architecture test fail build nếu vi phạm.

**Strangler fig:** đặt façade (BFF/gateway) phía trước → tách một capability ra sau façade → xóa path cũ khi hết traffic. Không đóng băng monolith 18 tháng để rewrite.

**BFF vs API gateway**

- Gateway: TLS, WAF, rate limit, routing.
- BFF: aggregate theo client, session cookie, giấu API nội bộ.

**Sync vs async:** HTTP khi user cần câu trả lời (validate, “có được check-in không?”). Queue khi HTTP 200 không cần side effect (email, push, index search).

## Data và consistency

Không 2PC phân tán giữa SQL và bus.

**Transactional outbox**

```text
BEGIN TRAN
  UPDATE JourneyPlan ...
  INSERT Outbox (Id, Type, Payload, CreatedUtc)
COMMIT
-- dispatcher publish, rồi đánh dấu processed
```

Giao **at-least-once**. Consumer phải **idempotent** (bảng inbox theo `EventId`, hoặc unique business key).

**Saga:** chỉ khi một transaction không đủ (publish campaign → sinh QR → notify agency hệ khác). Orchestration = một dòng process manager. Choreography = mỗi service phản ứng event. Compensation là command **tiến tới** (`CancelReservation`), không phải rollback.

**Mức CQRS**

0. Cùng model, `AsNoTracking` lúc đọc — mặc định.  
1. DTO / view SQL riêng.  
2. Store đọc riêng, event cập nhật.  
3. Event sourcing — hiếm.

Hai folder Command/Query mà shape giống nhau là diễn.

**SQL topology:** failover group cho DR; replica đọc + `ApplicationIntent=ReadOnly` cho report nặng; clustered key tăng dần; covering index cho màn search.

## Messaging

Cây quyết định:

```text
Caller cần kết quả cho UX?
  có → HTTP/gRPC, timeout chặt, retry chỉ khi idempotent
  không → async
        Một consumer trong process này?
          có → Hangfire (outbox nếu phải sống sau crash)
          không → message bus
                Cần replay / nhiều consumer / throughput rất cao?
                  có → Kafka / Event Hubs
                  không → Azure? session / duplicate detection?
                        có → Service Bus
                        không → RabbitMQ (tự vận hành)
```

**Exactly-once** xuyên HTTP + SQL + bus không tồn tại. Thiết kế at-least-once + idempotency.

### AMQP 0-9-1 (RabbitMQ)

Producer publish vào **exchange**. **Binding** định tuyến bản sao tới **queue**. Consumer nên subscribe (push), hạn chế pull.

| Exchange | Định tuyến |
|----------|------------|
| Default (direct không tên) | Binding key = tên queue — trông như “gửi thẳng queue” |
| Direct | Routing key khớp binding key |
| Fanout | Mọi queue đã bind; bỏ routing key |
| Topic | `*` = một từ, `#` = không hoặc nhiều từ |
| Headers | Khớp header (`x-match` any/all) |

Cờ queue: durable (metadata đĩa), exclusive (một connection, xóa khi đóng), auto-delete. Tên `amq.` dành riêng. Declare lại khác attribute → 406 PRECONDITION_FAILED.

**Durable queue không làm message sống sau restart.** Phải đánh persistent (`delivery_mode=2` / `persistent: true`).

**Ack:** tự động (broker xóa lúc deliver — có thể mất việc) vs tường minh (`basic.ack` sau khi xong). Consumer chết chưa ack → redeliver. Prefetch (`basic.qos`) giới hạn message chưa ack mỗi consumer (san tải).

**Connection** TCP dài. **Channel** multiplex nhẹ; một channel / thread, không share. **Vhost** cô lập môi trường.

Message không route được: drop hoặc trả publisher. Dead-letter sau max retry. Alert độ sâu DLQ.

### Azure Service Bus

Broker managed, consumer pull.

- **Queue:** một message, một competing consumer (phân việc, san tải).
- **Topic + subscription:** mỗi subscription một bản sao; filter SQL trên property.

PeekLock (lock, complete, abandon, dead-letter) vs ReceiveAndDelete. Duplicate detection, session (thứ tự / request-reply), message hẹn giờ. Basic chỉ queue; Standard/Premium có topic.

### Kafka / Event Hubs

Log append, partition, consumer giữ **offset**, replay. Throughput cao. Không phải work queue AMQP. Dùng stream/audit, không phải command “đặt hàng” trừ khi đã có nền tảng stream.

### Hangfire

Scheduler trong process (hoặc worker riêng): fire-and-forget, delay, cron. Dashboard succeeded/failed. Ví dụ: KPI đêm, day-off, nhãn SKU, email.

Khi job đói thread HTTP, chạy Hangfire **process thứ hai** (API chỉ enqueue). Hangfire không phải contract tích hợp team khác — cái đó là bus + outbox.

### Flash sale (5.000 req/s vs database 2.000 req/s)

Đừng đập SQL 5k/s. Đưa 5k vào queue bền; rút 2k/s. Phần còn lại xử sau giờ cao điểm. Consumer phải idempotent (hàng có hạn). Thêm dead-letter. Path sync vừa gửi mail, điểm, rank trong cùng request sẽ timeout — trả 200 và queue các side effect.

```text
Sync (xấu):  Client → API ──mail──điểm──rank──► 200 sau ~1000 ms

Async (tốt): Client → API → 200
                     └── queue → worker (mail, điểm, rank)

Spike: Client × 5000/s → Queue → API/DB 2000/s
```

## Security

**STRIDE** trên login và upload: spoofing (cookie bị cắp), tampering (IDOR id plan), repudiation (không audit), disclosure (token trong log, blob public), denial of service (upload khổng lồ), elevation (thiếu `[Authorize]`).

**IDOR** là rủi ro API số một app field-force. `GET /plans/{id}` phải check tenant + role + assignment, không chỉ authentication.

**OAuth / OIDC**

| Flow | Client | Dùng |
|------|--------|------|
| Authorization code + client secret | BFF | Web (ưu tiên) |
| Authorization code + PKCE | Mobile / public | App native |
| Client credentials | Worker | Daemon → Graph |
| On-behalf-of | Tầng giữa | Gọi downstream thay user |
| Resource Owner Password / Implicit | — | Không dùng |

**Token:** access JWT 15–60 phút, `aud` = API, claim = sub/tenant/role — không dump cả profile. Refresh dài hơn, **rotate**, lưu server (Redis); dùng lại refresh cũ thì thu hồi cả family (trộm). ID token cho client biết ai login; API không authZ bằng ID token. JWT không thu hồi được: TTL ngắn hoặc block-list `jti`.

**Cookie vs bearer:** cookie httpOnly Secure SameSite trên BFF (tốt hơn XSS; cần CSRF). Bearer trong JS bị cắp được. Mobile: PKCE, token trong OS secure store.

**AuthZ:** RBAC (role) rồi check resource. Redis **cache** quyền; database/CMS là nguồn sự thật. Invalidate khi đổi role.

**Secret và mạng:** Key Vault + managed identity; không commit connection string hay signing key. API private (không public IP). WAF ở mép BFF. Dashboard Hangfire chỉ admin.

## Reliability và Azure

**SLI** = cái đo (p95 submit timesheet, % 5xx). **SLO** = mục tiêu nội bộ. **SLA** = hợp đồng. **Error budget** = 1 − SLO; tiêu cho thay đổi, đóng freeze release nếu cháy quá nhanh. Alert burn rate, không “CPU > 80%”.

Golden journey (chọn khoảng năm): login, list plan, check-in, submit timesheet, admin search.

**Polly:** timeout mọi egress; retry jitter **chỉ khi idempotent**; circuit breaker; bulkhead để SharePoint không hút hết thread API. EF `EnableRetryOnFailure` cho SQL tạm thời, không cho vi phạm unique key.

**Health:** `/live` = process sống. `/ready` = SQL + Redis. **Không** fail ready vì dependency không critical chết nếu còn degrade (ẩn ảnh, queue notify).

**Tracing:** W3C `traceparent` xuyên BFF → API → job Hangfire. Log structured với `code` + `traceId`, không phone/email. Histogram, không average.

**DR:** hệ thống ghi = SQL. Mất Redis session có thể = mọi người login lại — nói rõ. Restore drill mỗi quý.

**Compute:** App Service cho API + worker mặc định. Container Apps nếu muốn container không Kubernetes. AKS khi có platform team. Functions cho glue blob/event, không phải OLTP API.

**Mép:** Internet → Front Door / App Gateway (WAF) → BFF public → API private → SQL/Redis/Service Bus private.

**Autoscale** theo độ dài HTTP queue **và** độ sâu Hangfire/bus, không chỉ CPU (app I/O-bound nói dối).

**.NET 6 hết hỗ trợ.** Lên 8 LTS: đổi TFM, audit package, chạy integration test — không gộp với rewrite.

## Tiến hóa (hai đến ba quý)

Nợ kỹ thuật là khoản vay có lãi. Trả N+1 liều lĩnh trên path nóng ngay. Schedule upgrade framework. Không vay “không test, ship thứ Sáu”.

1. Test golden journey, architecture test, dashboard p95.  
2. .NET 8.  
3. Hangfire process worker.  
4. Outbox cho fire-and-forget sau `SaveChanges`.  
5. Tường module (JourneyPlan không reference nội bộ Timesheet).  
6. SharePoint sau named `HttpClient`, timeout, breaker; khỏi path đồng bộ user.  
7. BFF nếu token còn trong JavaScript.  
8. Replica đọc nếu report hại OLTP.

Không: Kafka cho ba event/ngày; shared database với product khác; giả folder Clean Architecture khi Application vẫn reference DataAccess — chỉ chặn chảy máu ở **use case mới**.

**Nói không** kèm ADR và phương án thay là việc của architect: không ORM thứ hai, không SLO 99.99% thiếu ngân sách, không IP API public “tạm”.

**Câu chốt khi kết design:** “Modular monolith trên App Service + SQL + Redis, BFF cho web, JWT nội bộ, Hangfire cho job, outbox khi có consumer thứ hai, tách Notification nếu fan-out hoặc SLA lệch. ADR cho identity, data, messaging.”

## Độ sâu TA — panel vẫn hỏi

Người senior thuộc SOLID và “nên dùng Redis” vẫn trượt vòng TA. Panel muốn **số, failure mode, và cách vận hành lựa chọn**. Phần này là lớp đó.

### Ước lượng trên bảng

Giả định 10.000 staff, 30% active giờ cao điểm sáng, 2 request / user active / phút.

```text
Active ≈ 3.000
Peak ≈ 3.000 × 2 / 60 ≈ 100 req/s
p95 400 ms ⇒ in-flight ≈ 100 × 0.4 ≈ 40
SQL: 100 req/s × 3 query ≈ 300 query/s — một Azure SQL General Purpose đủ
Ảnh: 3.000 × 8 × 2 MB / ngày ≈ 48 GB/ngày → blob, không SQL
Connection: số instance × (DbContext pool 128) phải dưới max SQL (~150–300 SKU nhỏ)
```

Họ nhân 10 user thì scale **out API** trước, rồi Redis, rồi SQL (DTU/vCore, replica đọc). Không nhảy Kafka vì 100 req/s “trông nhỏ so với Netflix”.

### Đổi không downtime

**App:** slot swap / blue-green; ít nhất hai instance; drain Hangfire hoặc requeue lúc SIGTERM (`IHostApplicationLifetime`, 30 giây).

**Schema — expand/contract:** thêm cột nullable → deploy code ghi cả hai → backfill → deploy code đọc cột mới → drop cột cũ. Không expand và drop một migration khóa bảng nóng. Online index trên bảng lớn. Dual-write có cửa sổ đo và kill switch.

**Feature flag:** đổi hành vi sau flag để rollback là bật/tắt, không redeploy. Flag có hạn; flag sống một năm là hệ config thứ hai.

### Phân trang, khóa, xóa mềm

**Offset** (`SKIP/TAKE`) đơn giản và **sai** với trang sâu (engine vẫn đi qua hàng bỏ). **Keyset** (`WHERE (Date, Id) < (@d, @id) ORDER BY Date DESC, Id DESC`) ổn khi có insert. Luôn max `pageSize` (ví dụ 50). `ToList` không giới hạn là incident.

**PK:** `bigint` identity clustered cho OLTP. GUID ngẫu nhiên phân mảnh clustered. Sequential GUID chỉ khi client phải tự tạo id.

**Xóa mềm:** `IsDeleted` + index **filtered**. Mọi query phải nhớ filter (global query filter EF). Hard-delete PII trên job retention khi luật yêu cầu — soft delete không phải xóa.

### Isolation, deadlock, replica trễ

Mặc định `READ COMMITTED`. `READ UNCOMMITTED` / `NOLOCK` cho report là nói dối sẽ debug cả tuần. `SNAPSHOT` / RCSI giảm block đọc-ghi; biết SQL mình bật hay không.

Deadlock: hai transaction khóa A rồi B vs B rồi A. Fix: thứ tự khóa nhất quán, transaction ngắn, retry deadlock (EF execution strategy). Không retry unique-key như deadlock.

Replica đọc **stale**. Dashboard KPI: chấp nhận trễ. Sau ghi, **read-your-writes** từ primary (hoặc dính primary vài giây). Không đưa user sang replica cho đúng dòng vừa lưu.

### Domain event vs integration event

**Domain event:** trong một bounded context, sau commit, cùng process hoặc outbox nội bộ (ví dụ “TimesheetSubmitted” cập nhật projection local).

**Integration event:** hợp đồng với context khác, payload có version, `eventType` ổn định, không entity EF trên dây. Chỉ thêm field. Outbox lưu integration event, không lưu object domain.

### Rate limit, versioning, CORS

Rate limit ở mép (Front Door / gateway) **và** theo user/app trên API (`429` + `Retry-After`). Login và OTP chặt hơn GET search.

**Versioning:** thêm field JSON thì miễn phí. Breaking = route hoặc header mới (`v2`) kèm ngày sunset. Enum integer giữ integer; không “sửa” thành string.

CORS không phải bảo mật. API private; BFF là origin trình duyệt. Allowlist origin cụ thể, không `*`, khi có credential.

### Test là kiến trúc

- Unit: invariant domain, không I/O.
- Integration: golden journey trên SQL thật (container), assert `errors[].code`.
- Contract: BFF ↔ API OpenAPI.
- Load: một journey tới SLO, rồi cái tiếp; soak rò memory; **không** load test lần đầu trên production.
- Architecture test: cấm reference, bắt buộc phân trang.

Nếu test duy nhất là “architect review PR”, kiến trúc sẽ mục.

### Incident

Severity, commander, kênh comms, timeline, “cầm máu” trước kịch root cause. Rollback là lựa chọn hạng một. Postmortem không đổ lỗi: SLO nào cháy, fitness function nào thêm để không lặp. TA vào on-call đủ để sơ đồ gặp thực tế.

### ADR mẫu (nói hình này)

```text
Title: Transactional outbox cho side effect timesheet
Status: Accepted
Context: Hangfire sau SaveChanges mất nếu process chết; user vẫn nhận 201.
Decision: Dòng Outbox cùng transaction SQL với timesheet. Dispatcher publish. Consumer idempotent theo EventId.
Consequences: Thêm bảng, at-least-once, cần dashboard DLQ. Không đưa Kafka cho việc này.
```

### Múi giờ, đồng hồ, multi-tenant

Field-force nhiều thị trường: lưu **UTC**, hiện zone của user, “hôm nay” nghiệp vụ là lịch **outlet**, không phải server. Inject `TimeProvider` — không `DateTime.Now` trong domain.

Multi-tenant: bắt đầu **DB chung + TenantId** từ token (không chỉ query string). Filter hàng trên EF. Tách DB / thị trường chỉ vì residency hoặc noisy neighbour. Tenant trong mọi cache key.

### Sức chứa data plane

`DbContext` pool (ví dụ 128) × số instance < max session SQL. Redis multiplex (một multiplexer singleton). Blob cho ảnh; SQL giữ metadata và hash. Lifecycle xuống cool storage.

### Việc không làm năm đầu

Rewrite cả nền tảng. Event-source timesheet. Mười hai microservice một team. SLO 99.99% trên stack có third party chậm. ORM thứ hai “cho linh hoạt”.

---

---

# Phần 4 — Phỏng vấn

Panel nghe: bạn hỏi ràng buộc trước khi vẽ; chọn giải pháp nhàm đến khi số liệu buộc khác; tự nói failure mode của design mình; gắn tactic với SLO; xuống được một tầng (EF, token, index) mà không sống ở đó; đổi design khi họ đổi ràng buộc.

Trượt vì: salad buzzword, “Kubernetes và Kafka”, không ước lượng bậc độ lớn, đổ lỗi team không biết SOLID.

Ba phút đầu: “I’ll ask a few constraints, then draw C4 context and containers, then zoom into the riskiest flow — usually write path plus identity. I’ll mark what is synchronous, what is queued, and how it fails. Interrupt if the constraints are wrong.”

Hỏi họ: client (mobile offline?); write đỉnh (bậc độ lớn); region / data residency; stack sẵn; size team và ai vận hành production; incident khó nhất năm ngoái. Không cho số thì ghi giả định lên bảng: `10k staff, 50 writes/s peak, photos 2 MB × 8`.

## HR English (nói các đoạn này)

**Tell me about yourself**

> My name is Thai Doan Son. I am a software developer. I have worked in software development for the past 6 years. My main job responsibilities include developing and testing. I love performance tuning. It’s challenging and you get a feeling of accomplishment when done.
>
> I’m from DakLak province. I studied at the University of Science Ho Chi Minh City. I graduated with a degree in information technology in 2015. I like programming because it allows me to think more logically. In the future I want to be a technical architect. I would like to live and work in Ho Chi Minh City.

**Strengths:** I learn fast. I read documentation and source code until I know how it works.

**Weaknesses:** I stall when the work is dull. I time-box it and ask for a clear outcome so I still ship.

**Why leave:** I am ready for more responsibility / learning / there is no leadership slot. Không chửi công ty cũ.

**Education:** University of Science, Ho Chi Minh City, Information Technology, 2015. Algorithms and databases still shape how I think; since then I have learned on the job — .NET, SQL, performance.

**Five years:** stay on the technical path; become a technical architect; keep improving the skill set.

**Salary:** `$1800 NET` — cập nhật số của bạn. Architect tính theo **rủi ro giảm được**.

**Questions for them:** Which tools should I learn before day one? What is the culture of the team (ownership, speed, camaraderie)?

**Why this job**

> I’m interested because the stack matches what I already ship — ASP.NET Core, SQL Server, cache and messaging — and I can go deeper on performance and architecture. I want to own features end to end toward a technical architect path.

**Why hire you:** .NET experience, performance tuning, I learn from docs and source.

**Stress:** a signal that work is piling up. I break it down, time-box the risky parts, ask early if blocked, and keep a list so I do not hold everything in my head.

**Difficult situation:** first job, no documentation, several technologies stacked, a custom XML layer that broke MVC (business rules in XML next to the view), pages bound tightly to database fields. Hard because of technical debt, not algorithms. I mapped the system, isolated the custom framework, delivered incrementally.

**Giọng architect**

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

## Câu hỏi công ty

Mỗi câu: **Ngắn** (15–30 giây) rồi **Sâu** (khi họ hỏi tiếp). Ở mức Ngắn trừ khi họ hỏi “why / how / nếu”.

### Vòng A — backend sản phẩm

**Kể dự án và tech stack.**

- **Ngắn:** Hai–ba sản phẩm: vai trò, domain, stack — ASP.NET Core, EF Core, SQL Server, MediatR, Redis, Hangfire, Azure AD / BFF, RabbitMQ nếu có. Kết bằng một khó (query chậm, auth, hoặc queue).
- **Sâu:** Mỗi sản phẩm: user là ai, load ghi/đọc, mình sở hữu phần nào. Stack theo tầng — HTTP (controller mỏng, payload → MediatR), application (handler, FluentValidation), SQL qua EF, job Hangfire, session/token Redis, identity Azure AD qua BFF. Đóng bằng một incident: ví dụ list 5k dòng 7 giây; `AsNoTracking` + projection xuống dưới 1 giây; bài học là đo trước khi đoán.

**Có dùng Redis không?**

- **Ngắn:** Có. Session và token nằm đó; cache user cho filter quyền. Memory cache là một process; Redis cho nhiều instance.
- **Sâu:** `IMemoryCache` mất khi restart, không share sau load balancer — server A hit, server B miss. Redis qua `IDistributedCache` (`byte[]`, JSON) là cache-aside: miss → SQL → `Set` với absolute + sliding. Key gồm tenant. Stampede: lock ngắn để 1.000 miss không cùng đập SQL. Redis cache quyền, không phải nguồn sự thật — DB/CMS mới là; invalidate khi đổi role. Session trên Redis thì mất Redis = mọi người login lại; đó là đánh đổi DR nói rõ.

**Kafka hoặc RabbitMQ?**

- **Ngắn:** RabbitMQ là queue và exchange (direct / fanout / topic), durable + persistent, prefetch, ack. Kafka là log phân vùng, offset, replay — không phải broker AMQP.
- **Sâu:** Rabbit: producer → exchange → binding → queue → consumer; at-least-once với ack tường minh; prefetch cho competing consumer; durable queue **và** persistent message mới sống sau restart broker. Kafka: log append, consumer group, replay theo offset, throughput cao. Chọn Rabbit hoặc Service Bus cho command và work queue; Kafka khi cần replay, nhiều consumer độc lập, hoặc nền tảng stream. Hangfire cho cron trong process, không để team khác subscribe. Chi phí vận hành là thuộc tính chất lượng — không dựng Kafka cho ba event/ngày.

**Volo ABP?**

- **Ngắn:** Chưa dùng. Framework module DDD trên ASP.NET Core (Identity, multi-tenant, permission sẵn).
- **Sâu:** ABP cho chassis: module, permission, multi-tenant, AutoMapper, folder ước lệ. Không dùng thì không giả vờ. Cái lấy được: module có biên, Identity là sản phẩm, permission là data, không tự viết framework. Nhét ABP vào monolith đang sống là rủi ro rewrite; tách một module trước, không bọc cả solution.

**DistributedCache?**

- **Ngắn:** Cache dùng chung (Redis/SQL), `byte[]`, expire absolute hoặc sliding. Khác memory cache trong process. Cache-aside: miss → DB → Set.
- **Sâu:** `IDistributedCache` là abstraction ASP.NET; implement memory (dev), Redis, SQL Server. Mọi instance thấy cùng key. Value là byte nên phải serialize. `Refresh` gia hạn sliding. Không nhét graph khổng lồ hay secret plaintext. Invalidation khó: TTL + generation key theo tenant hơn là nhớ mọi key đã ghi.

**Optimize bằng SQL hay EF?**

- **Ngắn:** Cả hai. Mặc định EF LINQ (`IQueryable`, filter trên SQL). Stored procedure cho report nặng. Không query trong vòng lặp; không materialize rồi Contains tập lớn.
- **Sâu:** Giữ `IQueryable` đến `ToListAsync` để `Where`/`Select` thành SQL. `ToList` rồi `.Contains(ids)` hàng chục nghìn key = IN khổng lồ — join. Vòng lặp `await` từng item là N+1. Khi planner phải đóng đinh set-write hoặc batch DBA giữ, dùng stored procedure hoặc TVP. View cho hợp đồng đọc ổn định. Nhìn SQL thật (log/profiler) trước khi “tối ưu”.

**Optimize EF Core thế nào?**

- **Ngắn:** Giữ `IQueryable` đến `ToListAsync`; `AsNoTracking` lúc đọc; `Select` cột cần; `AsSplitQuery` nhiều collection; bulk insert; index khớp `WHERE`/`JOIN`.
- **Sâu:** Đọc không dùng change tracker (`AsNoTracking`). Project DTO thay vì `Include` cả graph. Hai collection Include một JOIN nhân hàng (cartesian) — `AsSplitQuery` hoặc hai câu. Compiled query chỉ khi thời gian compile lộ trên trace. `SaveChanges` một lần / use case, không từng dòng insert 5k. Tắt lazy-loading trên web API. Concurrency: `rowversion` trên aggregate người sửa. Đo bằng `Stopwatch` và query log; `let` trong query syntax có thể tốn allocation và chậm hơn.

**Hangfire dùng việc gì?**

- **Ngắn:** Nền và cron: email, report, KPI, nhãn. HTTP request không chờ.
- **Sâu:** Fire-and-forget, delay, recurring (cron), lưu SQL (hoặc Redis). Dashboard succeeded/failed, khóa admin. Ví dụ: day-off, absence, SP KPI, nhãn SKU. Job đói thread HTTP thì chạy Hangfire **process thứ hai** — API chỉ enqueue. Hangfire sau `SaveChanges` có thể mất job nếu process chết; event phải sống thì ghi outbox cùng transaction, dispatcher Hangfire mới publish. Hangfire không phải contract cho consumer team khác — cái đó là bus.

**Async vs sync?**

- **Ngắn:** Async cho I/O; sync cho CPU; action trả `Task`; không `.Result`.
- **Sâu:** `async`/`await` trả thread pool khi chờ SQL/HTTP. CPU-bound không nhanh hơn vì async giả. `.Result` / `.Wait()` deadlock hoặc đói pool. `Task.WhenAll` cho I/O độc lập. Không `Task.Run` trong request để “cho async” — vẫn đốt thread; dùng Hangfire hoặc queue. EF: `ToListAsync`, `SaveChangesAsync`. Named `HttpClient` với timeout tường minh.

### Vòng B — vận hành và giao hàng

**CI/CD — có đụng server không?**

- **Ngắn:** Nói thật. Pipeline build → test → deploy; secret trong variable group. Không có quyền server thì nói không.
- **Sâu:** Pipeline khỏe: restore, build, test đơn vị/architecture, integration trên SQL thật (container), deploy staging, smoke vài journey, approve production. Secret không trong git — variable group hoặc Key Vault. Viết được YAML, đọc log job fail dù chưa SSH máy. Role cần người vận hành IIS/K8s hàng ngày thì không thổi phồng.

### Vòng C — kiến trúc

**1. Vẽ kiến trúc microservices.**

- **Ngắn:** Client → BFF/gateway → auth, catalog, order, notification; mỗi cái DB riêng; Redis; worker; bus. Hôm nay vẫn có thể **deploy** một API modular — nói vậy.
- **Sâu:** Service deploy độc lập, database per service, API gateway hoặc BFF ở mép, messaging cho side effect, Redis session/cache. Dark energy (tự chủ team, scale) kéo tách; dark matter (ACID, ít hop mạng) kéo lại. Distributed monolith (12 repo, một team, một DB) tệ hơn modular monolith. Vẽ module trong một process nếu đó là production, và gọi tên extract đầu (Notification) nếu fan-out hoặc SLA lệch.

**2. Ngoài REST còn kiểu nào?**

- **Ngắn:** gRPC nội bộ, message (Rabbit/Service Bus), Hangfire, SignalR; SOAP nếu legacy.
- **Sâu:** REST cho public/BFF (GET cache được, tooling rộng). gRPC service-to-service (protobuf, latency thấp; browser cần grpc-web). Message khi caller không được chờ. Hangfire cho lịch trong biên mình. SignalR cho push. GraphQL chỉ khi BFF chết vì GET vụn — giá là N+1 resolver và authZ từng field. Không thêm style vì mốt.

**3. Xử lý lỗi API?**

- **Ngắn:** Exception middleware → log + 500 an toàn. Validation 400 với `errors[].code`. 404/401/403 exception domain. Không stack ra client.
- **Sâu:** FluentValidation trước handler (và trước transaction). `NotFoundException` → 404. Chưa auth 401, cấm 403. Envelope `{ succeeded, result, errors: [{ code }] }` — client khóa `code`, không khóa message. Middleware log `traceId`; body production không stack. `catch` rỗng cấm. Lỗi validate không phải 500.

**4. Authentication loại gì?**

- **Ngắn:** Cookie trên BFF; JWT trên API; OpenID Connect / Azure AD.
- **Sâu:** Browser không thấy token dài. BFF là confidential client (auth code + secret), lưu access + refresh Redis, set cookie httpOnly Secure SameSite. API validate JWT `iss`/`aud`/`exp`/chữ ký. Mobile: auth code + PKCE, token trong OS store. Worker: client credentials. Downstream thay user: on-behalf-of. Không ROPC, không implicit, không JWT trong `localStorage`.

**5. Token gần hết hạn thì sao?**

- **Ngắn:** Client đọc `exp` hoặc bắt 401 → refresh → access mới → retry. BFF có thể refresh thầm bằng Redis.
- **Sâu:** Access JWT 15–60 phút. Trước hết hạn, hoặc lúc 401, BFF dùng refresh rotate, lấy access mới, retry call gốc. Resource server từ chối JWT hết hạn — chúng không refresh. Dùng lại refresh cũ thì thu hồi cả family (trộm). TTL session (~2 giờ) độc lập; hết thì user login lại.

**6. Refresh token để làm gì?**

- **Ngắn:** Access mới không login lại; TTL dài hơn; rotate; thu hồi lúc logout.
- **Sâu:** Access là bearer, không thu hồi được, nên ngắn. Refresh sống trên server (Redis/httpOnly), rotate mỗi lần dùng, thu hồi lúc logout hoặc đổi password. Không đưa ra SPA. API không nhận refresh như access.

**7–8. N-layer vs Clean Architecture? Vẽ Clean.**

- **Ngắn:** N-layer là Presentation → Business → Data, dependency xuống. Clean là vòng tròn, dependency vào trong: API → Application → Core, Infrastructure/EF chỉ lên Core.
- **Sâu:**

```text
        UI / API
            │
            ▼
       Application  ──► ports (interface)
            │
            ▼
          Core
            ▲
            │
   Infrastructure / EF
```

N-layer ship nhanh; UI thường biết EF. Clean để domain không dính EF, test và đổi hạ tầng dễ hơn. Application vẫn reference DataAccess thì không giả folder. Việc mới: MediatR; architecture test để API không nhận thêm EF. Vertical slice (một folder / use case) ghép được cả hai.

**9. DI, DIP, IoC khác gì?**

- **Ngắn:** DIP là nguyên lý (phụ thuộc abstraction). IoC là container đảo `new`. DI là constructor injection.
- **Sâu:** DIP: policy cấp cao phụ thuộc `IDataAccess`, không `SqlDataAccess`. IoC: ai đó khác dựng graph. DI: constructor (mặc định), setter, hoặc method. Captive dependency — Singleton giữ Scoped `DbContext` — là bug mức architect. Lifetime: Singleton config/Redis; Scoped `DbContext`; Transient validator. `IHttpClientFactory` cho HTTP ra, không `new HttpClient()` trong vòng lặp.

**10. Middleware là gì?**

- **Ngắn:** Pipeline `RequestDelegate`: log, auth, exception, session. Filter chạy sau khi chọn action.
- **Sâu:** Mỗi middleware thấy mọi request (static, health, MVC). Thứ tự: exception → HTTPS/static → routing → CORS → authentication → authorization → session → endpoint. Filter sau đó với `ActionContext`/`ModelState`: authorization, resource, action, exception, result. Auth cookie/JWT thuộc middleware. Check feature/role cần action thuộc filter. Sai thứ tự middleware → `User` rỗng hoặc 401.

**11. Singleton / Scoped / Transient?**

- **Ngắn:** Singleton = cả app (config, Redis). Scoped = mỗi request (`DbContext`). Transient = mỗi lần resolve.
- **Sâu:** Job Hangfire không phải HTTP request — tạo scope bằng `IServiceScopeFactory` kẻo dùng `DbContext` đã dispose. Singleton `HttpClient` không qua `IHttpClientFactory` cạn socket. Transient cho service stateless nhẹ. Không Singleton `DbContext` (không thread-safe, giữ connection).

**12. AsNoTracking, AsSplitQuery?**

- **Ngắn:** NoTracking = đọc, không change tracker. SplitQuery = tách SQL khi include nhiều collection, tránh cartesian.
- **Sâu:** Tracking snapshot mọi property cho `SaveChanges` — phí trên list. Entity `AsNoTracking` không nên update trừ khi `Attach`. Một JOIN cha + hai collection nhân hàng (cartesian). `AsSplitQuery` một câu / collection; data có thể lệch nếu row đổi giữa các câu. Ưu tiên DTO `Select` khi không cần entity.

**13. Lazy vs eager loading?**

- **Ngắn:** Eager = `Include` sẵn. Lazy = load lúc đụng (tắt trên API). Explicit = `Load()`.
- **Sâu:** Eager khi chắc cần con và graph nhỏ. Lazy tiện và gây N+1 khi serializer đi navigation. Web API: tắt proxy. Explicit khi chỉ một số parent cần con. `ThenInclude` graph sâu; vẫn project khi màn hình cần năm cột.

**14. Lỗi 1-N do lazy hay eager?**

- **Ngắn:** N+1 từ lazy hoặc quên `Include`. Cartesian từ eager hai collection.
- **Sâu:** N parent, mỗi lần đụng `parent.Children` = N SQL thêm — lazy/quên Include. Một SQL `JOIN` hai collection = parent × children × collection kia — cartesian eager. Fix N+1 bằng Include/projection/split. Fix cartesian bằng split query hoặc hai câu hoặc DTO. Bật lazy “cho khỏi Include” là cách list endpoint chết production.

**15. User A đang xem; user B đã lưu.**

- **Ngắn:** Optimistic `rowversion` → 409 và reload. Check-in trùng: unique index mạnh hơn cột version.
- **Sâu:** Không lock lúc đọc. Cột `rowversion`; `UPDATE … WHERE Id=@id AND Version=@old`; 0 row → `DbUpdateConcurrencyException` → 409 bảo reload (hoặc merge). Last-write-wins chỉ khi chấp nhận mất field. Kho/chỗ ngồi: transaction `UPDLOCK` ngắn hoặc Redis reservation + reconcile async. Mobile offline: merge từng field hoặc hàng đợi conflict — last-write-wins thù user. Check-in trùng: unique `(StaffId, Day, OutletId)` cộng idempotency key từ client.

## Whiteboard (45 phút)

**L1:** app staff, admin web, Azure AD, SAP/master data, FCM, SharePoint (legacy), hệ thống mình.

**L2:** BFF, API và/hoặc worker, SQL, Redis, blob, Service Bus tùy chọn, store Hangfire.

**Sequence check-in**

1. App → BFF cookie hoặc token.  
2. BFF → API JWT.  
3. API authorize assignment outlet (IDOR).  
4. Transaction: dòng timesheet + outbox.  
5. 201 cho user.  
6. Worker: virus scan, FCM, projection search.

Nói rõ: unique index chống check-in trùng; SharePoint không trên path này; budget p95; breaker FCM — FCM fail không được fail check-in.

**“Microservice?”** Modular monolith hôm nay; tách Notification nếu fan-out hoặc SLA lệch.

**“Scale 100×?”** Connection pool và SQL trước; replica đọc cho report; queue ảnh; tách worker; rồi read model. Không hai mươi repo ngày một.

### Câu hỏi tiếp trên whiteboard

**N-layer vs Clean?**

- **Ngắn:** N-layer ship nhanh, dependency xuống. Clean đảo để domain không reference EF. Siết module và test trước khi theo đạo folder.
- **Sâu:** N-layer: UI → BLL → DAL; UI thường biết EF. Clean: Core giữa, Infrastructure implement port. Cả hai có thể là modular monolith. Không đổi tên folder khi Application vẫn reference DataAccess. Use case mới: MediatR, Core không kiểu EF, architecture test trên project API.

**Sao không Kafka?**

- **Ngắn:** Chưa cần replay hay stream nhiều consumer. Service Bus hoặc Rabbit cho command; Hangfire cho lịch. Kafka khi có platform và chuyện replay.
- **Sâu:** Kafka mạnh ở log throughput cao, consumer group độc lập, replay. Work queue ack/prefetch/routing là Rabbit hoặc Service Bus. Dựng cluster và canh lag consumer cho ba event/ngày là thất bại thuộc tính vận hành. Analytics sau này cần một năm event thì thêm Kafka (hoặc Event Hubs) làm sink từ outbox, không làm command bus.

**Token hết hạn khi user đang làm?**

- **Ngắn:** Access ngắn; BFF refresh với refresh rotate trên Redis; interceptor 401; TTL session tường minh. API từ chối JWT hết hạn.
- **Sâu:** Access 15–60 phút. BFF thấy `exp` hoặc 401, dùng refresh phía server, retry. Resource API không refresh. Refresh rotate: dùng token bị cắp thì hủy cả family. Cookie sliding expiration khác `IdleTimeout` session — cấu hình cả hai có chủ đích.

**User A đang xem; user B đã lưu.**

- **Ngắn:** Optimistic `rowversion`; 409 + reload.
- **Sâu:** Như Vòng C câu 15: version trong `WHERE`; 0 row → concurrency exception. Unique constraint cho “không được tồn tại hai lần”. Lock pesimist chỉ chỗ ngồi khan, transaction thật ngắn.

**Lazy vs eager 1-N?**

- **Ngắn:** N+1 là lazy. Cartesian là hai Include. API: tắt lazy.
- **Sâu:** Như Vòng C câu 14. Serializer đi navigation là N+1 thầm. Đưa log 1+N SQL làm bằng chứng.

**Biết production khỏe thế nào?**

- **Ngắn:** SLO năm journey, alert burn-rate, trace kể cả Hangfire, live vs ready, chính sách error budget cho release.
- **Sâu:** SLI = p95 check-in, % 5xx. SLO = 99.5% dưới 2 giây / 30 ngày. Alert khi error budget cháy quá nhanh, không khi CPU 80%. `/live` vs `/ready` — ready không fail vì SharePoint chết nếu còn degrade. W3C `traceparent` trên job Hangfire. Restore drill SQL mỗi quý.

**Rủi ro lớn codebase kiểu này?**

- **Ngắn:** Application couple DataAccess; Hangfire in-process; latency third-party trên request thread; .NET 6 hết hỗ trợ; sót IDOR; memory cache sau scale-out.
- **Sâu:** Chọn cái đang đốt SLO: SharePoint trên path đồng bộ, hoặc N+1 list. Rồi runtime hết hạn (vá bảo mật). Rồi IDOR nếu quên `[Feature]` — một authorization filter dùng chung hơn check từng action. Memory cache là incident tiềm ẩn lần đầu thêm instance thứ hai.

**Thiết kế flash 5.000 req/s.**

- **Ngắn:** Đừng đập SQL 5k nếu nó làm 2k. Queue, consumer idempotent, stock giữ chỗ, DLQ.
- **Sâu:** Trả 200, ghi idempotency key, enqueue. Rút đúng sức SQL. Stock: Redis reservation hoặc `rowversion` trên counter; reconcile async. Side effect (mail, điểm) là worker, không phải request. Poison → DLQ + alert. Sau giờ cao điểm queue rút. Stock 1.000 thì request dư fail nhanh trên reservation, không sau timeout SQL 30 giây.

**DIP vs IoC vs DI?**

- **Ngắn:** Nguyên lý / container / constructor injection.
- **Sâu:** Như Vòng C câu 9. Nêu captive dependency dù chưa hỏi — đó là câu họ định hỏi tiếp.

## Vòng D — panel TA

Câu panel technical architect hỏi sau whiteboard. **Ngắn** rồi **Sâu**.

**Ước lượng load hệ thống field-force này.**

- **Ngắn:** 10k staff, 30% giờ cao điểm, 2 call/phút → ~100 req/s, ~40 in-flight ở 400 ms. Một SKU SQL đủ; ảnh vào blob (chục GB/ngày), không SQL.
- **Sâu:** Viết công thức lên bảng, đánh dấu mọi giả định. Ngân sách connection: instance × pool so với max SQL. 10× user → scale API và Redis, rồi replica SQL cho report. Không đề Kafka hay Kubernetes ở 100 req/s trừ khi họ đưa ràng buộc khác (fan-out, topology team, residency).

**Migrate bảng nóng không downtime?**

- **Ngắn:** Expand/contract: thêm cột, dual-write, backfill, chuyển đọc, drop cũ. Online index. Feature flag cho đổi hành vi.
- **Sâu:** Một migration rebuild clustered index 50 triệu dòng trong cửa sổ change là cách có incident. Dual-write có kill switch. Backfill là job Hangfire batch + dòng tiến độ, không script SQL chặn. Rollback là “đọc lại cột cũ”, không restore cả DB.

**Offset pagination vs keyset?**

- **Ngắn:** Offset đơn giản, chậm trang sâu. Keyset `(sort, id)` cho infinite scroll và mobile. Luôn trần page size.
- **Sâu:** `OFFSET 100000` vẫn đọc 100000 hàng. Insert đồng thời làm trang 2 trùng hoặc nhảy. Keyset: `WHERE (UpdatedAt, Id) < (@t, @id)`. Đếm total đắt — trả `hasMore` trừ khi product thật sự cần `total`.

**Replica đọc sau khi ghi?**

- **Ngắn:** Không. Replica stale. User vừa lưu đọc primary. Dashboard được phép trễ.
- **Sâu:** Lag replica async vài giây lúc tải. Session “read-your-writes”: dính primary N giây sau POST, hoặc header `Consistency: strong` trên GET đó. Màn KPI ghi “cập nhật mỗi 5 phút”.

**Domain event vs integration event?**

- **Ngắn:** Domain = trong một context. Integration = hợp đồng có version sang context khác, không entity trên dây.
- **Sâu:** Sau commit, outbox giữ payload integration (`TimesheetSubmittedV1` với id và tổng), không `Timesheet` kèm navigation. Chỉ thêm field; không tái dụng field. Consumer idempotent. Hai context share một bảng thì không phải hai context.

**Test kiến trúc thế nào?**

- **Ngắn:** Unit invariant, integration golden journey trên SQL thật, contract test BFF, load theo SLO, architecture test cấm reference.
- **Sâu:** Không ký design mà test duy nhất là slide. Fitness function trên CI fail build nếu controller `new` `DbContext`. Load: tăng tới SLO, soak rò, dừng nếu cháy error budget staging. Production không phải load test lần đầu.

**Kể một incident.**

- **Ngắn:** Severity, commander, cầm máu (rollback/flag), rồi nguyên nhân, rồi fitness function để không lặp.
- **Sâu:** Check-in p95 8 giây. Nhìn trace, không CPU. Span SharePoint 6 giây — mở breaker, degrade ảnh, trả SLO. Postmortem: SharePoint khỏi path sync, timeout 3 giây, `HttpClient` riêng. Không restart App Service như “fix”.

**Rate limiting?**

- **Ngắn:** Mép cộng per-user trên API. Login chặt hơn GET. Trả 429 với `Retry-After`.
- **Sâu:** Bucket ẩn danh vs đã auth. GET idempotent có thể mạnh; POST check-in theo staff / phút. WAF gateway cho DDoS khối; API cho công bằng. Limit trong config, không magic number trong code.

**Version public API?**

- **Ngắn:** Thêm field, không rename. Breaking = `v2` kèm ngày sunset. Enum giữ integer nếu đó là hợp đồng.
- **Sâu:** Binary mobile trễ vài tuần. Thêm field; client cũ bỏ qua (tolerant reader). Không ship `JsonStringEnumConverter` như “sửa”. Header deprecation, telemetry traffic `v1`, rồi tắt.

**Multi-tenant?**

- **Ngắn:** Tenant từ token, cột mọi bảng, cache key gồm tenant. DB riêng chỉ vì residency hoặc noisy neighbour.
- **Sâu:** Tenant trên query string là IDOR. Global filter EF trên `TenantId`. Impersonate admin là claim tường minh, có audit. Só filter là incident lộ data — integration test hai tenant.

**GUID hay int PK?**

- **Ngắn:** `bigint` identity clustered cho OLTP. GUID ngẫu nhiên phân mảnh index.
- **Sâu:** Id ngoài có thể GUID ở cột unique nonclustered. Sequential GUID nếu client phải tạo id offline. Không cluster trên `NEWID()`.

**PO muốn microservice quý sau — làm sao?**

- **Ngắn:** Hỏi ràng buộc nào (team, scale, data, runtime) đang giải. Không có thì đề modular monolith và ADR.
- **Sâu:** Đổi thành kịch bản chất lượng và giá (ops, latency, 2PC). Cho strangler **một** capability với metric thành công. “Netflix làm vậy” không phải driver. Họ vẫn ép thì ghi rủi ro và rollback — không phá, không giả miễn phí.

**Lúc nào mình sai với tư cách architect?**

- **Ngắn:** Chọn case thật: cache không lock stampede, hoặc Hangfire sau `SaveChanges`. Đổi gì, fitness function nào thêm.
- **Sâu:** Panel chấm trung thực và vòng phản hồi. Cấu trúc: quyết định → bằng chứng production → đảo → cách không lặp. Không bịa chuyện mình bí mật đúng.

**App Service vs AKS?**

- **Ngắn:** App Service đến khi nhiều service, sidecar, và có platform team. AKS là sản phẩm mình phải vận hành.
- **Sâu:** App Service: TLS, scale-out, slot, đủ một API + worker. AKS: node pool, ingress, mesh, upgrade, on-call cluster. Container Apps ở giữa. Chọn **ít** platform nhất vẫn đạt SLO.

**Load-test check-in thế nào?**

- **Ngắn:** Script golden journey, tăng tới 2× peak kỳ vọng, assert p95 và tỉ lệ lỗi theo SLO, soak một giờ.
- **Sâu:** Volume data giống production (không 10 dòng). Idempotency key để retry không insert đôi. Nhìn DTU SQL, lock wait, CPU Redis, thread pool. Dừng nếu cháy error budget staging. Không “xem sao” trên prod thứ Sáu.

**OWASP API — rủi ro thật sự thiết kế?**

- **Ngắn:** IDOR (BOLA), auth gãy, payload không trần, injection, misconfig (blob public, Hangfire mở).
- **Sâu:** Mọi `GET /{id}` là test authorization, không chỉ authentication. BFF để token không trong JS. Max body, virus scan async ảnh. SQL parameterized / EF. Hangfire và Swagger khóa production. Rate limit login. Không stack.

**Soft delete vs hard delete?**

- **Ngắn:** Soft delete cho undo và audit, filtered index, global filter EF. Hard delete PII trên job retention khi bắt buộc.
- **Sâu:** Soft delete mà mọi query quên thì tệ hơn hard delete. Email unique + soft delete cần unique index filtered. “Xóa tài khoản” là quy trình: ẩn danh, thu hồi token, tombstone, ngoại lệ legal hold.

**UTC nằm đâu?**

- **Ngắn:** Lưu UTC. Hiện zone user. “Hôm nay nghiệp vụ” là ngày của outlet.
- **Sâu:** `DateTime.Now` trong handler là bug APAC. Inject `TimeProvider`. Job đêm dùng time zone đặt tên theo thị trường. `datetimeoffset` hoặc UTC `datetime2` + zone trên profile — chọn một và ADR.

---

## 30 / 60 / 90 ngày

**30:** C4 as-is, nháp SLO, review incident, sơ đồ dependency, rủi ro .NET 6, list N+1 và tích hợp chậm.

**60:** ADR identity, data, messaging, compute; test golden path; spike tách worker; vá Key Vault; breaker SharePoint.

**90:** .NET 8 đang chạy hoặc xong; outbox một domain event; test biên module; dashboard on-call; cost per journey.

## Đêm trước phỏng vấn

- [ ] C4 hệ thống field-force thuộc lòng
- [ ] Outbox versus 2PC trong một phút
- [ ] BFF cookie trong một phút
- [ ] PACELC và một ví dụ saga
- [ ] App Service versus AKS hai gạch
- [ ] Ba fitness function
- [ ] Một ADR tuần một (hình outbox)
- [ ] Một điều **không** làm (Kafka / microservice / rewrite)
- [ ] Ước lượng: 10k staff → ~100 req/s trên bảng
- [ ] Migration expand/contract bốn bước
- [ ] Keyset pagination vs offset
- [ ] Incident: cầm máu trước root cause
- [ ] Đọc HR English một lượt thành tiếng
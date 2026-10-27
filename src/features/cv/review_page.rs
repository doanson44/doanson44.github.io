use std::collections::HashSet;

use leptos::prelude::*;

use super::review::{review_cards, ReviewCard};
use crate::components::icons::{Icon, IconName};
use crate::i18n::*;
use crate::infrastructure::browser::{scroll_to_top, storage_get, storage_set};
use leptos_i18n::I18nContext;

const PROGRESS_STORAGE_KEY: &str = "cv.review.understood.v1";

#[component]
pub fn CvReviewPage(card_id: Option<String>) -> impl IntoView {
    let i18n = use_i18n();
    let cards = Memo::new(move |_| review_cards(i18n.get_locale()));
    let understood = RwSignal::new(load_understood());

    let save_progress = move |next: HashSet<String>| {
        if let Ok(value) = serde_json::to_string(&next) {
            let _ = storage_set(PROGRESS_STORAGE_KEY, &value);
        }
        understood.set(next);
    };

    let clear_progress = move |_| {
        let empty = HashSet::new();
        let _ = storage_set(PROGRESS_STORAGE_KEY, "[]");
        understood.set(empty);
    };

    let selected_card_id = card_id.clone();
    let selected_card = Memo::new(move |_| {
        let id = selected_card_id.as_deref()?;
        cards.get().iter().find(|card| card.id == id).copied()
    });

    let toggle_understood = move |id: String| {
        let mut next = understood.get();
        if next.contains(&id) {
            next.remove(&id);
        } else {
            next.insert(id);
        }
        save_progress(next);
    };

    view! {
        <main class="flex flex-1 flex-col">
            <div class="mx-auto w-full max-w-6xl px-4 py-8 sm:px-6 sm:py-12 lg:px-8">
                {move || {
                    if card_id.is_some() {
                        detail_view(
                            i18n,
                            selected_card.get(),
                            understood.get(),
                            toggle_understood,
                        )
                        .into_any()
                    } else {
                        list_view(
                            i18n,
                            cards.get(),
                            understood.get(),
                            clear_progress,
                        )
                        .into_any()
                    }
                }}
            </div>
        </main>
    }
}

fn list_view(
    i18n: I18nContext<crate::i18n::Locale>,
    cards: &'static [ReviewCard],
    understood: HashSet<String>,
    clear_progress: impl FnMut(leptos::ev::MouseEvent) + Copy + 'static,
) -> impl IntoView {
    let vi = i18n.get_locale() == crate::i18n::Locale::vi;
    let total = cards.len();
    let completed = cards
        .iter()
        .filter(|card| understood.contains(card.id))
        .count();

    view! {
        <header class="mb-8 flex flex-col gap-5 sm:flex-row sm:items-end sm:justify-between">
            <div>
                <a
                    href="#/cv"
                    class="inline-flex rounded-md text-sm font-medium text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                >
                    <Icon name=IconName::ArrowLeft class="mr-2 inline-block h-4 w-4" />{t_string!(i18n, cv_review_back)}
                </a>
                <p class="mt-5 text-xs font-semibold uppercase tracking-[0.2em] text-[var(--accent)]">
                    {t_string!(i18n, cv_review_label)}
                </p>
                <h1 class="mt-2 text-3xl font-bold tracking-tight text-[var(--text-primary)] sm:text-4xl">
                    {t_string!(i18n, cv_review_title)}
                </h1>
                <p class="mt-3 max-w-3xl text-sm leading-6 text-[var(--text-secondary)] sm:text-base">
                    {t_string!(i18n, cv_review_description)}
                </p>
            </div>

            <div class="flex shrink-0 flex-col items-start gap-3 sm:items-end">
                <span class="text-sm font-medium text-[var(--text-secondary)]">
                    {format!("{} / {}", completed, total)}
                </span>
                <button
                    type="button"
                    disabled=completed == 0
                    class="min-h-11 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] disabled:cursor-not-allowed disabled:opacity-50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    on:click=clear_progress
                >
                    {t_string!(i18n, cv_review_clear)}
                </button>
            </div>
        </header>


        <section class="mb-8 space-y-6">
            <div class="rounded-2xl border border-[var(--accent)]/30 bg-[var(--surface)] p-5 sm:p-7">
                <p class="text-xs font-bold uppercase tracking-[0.18em] text-[var(--accent)]">
                    {if vi { "LỘ TRÌNH PHỎNG VẤN — JTL SOFTWARE" } else { "INTERVIEW TRACK — JTL SOFTWARE" }}
                </p>
                <h2 class="mt-2 text-2xl font-bold tracking-tight text-[var(--text-primary)] sm:text-3xl">
                    {if vi { "Staff Software Engineer - AI Service Desk" } else { "Staff Software Engineer - AI Service Desk" }}
                </h2>
                <p class="mt-3 max-w-3xl text-sm leading-7 text-[var(--text-secondary)]">
                    {if vi {
                        "Mục tiêu không phải học thuộc flashcard. Ở cấp Staff, bạn phải giải thích cơ chế, nêu trade-off, đưa ra phương án vận hành và chứng minh tác động bằng trải nghiệm thực tế. Dùng nội dung bên dưới như đề cương luyện tập theo đúng 3 vòng."
                    } else {
                        "The goal is not to memorize flashcards. At Staff level, explain mechanisms, articulate trade-offs, design for operations, and prove impact with real experience. Use this guide as a rehearsal plan for the three interview rounds."
                    }}
                </p>
                <div class="mt-5 grid gap-3 md:grid-cols-3">
                    <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R1 - 30 min</p>
                        <h3 class="mt-2 font-semibold text-[var(--text-primary)]">{if vi { "Managing Director + CTO" } else { "Managing Director + CTO" }}</h3>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Tác động kinh doanh, quyết định kiến trúc, ownership, giao tiếp với stakeholder và lý do phù hợp với vai trò." } else { "Business impact, architecture decisions, ownership, stakeholder communication, and role motivation." }}</p>
                    </div>
                    <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R2 - 120 min</p>
                        <h3 class="mt-2 font-semibold text-[var(--text-primary)]">{if vi { "Principal Engineer + EM" } else { "Principal Engineer + EM" }}</h3>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "60 phút kỹ thuật: C#/.NET, SQL, hệ thống phân tán, thiết kế hệ thống. 60 phút sau: leadership, mentoring, xung đột, delivery và team fit." } else { "First 60 minutes: C#/.NET, SQL, distributed systems, system design. Final 60 minutes: leadership, mentoring, conflict, delivery, and team fit." }}</p>
                    </div>
                    <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R3 - 30 min</p>
                        <h3 class="mt-2 font-semibold text-[var(--text-primary)]">{if vi { "Head of AI - Germany" } else { "Head of AI - Germany" }}</h3>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Tích hợp AI vào workflow, đánh giá chất lượng, quyền dữ liệu, human-in-the-loop, chi phí, độ trễ và cơ chế fallback." } else { "AI workflow integration, evaluation, data permissions, human-in-the-loop, cost, latency, and fallback design." }}</p>
                    </div>
                </div>
                <p class="mt-4 text-xs leading-5 text-[var(--text-tertiary)]">
                    {if vi { "Phần R3 là dự đoán có căn cứ từ JD và chức danh người phỏng vấn, không phải câu hỏi đã được công ty xác nhận." } else { "The R3 focus is an informed inference from the JD and interviewer title, not a confirmed company question list." }}
                </p>
            </div>

            <div class="grid gap-4 lg:grid-cols-2">
                <section class="rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                    <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Nhóm kiến thức bắt buộc" } else { "Core technical domains" }}</h3>
                    <ul class="mt-4 space-y-3 text-sm leading-6 text-[var(--text-secondary)]">
                        <li><strong class="text-[var(--text-primary)]">{"C#/.NET:"}</strong> {if vi { "async/await, Task vs ValueTask, cancellation, thread-safety, GC/allocations, DI lifetime, middleware, hosted services, exception handling." } else { "async/await, Task vs ValueTask, cancellation, thread safety, GC/allocations, DI lifetimes, middleware, hosted services, exception handling." }}</li>
                        <li><strong class="text-[var(--text-primary)]">API + data:</strong> {if vi { "REST semantics, validation, pagination, authn/authz, rate limiting, API versioning, EF Core tracking, N+1, query plans, indexing, transactions and isolation." } else { "REST semantics, validation, pagination, authentication/authorization, rate limiting, versioning, EF Core tracking, N+1, query plans, indexing, transactions, isolation." }}</li>
                        <li><strong class="text-[var(--text-primary)]">Distributed systems:</strong> {if vi { "timeouts, retries with jitter, circuit breakers, bulkheads, idempotency, at-least-once delivery, ordering, outbox/inbox, saga, eventual consistency." } else { "timeouts, retries with jitter, circuit breakers, bulkheads, idempotency, at-least-once delivery, ordering, outbox/inbox, sagas, eventual consistency." }}</li>
                        <li><strong class="text-[var(--text-primary)]">Architecture + operations:</strong> {if vi { "modular monolith vs microservices, boundaries, capacity estimates, SLO/SLI, tracing, metrics/logs, incident response, rollout/rollback, DR and cost." } else { "modular monolith vs microservices, boundaries, capacity estimates, SLOs/SLIs, tracing, metrics/logs, incident response, rollout/rollback, disaster recovery, cost." }}</li>
                        <li><strong class="text-[var(--text-primary)]">Security + cloud:</strong> {if vi { "OAuth2/OIDC, service identity, least privilege, tenant isolation, secrets, audit trail, Azure App Service/Functions/Service Bus/Key Vault, containers and CI/CD." } else { "OAuth2/OIDC, service identity, least privilege, tenant isolation, secrets, audit trails, Azure App Service/Functions/Service Bus/Key Vault, containers, CI/CD." }}</li>
                    </ul>
                </section>
                <section class="rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                    <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "AI Service Desk: phần cần nói được ở mức kiến trúc" } else { "AI Service Desk: architecture-level fluency" }}</h3>
                    <ul class="mt-4 space-y-3 text-sm leading-6 text-[var(--text-secondary)]">
                        <li><strong class="text-[var(--text-primary)]">RAG:</strong> {if vi { "ingestion, chunking, embeddings, retrieval, reranking, citations, freshness và quyền truy cập theo tenant." } else { "ingestion, chunking, embeddings, retrieval, reranking, citations, freshness, and tenant-aware access control." }}</li>
                        <li><strong class="text-[var(--text-primary)]">Tool/function calling:</strong> {if vi { "AI đề xuất hành động; backend kiểm tra quyền, schema, policy và xác nhận trước hành động có tác động." } else { "AI proposes actions; backend validates identity, schema, policy, and confirmation before side effects." }}</li>
                        <li><strong class="text-[var(--text-primary)]">Reliability:</strong> {if vi { "timeout, token/cost budget, retry có giới hạn, fallback sang rule/manual queue, human approval và xử lý provider outage." } else { "timeouts, token/cost budgets, bounded retries, rule/manual-queue fallback, human approval, and provider outages." }}</li>
                        <li><strong class="text-[var(--text-primary)]">Evaluation + safety:</strong> {if vi { "golden dataset, groundedness, precision/recall, task success, escalation rate, prompt injection, PII redaction và audit." } else { "golden datasets, groundedness, precision/recall, task success, escalation rate, prompt injection, PII redaction, and auditability." }}</li>
                        <li><strong class="text-[var(--text-primary)]">Boundary:</strong> {if vi { "AI không phải nguồn sự thật hay nơi quyết định quyền truy cập. Business rules, authorization và trạng thái giao dịch phải do backend kiểm soát." } else { "AI is not the source of truth or authorization authority. The backend owns business rules, access decisions, and transactional state." }}</li>
                    </ul>
                    <p class="mt-4 rounded-lg border border-[var(--border-color)] bg-[var(--surface-hover)] p-3 text-sm leading-6 text-[var(--text-secondary)]">
                        {if vi { "Không cần giả vờ là ML researcher. Hãy chứng minh bạn biết tích hợp AI an toàn, đo lường được và vận hành được; phân biệt rõ điều đã triển khai với điều mới ở mức thiết kế." } else { "Do not pretend to be an ML researcher. Demonstrate that you can integrate AI safely, measurably, and operably; clearly distinguish production experience from design-level knowledge." }}
                    </p>
                </section>
            </div>

            <section class="rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Bài system design trọng tâm: AI Service Desk đa tenant" } else { "Anchor system-design exercise: multi-tenant AI Service Desk" }}</h3>
                <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Thiết kế nền tảng nhận ticket, phân loại và định tuyến, gợi ý câu trả lời từ knowledge base, gọi công cụ nội bộ có kiểm soát, đồng bộ CRM/ERP và chuyển cho nhân viên khi AI không chắc chắn." } else { "Design a platform that ingests tickets, classifies and routes them, drafts answers from a knowledge base, invokes approved internal tools, syncs CRM/ERP, and escalates when confidence is insufficient." }}</p>
                <div class="mt-4 grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
                    <div class="rounded-xl border border-[var(--border-color)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "1. Yêu cầu" } else { "1. Requirements" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Ai dùng? Ticket/ngày? p95 latency? SLO? Dữ liệu nhạy cảm? Những hành động nào cần phê duyệt?" } else { "Who uses it? Tickets/day? p95 latency? SLO? Sensitive data? Which actions require approval?" }}</p></div>
                    <div class="rounded-xl border border-[var(--border-color)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "2. Luồng dữ liệu" } else { "2. Data flow" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "API → DB + outbox → queue → workers → retrieval/LLM → policy check → result/event → UI hoặc human queue." } else { "API → DB + outbox → queue → workers → retrieval/LLM → policy check → result/event → UI or human queue." }}</p></div>
                    <div class="rounded-xl border border-[var(--border-color)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "3. Failure modes" } else { "3. Failure modes" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Message trùng, worker chết, LLM timeout, provider 429, event đến sai thứ tự, retry storm, dữ liệu tenant rò rỉ." } else { "Duplicate messages, worker crash, LLM timeout, provider 429, out-of-order events, retry storms, cross-tenant data leakage." }}</p></div>
                    <div class="rounded-xl border border-[var(--border-color)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "4. Kiểm soát" } else { "4. Controls" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Idempotency key, DLQ/replay, bounded retry, tenant-scoped retrieval, audit log, tracing, human fallback, feature flag." } else { "Idempotency keys, DLQ/replay, bounded retries, tenant-scoped retrieval, audit logs, tracing, human fallback, feature flags." }}</p></div>
                </div>
                <h4 class="mt-5 font-semibold text-[var(--text-primary)]">{if vi { "Câu hỏi đào sâu phải tự trả lời được" } else { "Follow-ups you must be able to answer" }}</h4>
                <ul class="mt-3 grid gap-x-6 gap-y-2 text-sm leading-6 text-[var(--text-secondary)] md:grid-cols-2">
                    <li>{if vi { "• Vì sao chọn queue thay vì xử lý đồng bộ?" } else { "• Why use a queue instead of synchronous processing?" }}</li>
                    <li>{if vi { "• DB commit thành công nhưng publish message thất bại?" } else { "• What if the DB commit succeeds but message publication fails?" }}</li>
                    <li>{if vi { "• Consumer xử lý xong nhưng chết trước ACK?" } else { "• What if a consumer finishes work but crashes before ACK?" }}</li>
                    <li>{if vi { "• Làm sao tránh retry storm và duplicate side effects?" } else { "• How do you prevent retry storms and duplicate side effects?" }}</li>
                    <li>{if vi { "• Làm sao đảm bảo tenant A không truy xuất dữ liệu tenant B?" } else { "• How do you prevent tenant A from retrieving tenant B's data?" }}</li>
                    <li>{if vi { "• Đo chất lượng AI và quyết định escalation bằng cách nào?" } else { "• How do you measure AI quality and decide when to escalate?" }}</li>
                    <li>{if vi { "• Nếu LLM chậm hoặc mất kết nối thì sản phẩm còn hoạt động ra sao?" } else { "• How does the product behave when the LLM is slow or unavailable?" }}</li>
                    <li>{if vi { "• Chọn modular monolith hay microservices ở giai đoạn đầu, vì sao?" } else { "• Would you start with a modular monolith or microservices, and why?" }}</li>
                </ul>
            </section>

            <div class="grid gap-4 lg:grid-cols-2">

                <section class="rounded-2xl border border-[var(--accent)]/30 bg-[var(--surface)] p-5 sm:p-6">
                    <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Đáp án mẫu: điều phải giải thích được, không học vẹt" } else { "Model answer blueprints: understand the reasoning, do not memorize" }}</h3>
                    <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Đây là khung trả lời kỹ thuật để luyện tập. Hãy thay bằng lựa chọn và kinh nghiệm thật của bạn; interviewer có thể đào sâu vào bất kỳ giả định nào." } else { "These are technical answer blueprints for practice. Replace them with your actual decisions and experience; an interviewer may probe any assumption." }}</p>
                    <div class="mt-4 space-y-4">
                        <article class="rounded-xl border border-[var(--border-color)] p-4">
                            <h4 class="font-semibold text-[var(--text-primary)]">1. {if vi { "DB commit thành công nhưng publish message thất bại — xử lý thế nào?" } else { "The DB commit succeeds but message publication fails. What do you do?" }}</h4>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Không thể dựa vào hai thao tác độc lập rồi hy vọng cả hai cùng thành công. Ghi thay đổi nghiệp vụ và một outbox record trong cùng transaction DB. Một background publisher đọc các record chưa gửi, publish với message ID ổn định rồi đánh dấu đã gửi; nếu crash sau publish nhưng trước khi đánh dấu, message có thể bị gửi lặp. Vì vậy consumer phải idempotent, thường bằng inbox/processed-message table hoặc unique business key trong cùng transaction với side effect. Theo dõi tuổi outbox backlog, publish failure, consumer lag và DLQ; có replay/quarantine có kiểm soát. Outbox đảm bảo không mất ý định phát sự kiện khi DB commit, nhưng không tự tạo exactly-once end-to-end." } else { "Do not rely on two independent operations both succeeding. Persist the business change and an outbox record in the same database transaction. A background publisher reads pending records, publishes with a stable message ID, then marks them sent; a crash after publish but before marking can cause duplicates. Therefore consumers must be idempotent, often using an inbox/processed-message table or a unique business key in the same transaction as the side effect. Monitor outbox age/backlog, publish failures, consumer lag, and the DLQ; provide controlled replay/quarantine. The outbox prevents losing the intent to publish when the DB commits, but does not create end-to-end exactly-once processing." }}</p>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]"><strong class="text-[var(--text-primary)]">{if vi { "Đào sâu:" } else { "Follow-ups:" }}</strong> {if vi { "Nếu consumer gọi API thanh toán bên ngoài thì idempotency được bảo đảm ở đâu? Làm sao xử lý thứ tự sự kiện? Khi nào cần partition theo aggregate ID?" } else { "Where is idempotency enforced if the consumer calls an external payment API? How do you handle ordering? When would you partition by aggregate ID?" }}</p>
                        </article>
                        <article class="rounded-xl border border-[var(--border-color)] p-4">
                            <h4 class="font-semibold text-[var(--text-primary)]">2. {if vi { "API chậm: điều tra và tối ưu theo bằng chứng" } else { "A slow API: investigate and optimize from evidence" }}</h4>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Trước tiên xác nhận scope bằng p50/p95/p99, error rate, request volume và thời điểm bắt đầu; dùng distributed trace để tách thời gian ở app, DB và downstream. Với SQL, kiểm tra số query/request, N+1, execution plan thực tế, logical reads, index selectivity, blocking và parameter sensitivity. Với EF Core, project thẳng sang DTO, tránh lazy-loading ngoài ý muốn, dùng AsNoTracking cho read-only khi phù hợp và kiểm tra kích thước Include/join trước khi cân nhắc split query. Đo lại bằng workload đại diện và so sánh latency, CPU, allocations, DB load; không thêm cache hoặc index chỉ vì phỏng đoán. Giữ correctness: pagination ổn định, transaction đúng mức và không trả dữ liệu nhạy cảm qua log." } else { "First establish scope with p50/p95/p99, error rate, request volume, and onset time; use distributed traces to separate application, database, and downstream latency. For SQL, inspect queries per request, N+1, actual execution plans, logical reads, index selectivity, blocking, and parameter sensitivity. With EF Core, project directly to DTOs, prevent accidental lazy loading, use AsNoTracking for suitable read-only paths, and inspect Include/join size before considering split queries. Re-measure with representative workloads and compare latency, CPU, allocations, and DB load; do not add a cache or index based on guesses. Preserve correctness: stable pagination, appropriate transactions, and no sensitive data in logs." }}</p>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]"><strong class="text-[var(--text-primary)]">{if vi { "Đào sâu:" } else { "Follow-ups:" }}</strong> {if vi { "Bạn sẽ rollback thế nào nếu tối ưu làm tăng tải DB? Khi nào cache làm dữ liệu sai? Đo p95 ở client hay server?" } else { "How would you roll back an optimization that increases DB load? When can caching make data incorrect? Where should p95 be measured?" }}</p>
                        </article>
                        <article class="rounded-xl border border-[var(--border-color)] p-4">
                            <h4 class="font-semibold text-[var(--text-primary)]">3. {if vi { "Thiết kế workflow AI Service Desk có thể vận hành" } else { "Design an operable AI Service Desk workflow" }}</h4>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Tách control plane nghiệp vụ khỏi inference. API xác thực user/tenant, validate request, ghi ticket và outbox trong transaction; queue hấp thụ burst, worker xử lý bất đồng bộ và cập nhật trạng thái. Retrieval chỉ tìm tài liệu mà principal hiện tại được phép đọc; model tạo phân loại/nháp câu trả lời hoặc đề xuất tool call, nhưng policy engine/backend vẫn quyết định authorization và validate tham số trước side effect. Ticket có trạng thái rõ ràng, correlation ID, deadline, retry budget, idempotency key, DLQ và đường chuyển sang human queue. Đặt SLO riêng cho ticket acceptance, time-to-first-response, task success và escalation; đo token cost, provider latency, groundedness và tỷ lệ hành động bị từ chối. Bắt đầu bằng modular monolith nếu ranh giới chưa rõ; tách service khi ownership, scale hoặc fault isolation chứng minh được nhu cầu." } else { "Separate business control from inference. The API authenticates user/tenant, validates the request, and writes the ticket plus outbox in one transaction; a queue absorbs bursts while workers process asynchronously and update state. Retrieval returns only documents the current principal may access; the model produces classification/draft answers or tool-call proposals, but a backend policy engine still authorizes and validates arguments before side effects. Use explicit ticket states, correlation IDs, deadlines, retry budgets, idempotency keys, a DLQ, and a human queue fallback. Define separate SLOs for ticket acceptance, time-to-first-response, task success, and escalation; measure token cost, provider latency, groundedness, and rejected actions. Start with a modular monolith if boundaries are unclear; split services when ownership, scaling, or fault isolation demonstrates the need." }}</p>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]"><strong class="text-[var(--text-primary)]">{if vi { "Đào sâu:" } else { "Follow-ups:" }}</strong> {if vi { "Điều gì phải đồng bộ? Kết quả AI hết hạn xử lý ra sao? Nếu provider trả 429 thì retry ở đâu và giới hạn bao nhiêu?" } else { "What must be synchronous? How do you handle stale AI results? Where do you retry provider 429s, and with what budget?" }}</p>
                        </article>
                        <article class="rounded-xl border border-[var(--border-color)] p-4">
                            <h4 class="font-semibold text-[var(--text-primary)]">4. {if vi { "RAG sai hoặc rò dữ liệu: defense-in-depth" } else { "Incorrect RAG answers or data leakage: defense in depth" }}</h4>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Không xem prompt như ranh giới bảo mật. Áp dụng quyền truy cập trước hoặc ngay trong retrieval, gắn tenant/ACL metadata cho document và kiểm thử lọc quyền ở mọi đường query; không dựa vào việc yêu cầu model bỏ qua dữ liệu không được phép. Tách instruction đáng tin khỏi nội dung truy xuất không đáng tin, xem tài liệu và ticket là nguồn có thể chứa prompt injection. Giới hạn tool theo allowlist, schema, user permission và policy; yêu cầu xác nhận cho hành động rủi ro cao. Đánh giá trên golden set có cả câu trả lời đúng/sai, citation correctness, retrieval recall, groundedness, refusal/escalation và cross-tenant adversarial cases. Có audit, redaction PII, retention policy, versioning và khả năng tắt model/feature nhanh." } else { "Do not treat a prompt as a security boundary. Enforce access before or within retrieval, attach tenant/ACL metadata to documents, and test authorization on every query path; never rely on asking the model to ignore unauthorized content. Separate trusted instructions from untrusted retrieved content, because documents and tickets can contain prompt injection. Restrict tools with allowlists, schemas, user permissions, and policies; require confirmation for high-risk actions. Evaluate on a golden set with correct/incorrect answers, citation correctness, retrieval recall, groundedness, refusal/escalation, and cross-tenant adversarial cases. Add audit trails, PII redaction, retention policies, versioning, and a fast model/feature kill switch." }}</p>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]"><strong class="text-[var(--text-primary)]">{if vi { "Đào sâu:" } else { "Follow-ups:" }}</strong> {if vi { "Bạn xử lý xóa tài liệu đã index thế nào? Đánh giá regression trước khi đổi prompt/model ra sao? Ai phê duyệt tool call?" } else { "How do you delete an already-indexed document? How do you test regressions before changing prompts/models? Who approves tool calls?" }}</p>
                        </article>
                        <article class="rounded-xl border border-[var(--border-color)] p-4">
                            <h4 class="font-semibold text-[var(--text-primary)]">5. {if vi { "Chọn modular monolith hay microservices?" } else { "Choose a modular monolith or microservices" }}</h4>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Không chọn theo xu hướng. So sánh domain boundaries, số đội độc lập, nhu cầu deploy/scale riêng, isolation, throughput, maturity vận hành và chi phí mạng/quan sát. Modular monolith cho transaction đơn giản, refactor nhanh và ít overhead khi domain/team còn nhỏ; bắt buộc enforce module boundaries và dependency rules để tránh thành big ball of mud. Microservices có ích khi service có ownership, data boundary và SLO riêng, nhưng thêm distributed failure, versioning, eventual consistency, deployment và on-call burden. Có thể tách worker inference khỏi API vì profile tải và failure mode khác nhau mà không cần chia mọi domain thành service. Ghi ADR, đo bottleneck thực tế và xác định trigger để xem xét lại quyết định." } else { "Do not choose by fashion. Compare domain boundaries, independent teams, independent deployment/scaling needs, isolation, throughput, operational maturity, and network/observability cost. A modular monolith simplifies transactions and refactoring with less overhead while the domain/team is small; enforce module boundaries and dependency rules to avoid a big ball of mud. Microservices help when a service has independent ownership, data boundaries, and SLOs, but add distributed failures, versioning, eventual consistency, deployment, and on-call burden. You may separate inference workers from the API because their load profiles and failure modes differ without splitting every domain into a service. Record an ADR, measure real bottlenecks, and define triggers for revisiting the decision." }}</p>
                        </article>
                        <article class="rounded-xl border border-[var(--border-color)] p-4">
                            <h4 class="font-semibold text-[var(--text-primary)]">6. {if vi { "Staff Engineer khác Senior Engineer ở điểm nào?" } else { "How does a Staff Engineer differ from a Senior Engineer?" }}</h4>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Không chỉ giải bài toán lớn hơn hoặc viết nhiều code hơn. Staff tạo leverage vượt khỏi task cá nhân: làm rõ vấn đề liên nhóm, thiết lập technical direction, giảm rủi ro dài hạn, giúp nhiều kỹ sư ra quyết định tốt hơn và gắn chất lượng kỹ thuật với outcome sản phẩm. Tôi bắt đầu từ vấn đề và metric, đưa các bên liên quan vào sớm, viết decision record cho trade-off quan trọng, chia roadmap thành bước có thể giao an toàn và xác định cách đo kết quả. Tôi không cần là người quyết định mọi thứ; mục tiêu là tạo cơ chế để nhóm tự ra quyết định tốt và biết khi nào cần escalation. Bằng chứng phải là kết quả của nhóm/hệ thống, không chỉ số dòng code hoặc danh sách công nghệ." } else { "It is not simply solving larger problems or writing more code. Staff engineers create leverage beyond individual tasks: clarify cross-team problems, set technical direction, reduce long-term risk, help other engineers make better decisions, and connect engineering quality to product outcomes. I start from the problem and metrics, involve stakeholders early, document important trade-offs, break the roadmap into safely deliverable steps, and define how outcomes will be measured. I do not need to make every decision; the goal is to create mechanisms that help the team decide well and know when to escalate. Evidence should be team/system outcomes, not lines of code or a list of technologies." }}</p>
                        </article>
                    </div>
                </section>

                <section class="rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                    <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Khung trả lời cấp Staff" } else { "Staff-level answer structure" }}</h3>
                    <ol class="mt-4 list-decimal space-y-2 pl-5 text-sm leading-6 text-[var(--text-secondary)]">
                        <li>{if vi { "Làm rõ mục tiêu, constraint, scale và điều chưa biết; hỏi lại trước khi thiết kế." } else { "Clarify goals, constraints, scale, and unknowns; ask questions before designing." }}</li>
                        <li>{if vi { "Nêu phương án đơn giản nhất đáp ứng yêu cầu, rồi vẽ luồng và ranh giới ownership." } else { "Start with the simplest design that meets requirements, then draw flows and ownership boundaries." }}</li>
                        <li>{if vi { "Giải thích trade-off: consistency, latency, cost, complexity, operability và time-to-market." } else { "Explain trade-offs: consistency, latency, cost, complexity, operability, and time-to-market." }}</li>
                        <li>{if vi { "Chủ động phân tích lỗi, bảo mật, quan sát hệ thống, rollout, rollback và phục hồi dữ liệu." } else { "Proactively cover failures, security, observability, rollout, rollback, and data recovery." }}</li>
                        <li>{if vi { "Kết thúc bằng quyết định, giả định, metric xác nhận thành công và điều kiện xem xét lại." } else { "Close with the decision, assumptions, success metrics, and conditions that would trigger a revisit." }}</li>
                    </ol>
                    <p class="mt-4 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Khi kể kinh nghiệm: dùng STAR, nhưng dành nhiều thời gian cho phần bạn trực tiếp quyết định, dữ liệu dùng để quyết định và kết quả đo được." } else { "For experience stories, use STAR but spend most of the time on your own decisions, the evidence behind them, and measurable outcomes." }}</p>
                </section>
                <section class="rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                    <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Thang tự chấm sau mỗi câu" } else { "Self-score every answer" }}</h3>
                    <ul class="mt-4 space-y-3 text-sm leading-6 text-[var(--text-secondary)]">
                        <li><strong class="text-[var(--text-primary)]">{if vi { "0 — Chưa đạt" } else { "0 — Weak" }}:</strong> {if vi { "Chỉ nêu định nghĩa hoặc buzzword; không giải thích cơ chế." } else { "Only definitions or buzzwords; no mechanism." }}</li>
                        <li><strong class="text-[var(--text-primary)]">{if vi { "1 — Cơ bản" } else { "1 — Basic" }}:</strong> {if vi { "Giải thích đúng nhưng thiếu ví dụ, failure mode hoặc trade-off." } else { "Correct explanation but missing examples, failure modes, or trade-offs." }}</li>
                        <li><strong class="text-[var(--text-primary)]">{if vi { "2 — Senior" } else { "2 — Senior" }}:</strong> {if vi { "Đưa ra lựa chọn hợp lý, code/flow cụ thể và cách debug/monitor." } else { "Makes a justified choice and provides concrete code/flow plus debugging and monitoring." }}</li>
                        <li><strong class="text-[var(--text-primary)]">{if vi { "3 — Staff" } else { "3 — Staff" }}:</strong> {if vi { "Liên hệ business impact, tổ chức/đội ngũ, migration, vận hành, chi phí và điều kiện đổi quyết định." } else { "Connects business impact, teams, migration, operations, cost, and decision-reversal criteria." }}</li>
                    </ul>
                </section>
            </div>


            <section class="rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Bộ câu hỏi mock interview theo từng vòng" } else { "Round-by-round mock interview question bank" }}</h3>
                <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Tập trả lời thành tiếng trong giới hạn thời gian. Với mỗi câu, chuẩn bị câu trả lời 30 giây, bản phân tích 2 phút và ít nhất ba câu hỏi đào sâu." } else { "Answer aloud under time pressure. For every prompt, prepare a 30-second answer, a two-minute deep dive, and at least three follow-up questions." }}</p>
                <div class="mt-4 grid gap-4 lg:grid-cols-3">
                    <div class="rounded-xl border border-[var(--border-color)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R1 - Leadership & impact</p>
                        <ol class="mt-3 list-decimal space-y-3 pl-5 text-sm leading-6 text-[var(--text-secondary)]">
                            <li>{if vi { "Hãy giới thiệu bản thân và vì sao kinh nghiệm của bạn phù hợp với vai trò Staff Engineer này." } else { "Introduce yourself and explain why your experience fits this Staff Engineer role." }}</li>
                            <li>{if vi { "Quyết định kỹ thuật nào của bạn tạo tác động lớn nhất? Đo tác động ra sao?" } else { "Which technical decision had your greatest impact, and how did you measure it?" }}</li>
                            <li>{if vi { "Khi Product muốn giao nhanh nhưng hệ thống có rủi ro, bạn xử lý thế nào?" } else { "How do you respond when Product wants a fast release but the system has material risks?" }}</li>
                            <li>{if vi { "Tại sao JTL, tại sao AI Service Desk, và 90 ngày đầu bạn sẽ làm gì?" } else { "Why JTL and AI Service Desk, and what would you do in your first 90 days?" }}</li>
                            <li>{if vi { "Kể về một lần bạn thay đổi quyết định sau khi nhận phản biện." } else { "Tell us about a time you changed your mind after technical pushback." }}</li>
                        </ol>
                        <p class="mt-4 text-sm leading-6 text-[var(--text-secondary)]"><strong class="text-[var(--text-primary)]">{if vi { "Bằng chứng cần có:" } else { "Evidence to prepare:" }}</strong> {if vi { "phạm vi ownership, số liệu trước/sau, stakeholder liên quan, điều bạn trực tiếp làm và bài học." } else { "ownership scope, before/after metrics, stakeholders, your personal contribution, and lessons learned." }}</p>
                    </div>
                    <div class="rounded-xl border border-[var(--border-color)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R2 - Principal Engineer</p>
                        <ol class="mt-3 list-decimal space-y-3 pl-5 text-sm leading-6 text-[var(--text-secondary)]">
                            <li>{if vi { "Thiết kế API xử lý ticket với SLA, idempotency, pagination, auth và audit." } else { "Design a ticket-processing API with SLAs, idempotency, pagination, auth, and audit." }}</li>
                            <li>{if vi { "Một endpoint chậm 4 giây và phát sinh hàng trăm SQL query: bạn điều tra theo thứ tự nào?" } else { "An endpoint takes four seconds and issues hundreds of SQL queries. How do you investigate?" }}</li>
                            <li>{if vi { "Giải thích outbox, consumer idempotency, retry/DLQ và cách replay an toàn." } else { "Explain the outbox, consumer idempotency, retries/DLQ, and safe replay." }}</li>
                            <li>{if vi { "Khi nào chọn modular monolith thay vì microservices? Dấu hiệu nào khiến bạn tách service?" } else { "When would you choose a modular monolith over microservices, and what would justify a split?" }}</li>
                            <li>{if vi { "Thiết kế multi-tenant authorization để ngăn truy cập chéo tenant ở API, cache, search và logs." } else { "Design multi-tenant authorization across APIs, caches, search, and logs." }}</li>
                            <li>{if vi { "Một dependency downstream timeout: deadline, retry, circuit breaker và fallback phối hợp thế nào?" } else { "A downstream dependency times out. How do deadlines, retries, circuit breakers, and fallbacks interact?" }}</li>
                        </ol>
                        <p class="mt-4 text-sm leading-6 text-[var(--text-secondary)]"><strong class="text-[var(--text-primary)]">{if vi { "Bằng chứng cần có:" } else { "Evidence to prepare:" }}</strong> {if vi { "vẽ sequence diagram, nêu failure modes, dữ liệu/metric cần xem và trade-off của ít nhất hai phương án." } else { "draw a sequence diagram, name failure modes, identify diagnostic metrics, and compare at least two options." }}</p>
                    </div>
                    <div class="rounded-xl border border-[var(--border-color)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R2 EM + R3 Head of AI</p>
                        <ol class="mt-3 list-decimal space-y-3 pl-5 text-sm leading-6 text-[var(--text-secondary)]">
                            <li>{if vi { "Bạn nâng chất lượng code review và mentoring một kỹ sư yếu ở một chủ đề như thế nào?" } else { "How do you improve code reviews and mentor an engineer who is struggling in a specific area?" }}</li>
                            <li>{if vi { "Hai senior bất đồng kiến trúc; bạn đưa nhóm đến quyết định bằng cách nào?" } else { "Two senior engineers disagree on architecture. How do you drive a decision?" }}</li>
                            <li>{if vi { "Đo AI answer quality thế nào ngoài việc hỏi người dùng có thích câu trả lời không?" } else { "How do you measure AI answer quality beyond asking users whether they liked it?" }}</li>
                            <li>{if vi { "RAG trả lời sai nhưng rất tự tin; làm sao giới hạn tác hại và điều tra nguyên nhân?" } else { "RAG returns a confident but incorrect answer. How do you limit harm and diagnose it?" }}</li>
                            <li>{if vi { "AI đề xuất refund hoặc thay đổi dữ liệu khách hàng: quyền thực thi và approval nằm ở đâu?" } else { "AI proposes a refund or customer-data change. Where do execution authority and approval live?" }}</li>
                            <li>{if vi { "LLM provider tăng gấp đôi latency/price hoặc ngừng hoạt động: bạn bảo vệ SLO ra sao?" } else { "The LLM provider doubles latency/pricing or goes down. How do you protect the SLO?" }}</li>
                        </ol>
                        <p class="mt-4 text-sm leading-6 text-[var(--text-secondary)]"><strong class="text-[var(--text-primary)]">{if vi { "Bằng chứng cần có:" } else { "Evidence to prepare:" }}</strong> {if vi { "đánh giá theo risk, human review, rollout có kiểm soát, metric, escalation và phối hợp liên nhóm." } else { "risk-based evaluation, human review, controlled rollout, metrics, escalation, and cross-team collaboration." }}</p>
                    </div>
                </div>
                <div class="mt-5 rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-4">
                    <h4 class="font-semibold text-[var(--text-primary)]">{if vi { "Chuẩn bị 6 câu chuyện thật theo STAR" } else { "Prepare six real STAR stories" }}</h4>
                    <div class="mt-3 grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
                        <p class="text-sm leading-6 text-[var(--text-secondary)]">{if vi { "1. Sự cố production: tác động, triage, mitigation, root cause và phòng ngừa tái diễn." } else { "1. Production incident: impact, triage, mitigation, root cause, and prevention." }}</p>
                        <p class="text-sm leading-6 text-[var(--text-secondary)]">{if vi { "2. Tối ưu hiệu năng: baseline, profiling, bottleneck, thay đổi và số liệu sau tối ưu." } else { "2. Performance improvement: baseline, profiling, bottleneck, change, and measured outcome." }}</p>
                        <p class="text-sm leading-6 text-[var(--text-secondary)]">{if vi { "3. Quyết định kiến trúc: lựa chọn, phương án bị loại, trade-off và kết quả." } else { "3. Architecture decision: chosen option, rejected alternatives, trade-offs, and outcome." }}</p>
                        <p class="text-sm leading-6 text-[var(--text-secondary)]">{if vi { "4. Bất đồng: lắng nghe, tiêu chí quyết định, đồng thuận và kết quả." } else { "4. Conflict: listening, decision criteria, alignment, and outcome." }}</p>
                        <p class="text-sm leading-6 text-[var(--text-secondary)]">{if vi { "5. Mentoring/technical leadership: cách nâng năng lực người khác hoặc chuẩn kỹ thuật." } else { "5. Mentoring/technical leadership: how you improved another engineer or engineering standards." }}</p>
                        <p class="text-sm leading-6 text-[var(--text-secondary)]">{if vi { "6. Thất bại hoặc quyết định sai: dấu hiệu bỏ lỡ, cách chịu trách nhiệm và thay đổi sau đó." } else { "6. Failure or a wrong decision: missed signals, accountability, and what changed afterward." }}</p>
                    </div>
                </div>
            </section>

            <section class="rounded-2xl border border-[var(--accent)]/30 bg-[var(--surface)] p-5 sm:p-6">
                <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Đáp án học tập cho từng nhóm kiến thức và câu hỏi trọng tâm" } else { "Study answers for each core domain and interview question" }}</h3>
                <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Hãy học theo mạch: nguyên lý, cách áp dụng, failure mode, trade-off và cách đo. Đây là câu trả lời mẫu để hiểu và luyện nói, không phải tuyên bố rằng bạn đã triển khai các hệ thống này." } else { "Study each topic through its mechanism, application, failure modes, trade-offs, and measurement. These are model answers for learning, not claims that you have implemented these systems." }}</p>
                <div class="mt-4 grid gap-4 lg:grid-cols-2">
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">1. {if vi { "C#/.NET: async/await và thread safety khác nhau thế nào?" } else { "C#/.NET: How do async/await and thread safety differ?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "async/await giúp chờ I/O mà không giữ một thread bị block trong lúc chờ; nó không tự làm CPU-bound work chạy song song và cũng không tự bảo vệ dữ liệu dùng chung. Thread safety cần đồng bộ hoặc thiết kế tránh chia sẻ mutable state, chẳng hạn lock, immutable data, ConcurrentDictionary hoặc actor/message passing. Không dùng Task.Run cho mọi I/O; truyền CancellationToken, đặt timeout/deadline và không nuốt cancellation." } else { "async/await lets code await I/O without blocking a thread while waiting; it does not automatically parallelize CPU-bound work or protect shared state. Thread safety requires synchronization or avoiding shared mutable state, for example with locks, immutable data, ConcurrentDictionary, or actor/message passing. Do not wrap every I/O call in Task.Run; propagate CancellationToken, set timeouts/deadlines, and do not swallow cancellation." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">2. {if vi { "Task hay ValueTask? DI lifetime nào thường gây lỗi?" } else { "Task or ValueTask? Which DI lifetimes commonly cause bugs?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Mặc định dùng Task vì đơn giản và dễ compose. Chỉ cân nhắc ValueTask khi profiling chứng minh đường gọi rất nóng thường hoàn tất đồng bộ và allocation của Task đáng kể; ValueTask có quy tắc tiêu thụ phức tạp hơn. Singleton phải thread-safe và không được giữ scoped dependency như DbContext trực tiếp; scoped phù hợp một request/unit of work, transient tạo instance mỗi lần resolve. Tránh service locator và kiểm tra lifetime khi đăng ký background service." } else { "Prefer Task by default because it is simple to compose. Consider ValueTask only when profiling shows a hot path frequently completes synchronously and Task allocations matter; ValueTask has more complex consumption rules. A singleton must be thread-safe and must not directly capture a scoped dependency such as DbContext; scoped fits a request/unit of work, while transient creates an instance per resolution. Avoid service location and check lifetimes in background services." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">3. {if vi { "API và database: điều tra endpoint chậm như thế nào?" } else { "API and data: how do you investigate a slow endpoint?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Xác định phạm vi bằng p50/p95/p99, error rate, traffic và thời điểm bắt đầu; dùng trace để tách thời gian app, DB và downstream. Kiểm tra query count, N+1, execution plan thực tế, logical reads, index selectivity, lock/blocking và payload size. Với EF Core, projection sang DTO và AsNoTracking có thể giảm dữ liệu/overhead khi phù hợp. Đo lại trên workload đại diện; chỉ thêm index/cache sau khi có bằng chứng và đánh giá tác động ghi, độ mới dữ liệu, invalidation và chi phí." } else { "Establish scope with p50/p95/p99, error rate, traffic, and onset time; use traces to separate application, database, and downstream latency. Inspect query count, N+1, actual execution plans, logical reads, index selectivity, locks/blocking, and payload size. With EF Core, DTO projection and AsNoTracking can reduce data transfer/overhead where appropriate. Re-measure on representative workloads; add indexes or caches only with evidence and assess write impact, freshness, invalidation, and cost." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">4. {if vi { "Transaction isolation và optimistic concurrency giải quyết vấn đề gì?" } else { "What do transaction isolation and optimistic concurrency solve?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Transaction gom thay đổi thành một đơn vị nguyên tử; isolation xác định các trạng thái trung gian mà transaction đồng thời có thể quan sát. Isolation mạnh hơn có thể giảm anomaly nhưng tăng blocking/deadlock. Optimistic concurrency dùng version/rowversion để phát hiện dữ liệu bị sửa sau khi đọc; khi conflict, ứng dụng phải retry có giới hạn hoặc trả conflict cho caller, không ghi đè mù quáng. Chọn mức isolation theo invariant nghiệp vụ và đo contention." } else { "A transaction groups changes into an atomic unit; isolation determines which intermediate states concurrent transactions can observe. Stronger isolation can reduce anomalies but increase blocking/deadlocks. Optimistic concurrency uses a version/rowversion to detect changes since the read; on conflict, the application should retry within a limit or return a conflict rather than overwrite blindly. Choose isolation based on business invariants and measure contention." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">5. {if vi { "Vì sao cần queue, outbox và idempotent consumer?" } else { "Why use a queue, an outbox, and an idempotent consumer?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Queue tách tốc độ nhận request khỏi tốc độ xử lý, hấp thụ burst và cho phép retry; đổi lại cần chấp nhận xử lý bất đồng bộ và quan sát queue lag. Outbox ghi business change cùng event intent trong một DB transaction, rồi publisher phát event sau commit. Vì message có thể được giao lại, consumer cần idempotency key hoặc bảng inbox/unique constraint để một business operation không bị áp dụng hai lần. DLQ phải có quy trình điều tra và replay an toàn." } else { "A queue decouples request intake from processing speed, absorbs bursts, and enables retries, at the cost of asynchronous completion and the need to monitor queue lag. An outbox records the business change and event intent in one database transaction, then a publisher emits the event after commit. Since messages can be redelivered, consumers need an idempotency key or inbox/unique constraint to avoid applying the same business operation twice. A DLQ needs a safe investigation and replay process." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">6. {if vi { "Retry, timeout, circuit breaker và bulkhead phối hợp ra sao?" } else { "How do retries, timeouts, circuit breakers, and bulkheads work together?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Deadline giới hạn tổng thời gian request; timeout giới hạn một lần gọi. Chỉ retry lỗi tạm thời và thao tác an toàn/idempotent, dùng exponential backoff với jitter, giới hạn số lần và retry budget. Circuit breaker tạm ngừng gọi dependency đang lỗi để tránh dồn tải; bulkhead giới hạn concurrency/tài nguyên để một dependency không làm sập toàn hệ thống. Không xếp nhiều tầng retry độc lập vì tổng số lần gọi có thể nhân lên." } else { "A deadline caps total request time; a timeout caps one call. Retry only transient failures and safe/idempotent operations, using exponential backoff with jitter, bounded attempts, and a retry budget. A circuit breaker temporarily stops calls to a failing dependency to prevent overload; a bulkhead limits concurrency/resources so one dependency cannot exhaust the whole system. Avoid independent retries at multiple layers because attempts can multiply." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">7. {if vi { "Modular monolith hay microservices?" } else { "Modular monolith or microservices?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Mặc định bắt đầu bằng modular monolith nếu domain, ownership và scale chưa chứng minh nhu cầu tách service. Giữ module boundary rõ, API nội bộ có hợp đồng và hạn chế truy cập chéo dữ liệu. Microservices hữu ích khi cần deploy/scale độc lập, ownership riêng hoặc cô lập lỗi; nhưng thêm network failure, eventual consistency, observability, CI/CD và chi phí vận hành. Tách khi có bằng chứng từ bottleneck tổ chức/kỹ thuật, không chỉ vì số lượng team hoặc xu hướng." } else { "Default to a modular monolith until domain boundaries, ownership, or scaling requirements justify splitting. Keep module boundaries explicit, define internal contracts, and limit cross-module data access. Microservices help when independent deployment/scaling, ownership, or fault isolation is needed, but add network failures, eventual consistency, observability, CI/CD, and operational cost. Split based on evidence of organizational or technical bottlenecks, not fashion." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">8. {if vi { "SLO, SLI và error budget dùng thế nào?" } else { "How do you use SLIs, SLOs, and error budgets?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "SLI là phép đo chất lượng dịch vụ, ví dụ tỷ lệ request thành công hoặc latency; SLO là mục tiêu trong một cửa sổ thời gian, ví dụ 99.9% request thành công trong 30 ngày. Error budget là phần lỗi cho phép theo SLO. Khi tiêu hao nhanh, giảm rủi ro bằng cách hoãn rollout nguy hiểm, ưu tiên reliability work và điều tra nguyên nhân; không tối ưu một metric nếu làm hỏng trải nghiệm người dùng khác." } else { "An SLI measures service quality, such as successful-request ratio or latency; an SLO is a target over a time window, such as 99.9% successful requests over 30 days. The error budget is the failure allowance implied by the SLO. If it burns quickly, reduce risk by pausing risky rollouts, prioritizing reliability work, and investigating causes; do not optimize one metric at the expense of another user outcome." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">9. {if vi { "Làm sao bảo vệ dữ liệu đa tenant?" } else { "How do you protect multi-tenant data?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Lấy tenant identity từ danh tính đã xác thực và membership được backend kiểm tra, không tin tenant_id do client tự khai. Áp dụng authorization ở mọi đường truy cập: API, DB query, cache key, search/vector retrieval, background job, export và log. Dùng least privilege, tenant-scoped keys/indexes hoặc row-level security khi phù hợp, audit truy cập nhạy cảm và kiểm thử âm tính giữa tenant. Với RAG, lọc quyền trước khi đưa context vào model; lọc sau khi sinh câu trả lời là quá muộn." } else { "Derive tenant identity from authenticated identity and backend-validated membership; do not trust a client-supplied tenant_id. Enforce authorization across APIs, DB queries, cache keys, search/vector retrieval, background jobs, exports, and logs. Use least privilege, tenant-scoped keys/indexes or row-level security where appropriate, audit sensitive access, and test cross-tenant denial. For RAG, filter permissions before context reaches the model; filtering after generation is too late." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">10. {if vi { "RAG hoạt động thế nào và vì sao vẫn có thể trả lời sai?" } else { "How does RAG work, and why can it still be wrong?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Pipeline thường gồm ingest tài liệu có version, chia chunk, gắn metadata/ACL, tạo embedding, retrieve theo query, có thể rerank, rồi đưa đoạn liên quan vào prompt để model trả lời kèm citation. RAG có thể sai nếu tài liệu cũ, chunk mất ngữ cảnh, retrieval bỏ sót nguồn đúng, ranking kém, ACL sai hoặc model suy diễn vượt bằng chứng. Đo retrieval recall/precision riêng với groundedness/correctness của câu trả lời; cập nhật index, kiểm tra citation và abstain/escalate khi bằng chứng không đủ." } else { "A typical pipeline versions and ingests documents, chunks them, attaches metadata/ACLs, creates embeddings, retrieves candidates, optionally reranks them, and supplies relevant passages for a cited answer. RAG can fail when documents are stale, chunks lose context, retrieval misses the right source, ranking is poor, ACLs are wrong, or the model extrapolates beyond evidence. Measure retrieval recall/precision separately from answer groundedness/correctness; refresh indexes, verify citations, and abstain/escalate when evidence is insufficient." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">11. {if vi { "AI tool calling có an toàn để thực thi trực tiếp không?" } else { "Is it safe to execute AI tool calls directly?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Không. Tool call là đề xuất không đáng tin cậy từ model. Backend phải xác thực caller, kiểm tra authorization, validate schema và business invariant, giới hạn tham số, chống replay/idempotency và ghi audit. Hành động rủi ro như refund hoặc sửa dữ liệu cần approval rõ ràng theo policy. Xem nội dung retrieved/user input là dữ liệu không đáng tin để giảm prompt injection; model không được tự cấp quyền cho chính nó." } else { "No. A tool call is an untrusted proposal from the model. The backend must authenticate the caller, authorize the action, validate schema and business invariants, constrain arguments, enforce replay/idempotency controls, and audit execution. Risky actions such as refunds or data changes need explicit policy-based approval. Treat retrieved/user content as untrusted to mitigate prompt injection; the model cannot grant itself authority." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">12. {if vi { "Đo chất lượng AI ngoài thumbs-up/down bằng cách nào?" } else { "How do you evaluate AI beyond thumbs-up/down?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Tạo golden set có nhãn từ các tình huống đại diện và rủi ro cao. Đo retrieval recall, answer correctness/groundedness, citation validity, task completion, escalation/override rate, unsafe action rate, latency và cost trên mỗi ticket được giải quyết. Kết hợp đánh giá tự động với human review và phân tích lỗi theo tenant, ngôn ngữ, loại ticket và phiên bản prompt/model. Chạy offline trước, shadow/canary khi triển khai, đặt ngưỡng rollback; không dùng LLM-as-judge làm nguồn đánh giá duy nhất." } else { "Build a labeled golden set covering representative and high-risk cases. Measure retrieval recall, answer correctness/groundedness, citation validity, task completion, escalation/override rate, unsafe-action rate, latency, and cost per resolved ticket. Combine automated evaluation with human review and slice failures by tenant, language, ticket type, and prompt/model version. Run offline evaluation first, then shadow/canary deployment with rollback thresholds; do not rely solely on an LLM-as-judge." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">13. {if vi { "Nếu LLM timeout, trả 429 hoặc tăng chi phí thì sao?" } else { "What if the LLM times out, returns 429, or becomes more expensive?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Đặt deadline và token/cost budget cho mỗi workflow; giới hạn concurrency và retry theo Retry-After/backoff khi phù hợp. Dùng circuit breaker, cache kết quả an toàn, model/routing policy theo độ khó và fallback sang rule-based triage hoặc human queue. Giữ ticket đã nhận và trạng thái tiến trình ngay cả khi inference lỗi. Theo dõi latency, quota, error rate, cost/ticket và backlog; không retry vô hạn hay âm thầm trả câu trả lời thiếu căn cứ." } else { "Set deadlines and per-workflow token/cost budgets; cap concurrency and retry according to Retry-After/backoff where appropriate. Use a circuit breaker, safe caching, difficulty-based model routing, and fallback to rule-based triage or a human queue. Preserve accepted tickets and workflow state when inference fails. Monitor latency, quota, error rate, cost per ticket, and backlog; never retry indefinitely or silently return unsupported answers." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">14. {if vi { "Khi nào AI phải chuyển cho con người?" } else { "When should AI escalate to a human?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Escalate khi bằng chứng retrieval yếu/mâu thuẫn, confidence đã hiệu chỉnh dưới ngưỡng, yêu cầu ngoài policy, tác động cao, dữ liệu nhạy cảm hoặc người dùng yêu cầu. Ngưỡng phụ thuộc chi phí sai và loại ticket, không chỉ một confidence score do model tự báo. Chuyển kèm nguồn đã truy xuất, lý do escalation và các bước đã thực hiện để nhân viên tiếp quản an toàn; ghi nhận kết quả để cải thiện evaluation set." } else { "Escalate when retrieved evidence is weak/conflicting, calibrated confidence is below threshold, the request is outside policy, impact is high, sensitive data is involved, or the user asks for a person. Thresholds depend on the cost of error and ticket type, not merely a model-reported confidence score. Pass retrieved sources, escalation reason, and completed steps to the agent, and capture outcomes to improve the evaluation set." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">15. {if vi { "Hai senior bất đồng kiến trúc, Staff Engineer nên làm gì?" } else { "Two senior engineers disagree on architecture. What should a Staff Engineer do?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Tách quan điểm khỏi tiêu chí: làm rõ mục tiêu, constraint, rủi ro và dữ liệu còn thiếu; so sánh phương án theo latency, reliability, cost, delivery và khả năng đảo ngược. Nếu bất định lớn, làm prototype hoặc time-boxed experiment. Ghi quyết định và lý do trong ADR, xác định owner cùng điều kiện xem xét lại. Vai trò Staff là nâng chất lượng quyết định và giúp đội ngũ đồng thuận, không phải thắng tranh luận bằng chức danh." } else { "Separate opinions from criteria: clarify goals, constraints, risks, and missing evidence; compare options on latency, reliability, cost, delivery, and reversibility. If uncertainty is high, run a prototype or time-boxed experiment. Record the decision and rationale in an ADR, name an owner, and define revisit conditions. A Staff Engineer improves decision quality and alignment rather than winning by title." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">16. {if vi { "Trả lời câu hỏi kinh nghiệm theo STAR thế nào?" } else { "How should you answer experience questions using STAR?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Situation: bối cảnh và mức độ ảnh hưởng. Task: trách nhiệm cụ thể của bạn. Action: quyết định và hành động do chính bạn thực hiện, gồm phương án cân nhắc và cách phối hợp. Result: kết quả đo được, tác động tới khách hàng/đội ngũ/hệ thống và điều học được. Chỉ dùng số liệu và sự kiện thật; nếu không có metric chính xác, nói rõ cách ước lượng hoặc bằng chứng định tính thay vì bịa số." } else { "Situation: context and impact. Task: your specific responsibility. Action: decisions and actions you personally took, including alternatives considered and collaboration. Result: measurable outcome, impact on customers/team/system, and lessons learned. Use only real facts and metrics; if an exact metric is unavailable, explain the estimate or qualitative evidence instead of inventing a number." }}</p>
                    </article>
                    <article class="rounded-xl border border-[var(--border-color)] p-4">
                        <h4 class="font-semibold text-[var(--text-primary)]">17. {if vi { "Vì sao JTL và bạn sẽ làm gì trong 90 ngày đầu?" } else { "Why JTL, and what would you do in the first 90 days?" }}</h4>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Câu trả lời phải gắn với sản phẩm, khách hàng, yêu cầu của JD và điều bạn thực sự muốn đóng góp; tránh lời khen chung chung. 30 ngày đầu: hiểu domain, kiến trúc, incident, SLO, dữ liệu và stakeholder. Ngày 31–60: chọn một vấn đề có tác động rõ, xác lập baseline và phối hợp giao giải pháp nhỏ có thể đo. Ngày 61–90: chứng minh kết quả, đề xuất roadmap kỹ thuật dựa trên bằng chứng và cải thiện một cơ chế giúp nhiều kỹ sư cùng làm việc tốt hơn. Điều chỉnh kế hoạch sau khi hiểu thực tế, không áp đặt trước khi quan sát." } else { "Connect your answer to the product, customers, job requirements, and the contribution you genuinely want to make; avoid generic praise. Days 1–30: learn the domain, architecture, incidents, SLOs, data, and stakeholders. Days 31–60: choose a clearly impactful problem, establish a baseline, and deliver a small measurable improvement with the team. Days 61–90: demonstrate outcomes, propose an evidence-based technical roadmap, and improve one mechanism that helps multiple engineers work better. Adapt after observing reality rather than imposing a plan before learning." }}</p>
                    </article>
                </div>
            </section>

            <section class="rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] p-5 sm:p-6">
                <h3 class="text-lg font-bold text-[var(--text-primary)]">{if vi { "Kế hoạch ôn tập 14 ngày" } else { "14-day preparation plan" }}</h3>
                <div class="mt-4 grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
                    <div class="rounded-xl bg-[var(--surface-hover)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "Ngày 1–3" } else { "Days 1–3" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "C#/.NET, async/concurrency, DI, ASP.NET Core, API security; luyện nói và review code." } else { "C#/.NET, async/concurrency, DI, ASP.NET Core, API security; speak answers aloud and review code." }}</p></div>
                    <div class="rounded-xl bg-[var(--surface-hover)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "Ngày 4–6" } else { "Days 4–6" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "SQL/EF Core, transaction, indexing, N+1, profiling và tối ưu p95." } else { "SQL/EF Core, transactions, indexing, N+1, profiling, and p95 optimization." }}</p></div>
                    <div class="rounded-xl bg-[var(--surface-hover)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "Ngày 7–10" } else { "Days 7–10" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Messaging, outbox, idempotency, retries, distributed systems, system design AI Service Desk." } else { "Messaging, outbox, idempotency, retries, distributed systems, AI Service Desk system design." }}</p></div>
                    <div class="rounded-xl bg-[var(--surface-hover)] p-4"><p class="font-semibold text-[var(--text-primary)]">{if vi { "Ngày 11–14" } else { "Days 11–14" }}</p><p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Azure/observability/security, AI integration, 6 câu chuyện STAR, mock interview 30/60/60/30 phút." } else { "Azure/observability/security, AI integration, six STAR stories, and 30/60/60/30-minute mock interviews." }}</p></div>
                </div>
                <p class="mt-4 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Mỗi ngày: 60–90 phút học sâu, 30 phút trả lời thành tiếng, 15 phút ghi lại điểm yếu. Không đánh dấu đã hiểu chỉ vì đọc xong — hãy trả lời không nhìn tài liệu và tự chịu 2–3 câu hỏi đào sâu." } else { "Daily: 60–90 minutes of deep study, 30 minutes answering aloud, and 15 minutes logging gaps. Do not mark a topic understood just because you read it; answer without notes and survive two or three follow-ups." }}</p>
            </section>
        </section>

        <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            {cards
                .iter()
                .enumerate()
                .map(|(index, card)| {
                    let id = card.id;
                    let is_understood = understood.contains(id);
                    view! {
                        <a
                            href=format!("#/cv/review/{id}")
                            class="group rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-5 transition hover:-translate-y-0.5 hover:border-[var(--accent)] hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                        >
                            <div class="flex items-start justify-between gap-3">
                                <span class="text-xs font-semibold uppercase tracking-[0.12em] text-[var(--text-tertiary)]">
                                    {format!("#{}", index + 1)}
                                </span>
                                <span class=if is_understood {
                                    "rounded-full border border-[var(--accent)]/40 bg-[var(--accent)]/10 px-2 py-1 text-xs font-semibold text-[var(--accent)]"
                                } else {
                                    "rounded-full border border-[var(--border-color)] px-2 py-1 text-xs font-semibold text-[var(--text-tertiary)]"
                                }>
                                    {if is_understood {
                                        t_string!(i18n, cv_review_understood)
                                    } else {
                                        t_string!(i18n, cv_review_not_understood)
                                    }}
                                </span>
                            </div>
                            <p class="mt-4 text-xs font-medium text-[var(--accent)]">{card.section}</p>
                            <h2 class="mt-2 text-lg font-semibold leading-7 text-[var(--text-primary)] group-hover:text-[var(--accent)]">
                                {card.title}
                            </h2>
                            <p class="mt-3 text-sm font-medium leading-6 text-[var(--text-primary)]">
                                {card.question}
                            </p>
                            <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">
                                {card.simple_explanation}
                            </p>
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}

fn detail_view(
    i18n: I18nContext<crate::i18n::Locale>,
    card: Option<ReviewCard>,
    understood: HashSet<String>,
    toggle_understood: impl Fn(String) + Copy + 'static,
) -> impl IntoView {
    let Some(card) = card else {
        return view! {
            <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface)] p-8 text-center">
                <p class="text-sm text-[var(--text-secondary)]">{t_string!(i18n, cv_review_empty)}</p>
                <a
                    href="#/cv/review"
                    class="mt-5 inline-flex min-h-11 items-center rounded-lg border border-[var(--border-color)] px-4 py-2 text-sm font-semibold text-[var(--text-primary)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                >
                    {t_string!(i18n, cv_review_back_to_list)}
                </a>
            </div>
        }.into_any();
    };

    let id = card.id.to_string();
    let is_understood = understood.contains(&id);

    view! {
        <div>
            <a
                href="#/cv/review"
                class="inline-flex rounded-md text-sm font-medium text-[var(--accent)] underline decoration-transparent underline-offset-4 transition hover:decoration-current focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
            >
                <Icon name=IconName::ArrowLeft class="mr-2 inline-block h-4 w-4" />{t_string!(i18n, cv_review_back_to_list)}
            </a>

            <article class="mt-6 overflow-hidden rounded-2xl border border-[var(--border-color)] bg-[var(--surface)] shadow-sm">
                <div class="border-b border-[var(--border-color)] px-5 py-5 sm:px-8">
                    <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--accent)]">
                        {card.section}
                    </p>
                    <h1 class="mt-2 text-2xl font-bold tracking-tight text-[var(--text-primary)] sm:text-3xl">
                        {card.title}
                    </h1>
                </div>

                <div class="space-y-6 p-5 sm:p-8">
                    <section class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-5">
                        <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--accent)]">
                            {t_string!(i18n, cv_review_question)}
                        </p>
                        <h2 class="mt-3 text-lg font-semibold leading-8 text-[var(--text-primary)]">
                            {card.question}
                        </h2>
                    </section>

                    <section>
                        <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--accent)]">
                            {t_string!(i18n, cv_review_quick_answer)}
                        </p>
                        <p class="mt-3 whitespace-pre-line text-base font-medium leading-7 text-[var(--text-primary)]">
                            {card.quick_answer}
                        </p>
                    </section>

                    <section class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-5">
                        <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--accent)]">
                            {t_string!(i18n, cv_review_simple)}
                        </p>
                        <p class="mt-3 whitespace-pre-line text-base leading-7 text-[var(--text-primary)]">
                            {card.simple_explanation}
                        </p>
                    </section>

                    <section>
                        <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">
                            {t_string!(i18n, cv_review_explanation)}
                        </p>
                        <p class="mt-3 whitespace-pre-line text-sm leading-7 text-[var(--text-secondary)] sm:text-base">
                            {card.explanation}
                        </p>
                    </section>

                    {if card.example.is_empty() {
                        ().into_any()
                    } else {
                        view! {
                            <section class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-5">
                                <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">
                                    {t_string!(i18n, cv_review_example)}
                                </p>
                                <pre class="mt-3 overflow-x-auto whitespace-pre-wrap break-words text-sm leading-6 text-[var(--text-secondary)]"><code>{card.example}</code></pre>
                            </section>
                        }
                        .into_any()
                    }}

                    <section class="rounded-xl border border-[var(--border-color)] p-5">
                        <p class="text-xs font-semibold uppercase tracking-[0.14em] text-[var(--text-tertiary)]">
                            {t_string!(i18n, cv_review_interview_tip)}
                        </p>
                        <p class="mt-3 whitespace-pre-line text-sm leading-6 text-[var(--text-secondary)]">
                            {card.interview_tip}
                        </p>
                    </section>
                </div>
            </article>

            <div class="mt-5 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
                <span class="text-sm text-[var(--text-secondary)]">
                    {if is_understood {
                        t_string!(i18n, cv_review_understood)
                    } else {
                        t_string!(i18n, cv_review_not_understood)
                    }}
                </span>

                <button
                    type="button"
                    class=if is_understood {
                        "min-h-11 rounded-lg border border-[var(--accent)] bg-[var(--accent)]/10 px-5 py-2 text-sm font-semibold text-[var(--accent)] transition hover:bg-[var(--accent)]/15 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    } else {
                        "min-h-11 rounded-lg border border-[var(--border-color)] bg-[var(--surface)] px-5 py-2 text-sm font-semibold text-[var(--text-primary)] transition hover:bg-[var(--surface-hover)] focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--accent)]"
                    }
                    on:click=move |_| {
                        toggle_understood(id.to_string());
                        scroll_to_top();
                    }
                >
                    {if is_understood {
                        t_string!(i18n, cv_review_mark_not_understood)
                    } else {
                        t_string!(i18n, cv_review_mark_understood)
                    }}
                </button>
            </div>
        </div>
    }
    .into_any()
}

fn load_understood() -> HashSet<String> {
    storage_get(PROGRESS_STORAGE_KEY)
        .and_then(|value| serde_json::from_str::<HashSet<String>>(&value).ok())
        .unwrap_or_default()
}

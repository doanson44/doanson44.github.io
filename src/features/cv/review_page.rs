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
                    {if vi { "Staff Software Engineer · AI Service Desk" } else { "Staff Software Engineer · AI Service Desk" }}
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
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R1 · 30 min</p>
                        <h3 class="mt-2 font-semibold text-[var(--text-primary)]">{if vi { "Managing Director + CTO" } else { "Managing Director + CTO" }}</h3>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "Tác động kinh doanh, quyết định kiến trúc, ownership, giao tiếp với stakeholder và lý do phù hợp với vai trò." } else { "Business impact, architecture decisions, ownership, stakeholder communication, and role motivation." }}</p>
                    </div>
                    <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R2 · 120 min</p>
                        <h3 class="mt-2 font-semibold text-[var(--text-primary)]">{if vi { "Principal Engineer + EM" } else { "Principal Engineer + EM" }}</h3>
                        <p class="mt-2 text-sm leading-6 text-[var(--text-secondary)]">{if vi { "60 phút kỹ thuật: C#/.NET, SQL, hệ thống phân tán, thiết kế hệ thống. 60 phút sau: leadership, mentoring, xung đột, delivery và team fit." } else { "First 60 minutes: C#/.NET, SQL, distributed systems, system design. Final 60 minutes: leadership, mentoring, conflict, delivery, and team fit." }}</p>
                    </div>
                    <div class="rounded-xl border border-[var(--border-color)] bg-[var(--surface-hover)] p-4">
                        <p class="text-xs font-bold uppercase tracking-wider text-[var(--accent)]">R3 · 30 min</p>
                        <h3 class="mt-2 font-semibold text-[var(--text-primary)]">{if vi { "Head of AI · Germany" } else { "Head of AI · Germany" }}</h3>
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
                        <li><strong class="text-[var(--text-primary)]">C#/.NET:</strong> {if vi { "async/await, Task vs ValueTask, cancellation, thread-safety, GC/allocations, DI lifetime, middleware, hosted services, exception handling." } else { "async/await, Task vs ValueTask, cancellation, thread safety, GC/allocations, DI lifetimes, middleware, hosted services, exception handling." }}</li>
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

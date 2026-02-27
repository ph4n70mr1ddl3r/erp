use crate::db::AppState;
use crate::handlers;
use crate::middleware::{rate_limit_middleware, RateLimiter};
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Extension, Router,
};
use http::{header, HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

const MAX_REQUEST_BODY_SIZE: usize = 1024 * 1024;

pub fn create_router(state: AppState) -> Router {
    let rate_limiter = RateLimiter::new();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(
            state
                .config
                .cors_allowed_origins
                .iter()
                .filter_map(|origin| origin.parse::<HeaderValue>().ok()),
        ))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .allow_credentials(true);

    let public_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/ws", get(handlers::websocket::websocket_handler))
        .route(
            "/oauth/authorize",
            get(handlers::security::get_oauth_authorize_url),
        )
        .route("/oauth/callback", get(handlers::security::oauth_callback))
        .route(
            "/oauth/providers",
            get(handlers::security::list_oauth_providers),
        )
        .layer(middleware::from_fn(rate_limit_middleware))
        .layer(Extension(rate_limiter.clone()))
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_BODY_SIZE));

    let protected_routes = Router::new()
        .route("/auth/me", get(handlers::auth::me))
        .nest("/api/v1", api_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            handlers::auth::auth_middleware,
        ))
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_BODY_SIZE));

    public_routes
        .merge(protected_routes)
        .with_state(state)
        .layer(cors)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/finance", finance_routes(state.clone()))
        .nest("/inventory", inventory_routes(state.clone()))
        .nest("/sales", sales_routes(state.clone()))
        .nest("/purchasing", purchasing_routes(state.clone()))
        .nest("/manufacturing", manufacturing_routes(state.clone()))
        .nest("/hr", hr_routes(state.clone()))
        .nest("/service", service_routes(state.clone()))
        .nest("/assets", assets_routes(state.clone()))
        .nest("/returns", handlers::returns::routes())
        .nest("/pos", handlers::pos::routes())
        .nest("/ecommerce", handlers::ecommerce::routes())
        .nest("/tax", handlers::tax::routes())
        .nest("/reports", handlers::reports::routes())
        .nest("/barcodes", handlers::barcode::routes())
        .nest("/ai", handlers::ai::routes())
        .nest("/portals", handlers::portals::routes())
        .nest("/iot", handlers::iot::routes())
        .nest("/automation", handlers::automation::routes())
        .route("/audit-logs", get(handlers::audit::list_audit_logs))
        .route(
            "/workflows",
            get(handlers::workflow::list_workflows).post(handlers::workflow::create_workflow),
        )
        .route(
            "/approvals",
            get(handlers::workflow::list_pending_approvals),
        )
        .route(
            "/approvals/:id/approve",
            post(handlers::workflow::approve_request),
        )
        .route(
            "/approvals/:id/reject",
            post(handlers::workflow::reject_request),
        )
        .route(
            "/notifications",
            get(handlers::workflow::list_notifications),
        )
        .route(
            "/notifications/read",
            post(handlers::workflow::mark_all_notifications_read),
        )
        .route(
            "/notifications/:id/read",
            post(handlers::workflow::mark_notification_read),
        )
        .route(
            "/notifications/unread-count",
            get(handlers::workflow::unread_notification_count),
        )
        .route(
            "/attachments",
            get(handlers::attachment::list_attachments)
                .post(handlers::attachment::upload_attachment),
        )
        .route(
            "/attachments/:id",
            get(handlers::attachment::get_attachment)
                .delete(handlers::attachment::delete_attachment),
        )
        .route("/currencies", get(handlers::extended::list_currencies))
        .route(
            "/exchange-rates",
            post(handlers::extended::set_exchange_rate),
        )
        .route("/convert", get(handlers::extended::convert_currency))
        .route(
            "/budgets",
            get(handlers::extended::list_budgets).post(handlers::extended::create_budget),
        )
        .route(
            "/lots",
            get(handlers::extended::list_lots).post(handlers::extended::create_lot),
        )
        .route("/leave-types", get(handlers::extended::list_leave_types))
        .route(
            "/leave-requests",
            get(handlers::extended::list_pending_leave)
                .post(handlers::extended::create_leave_request),
        )
        .route(
            "/leave-requests/:id/approve",
            post(handlers::extended::approve_leave),
        )
        .route(
            "/leave-requests/:id/reject",
            post(handlers::extended::reject_leave),
        )
        .route(
            "/expense-categories",
            get(handlers::extended::list_expense_categories),
        )
        .route(
            "/expense-reports",
            get(handlers::extended::list_expense_reports)
                .post(handlers::extended::create_expense_report),
        )
        .route(
            "/expense-reports/:id/submit",
            post(handlers::extended::submit_expense),
        )
        .route(
            "/expense-reports/:id/approve",
            post(handlers::extended::approve_expense),
        )
        .route(
            "/expense-reports/:id/reject",
            post(handlers::extended::reject_expense),
        )
        .route(
            "/fixed-assets",
            get(handlers::extended::list_fixed_assets).post(handlers::extended::create_fixed_asset),
        )
        .route(
            "/fixed-assets/:id/depreciate",
            post(handlers::extended::depreciate_asset),
        )
        .route("/inspections", post(handlers::extended::create_inspection))
        .route(
            "/inspections/:id/complete",
            post(handlers::extended::complete_inspection),
        )
        .route("/ncrs", post(handlers::extended::create_ncr))
        .route(
            "/leads",
            get(handlers::extended::list_leads).post(handlers::extended::create_lead),
        )
        .route(
            "/opportunities",
            get(handlers::extended::list_opportunities)
                .post(handlers::extended::create_opportunity),
        )
        .route(
            "/opportunities/:id/stage",
            post(handlers::extended::update_opportunity_stage),
        )
        .route(
            "/schedules",
            get(handlers::extended::list_schedules)
                .post(handlers::extended::create_production_schedule),
        )
        .route("/scorecards", post(handlers::extended::create_scorecard))
        .route(
            "/scorecards/:vendor_id",
            get(handlers::extended::list_scorecards),
        )
        .route(
            "/custom-fields",
            post(handlers::extended::create_custom_field),
        )
        .route(
            "/custom-fields/:entity_type",
            get(handlers::extended::list_custom_fields),
        )
        .route("/custom-values", post(handlers::extended::set_custom_value))
        .route("/export", get(handlers::import_export::export_csv))
        .route("/import", post(handlers::import_export::import_csv))
        .nest("/compliance", compliance_routes(state.clone()))
        .nest("/projects", projects_routes(state.clone()))
        .nest("/notifications", handlers::notifications::routes())
        .nest("/webhooks", handlers::webhooks::routes())
        .nest("/jobs", handlers::jobs::routes())
        .nest("/integration", handlers::integration::routes())
        .nest("/templates", handlers::templates::routes())
        .nest("/documents", handlers::documents::routes())
        .nest("/pricing", handlers::pricing::routes())
        .nest("/sourcing", handlers::sourcing::routes())
        .nest("/config", handlers::config::routes())
        .nest("/rules", handlers::rules::routes())
        .nest("/company", handlers::company::routes())
        .nest("/subscription", handlers::subscription::routes())
        .nest("/shipping", handlers::shipping::routes())
        .nest("/payments", handlers::payments::routes())
        .nest("/risk", handlers::risk::routes())
        .nest("/security", security_routes())
        .nest("/search", search_routes())
        .nest("/email", email_routes())
        .nest("/bulk", bulk_routes())
        .nest("/archival", archival_routes())
        .nest("/features", handlers::features::routes())
        .nest("/keys", handlers::keys::routes())
        .nest("/backup", handlers::backup::routes())
        .nest("/monitoring", handlers::monitoring::routes())
        .nest("/rbac", handlers::rbac::routes())
        .nest("/cpq", handlers::cpq::routes())
        .nest("/clm", handlers::clm::routes())
        .nest("/commission", handlers::commission::routes())
        .nest("/aps", handlers::aps::routes())
        .nest("/spend-analytics", handlers::spend_analytics::routes())
        .nest("/compensation", handlers::compensation::routes())
        .nest("/tms", tms_routes())
        .nest("/plm", plm_routes())
        .nest("/mdm", mdm_routes())
        .nest("/fsm", fsm_routes())
        .nest("/tpm", tpm_routes())
        .nest("/wms", handlers::wms::routes())
        .nest("/demand", handlers::demand::routes())
        .nest("/lease", handlers::lease::routes())
        .nest("/bank", handlers::bank::routes())
        .nest("/loyalty", handlers::loyalty::routes())
        .nest("/partner", handlers::partner::routes())
        .nest("/pcard", handlers::pcard::routes())
        .nest("/territory", handlers::territory::routes())
        .nest("/predictive", handlers::predictive::routes())
        .nest("/mrp", handlers::mrp::routes())
        .nest("/eam", handlers::eam::routes())
        .nest("/bi", handlers::bi::routes())
        .nest("/i18n", handlers::i18n::routes())
        .nest("/push", handlers::push::routes())
        .nest("/bpm", handlers::bpm::routes())
        .nest("/graphql", handlers::graphql::routes())
        .nest("/assistant", handlers::assistant::routes())
        .nest("/ocr", handlers::ocr::routes())
        .nest("/fraud", handlers::fraud::routes())
        .nest("/processmining", handlers::processmining::routes())
        .nest("/promotions", promotions_routes())
        .nest("/approval-workflow", approval_workflow_routes())
        .nest("/credit", credit_routes())
        .route("/ws-stats", get(handlers::websocket::get_ws_stats))
}

fn compliance_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/stats", get(handlers::compliance::stats))
        .route(
            "/data-subjects",
            get(handlers::compliance::list_data_subjects)
                .post(handlers::compliance::create_data_subject),
        )
        .route(
            "/consents",
            get(handlers::compliance::list_consents).post(handlers::compliance::create_consent),
        )
        .route(
            "/consents/:id/withdraw",
            post(handlers::compliance::withdraw_consent),
        )
        .route(
            "/dsars",
            get(handlers::compliance::list_dsars).post(handlers::compliance::create_dsar),
        )
        .route(
            "/dsars/:id/complete",
            post(handlers::compliance::complete_dsar),
        )
        .route(
            "/breaches",
            get(handlers::compliance::list_breaches).post(handlers::compliance::create_breach),
        )
        .route("/policies", get(handlers::compliance::list_policies))
        .route("/processors", get(handlers::compliance::list_processors))
        .with_state(state)
}

fn projects_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::projects::list_projects).post(handlers::projects::create_project),
        )
        .route("/:id", get(handlers::projects::get_project))
        .route("/:id/status", post(handlers::projects::update_status))
        .route("/:id/tasks", get(handlers::projects::list_tasks))
        .route("/tasks", post(handlers::projects::create_task))
        .route(
            "/tasks/:id/complete",
            post(handlers::projects::complete_task),
        )
        .route("/:id/milestones", get(handlers::projects::list_milestones))
        .route("/milestones", post(handlers::projects::create_milestone))
        .route(
            "/milestones/:id/complete",
            post(handlers::projects::complete_milestone),
        )
        .route(
            "/timesheets",
            get(handlers::projects::list_timesheets).post(handlers::projects::create_timesheet),
        )
        .route(
            "/timesheets/:id/submit",
            post(handlers::projects::submit_timesheet),
        )
        .route(
            "/timesheets/:id/approve",
            post(handlers::projects::approve_timesheet),
        )
        .with_state(state)
}

fn finance_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/accounts",
            get(handlers::finance::list_accounts).post(handlers::finance::create_account),
        )
        .route(
            "/accounts/:id",
            get(handlers::finance::get_account)
                .put(handlers::finance::update_account)
                .delete(handlers::finance::delete_account),
        )
        .route(
            "/journal-entries",
            get(handlers::finance::list_journal_entries)
                .post(handlers::finance::create_journal_entry),
        )
        .route(
            "/journal-entries/:id",
            get(handlers::finance::get_journal_entry),
        )
        .route(
            "/journal-entries/:id/post",
            post(handlers::finance::post_journal_entry),
        )
        .route(
            "/fiscal-years",
            get(handlers::finance::list_fiscal_years).post(handlers::finance::create_fiscal_year),
        )
        .route(
            "/reports/balance-sheet",
            get(handlers::finance::get_balance_sheet),
        )
        .route(
            "/reports/profit-and-loss",
            get(handlers::finance::get_profit_and_loss),
        )
        .route(
            "/reports/trial-balance",
            get(handlers::finance::get_trial_balance),
        )
        .route(
            "/dunning/policies",
            post(handlers::finance::create_dunning_policy),
        )
        .route(
            "/dunning/policies/:policy_id/levels",
            post(handlers::finance::add_dunning_level),
        )
        .route("/dunning/runs", post(handlers::finance::create_dunning_run))
        .route(
            "/dunning/runs/:id/execute",
            post(handlers::finance::execute_dunning_run),
        )
        .route("/dunning/aging", get(handlers::finance::get_aging_report))
        .route(
            "/collections",
            post(handlers::finance::create_collection_case),
        )
        .route(
            "/collections/:id/activities",
            post(handlers::finance::add_collection_activity),
        )
        .route("/periods", get(handlers::finance::list_periods))
        .route(
            "/periods/create/:fiscal_year_id",
            post(handlers::finance::create_periods),
        )
        .route("/periods/:id/lock", post(handlers::finance::lock_period))
        .route(
            "/periods/:id/unlock",
            post(handlers::finance::unlock_period),
        )
        .route(
            "/periods/:id/checklist",
            post(handlers::finance::create_close_checklist),
        )
        .route(
            "/periods/checklist/:task_id/complete",
            post(handlers::finance::complete_checklist_task),
        )
        .route(
            "/recurring-journals",
            get(handlers::finance::list_recurring_journals)
                .post(handlers::finance::create_recurring_journal),
        )
        .route(
            "/recurring-journals/process",
            post(handlers::finance::process_recurring_journals),
        )
        .route(
            "/recurring-journals/:id/deactivate",
            post(handlers::finance::deactivate_recurring_journal),
        )
        .route(
            "/currency-revaluations",
            get(handlers::finance::list_currency_revaluations)
                .post(handlers::finance::create_currency_revaluation),
        )
        .route(
            "/currency-revaluations/preview",
            post(handlers::finance::preview_currency_revaluation),
        )
        .route(
            "/currency-revaluations/:id",
            get(handlers::finance::get_currency_revaluation),
        )
        .route(
            "/currency-revaluations/:id/lines",
            get(handlers::finance::get_currency_revaluation_lines),
        )
        .route(
            "/currency-revaluations/:id/post",
            post(handlers::finance::post_currency_revaluation),
        )
        .route(
            "/currency-revaluations/:id/reverse",
            post(handlers::finance::reverse_currency_revaluation),
        )
        .with_state(state)
}

fn inventory_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/products",
            get(handlers::inventory::list_products).post(handlers::inventory::create_product),
        )
        .route(
            "/products/:id",
            get(handlers::inventory::get_product)
                .put(handlers::inventory::update_product)
                .delete(handlers::inventory::delete_product),
        )
        .route(
            "/warehouses",
            get(handlers::inventory::list_warehouses).post(handlers::inventory::create_warehouse),
        )
        .route("/warehouses/:id", get(handlers::inventory::get_warehouse))
        .route(
            "/stock-movements",
            post(handlers::inventory::create_stock_movement),
        )
        .route("/stock/:product_id", get(handlers::inventory::get_stock))
        .with_state(state)
}

fn sales_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/customers",
            get(handlers::sales::list_customers).post(handlers::sales::create_customer),
        )
        .route("/customers/:id", get(handlers::sales::get_customer))
        .route(
            "/orders",
            get(handlers::sales::list_orders).post(handlers::sales::create_order),
        )
        .route("/orders/:id", get(handlers::sales::get_order))
        .route("/orders/:id/confirm", post(handlers::sales::confirm_order))
        .route(
            "/invoices",
            get(handlers::sales::list_invoices).post(handlers::sales::create_invoice),
        )
        .route(
            "/quotations",
            get(handlers::sales::list_quotations).post(handlers::sales::create_quotation),
        )
        .route("/quotations/:id", get(handlers::sales::get_quotation))
        .route(
            "/quotations/:id/send",
            post(handlers::sales::send_quotation),
        )
        .route(
            "/quotations/:id/accept",
            post(handlers::sales::accept_quotation),
        )
        .route(
            "/quotations/:id/reject",
            post(handlers::sales::reject_quotation),
        )
        .route(
            "/quotations/:id/convert",
            post(handlers::sales::convert_quotation),
        )
        .with_state(state)
}

fn purchasing_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/vendors",
            get(handlers::purchasing::list_vendors).post(handlers::purchasing::create_vendor),
        )
        .route("/vendors/:id", get(handlers::purchasing::get_vendor))
        .route(
            "/orders",
            get(handlers::purchasing::list_orders).post(handlers::purchasing::create_order),
        )
        .route("/orders/:id", get(handlers::purchasing::get_order))
        .route(
            "/orders/:id/approve",
            post(handlers::purchasing::approve_order),
        )
        .with_state(state)
}

fn manufacturing_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/boms",
            get(handlers::manufacturing::list_boms).post(handlers::manufacturing::create_bom),
        )
        .route("/boms/:id", get(handlers::manufacturing::get_bom))
        .route(
            "/work-orders",
            get(handlers::manufacturing::list_work_orders)
                .post(handlers::manufacturing::create_work_order),
        )
        .route(
            "/work-orders/:id/start",
            post(handlers::manufacturing::start_work_order),
        )
        .route(
            "/work-orders/:id/complete",
            post(handlers::manufacturing::complete_work_order),
        )
        .with_state(state)
}

fn hr_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/employees",
            get(handlers::hr::list_employees).post(handlers::hr::create_employee),
        )
        .route("/employees/:id", get(handlers::hr::get_employee))
        .route("/attendance/check-in", post(handlers::hr::check_in))
        .route("/attendance/check-out", post(handlers::hr::check_out))
        .route(
            "/leave-requests",
            get(handlers::hr::list_leave_requests).post(handlers::hr::create_leave_request),
        )
        .route(
            "/payroll",
            get(handlers::hr::list_payroll).post(handlers::hr::create_payroll),
        )
        .with_state(state)
}

fn service_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/tickets",
            get(handlers::service::list_tickets).post(handlers::service::create_ticket),
        )
        .route("/tickets/:id", get(handlers::service::get_ticket))
        .route(
            "/tickets/:id/assign",
            post(handlers::service::assign_ticket),
        )
        .route(
            "/tickets/:id/status",
            post(handlers::service::update_ticket_status),
        )
        .route(
            "/tickets/:id/satisfaction",
            post(handlers::service::set_satisfaction),
        )
        .route("/tickets/stats", get(handlers::service::ticket_stats))
        .route(
            "/articles",
            get(handlers::service::list_articles).post(handlers::service::create_article),
        )
        .route("/articles/search", get(handlers::service::search_articles))
        .route("/articles/:id", get(handlers::service::get_article))
        .route(
            "/articles/:id/publish",
            post(handlers::service::publish_article),
        )
        .route(
            "/articles/:id/archive",
            post(handlers::service::archive_article),
        )
        .route(
            "/articles/:id/feedback",
            post(handlers::service::article_feedback),
        )
        .route(
            "/slas",
            get(handlers::service::list_slas).post(handlers::service::create_sla),
        )
        .with_state(state)
}

fn assets_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/assets",
            get(handlers::assets::list_assets).post(handlers::assets::create_asset),
        )
        .route("/assets/:id", get(handlers::assets::get_asset))
        .route("/assets/:id/assign", post(handlers::assets::assign_asset))
        .route("/assets/:id/return", post(handlers::assets::return_asset))
        .route(
            "/assets/:id/status",
            post(handlers::assets::update_asset_status),
        )
        .route("/assets/stats", get(handlers::assets::asset_stats))
        .route(
            "/licenses",
            get(handlers::assets::list_licenses).post(handlers::assets::create_license),
        )
        .route("/licenses/:id", get(handlers::assets::get_license))
        .route(
            "/licenses/:id/use",
            post(handlers::assets::use_license_seat),
        )
        .route(
            "/licenses/:id/release",
            post(handlers::assets::release_license_seat),
        )
        .route(
            "/licenses/expiring",
            get(handlers::assets::expiring_licenses),
        )
        .with_state(state)
}

fn security_routes() -> Router<AppState> {
    Router::new()
        .route("/2fa/setup", post(handlers::security::setup_two_factor))
        .route("/2fa/verify", post(handlers::security::verify_two_factor))
        .route(
            "/2fa/status",
            get(handlers::security::get_two_factor_status),
        )
        .route("/2fa/disable", post(handlers::security::disable_two_factor))
        .route(
            "/2fa/backup-codes",
            post(handlers::security::regenerate_backup_codes),
        )
        .route("/oauth/link", post(handlers::security::link_oauth_account))
        .route(
            "/oauth/connections",
            get(handlers::security::get_oauth_connections),
        )
        .route(
            "/oauth/connections/:id",
            delete(handlers::security::unlink_oauth_account),
        )
        .route("/sessions", get(handlers::security::get_user_sessions))
        .route("/sessions/:id", delete(handlers::security::revoke_session))
        .route(
            "/sessions/all",
            delete(handlers::security::revoke_all_sessions),
        )
}

fn search_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::search::search))
        .route("/index", post(handlers::search::index_entity))
        .route(
            "/index/:entity_type/:entity_id",
            delete(handlers::search::remove_from_index),
        )
        .route("/rebuild", post(handlers::search::rebuild_index))
        .route("/stats", get(handlers::search::search_stats))
}

fn email_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/templates",
            get(handlers::email_templates::list_templates)
                .post(handlers::email_templates::create_template),
        )
        .route(
            "/templates/:name",
            get(handlers::email_templates::get_template)
                .put(handlers::email_templates::update_template)
                .delete(handlers::email_templates::delete_template),
        )
        .route("/queue", post(handlers::email_templates::queue_email))
        .route(
            "/queue/pending",
            get(handlers::email_templates::get_pending_emails),
        )
        .route(
            "/queue/stats",
            get(handlers::email_templates::get_email_queue_stats),
        )
}

fn bulk_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::bulk_operations::list_operations)
                .post(handlers::bulk_operations::create_operation),
        )
        .route(
            "/:id",
            get(handlers::bulk_operations::get_operation)
                .delete(handlers::bulk_operations::cancel_operation),
        )
        .route(
            "/cleanup",
            delete(handlers::bulk_operations::cleanup_operations),
        )
}

fn archival_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/policies",
            get(handlers::archival::list_retention_policies)
                .post(handlers::archival::create_retention_policy),
        )
        .route(
            "/policies/:entity_type",
            get(handlers::archival::get_retention_policy),
        )
        .route(
            "/records",
            get(handlers::archival::list_archived_records).post(handlers::archival::archive_record),
        )
        .route(
            "/records/:id",
            get(handlers::archival::get_archived_record)
                .delete(handlers::archival::delete_archived_record),
        )
        .route(
            "/records/:id/restore",
            post(handlers::archival::restore_record),
        )
        .route("/purge", post(handlers::archival::purge_expired))
        .route("/stats", get(handlers::archival::archival_stats))
}

fn tms_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/vehicles",
            get(handlers::tms::list_vehicles).post(handlers::tms::create_vehicle),
        )
        .route("/vehicles/:id", get(handlers::tms::get_vehicle))
        .route(
            "/drivers",
            get(handlers::tms::list_drivers).post(handlers::tms::create_driver),
        )
        .route("/drivers/:id", get(handlers::tms::get_driver))
        .route(
            "/loads",
            get(handlers::tms::list_loads).post(handlers::tms::create_load),
        )
        .route("/loads/:id", get(handlers::tms::get_load))
        .route("/loads/:id/assign", post(handlers::tms::assign_load))
        .route("/loads/:id/dispatch", post(handlers::tms::dispatch_load))
        .route("/loads/:id/deliver", post(handlers::tms::deliver_load))
        .route("/routes/optimize", post(handlers::tms::optimize_route))
        .route(
            "/freight-invoices/:id/audit",
            post(handlers::tms::audit_freight_invoice),
        )
}

fn plm_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/items",
            get(handlers::plm::list_items).post(handlers::plm::create_item),
        )
        .route("/items/:id", get(handlers::plm::get_item))
        .route("/items/:id/release", post(handlers::plm::release_item))
        .route("/ecrs", post(handlers::plm::create_ecr))
        .route("/ecrs/:id/submit", post(handlers::plm::submit_ecr))
        .route("/ecrs/:id/approve", post(handlers::plm::approve_ecr))
        .route("/ecrs/:id/reject", post(handlers::plm::reject_ecr))
        .route("/boms", post(handlers::plm::create_bom))
        .route("/specifications", post(handlers::plm::create_specification))
        .route("/design-reviews", post(handlers::plm::create_design_review))
}

fn mdm_routes() -> Router<AppState> {
    Router::new()
        .route("/golden-records", post(handlers::mdm::create_golden_record))
        .route("/golden-records/:id", get(handlers::mdm::get_golden_record))
        .route("/quality-rules", post(handlers::mdm::create_quality_rule))
        .route("/quality-check/:id", post(handlers::mdm::run_quality_check))
        .route(
            "/violations/:id/resolve",
            post(handlers::mdm::resolve_violation),
        )
        .route("/duplicates/find", post(handlers::mdm::find_duplicates))
        .route("/merge", post(handlers::mdm::merge_records))
        .route(
            "/dashboard/:entity_type",
            get(handlers::mdm::get_quality_dashboard),
        )
        .route("/import-jobs", post(handlers::mdm::create_import_job))
        .route(
            "/import-jobs/:id/start",
            post(handlers::mdm::start_import_job),
        )
}

fn fsm_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/orders",
            get(handlers::fsm::list_service_orders).post(handlers::fsm::create_service_order),
        )
        .route("/orders/:id", get(handlers::fsm::get_service_order))
        .route("/orders/dispatch", post(handlers::fsm::dispatch_order))
        .route("/orders/:id/start", post(handlers::fsm::start_service))
        .route(
            "/orders/:id/complete",
            post(handlers::fsm::complete_service),
        )
        .route("/orders/:id/feedback", post(handlers::fsm::record_feedback))
        .route(
            "/technicians",
            get(handlers::fsm::list_technicians).post(handlers::fsm::create_technician),
        )
        .route("/routes/optimize", post(handlers::fsm::optimize_route))
        .route(
            "/technicians/find",
            post(handlers::fsm::find_available_technician),
        )
}

fn tpm_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/promotions",
            get(handlers::tpm::list_promotions).post(handlers::tpm::create_promotion),
        )
        .route("/promotions/:id", get(handlers::tpm::get_promotion))
        .route(
            "/promotions/:id/activate",
            post(handlers::tpm::activate_promotion),
        )
        .route(
            "/promotions/:id/performance",
            post(handlers::tpm::calculate_promotion_performance),
        )
        .route(
            "/rebate-agreements",
            post(handlers::tpm::create_rebate_agreement),
        )
        .route(
            "/rebate-agreements/:id",
            get(handlers::tpm::get_rebate_agreement),
        )
        .route(
            "/rebate-agreements/:id/calculate",
            post(handlers::tpm::calculate_rebate),
        )
        .route(
            "/rebate-agreements/:id/payment",
            post(handlers::tpm::process_rebate_payment),
        )
        .route("/chargebacks", post(handlers::tpm::submit_chargeback))
        .route(
            "/chargebacks/:id/review",
            post(handlers::tpm::review_chargeback),
        )
        .route("/funds", post(handlers::tpm::create_trade_fund))
        .route("/funds/:id/commit", post(handlers::tpm::commit_fund))
}

fn promotions_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/campaigns",
            get(handlers::promotions::list_promotions).post(handlers::promotions::create_promotion),
        )
        .route("/campaigns/:id", get(handlers::promotions::get_promotion))
        .route(
            "/campaigns/:id",
            put(handlers::promotions::update_promotion),
        )
        .route(
            "/campaigns/:id/activate",
            post(handlers::promotions::activate_promotion),
        )
        .route(
            "/campaigns/:id/deactivate",
            post(handlers::promotions::deactivate_promotion),
        )
        .route(
            "/campaigns/:id/calculate",
            post(handlers::promotions::calculate_promotion_discount),
        )
        .route(
            "/campaigns/:id/report",
            get(handlers::promotions::get_promotion_report),
        )
        .route(
            "/coupons",
            get(handlers::promotions::list_coupons).post(handlers::promotions::create_coupon),
        )
        .route("/coupons/:id", get(handlers::promotions::get_coupon))
        .route(
            "/coupons/validate",
            post(handlers::promotions::validate_coupon),
        )
        .route("/coupons/apply", post(handlers::promotions::apply_coupon))
        .route(
            "/coupons/generate-batch",
            post(handlers::promotions::generate_coupon_batch),
        )
}

fn approval_workflow_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/workflows",
            get(handlers::approval_workflow::list_workflows)
                .post(handlers::approval_workflow::create_workflow),
        )
        .route(
            "/workflows/:id",
            get(handlers::approval_workflow::get_workflow)
                .put(handlers::approval_workflow::update_workflow)
                .delete(handlers::approval_workflow::delete_workflow),
        )
        .route(
            "/requests",
            get(handlers::approval_workflow::list_requests)
                .post(handlers::approval_workflow::submit_for_approval),
        )
        .route(
            "/requests/:id",
            get(handlers::approval_workflow::get_request),
        )
        .route(
            "/requests/:id/approve",
            post(handlers::approval_workflow::approve_request),
        )
        .route(
            "/requests/:id/reject",
            post(handlers::approval_workflow::reject_request),
        )
        .route(
            "/requests/:id/cancel",
            post(handlers::approval_workflow::cancel_request),
        )
        .route(
            "/pending/:user_id",
            get(handlers::approval_workflow::get_pending_approvals),
        )
        .route(
            "/pending/:user_id/summary",
            get(handlers::approval_workflow::get_pending_summary),
        )
}

fn credit_routes() -> Router<AppState> {
    Router::new()
        .route("/check", post(handlers::credit::check_credit))
        .route("/summary", get(handlers::credit::get_summary))
        .route("/profiles", get(handlers::credit::list_profiles))
        .route("/on-hold", get(handlers::credit::list_on_hold))
        .route("/high-risk", get(handlers::credit::list_high_risk))
        .route("/alerts", get(handlers::credit::list_alerts))
        .route(
            "/alerts/:id/acknowledge",
            post(handlers::credit::acknowledge_alert),
        )
        .route("/invoice", post(handlers::credit::record_invoice))
        .route("/payment", post(handlers::credit::record_payment))
        .route("/:customer_id", get(handlers::credit::get_profile))
        .route(
            "/:customer_id/limit",
            post(handlers::credit::update_credit_limit),
        )
        .route("/:customer_id/hold", post(handlers::credit::place_hold))
        .route(
            "/:customer_id/release",
            post(handlers::credit::release_hold),
        )
        .route(
            "/:customer_id/transactions",
            get(handlers::credit::list_transactions),
        )
        .route("/:customer_id/holds", get(handlers::credit::list_holds))
        .route(
            "/:customer_id/limit-changes",
            get(handlers::credit::list_limit_changes),
        )
}

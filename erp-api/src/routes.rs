use crate::db::AppState;
use crate::handlers;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

const MAX_REQUEST_BODY_SIZE: usize = 1024 * 1024;

pub fn create_router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login));

    let protected_routes = Router::new()
        .route("/auth/me", get(handlers::auth::me))
        .nest("/api/v1", api_routes(state.clone()))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            handlers::auth::auth_middleware,
        ))
        .layer(RequestBodyLimitLayer::new(MAX_REQUEST_BODY_SIZE));

    public_routes.merge(protected_routes).with_state(state)
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

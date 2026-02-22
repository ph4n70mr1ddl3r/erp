use crate::db::AppState;
use crate::handlers;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

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
        ));

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
        .route("/export", get(handlers::import_export::export_csv))
        .route("/import", post(handlers::import_export::import_csv))
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

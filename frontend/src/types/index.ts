export interface User {
  id: string;
  username: string;
  email: string;
  full_name: string;
  role: string;
}

export interface AuthResponse {
  token: string;
  expires_at: string;
  user: User;
}

export interface Paginated<T> {
  items: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface Account {
  id: string;
  code: string;
  name: string;
  account_type: string;
  status: string;
  description?: string;
}

export interface JournalEntry {
  id: string;
  entry_number: string;
  date: string;
  description: string;
  status: string;
  total_debit: number;
  total_credit: number;
  lines: JournalLine[];
}

export interface JournalLine {
  id: string;
  account_id: string;
  debit: number;
  credit: number;
  description?: string;
}

export interface Product {
  id: string;
  sku: string;
  name: string;
  product_type: string;
  unit_of_measure: string;
  status: string;
}

export interface Warehouse {
  id: string;
  code: string;
  name: string;
  status: string;
}

export interface Customer {
  id: string;
  code: string;
  name: string;
  email?: string;
  status: string;
}

export interface SalesOrder {
  id: string;
  order_number: string;
  customer_id: string;
  status: string;
  total: number;
}

export interface Vendor {
  id: string;
  code: string;
  name: string;
  email?: string;
  status: string;
}

export interface PurchaseOrder {
  id: string;
  po_number: string;
  vendor_id: string;
  status: string;
  total: number;
}

export interface Employee {
  id: string;
  employee_number: string;
  first_name: string;
  last_name: string;
  email: string;
  status: string;
}

export interface AccountBalanceResponse {
  account_id: string;
  account_code: string;
  account_name: string;
  account_type: string;
  balance: number;
}

export interface Lead {
  id: string;
  lead_number: string;
  company_name: string;
  contact_name?: string;
  email?: string;
  estimated_value: number;
  status: string;
}

export interface Opportunity {
  id: string;
  opportunity_number: string;
  name: string;
  stage: string;
  probability: number;
  amount: number;
  status: string;
}

export interface ApiError {
  error?: string;
  message?: string;
}

export interface LeaveType {
  id: string;
  name: string;
  code: string;
  days_per_year: number;
  carry_over: boolean;
  status: string;
}

export interface LeaveRequest {
  id: string;
  employee_id: string;
  leave_type_id: string;
  start_date: string;
  end_date: string;
  days: number;
  reason?: string;
  status: string;
  created_at: string;
}

export interface ExpenseCategory {
  id: string;
  name: string;
  code: string;
  description?: string;
  status: string;
}

export interface ExpenseReport {
  id: string;
  report_number: string;
  employee_id: string;
  description: string;
  total_amount: number;
  currency: string;
  status: string;
  submitted_at?: string;
  approved_at?: string;
  rejected_at?: string;
  rejection_reason?: string;
  created_at: string;
  lines: ExpenseLine[];
}

export interface ExpenseLine {
  id: string;
  category_id: string;
  date: string;
  amount: number;
  description?: string;
  status: string;
}

export interface BankAccount {
  id: string;
  connection_id: string;
  account_number: string;
  masked_account_number: string;
  account_name: string;
  account_type: string;
  currency: string;
  gl_account_id?: string;
  company_id: string;
  bank_branch?: string;
  iban?: string;
  routing_number?: string;
  auto_reconcile: boolean;
  status: string;
}

export interface BankStatement {
  id: string;
  statement_number: string;
  bank_account_id: string;
  statement_date: string;
  currency: string;
  opening_balance: number;
  closing_balance: number;
  total_credits: number;
  total_debits: number;
  credit_count: number;
  debit_count: number;
  status: string;
}

export interface BankTransaction {
  id: string;
  statement_id: string;
  bank_account_id: string;
  transaction_date: string;
  value_date?: string;
  transaction_type: string;
  amount: number;
  currency: string;
  reference_number?: string;
  description: string;
  payee_name?: string;
  reconciliation_status: string;
  matched_entity_type?: string;
  matched_entity_id?: string;
  match_confidence?: number;
}

export interface ReconciliationSession {
  id: string;
  session_number: string;
  bank_account_id: string;
  period_start: string;
  period_end: string;
  total_transactions: number;
  matched_count: number;
  unmatched_count: number;
  opening_balance: number;
  closing_balance: number;
  calculated_balance: number;
  variance: number;
  status: string;
  started_at: string;
  completed_at?: string;
}

export interface ReconciliationMatch {
  id: string;
  session_id: string;
  bank_transaction_id: string;
  entity_type: string;
  entity_id: string;
  entity_reference: string;
  transaction_amount: number;
  entity_amount: number;
  match_difference: number;
  match_type: string;
  match_confidence: number;
  matched_at: string;
}

export interface ReconciliationSummary {
  bank_account_id: string;
  account_name: string;
  currency: string;
  gl_balance: number;
  bank_balance?: number;
  unreconciled_count: number;
  unreconciled_debits: number;
  unreconciled_credits: number;
  deposits_in_transit: number;
  outstanding_checks: number;
  adjusted_balance: number;
  variance: number;
  last_reconciled_at?: string;
}

export interface PayrollRun {
  id: string;
  run_number: string;
  pay_period_start: string;
  pay_period_end: string;
  pay_date: string;
  total_gross: number;
  total_deductions: number;
  total_net: number;
  status: string;
  processed_at?: string;
  approved_at?: string;
  created_at: string;
}

export interface PayrollEntry {
  id: string;
  payroll_run_id: string;
  employee_id: string;
  employee_name: string;
  gross_pay: number;
  total_deductions: number;
  net_pay: number;
  payment_method: string;
  bank_account?: string;
  status: string;
}

export interface BudgetLine {
  account_id: string;
  period: number;
  amount: number;
  actual?: number;
  variance?: number;
}

export interface Budget {
  id: string;
  name: string;
  start_date: string;
  end_date: string;
  total_amount: number;
  total_actual: number;
  total_variance: number;
  variance_percent: number;
  status: string;
  lines: BudgetLine[];
}

export interface SupplierScorecard {
  id: string;
  vendor_id: string;
  period: string;
  on_time_delivery: number;
  quality_score: number;
  price_competitiveness: number;
  responsiveness: number;
  overall_score: number;
  total_orders: number;
  total_value: number;
  created_at: string;
}

export interface VendorPerformance {
  id: string;
  vendor_id: string;
  order_id: string;
  delivery_date?: string;
  expected_date?: string;
  on_time: boolean;
  quality_rating: number;
  notes?: string;
  created_at: string;
}

export interface Shift {
  id: string;
  code: string;
  name: string;
  description?: string;
  start_time: string;
  end_time: string;
  break_minutes: number;
  grace_period_minutes: number;
  color_code?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface Schedule {
  id: string;
  code: string;
  name: string;
  description?: string;
  department_id?: string;
  start_date: string;
  end_date: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface ShiftAssignment {
  id: string;
  schedule_id: string;
  shift_id: string;
  employee_id: string;
  assignment_date: string;
  actual_start_time?: string;
  actual_end_time?: string;
  overtime_minutes: number;
  notes?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

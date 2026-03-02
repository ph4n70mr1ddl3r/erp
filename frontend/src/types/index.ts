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

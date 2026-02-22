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

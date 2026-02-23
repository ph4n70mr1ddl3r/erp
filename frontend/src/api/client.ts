import axios from 'axios';
import type { 
  AuthResponse, User, Paginated, Account, JournalEntry, Product, 
  Warehouse, Customer, SalesOrder, Vendor, PurchaseOrder, Employee 
} from '../types';

const API_URL = import.meta.env.VITE_API_URL || '';

const api = axios.create({
  baseURL: API_URL || undefined,
  headers: { 'Content-Type': 'application/json' },
});

api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export default api;

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
  full_name: string;
}

export interface CreateAccountRequest {
  code: string;
  name: string;
  account_type: 'Asset' | 'Liability' | 'Equity' | 'Revenue' | 'Expense';
  description?: string;
  parent_id?: string;
}

export interface CreateJournalEntryRequest {
  date: string;
  description: string;
  reference?: string;
  lines: {
    account_id: string;
    debit: number;
    credit: number;
    description?: string;
  }[];
}

export interface CreateProductRequest {
  sku: string;
  name: string;
  description?: string;
  product_type: 'Goods' | 'Service' | 'Digital';
  unit_of_measure: string;
  category_id?: string;
}

export interface CreateWarehouseRequest {
  code: string;
  name: string;
  address?: {
    street?: string;
    city?: string;
    state?: string;
    postal_code?: string;
    country?: string;
  };
}

export interface CreateCustomerRequest {
  code: string;
  name: string;
  email?: string;
  phone?: string;
  billing_address?: {
    street?: string;
    city?: string;
    state?: string;
    postal_code?: string;
    country?: string;
  };
}

export interface CreateOrderRequest {
  customer_id: string;
  order_date: string;
  lines: {
    product_id: string;
    quantity: number;
    unit_price: number;
  }[];
}

export interface CreateVendorRequest {
  code: string;
  name: string;
  email?: string;
  phone?: string;
  address?: {
    street?: string;
    city?: string;
    state?: string;
    postal_code?: string;
    country?: string;
  };
}

export interface CreatePurchaseOrderRequest {
  vendor_id: string;
  order_date: string;
  lines: {
    product_id: string;
    quantity: number;
    unit_price: number;
  }[];
}

export interface CreateEmployeeRequest {
  employee_number: string;
  first_name: string;
  last_name: string;
  email: string;
  department_id?: string;
  hire_date: string;
}

export interface CreateStockMovementRequest {
  product_id: string;
  to_location_id: string;
  from_location_id?: string;
  quantity: number;
  movement_type: 'Receipt' | 'Issue' | 'Transfer';
}

export interface CreateQuotationRequest {
  customer_id: string;
  valid_until: string;
  lines: {
    product_id: string;
    quantity: number;
    unit_price: number;
  }[];
}

export interface CreateBudgetRequest {
  name: string;
  fiscal_year_id: string;
  lines: {
    account_id: string;
    period: number;
    amount: number;
  }[];
}

export interface CreateLeaveRequestPayload {
  employee_id: string;
  leave_type_id: string;
  start_date: string;
  end_date: string;
  reason?: string;
}

export interface CreateExpenseReportRequest {
  employee_id: string;
  description: string;
  lines: {
    category_id: string;
    date: string;
    amount: number;
    description?: string;
  }[];
}

export interface CreateLotRequest {
  product_id: string;
  lot_number: string;
  quantity: number;
  expiry_date?: string;
}

// Auth
export const auth = {
  login: (data: LoginRequest) => api.post<AuthResponse>('/auth/login', data),
  register: (data: RegisterRequest) => api.post<AuthResponse>('/auth/register', data),
  me: () => api.get<{ user: User }>('/auth/me'),
};

// Finance
export const finance = {
  getAccounts: (page = 1, perPage = 20) => api.get<Paginated<Account>>(`/api/v1/finance/accounts?page=${page}&per_page=${perPage}`),
  createAccount: (data: CreateAccountRequest) => api.post<Account>('/api/v1/finance/accounts', data),
  getJournalEntries: (page = 1, perPage = 20) => api.get<Paginated<JournalEntry>>(`/api/v1/finance/journal-entries?page=${page}&per_page=${perPage}`),
  createJournalEntry: (data: CreateJournalEntryRequest) => api.post<JournalEntry>('/api/v1/finance/journal-entries', data),
  postJournalEntry: (id: string) => api.post(`/api/v1/finance/journal-entries/${id}/post`),
  getBalanceSheet: () => api.get('/api/v1/finance/reports/balance-sheet'),
  getProfitAndLoss: () => api.get('/api/v1/finance/reports/profit-and-loss'),
  getTrialBalance: () => api.get('/api/v1/finance/reports/trial-balance'),
};

// Inventory
export const inventory = {
  getProducts: (page = 1, perPage = 20) => api.get<Paginated<Product>>(`/api/v1/inventory/products?page=${page}&per_page=${perPage}`),
  createProduct: (data: CreateProductRequest) => api.post<Product>('/api/v1/inventory/products', data),
  updateProduct: (id: string, data: Partial<CreateProductRequest>) => api.put<Product>(`/api/v1/inventory/products/${id}`, data),
  deleteProduct: (id: string) => api.delete(`/api/v1/inventory/products/${id}`),
  getWarehouses: () => api.get<Warehouse[]>('/api/v1/inventory/warehouses'),
  createWarehouse: (data: CreateWarehouseRequest) => api.post<Warehouse>('/api/v1/inventory/warehouses', data),
  getStock: (productId: string) => api.get(`/api/v1/inventory/stock/${productId}`),
  createStockMovement: (data: CreateStockMovementRequest) => api.post('/api/v1/inventory/stock-movements', data),
};

// Sales
export const sales = {
  getCustomers: (page = 1, perPage = 20) => api.get<Paginated<Customer>>(`/api/v1/sales/customers?page=${page}&per_page=${perPage}`),
  createCustomer: (data: CreateCustomerRequest) => api.post<Customer>('/api/v1/sales/customers', data),
  getOrders: (page = 1, perPage = 20) => api.get<Paginated<SalesOrder>>(`/api/v1/sales/orders?page=${page}&per_page=${perPage}`),
  createOrder: (data: CreateOrderRequest) => api.post<SalesOrder>('/api/v1/sales/orders', data),
  confirmOrder: (id: string) => api.post(`/api/v1/sales/orders/${id}/confirm`),
  getQuotations: (page = 1, perPage = 20) => api.get(`/api/v1/sales/quotations?page=${page}&per_page=${perPage}`),
  createQuotation: (data: CreateQuotationRequest) => api.post('/api/v1/sales/quotations', data),
  sendQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/send`),
  acceptQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/accept`),
  rejectQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/reject`),
  convertQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/convert`),
};

// Purchasing
export const purchasing = {
  getVendors: (page = 1, perPage = 20) => api.get<Paginated<Vendor>>(`/api/v1/purchasing/vendors?page=${page}&per_page=${perPage}`),
  createVendor: (data: CreateVendorRequest) => api.post<Vendor>('/api/v1/purchasing/vendors', data),
  getOrders: (page = 1, perPage = 20) => api.get<Paginated<PurchaseOrder>>(`/api/v1/purchasing/orders?page=${page}&per_page=${perPage}`),
  createOrder: (data: CreatePurchaseOrderRequest) => api.post<PurchaseOrder>('/api/v1/purchasing/orders', data),
  approveOrder: (id: string) => api.post(`/api/v1/purchasing/orders/${id}/approve`),
};

// HR
export const hr = {
  getEmployees: (page = 1, perPage = 20) => api.get<Paginated<Employee>>(`/api/v1/hr/employees?page=${page}&per_page=${perPage}`),
  createEmployee: (data: CreateEmployeeRequest) => api.post<Employee>('/api/v1/hr/employees', data),
  checkIn: (employeeId: string) => api.post('/api/v1/hr/attendance/check-in', { employee_id: employeeId }),
  checkOut: (employeeId: string) => api.post('/api/v1/hr/attendance/check-out', { employee_id: employeeId }),
};

// Audit
export const audit = {
  getLogs: (params?: { entity_type?: string; entity_id?: string; page?: number; per_page?: number }) => {
    const query = new URLSearchParams();
    if (params?.entity_type) query.set('entity_type', params.entity_type);
    if (params?.entity_id) query.set('entity_id', params.entity_id);
    if (params?.page) query.set('page', params.page.toString());
    if (params?.per_page) query.set('per_page', params.per_page.toString());
    const qs = query.toString();
    return api.get(`/api/v1/audit-logs${qs ? '?' + qs : ''}`);
  },
};

// Currency
export const currency = {
  listCurrencies: () => api.get('/api/v1/currencies'),
  setExchangeRate: (data: { from: string; to: string; rate: number }) => api.post('/api/v1/exchange-rates', data),
  convert: (from: string, to: string, amount: number) => api.get(`/api/v1/convert?from=${from}&to=${to}&amount=${amount}`),
};

// Budgets
export const budget = {
  list: () => api.get('/api/v1/budgets'),
  create: (data: CreateBudgetRequest) => api.post('/api/v1/budgets', data),
};

// Lots
export const lot = {
  list: (productId: string) => api.get(`/api/v1/lots?product_id=${productId}`),
  create: (data: CreateLotRequest) => api.post('/api/v1/lots', data),
};

// Leave
export const leave = {
  listTypes: () => api.get('/api/v1/leave-types'),
  listRequests: () => api.get('/api/v1/leave-requests'),
  createRequest: (data: CreateLeaveRequestPayload) => api.post('/api/v1/leave-requests', data),
  approve: (id: string) => api.post(`/api/v1/leave-requests/${id}/approve`),
  reject: (id: string, reason: string) => api.post(`/api/v1/leave-requests/${id}/reject`, { reason }),
};

// Expenses
export const expense = {
  listCategories: () => api.get('/api/v1/expense-categories'),
  listReports: (employeeId?: string) => api.get(`/api/v1/expense-reports${employeeId ? `?employee_id=${employeeId}` : ''}`),
  createReport: (data: CreateExpenseReportRequest) => api.post('/api/v1/expense-reports', data),
  submit: (id: string) => api.post(`/api/v1/expense-reports/${id}/submit`),
  approve: (id: string) => api.post(`/api/v1/expense-reports/${id}/approve`),
  reject: (id: string, reason: string) => api.post(`/api/v1/expense-reports/${id}/reject`, { reason }),
};

// Import/Export
export const data = {
  exportCsv: (entity: string) => api.get(`/api/v1/export?entity=${entity}`, { responseType: 'blob' }),
  importCsv: (entity: string, csvContent: string) => 
    api.post(`/api/v1/import?entity=${entity}`, csvContent, {
      headers: { 'Content-Type': 'text/csv' }
    }),
};

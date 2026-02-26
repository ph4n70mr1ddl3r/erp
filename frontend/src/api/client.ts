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

// Service Desk
export interface Ticket {
  id: string;
  ticket_number: string;
  subject: string;
  description: string;
  customer_id?: string;
  assigned_to?: string;
  priority: string;
  status: string;
  ticket_type: string;
  created_at: string;
}

export interface CreateTicketRequest {
  subject: string;
  description: string;
  customer_id?: string;
  priority?: string;
  ticket_type?: string;
  source?: string;
}

export interface KnowledgeArticle {
  id: string;
  title: string;
  content: string;
  summary?: string;
  status: string;
  view_count: number;
  created_at: string;
}

export interface CreateArticleRequest {
  title: string;
  content: string;
  author_id: string;
  category_id?: string;
  summary?: string;
  tags?: string[];
}

export const service = {
  getTickets: (page = 1, perPage = 20) => api.get<Paginated<Ticket>>(`/api/v1/service/tickets?page=${page}&per_page=${perPage}`),
  getTicket: (id: string) => api.get<Ticket>(`/api/v1/service/tickets/${id}`),
  createTicket: (data: CreateTicketRequest) => api.post<Ticket>('/api/v1/service/tickets', data),
  assignTicket: (id: string, assigneeId: string) => api.post<Ticket>(`/api/v1/service/tickets/${id}/assign`, { assignee_id: assigneeId }),
  updateTicketStatus: (id: string, status: string) => api.post<Ticket>(`/api/v1/service/tickets/${id}/status`, { status }),
  getTicketStats: () => api.get('/api/v1/service/tickets/stats'),
  getArticles: (page = 1, perPage = 20) => api.get<Paginated<KnowledgeArticle>>(`/api/v1/service/articles?page=${page}&per_page=${perPage}`),
  searchArticles: (query: string) => api.get<KnowledgeArticle[]>(`/api/v1/service/articles/search?q=${encodeURIComponent(query)}`),
  getArticle: (id: string) => api.get<KnowledgeArticle>(`/api/v1/service/articles/${id}`),
  createArticle: (data: CreateArticleRequest) => api.post<KnowledgeArticle>('/api/v1/service/articles', data),
  publishArticle: (id: string) => api.post<KnowledgeArticle>(`/api/v1/service/articles/${id}/publish`),
  archiveArticle: (id: string) => api.post<KnowledgeArticle>(`/api/v1/service/articles/${id}/archive`),
};

// IT Assets
export interface ITAsset {
  id: string;
  asset_tag: string;
  name: string;
  description?: string;
  asset_type: string;
  status: string;
  assigned_to?: string;
  created_at: string;
}

export interface CreateITAssetRequest {
  asset_tag: string;
  name: string;
  description?: string;
  asset_type?: string;
  model?: string;
  manufacturer?: string;
  serial_number?: string;
  purchase_date?: string;
  purchase_cost: number;
  currency?: string;
  warranty_expiry?: string;
  location_id?: string;
}

export interface SoftwareLicense {
  id: string;
  product_name: string;
  vendor: string;
  license_type: string;
  seats_purchased: number;
  seats_used: number;
  seats_available: number;
  status: string;
  expiry_date?: string;
}

export interface CreateLicenseRequest {
  license_key: string;
  product_name: string;
  vendor: string;
  license_type?: string;
  seats_purchased: number;
  purchase_cost: number;
  currency?: string;
  purchase_date: string;
  start_date: string;
  expiry_date?: string;
}

export const assets = {
  getAssets: (page = 1, perPage = 20) => api.get<Paginated<ITAsset>>(`/api/v1/assets/assets?page=${page}&per_page=${perPage}`),
  getAsset: (id: string) => api.get<ITAsset>(`/api/v1/assets/assets/${id}`),
  createAsset: (data: CreateITAssetRequest) => api.post<ITAsset>('/api/v1/assets/assets', data),
  assignAsset: (id: string, userId: string, assignedBy: string) => 
    api.post<ITAsset>(`/api/v1/assets/assets/${id}/assign`, { user_id: userId, assigned_by: assignedBy }),
  returnAsset: (id: string, returnedBy: string) => 
    api.post<ITAsset>(`/api/v1/assets/assets/${id}/return`, { returned_by: returnedBy }),
  updateAssetStatus: (id: string, status: string) => 
    api.post<ITAsset>(`/api/v1/assets/assets/${id}/status`, { status }),
  getAssetStats: () => api.get('/api/v1/assets/assets/stats'),
  getLicenses: (page = 1, perPage = 20) => api.get<Paginated<SoftwareLicense>>(`/api/v1/assets/licenses?page=${page}&per_page=${perPage}`),
  getLicense: (id: string) => api.get<SoftwareLicense>(`/api/v1/assets/licenses/${id}`),
  createLicense: (data: CreateLicenseRequest) => api.post<SoftwareLicense>('/api/v1/assets/licenses', data),
  useLicenseSeat: (id: string) => api.post<SoftwareLicense>(`/api/v1/assets/licenses/${id}/use`),
  releaseLicenseSeat: (id: string) => api.post<SoftwareLicense>(`/api/v1/assets/licenses/${id}/release`),
  getExpiringLicenses: (days = 30) => api.get<SoftwareLicense[]>(`/api/v1/assets/licenses/expiring?days=${days}`),
};

export interface DataSubject {
  id: string;
  email: string;
  first_name?: string;
  last_name?: string;
  verification_status: string;
  created_at: string;
}

export interface ConsentRecord {
  id: string;
  data_subject_id: string;
  consent_type: string;
  purpose: string;
  legal_basis: string;
  status: string;
  granted_at?: string;
  withdrawn_at?: string;
}

export interface DSARRequest {
  id: string;
  request_number: string;
  data_subject_id: string;
  request_type: string;
  description?: string;
  status: string;
  due_date: string;
  received_at: string;
}

export interface DataBreach {
  id: string;
  breach_number: string;
  title: string;
  description: string;
  severity: string;
  status: string;
  discovered_at: string;
}

export interface ComplianceStats {
  data_subjects: number;
  active_consents: number;
  pending_dsars: number;
  active_breaches: number;
  active_processors: number;
}

export const compliance = {
  getStats: () => api.get<ComplianceStats>('/api/v1/compliance/stats'),
  getDataSubjects: () => api.get<DataSubject[]>('/api/v1/compliance/data-subjects'),
  createDataSubject: (data: { email: string; first_name?: string; last_name?: string }) =>
    api.post<DataSubject>('/api/v1/compliance/data-subjects', data),
  getConsents: () => api.get<ConsentRecord[]>('/api/v1/compliance/consents'),
  createConsent: (data: { data_subject_id: string; consent_type: string; purpose: string; legal_basis: string }) =>
    api.post<ConsentRecord>('/api/v1/compliance/consents', data),
  withdrawConsent: (id: string) => api.post<ConsentRecord>(`/api/v1/compliance/consents/${id}/withdraw`),
  getDSARs: () => api.get<DSARRequest[]>('/api/v1/compliance/dsars'),
  createDSAR: (data: { data_subject_id: string; request_type: string; description?: string }) =>
    api.post<DSARRequest>('/api/v1/compliance/dsars', data),
  completeDSAR: (id: string, response: string) =>
    api.post<DSARRequest>(`/api/v1/compliance/dsars/${id}/complete`, { response }),
  getBreaches: () => api.get<DataBreach[]>('/api/v1/compliance/breaches'),
  createBreach: (data: { title: string; description: string; breach_type: string; severity: string }) =>
    api.post<DataBreach>('/api/v1/compliance/breaches', data),
};

export interface Project {
  id: string;
  project_number: string;
  name: string;
  description?: string;
  status: string;
  start_date: string;
  end_date?: string;
  budget?: number;
  percent_complete: number;
}

export interface ProjectTask {
  id: string;
  project_id: string;
  name: string;
  description?: string;
  status: string;
  percent_complete: number;
  start_date?: string;
  due_date?: string;
}

export interface ProjectMilestone {
  id: string;
  project_id: string;
  name: string;
  description?: string;
  status: string;
  planned_date?: string;
}

export interface Timesheet {
  id: string;
  timesheet_number: string;
  employee_id: string;
  period_start: string;
  period_end: string;
  total_hours: number;
  status: string;
}

export const projects = {
  getProjects: () => api.get<Project[]>('/api/v1/projects'),
  createProject: (data: { name: string; description?: string; start_date: string; end_date?: string; budget?: number }) =>
    api.post<Project>('/api/v1/projects', data),
  getProject: (id: string) => api.get<Project>(`/api/v1/projects/${id}`),
  updateStatus: (id: string, status: string) =>
    api.post<Project>(`/api/v1/projects/${id}/status`, { status }),
  getTasks: (projectId: string) => api.get<ProjectTask[]>(`/api/v1/projects/${projectId}/tasks`),
  createTask: (data: { project_id: string; name: string; start_date: string }) =>
    api.post<ProjectTask>('/api/v1/projects/tasks', data),
  completeTask: (id: string) => api.post<ProjectTask>(`/api/v1/projects/tasks/${id}/complete`),
  getMilestones: (projectId: string) => api.get<ProjectMilestone[]>(`/api/v1/projects/${projectId}/milestones`),
  createMilestone: (data: { project_id: string; name: string; planned_date: string }) =>
    api.post<ProjectMilestone>('/api/v1/projects/milestones', data),
  completeMilestone: (id: string) => api.post<ProjectMilestone>(`/api/v1/projects/milestones/${id}/complete`),
  getTimesheets: () => api.get<Timesheet[]>('/api/v1/projects/timesheets'),
  createTimesheet: (data: { employee_id: string; period_start: string; period_end: string }) =>
    api.post<Timesheet>('/api/v1/projects/timesheets', data),
  submitTimesheet: (id: string) => api.post(`/api/v1/projects/timesheets/${id}/submit`),
  approveTimesheet: (id: string) => api.post(`/api/v1/projects/timesheets/${id}/approve`),
};

export interface Notification {
  id: string;
  title: string;
  message: string;
  notification_type: string;
  read: boolean;
  entity_type?: string;
  entity_id?: string;
  created_at: string;
}

export const notifications = {
  list: (unreadOnly = false) => api.get<Notification[]>(`/api/v1/notifications${unreadOnly ? '?unread_only=true' : ''}`),
  markRead: (id: string) => api.post(`/api/v1/notifications/${id}/read`),
  markAllRead: () => api.post('/api/v1/notifications/read'),
  unreadCount: () => api.get<{ count: number }>('/api/v1/notifications/unread-count'),
};

// Documents
export const documents = {
  listFolders: (parentId?: string | null) => 
    api.get(`/api/v1/documents/folders${parentId ? `?parent_id=${parentId}` : ''}`),
  createFolder: (data: { name: string; parent_id?: string | null; description?: string }) =>
    api.post('/api/v1/documents/folders', data),
  listDocuments: (folderId?: string | null) =>
    api.get(`/api/v1/documents/documents${folderId ? `?folder_id=${folderId}` : ''}`),
  createDocument: (data: { title: string; file_name: string; file_path: string; file_size: number; mime_type: string; checksum: string; folder_id?: string | null }) =>
    api.post('/api/v1/documents/documents', data),
  checkout: (id: string, userId: string) =>
    api.post(`/api/v1/documents/documents/${id}/checkout`, { user_id: userId }),
  checkin: (checkoutId: string) =>
    api.post('/api/v1/documents/documents/checkin', { checkout_id: checkoutId }),
  requestReview: (documentId: string, version: number, reviewerId: string) =>
    api.post('/api/v1/documents/documents/review', { document_id: documentId, version, reviewer_id: reviewerId }),
  createRetentionPolicy: (data: { name: string; retention_years: number; disposition: string }) =>
    api.post('/api/v1/documents/retention-policies', data),
};

// Pricing
export const pricing = {
  listPriceBooks: () => api.get('/api/v1/pricing/price-books'),
  createPriceBook: (data: { name: string; code: string; currency: string }) =>
    api.post('/api/v1/pricing/price-books', data),
  setProductPrice: (data: { price_book_id: string; product_id: string; unit_price: number; currency: string }) =>
    api.post('/api/v1/pricing/prices', data),
  calculatePrice: (data: { price_book_id: string; product_id: string; quantity: number }) =>
    api.post('/api/v1/pricing/prices/calculate', data),
  listDiscounts: () => api.get('/api/v1/pricing/discounts'),
  createDiscount: (data: { name: string; code: string; discount_type: string; value: number; requires_code: boolean }) =>
    api.post('/api/v1/pricing/discounts', data),
  validateDiscount: (code: string) =>
    api.post('/api/v1/pricing/discounts/validate', { code }),
  createCoupon: (data: { code: string; discount_id: string }) =>
    api.post('/api/v1/pricing/coupons', data),
  listPromotions: () => api.get('/api/v1/pricing/promotions'),
  createPromotion: (data: { name: string; code: string; start_date: string; end_date: string; rules: string; rewards: string }) =>
    api.post('/api/v1/pricing/promotions', data),
  createPriceTier: (data: { product_id: string; min_quantity: number; max_quantity?: number; unit_price: number; currency: string }) =>
    api.post('/api/v1/pricing/price-tiers', data),
};

// Sourcing
export const sourcing = {
  listEvents: () => api.get('/api/v1/sourcing/events'),
  createEvent: (data: { title: string; event_type: string; start_date: string; end_date: string; currency: string; estimated_value: number }) =>
    api.post('/api/v1/sourcing/events', data),
  getEvent: (id: string) => api.get(`/api/v1/sourcing/events/${id}`),
  publishEvent: (id: string) => api.post(`/api/v1/sourcing/events/${id}/publish`),
  addItem: (data: { event_id: string; name: string; quantity: number; unit_of_measure: string; target_price?: number }) =>
    api.post('/api/v1/sourcing/items', data),
  submitBid: (data: { event_id: string; vendor_id: string; total_amount: number; currency: string }) =>
    api.post('/api/v1/sourcing/bids', data),
  listBids: (eventId: string) => api.get(`/api/v1/sourcing/bids/${eventId}`),
  acceptBid: (id: string) => api.post(`/api/v1/sourcing/bids/${id}/accept`),
  awardBid: (data: { event_id: string; bid_id: string; vendor_id: string; total_value: number; currency: string }) =>
    api.post('/api/v1/sourcing/award', data),
  inviteSupplier: (data: { event_id: string; vendor_id: string }) =>
    api.post('/api/v1/sourcing/invite', data),
};

// Config
export const config = {
  listConfigs: (category?: string) =>
    api.get(`/api/v1/config/configs${category ? `?category=${category}` : ''}`),
  setConfig: (data: { category: string; key: string; value: string }) =>
    api.post('/api/v1/config/configs', data),
  getConfig: (category: string, key: string) =>
    api.get(`/api/v1/config/configs/${category}/${key}`),
  deleteConfig: (id: string) => api.delete(`/api/v1/config/configs/${id}`),
  getCompanySettings: () => api.get('/api/v1/config/company'),
  updateCompanySettings: (data: { company_name: string; currency: string; timezone: string }) =>
    api.post('/api/v1/config/company', data),
  createSequence: (data: { name: string; code: string; prefix?: string; padding: number }) =>
    api.post('/api/v1/config/sequences', data),
  getNextNumber: (code: string) => api.get(`/api/v1/config/sequences/${code}/next`),
  getAuditSettings: () => api.get('/api/v1/config/audit'),
  updateAuditSettings: (data: { log_retention_days: number; max_login_attempts: number; require_mfa: boolean }) =>
    api.post('/api/v1/config/audit', data),
  listIntegrations: () => api.get('/api/v1/config/integrations'),
};

// Rules
export const rules = {
  listRules: (entityType?: string) =>
    api.get(`/api/v1/rules/rules${entityType ? `?entity_type=${entityType}` : ''}`),
  createRule: (data: { name: string; code: string; entity_type: string; conditions: string; actions: string; rule_type?: string }) =>
    api.post('/api/v1/rules/rules', data),
  getRule: (id: string) => api.get(`/api/v1/rules/rules/${id}`),
  deleteRule: (id: string) => api.delete(`/api/v1/rules/rules/${id}`),
  executeRules: (data: { entity_type: string; entity_id: string; context: Record<string, unknown> }) =>
    api.post('/api/v1/rules/rules/execute', data),
  listRulesets: () => api.get('/api/v1/rules/rulesets'),
  createRuleset: (data: { name: string; code: string; entity_type: string; execution_mode?: string }) =>
    api.post('/api/v1/rules/rulesets', data),
  addRuleToRuleset: (data: { ruleset_id: string; rule_id: string; sort_order: number }) =>
    api.post('/api/v1/rules/rulesets/add-rule', data),
  createDecisionTable: (data: { name: string; code: string; entity_type: string; input_columns: string; output_columns: string }) =>
    api.post('/api/v1/rules/decision-tables', data),
  addDecisionRow: (data: { table_id: string; row_number: number; inputs: string; outputs: string }) =>
    api.post('/api/v1/rules/decision-tables/rows', data),
  evaluateTable: (data: { table_id: string; inputs: Record<string, unknown> }) =>
    api.post('/api/v1/rules/decision-tables/evaluate', data),
};

export interface ApprovalLevel {
  id: string;
  level_number: number;
  name: string;
  description?: string;
  approver_type: string;
  approver_ids: string[];
  min_approvers: number;
  skip_if_approved_above: boolean;
  due_hours?: number;
  escalation_to?: string;
}

export interface ApprovalWorkflow {
  id: string;
  code: string;
  name: string;
  description?: string;
  document_type: string;
  approval_type: string;
  min_amount?: number;
  max_amount?: number;
  auto_approve_below?: number;
  escalation_hours?: number;
  notify_requester: boolean;
  notify_approver: boolean;
  allow_delegation: boolean;
  allow_reassignment: boolean;
  require_comments: boolean;
  status: string;
  levels: ApprovalLevel[];
  created_at: string;
  updated_at: string;
}

export interface ApprovalRequest {
  id: string;
  request_number: string;
  workflow_id: string;
  document_type: string;
  document_id: string;
  document_number: string;
  requested_by: string;
  requested_at: string;
  amount: number;
  currency: string;
  status: string;
  current_level?: number;
  due_date?: string;
  approved_at?: string;
  approved_by?: string;
  rejected_at?: string;
  rejected_by?: string;
  rejection_reason?: string;
  approvals: {
    id: string;
    level_number: number;
    approver_id: string;
    action: string;
    comments?: string;
    delegated_to?: string;
    created_at: string;
  }[];
}

export interface PendingApprovalSummary {
  user_id: string;
  pending_count: number;
  total_amount: number;
  overdue_count: number;
  by_document_type: {
    document_type: string;
    count: number;
    total_amount: number;
  }[];
}

export const approvalWorkflow = {
  listWorkflows: (page = 1, perPage = 20) =>
    api.get<Paginated<ApprovalWorkflow>>(`/api/v1/approval-workflow/workflows?page=${page}&per_page=${perPage}`),
  getWorkflow: (id: string) =>
    api.get<ApprovalWorkflow>(`/api/v1/approval-workflow/workflows/${id}`),
  createWorkflow: (data: {
    code: string;
    name: string;
    description?: string;
    document_type: string;
    approval_type?: string;
    min_amount?: number;
    max_amount?: number;
    auto_approve_below?: number;
    escalation_hours?: number;
    notify_requester?: boolean;
    notify_approver?: boolean;
    allow_delegation?: boolean;
    allow_reassignment?: boolean;
    require_comments?: boolean;
    levels: {
      name: string;
      description?: string;
      approver_type?: string;
      approver_ids: string[];
      min_approvers?: number;
      skip_if_approved_above?: boolean;
      due_hours?: number;
      escalation_to?: string;
    }[];
  }) => api.post<ApprovalWorkflow>('/api/v1/approval-workflow/workflows', data),
  updateWorkflow: (id: string, data: Partial<{
    name: string;
    description: string;
    approval_type: string;
    min_amount: number;
    max_amount: number;
    auto_approve_below: number;
    escalation_hours: number;
    notify_requester: boolean;
    notify_approver: boolean;
    allow_delegation: boolean;
    allow_reassignment: boolean;
    require_comments: boolean;
    status: string;
    levels: {
      name: string;
      description?: string;
      approver_type?: string;
      approver_ids: string[];
      min_approvers?: number;
      skip_if_approved_above?: boolean;
      due_hours?: number;
      escalation_to?: string;
    }[];
  }>) => api.put<ApprovalWorkflow>(`/api/v1/approval-workflow/workflows/${id}`, data),
  deleteWorkflow: (id: string) =>
    api.delete(`/api/v1/approval-workflow/workflows/${id}`),
  
  listRequests: (page = 1, perPage = 20) =>
    api.get<Paginated<ApprovalRequest>>(`/api/v1/approval-workflow/requests?page=${page}&per_page=${perPage}`),
  getRequest: (id: string) =>
    api.get<ApprovalRequest>(`/api/v1/approval-workflow/requests/${id}`),
  submitForApproval: (data: {
    document_type: string;
    document_id: string;
    document_number: string;
    requested_by: string;
    amount: number;
    currency?: string;
  }) => api.post<ApprovalRequest>('/api/v1/approval-workflow/requests', data),
  approveRequest: (id: string, data: { approver_id: string; comments?: string }) =>
    api.post<ApprovalRequest>(`/api/v1/approval-workflow/requests/${id}/approve`, data),
  rejectRequest: (id: string, data: { approver_id: string; reason: string }) =>
    api.post<ApprovalRequest>(`/api/v1/approval-workflow/requests/${id}/reject`, data),
  cancelRequest: (id: string) =>
    api.post<ApprovalRequest>(`/api/v1/approval-workflow/requests/${id}/cancel`),
  
  getPendingApprovals: (userId: string, page = 1, perPage = 20) =>
    api.get<Paginated<ApprovalRequest>>(`/api/v1/approval-workflow/pending/${userId}?page=${page}&per_page=${perPage}`),
  getPendingSummary: (userId: string) =>
    api.get<PendingApprovalSummary>(`/api/v1/approval-workflow/pending/${userId}/summary`),
};

export const credit = {
  checkCredit: (data: { customer_id: string; order_id?: string; order_amount: number; currency: string }) =>
    api.post('/api/v1/credit/check', data),
  getSummary: () => api.get<CreditSummary>('/api/v1/credit/summary'),
  getProfiles: (page = 1, limit = 20) =>
    api.get<ApiResponse<CreditProfile[]>>(`/api/v1/credit/profiles?page=${page}&limit=${limit}`),
  getOnHold: () => api.get<ApiResponse<CreditProfile[]>>('/api/v1/credit/on-hold'),
  getHighRisk: () => api.get<ApiResponse<CreditProfile[]>>('/api/v1/credit/high-risk'),
  getProfile: (customerId: string) => api.get<CreditProfile>(`/api/v1/credit/${customerId}`),
  updateLimit: (customerId: string, data: { credit_limit: number; reason: string }) =>
    api.post<CreditProfile>(`/api/v1/credit/${customerId}/limit`, data),
  placeHold: (customerId: string, data: { reason: string }) =>
    api.post(`/api/v1/credit/${customerId}/hold`, data),
  releaseHold: (customerId: string, data: { override_reason: string }) =>
    api.post(`/api/v1/credit/${customerId}/release`, data),
  getTransactions: (customerId: string, limit = 50) =>
    api.get<ApiResponse<CreditTransaction[]>>(`/api/v1/credit/${customerId}/transactions?limit=${limit}`),
  getHolds: (customerId: string) =>
    api.get<ApiResponse<CreditHold[]>>(`/api/v1/credit/${customerId}/holds`),
  recordInvoice: (data: { customer_id: string; invoice_id: string; invoice_number: string; amount: number }) =>
    api.post<CreditProfile>('/api/v1/credit/invoice', data),
  recordPayment: (data: { customer_id: string; invoice_id?: string; amount: number }) =>
    api.post<CreditProfile>('/api/v1/credit/payment', data),
};

export interface CreditSummary {
  total_customers: number;
  total_credit_limit: number;
  total_credit_used: number;
  total_available_credit: number;
  total_overdue: number;
  customers_on_hold: number;
  high_risk_customers: number;
  avg_utilization_percent: number;
}

export interface CreditProfile {
  id: string;
  customer_id: string;
  credit_limit: number;
  credit_used: number;
  available_credit: number;
  outstanding_invoices: number;
  pending_orders: number;
  overdue_amount: number;
  overdue_days_avg: number;
  credit_score: number | null;
  risk_level: 'Low' | 'Medium' | 'High' | 'Critical';
  payment_history_score: number | null;
  last_credit_review: string | null;
  next_review_date: string | null;
  auto_hold_enabled: boolean;
  hold_threshold_percent: number;
  status: string;
  utilization_percent: number;
  is_on_hold: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreditTransaction {
  id: string;
  profile_id: string;
  customer_id: string;
  transaction_type: string;
  amount: number;
  previous_credit_used: number;
  new_credit_used: number;
  reference_type: string | null;
  reference_id: string | null;
  reference_number: string | null;
  description: string | null;
  created_by: string | null;
  created_at: string;
}

export interface CreditHold {
  id: string;
  profile_id: string;
  customer_id: string;
  hold_type: string;
  reason: string;
  amount_over_limit: number;
  related_order_id: string | null;
  related_invoice_id: string | null;
  status: string;
  placed_by: string | null;
  placed_at: string;
  released_by: string | null;
  released_at: string | null;
  override_reason: string | null;
  notes: string | null;
  created_at: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T;
}

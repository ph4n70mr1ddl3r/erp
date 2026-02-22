import axios from 'axios';

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

// Auth
export const auth = {
  login: (data: { username: string; password: string }) => api.post('/auth/login', data),
  register: (data: { username: string; email: string; password: string; full_name: string }) => 
    api.post('/auth/register', data),
  me: () => api.get('/auth/me'),
};

// Finance
export const finance = {
  getAccounts: (page = 1, perPage = 20) => api.get(`/api/v1/finance/accounts?page=${page}&per_page=${perPage}`),
  createAccount: (data: any) => api.post('/api/v1/finance/accounts', data),
  getJournalEntries: (page = 1, perPage = 20) => api.get(`/api/v1/finance/journal-entries?page=${page}&per_page=${perPage}`),
  createJournalEntry: (data: any) => api.post('/api/v1/finance/journal-entries', data),
  postJournalEntry: (id: string) => api.post(`/api/v1/finance/journal-entries/${id}/post`),
  getBalanceSheet: () => api.get('/api/v1/finance/reports/balance-sheet'),
  getProfitAndLoss: () => api.get('/api/v1/finance/reports/profit-and-loss'),
  getTrialBalance: () => api.get('/api/v1/finance/reports/trial-balance'),
};

// Inventory
export const inventory = {
  getProducts: (page = 1, perPage = 20) => api.get(`/api/v1/inventory/products?page=${page}&per_page=${perPage}`),
  createProduct: (data: any) => api.post('/api/v1/inventory/products', data),
  updateProduct: (id: string, data: any) => api.put(`/api/v1/inventory/products/${id}`, data),
  deleteProduct: (id: string) => api.delete(`/api/v1/inventory/products/${id}`),
  getWarehouses: () => api.get('/api/v1/inventory/warehouses'),
  createWarehouse: (data: any) => api.post('/api/v1/inventory/warehouses', data),
  getStock: (productId: string) => api.get(`/api/v1/inventory/stock/${productId}`),
  createStockMovement: (data: any) => api.post('/api/v1/inventory/stock-movements', data),
};

// Sales
export const sales = {
  getCustomers: (page = 1, perPage = 20) => api.get(`/api/v1/sales/customers?page=${page}&per_page=${perPage}`),
  createCustomer: (data: any) => api.post('/api/v1/sales/customers', data),
  getOrders: (page = 1, perPage = 20) => api.get(`/api/v1/sales/orders?page=${page}&per_page=${perPage}`),
  createOrder: (data: any) => api.post('/api/v1/sales/orders', data),
  confirmOrder: (id: string) => api.post(`/api/v1/sales/orders/${id}/confirm`),
  getQuotations: (page = 1, perPage = 20) => api.get(`/api/v1/sales/quotations?page=${page}&per_page=${perPage}`),
  createQuotation: (data: any) => api.post('/api/v1/sales/quotations', data),
  sendQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/send`),
  acceptQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/accept`),
  rejectQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/reject`),
  convertQuotation: (id: string) => api.post(`/api/v1/sales/quotations/${id}/convert`),
};

// Purchasing
export const purchasing = {
  getVendors: (page = 1, perPage = 20) => api.get(`/api/v1/purchasing/vendors?page=${page}&per_page=${perPage}`),
  createVendor: (data: any) => api.post('/api/v1/purchasing/vendors', data),
  getOrders: (page = 1, perPage = 20) => api.get(`/api/v1/purchasing/orders?page=${page}&per_page=${perPage}`),
  createOrder: (data: any) => api.post('/api/v1/purchasing/orders', data),
  approveOrder: (id: string) => api.post(`/api/v1/purchasing/orders/${id}/approve`),
};

// HR
export const hr = {
  getEmployees: (page = 1, perPage = 20) => api.get(`/api/v1/hr/employees?page=${page}&per_page=${perPage}`),
  createEmployee: (data: any) => api.post('/api/v1/hr/employees', data),
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

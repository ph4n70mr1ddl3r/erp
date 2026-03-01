import { useEffect, useState } from 'react';
import { sales } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import { getErrorMessage } from '../utils/errors';
import type { Customer, SalesOrder } from '../types';

export default function Sales() {
  const toast = useToast();
  const [customers, setCustomers] = useState<Customer[]>([]);
  const [orders, setOrders] = useState<SalesOrder[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [customerSearch, setCustomerSearch] = useState('');
  const [orderSearch, setOrderSearch] = useState('');
  const [showCustomerModal, setShowCustomerModal] = useState(false);
  const [newCustomer, setNewCustomer] = useState({ code: '', name: '', email: '' });

  useEffect(() => { loadData(); }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [custRes, ordRes] = await Promise.all([sales.getCustomers(1, 50), sales.getOrders(1, 20)]);
      setCustomers(custRes.data.items);
      setOrders(ordRes.data.items);
    } catch {
      toast.error('Failed to load sales data');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateCustomer = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await sales.createCustomer(newCustomer);
      toast.success('Customer created successfully');
      setShowCustomerModal(false);
      setNewCustomer({ code: '', name: '', email: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create customer'));
    } finally {
      setSaving(false);
    }
  };

  const handleConfirmOrder = async (id: string) => {
    try {
      await sales.confirmOrder(id);
      toast.success('Order confirmed successfully');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to confirm order'));
    }
  };

  const filteredCustomers = customers.filter(c =>
    c.code.toLowerCase().includes(customerSearch.toLowerCase()) ||
    c.name.toLowerCase().includes(customerSearch.toLowerCase())
  );

  const filteredOrders = orders.filter(o =>
    o.order_number.toLowerCase().includes(orderSearch.toLowerCase())
  );

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Sales</h1>
        <button onClick={() => setShowCustomerModal(true)} className="btn btn-primary">Add Customer</button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4"><p className="text-sm text-gray-500">Customers</p><p className="text-2xl font-bold">{customers.length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Orders</p><p className="text-2xl font-bold">{orders.length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Pending</p><p className="text-2xl font-bold">{orders.filter(o => o.status === 'Draft').length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Confirmed</p><p className="text-2xl font-bold">{orders.filter(o => o.status !== 'Draft').length}</p></div>
      </div>

      <div className="card mb-6">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Recent Orders</h2>
          <SearchInput value={orderSearch} onChange={setOrderSearch} placeholder="Search orders..." />
        </div>
        {filteredOrders.length === 0 ? (
          <div className="p-8 text-center text-gray-500">{orderSearch ? 'No orders match your search' : 'No orders found'}</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Order #</th>
                <th className="table-header">Customer</th>
                <th className="table-header">Total</th>
                <th className="table-header">Status</th>
                <th className="table-header">Action</th>
              </tr>
            </thead>
            <tbody>
              {filteredOrders.map((o) => (
                <tr key={o.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{o.order_number}</td>
                  <td className="table-cell">{customers.find(c => c.id === o.customer_id)?.name || 'Unknown'}</td>
                  <td className="table-cell">${o.total.toFixed(2)}</td>
                  <td className="table-cell">
                    <span className={`badge ${o.status === 'Draft' ? 'badge-warning' : o.status === 'Approved' || o.status === 'Confirmed' ? 'badge-success' : 'badge-info'}`}>{o.status}</span>
                  </td>
                  <td className="table-cell">
                    {o.status === 'Draft' && (
                      <button onClick={() => handleConfirmOrder(o.id)} className="btn btn-primary text-xs py-1">Confirm</button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      <div className="card">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Customers</h2>
          <SearchInput value={customerSearch} onChange={setCustomerSearch} placeholder="Search customers..." />
        </div>
        {filteredCustomers.length === 0 ? (
          <div className="p-8 text-center text-gray-500">{customerSearch ? 'No customers match your search' : 'No customers found'}</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Code</th>
                <th className="table-header">Name</th>
                <th className="table-header">Email</th>
                <th className="table-header">Status</th>
              </tr>
            </thead>
            <tbody>
              {filteredCustomers.map((c) => (
                <tr key={c.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{c.code}</td>
                  <td className="table-cell">{c.name}</td>
                  <td className="table-cell">{c.email}</td>
                  <td className="table-cell"><span className={`badge ${c.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>{c.status}</span></td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {showCustomerModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Customer</h2>
            <form onSubmit={handleCreateCustomer} className="space-y-4">
              <div>
                <label className="label">Code</label>
                <input className="input" value={newCustomer.code} onChange={(e) => setNewCustomer({ ...newCustomer, code: e.target.value })} required />
              </div>
              <div>
                <label className="label">Name</label>
                <input className="input" value={newCustomer.name} onChange={(e) => setNewCustomer({ ...newCustomer, name: e.target.value })} required />
              </div>
              <div>
                <label className="label">Email</label>
                <input type="email" className="input" value={newCustomer.email} onChange={(e) => setNewCustomer({ ...newCustomer, email: e.target.value })} />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowCustomerModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

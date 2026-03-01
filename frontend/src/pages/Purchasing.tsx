import { useEffect, useState } from 'react';
import { purchasing } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import { getErrorMessage } from '../utils/errors';
import type { Vendor, PurchaseOrder } from '../types';

export default function Purchasing() {
  const toast = useToast();
  const [vendors, setVendors] = useState<Vendor[]>([]);
  const [orders, setOrders] = useState<PurchaseOrder[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [vendorSearch, setVendorSearch] = useState('');
  const [orderSearch, setOrderSearch] = useState('');
  const [showVendorModal, setShowVendorModal] = useState(false);
  const [newVendor, setNewVendor] = useState({ code: '', name: '', email: '' });

  useEffect(() => { loadData(); }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [venRes, ordRes] = await Promise.all([purchasing.getVendors(1, 50), purchasing.getOrders(1, 20)]);
      setVendors(venRes.data.items);
      setOrders(ordRes.data.items);
    } catch (err) {
      toast.error('Failed to load purchasing data');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateVendor = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await purchasing.createVendor(newVendor);
      toast.success('Vendor created successfully');
      setShowVendorModal(false);
      setNewVendor({ code: '', name: '', email: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create vendor'));
    } finally {
      setSaving(false);
    }
  };

  const handleApproveOrder = async (id: string) => {
    try {
      await purchasing.approveOrder(id);
      toast.success('Order approved successfully');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to approve order'));
    }
  };

  const filteredVendors = vendors.filter(v =>
    v.code.toLowerCase().includes(vendorSearch.toLowerCase()) ||
    v.name.toLowerCase().includes(vendorSearch.toLowerCase())
  );

  const filteredOrders = orders.filter(o =>
    o.po_number.toLowerCase().includes(orderSearch.toLowerCase())
  );

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Purchasing</h1>
        <button onClick={() => setShowVendorModal(true)} className="btn btn-primary">Add Vendor</button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4"><p className="text-sm text-gray-500">Vendors</p><p className="text-2xl font-bold">{vendors.length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Purchase Orders</p><p className="text-2xl font-bold">{orders.length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Pending</p><p className="text-2xl font-bold">{orders.filter(o => o.status === 'Draft').length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Approved</p><p className="text-2xl font-bold">{orders.filter(o => o.status === 'Approved').length}</p></div>
      </div>

      <div className="card mb-6">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Purchase Orders</h2>
          <SearchInput value={orderSearch} onChange={setOrderSearch} placeholder="Search orders..." />
        </div>
        {filteredOrders.length === 0 ? (
          <div className="p-8 text-center text-gray-500">{orderSearch ? 'No orders match your search' : 'No orders found'}</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">PO #</th>
                <th className="table-header">Vendor</th>
                <th className="table-header">Total</th>
                <th className="table-header">Status</th>
                <th className="table-header">Action</th>
              </tr>
            </thead>
            <tbody>
              {filteredOrders.map((o) => (
                <tr key={o.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{o.po_number}</td>
                  <td className="table-cell">{vendors.find(v => v.id === o.vendor_id)?.name || 'Unknown'}</td>
                  <td className="table-cell">${o.total.toFixed(2)}</td>
                  <td className="table-cell">
                    <span className={`badge ${o.status === 'Draft' ? 'badge-warning' : 'badge-success'}`}>{o.status}</span>
                  </td>
                  <td className="table-cell">
                    {o.status === 'Draft' && (
                      <button onClick={() => handleApproveOrder(o.id)} className="btn btn-primary text-xs py-1">Approve</button>
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
          <h2 className="text-lg font-semibold">Vendors</h2>
          <SearchInput value={vendorSearch} onChange={setVendorSearch} placeholder="Search vendors..." />
        </div>
        {filteredVendors.length === 0 ? (
          <div className="p-8 text-center text-gray-500">{vendorSearch ? 'No vendors match your search' : 'No vendors found'}</div>
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
              {filteredVendors.map((v) => (
                <tr key={v.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{v.code}</td>
                  <td className="table-cell">{v.name}</td>
                  <td className="table-cell">{v.email}</td>
                  <td className="table-cell"><span className={`badge ${v.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>{v.status}</span></td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {showVendorModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Vendor</h2>
            <form onSubmit={handleCreateVendor} className="space-y-4">
              <div>
                <label className="label">Code</label>
                <input className="input" value={newVendor.code} onChange={(e) => setNewVendor({ ...newVendor, code: e.target.value })} required />
              </div>
              <div>
                <label className="label">Name</label>
                <input className="input" value={newVendor.name} onChange={(e) => setNewVendor({ ...newVendor, name: e.target.value })} required />
              </div>
              <div>
                <label className="label">Email</label>
                <input type="email" className="input" value={newVendor.email} onChange={(e) => setNewVendor({ ...newVendor, email: e.target.value })} />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowVendorModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

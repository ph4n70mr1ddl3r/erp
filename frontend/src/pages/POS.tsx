import { useState, useEffect } from 'react';
import { CreditCard, Store, Ticket, TrendingUp } from 'lucide-react';
import api from '../api/client';

interface POSStore {
  id: string;
  store_code: string;
  name: string;
  city: string;
  status: string;
}

interface POSTransaction {
  id: string;
  transaction_number: string;
  store_id: string;
  transaction_type: string;
  total: number;
  status: string;
}

export default function POS() {
  const [stores, setStores] = useState<POSStore[]>([]);
  const [transactions, setTransactions] = useState<POSTransaction[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<'stores' | 'transactions' | 'giftcards'>('stores');
  const [showStoreForm, setShowStoreForm] = useState(false);
  const [storeForm, setStoreForm] = useState({
    store_code: '',
    name: '',
    address: '',
    city: '',
    state: '',
    postal_code: '',
    country: 'USA',
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [storesRes, txRes] = await Promise.all([
        api.get('/pos/stores'),
        api.get('/pos/transactions'),
      ]);
      setStores(storesRes.data.items || []);
      setTransactions(txRes.data.items || []);
    } catch (err) {
      console.error('Failed to load POS data:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleStoreSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await api.post('/pos/stores', storeForm);
      setStoreForm({ store_code: '', name: '', address: '', city: '', state: '', postal_code: '', country: 'USA' });
      setShowStoreForm(false);
      loadData();
    } catch (err) {
      console.error('Failed to create store:', err);
    }
  };

  const statusColors: Record<string, string> = {
    Active: 'bg-green-100 text-green-800',
    Inactive: 'bg-gray-100 text-gray-800',
    Maintenance: 'bg-yellow-100 text-yellow-800',
  };

  if (loading) {
    return <div className="flex justify-center items-center h-64"><div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div></div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Point of Sale</h1>
          <p className="text-gray-500 mt-1">Manage retail stores, transactions, and gift cards</p>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Total Stores</p>
              <p className="text-2xl font-bold">{stores.length}</p>
            </div>
            <Store className="w-8 h-8 text-blue-500" />
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Transactions</p>
              <p className="text-2xl font-bold">{transactions.length}</p>
            </div>
            <CreditCard className="w-8 h-8 text-green-500" />
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Today's Sales</p>
              <p className="text-2xl font-bold">
                ${transactions.reduce((sum, t) => sum + t.total, 0).toFixed(2)}
              </p>
            </div>
            <TrendingUp className="w-8 h-8 text-purple-500" />
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Gift Cards</p>
              <p className="text-2xl font-bold">0</p>
            </div>
            <Ticket className="w-8 h-8 text-orange-500" />
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          {[
            { id: 'stores', label: 'Stores' },
            { id: 'transactions', label: 'Transactions' },
            { id: 'giftcards', label: 'Gift Cards' },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as typeof activeTab)}
              className={`py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      {/* Content */}
      {activeTab === 'stores' && (
        <div className="bg-white rounded-lg shadow">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Retail Stores</h2>
            <button
              onClick={() => setShowStoreForm(true)}
              className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700"
            >
              Add Store
            </button>
          </div>
          
          {showStoreForm && (
            <div className="p-4 border-b bg-gray-50">
              <form onSubmit={handleStoreSubmit} className="grid grid-cols-2 gap-4">
                <input
                  type="text"
                  placeholder="Store Code"
                  value={storeForm.store_code}
                  onChange={(e) => setStoreForm({ ...storeForm, store_code: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                  required
                />
                <input
                  type="text"
                  placeholder="Store Name"
                  value={storeForm.name}
                  onChange={(e) => setStoreForm({ ...storeForm, name: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                  required
                />
                <input
                  type="text"
                  placeholder="Address"
                  value={storeForm.address}
                  onChange={(e) => setStoreForm({ ...storeForm, address: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                  required
                />
                <input
                  type="text"
                  placeholder="City"
                  value={storeForm.city}
                  onChange={(e) => setStoreForm({ ...storeForm, city: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                  required
                />
                <div className="col-span-2 flex gap-2">
                  <button type="submit" className="bg-blue-600 text-white px-4 py-2 rounded-lg">
                    Create Store
                  </button>
                  <button
                    type="button"
                    onClick={() => setShowStoreForm(false)}
                    className="bg-gray-300 text-gray-700 px-4 py-2 rounded-lg"
                  >
                    Cancel
                  </button>
                </div>
              </form>
            </div>
          )}

          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Code</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">City</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {stores.map((store) => (
                <tr key={store.id}>
                  <td className="px-6 py-4 whitespace-nowrap font-medium">{store.store_code}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{store.name}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{store.city}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${statusColors[store.status] || 'bg-gray-100'}`}>
                      {store.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'transactions' && (
        <div className="bg-white rounded-lg shadow">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Recent Transactions</h2>
          </div>
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Number</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Type</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Total</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {transactions.map((tx) => (
                <tr key={tx.id}>
                  <td className="px-6 py-4 whitespace-nowrap font-medium">{tx.transaction_number}</td>
                  <td className="px-6 py-4 whitespace-nowrap">{tx.transaction_type}</td>
                  <td className="px-6 py-4 whitespace-nowrap">${tx.total.toFixed(2)}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${statusColors[tx.status] || 'bg-green-100 text-green-800'}`}>
                      {tx.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'giftcards' && (
        <div className="bg-white rounded-lg shadow p-6">
          <p className="text-gray-500 text-center">Gift card management coming soon...</p>
        </div>
      )}
    </div>
  );
}

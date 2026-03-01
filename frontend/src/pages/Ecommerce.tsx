import { useState, useEffect } from 'react';
import { ShoppingBag, Link, Package, Truck, RefreshCw } from 'lucide-react';
import api from '../api/client';
import { useToast } from '../components/Toast';
import { getErrorMessage } from '../utils/errors';

interface EcommercePlatform {
  id: string;
  name: string;
  platform_type: string;
  status: string;
  last_sync_at: string | null;
}

interface EcommerceOrder {
  id: string;
  order_number: string;
  platform_id: string;
  status: string;
  total: number;
  sync_status: string;
}

export default function Ecommerce() {
  const toast = useToast();
  const [platforms, setPlatforms] = useState<EcommercePlatform[]>([]);
  const [orders, setOrders] = useState<EcommerceOrder[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<'platforms' | 'orders' | 'listings'>('platforms');
  const [showPlatformForm, setShowPlatformForm] = useState(false);
  const [platformForm, setPlatformForm] = useState({
    name: '',
    platform_type: 'Shopify',
    base_url: '',
    api_key: '',
    api_secret: '',
    store_id: '',
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [platformsRes, ordersRes] = await Promise.all([
        api.get('/ecommerce/platforms'),
        api.get('/ecommerce/orders'),
      ]);
      setPlatforms(platformsRes.data.items || []);
      setOrders(ordersRes.data.items || []);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load e-commerce data'));
    } finally {
      setLoading(false);
    }
  };

  const handlePlatformSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await api.post('/ecommerce/platforms', platformForm);
      setPlatformForm({ name: '', platform_type: 'Shopify', base_url: '', api_key: '', api_secret: '', store_id: '' });
      setShowPlatformForm(false);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create platform'));
    }
  };

  const platformTypeColors: Record<string, string> = {
    Shopify: 'bg-green-100 text-green-800',
    WooCommerce: 'bg-purple-100 text-purple-800',
    Magento: 'bg-orange-100 text-orange-800',
    Amazon: 'bg-yellow-100 text-yellow-800',
    EBay: 'bg-blue-100 text-blue-800',
    BigCommerce: 'bg-indigo-100 text-indigo-800',
  };

  const statusColors: Record<string, string> = {
    Active: 'bg-green-100 text-green-800',
    Pending: 'bg-yellow-100 text-yellow-800',
    Synced: 'bg-blue-100 text-blue-800',
    Failed: 'bg-red-100 text-red-800',
  };

  if (loading) {
    return <div className="flex justify-center items-center h-64"><div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div></div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">E-Commerce Integration</h1>
          <p className="text-gray-500 mt-1">Connect and sync with online marketplaces</p>
        </div>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Platforms</p>
              <p className="text-2xl font-bold">{platforms.length}</p>
            </div>
            <Link className="w-8 h-8 text-blue-500" />
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Imported Orders</p>
              <p className="text-2xl font-bold">{orders.length}</p>
            </div>
            <ShoppingBag className="w-8 h-8 text-green-500" />
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Pending Sync</p>
              <p className="text-2xl font-bold">{orders.filter(o => o.sync_status === 'Pending').length}</p>
            </div>
            <RefreshCw className="w-8 h-8 text-orange-500" />
          </div>
        </div>
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Total Sales</p>
              <p className="text-2xl font-bold">${orders.reduce((sum, o) => sum + o.total, 0).toFixed(2)}</p>
            </div>
            <Truck className="w-8 h-8 text-purple-500" />
          </div>
        </div>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          {[
            { id: 'platforms', label: 'Platforms' },
            { id: 'orders', label: 'Orders' },
            { id: 'listings', label: 'Listings' },
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
      {activeTab === 'platforms' && (
        <div className="bg-white rounded-lg shadow">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Connected Platforms</h2>
            <button
              onClick={() => setShowPlatformForm(true)}
              className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700"
            >
              Add Platform
            </button>
          </div>
          
          {showPlatformForm && (
            <div className="p-4 border-b bg-gray-50">
              <form onSubmit={handlePlatformSubmit} className="grid grid-cols-2 gap-4">
                <input
                  type="text"
                  placeholder="Platform Name"
                  value={platformForm.name}
                  onChange={(e) => setPlatformForm({ ...platformForm, name: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                  required
                />
                <select
                  value={platformForm.platform_type}
                  onChange={(e) => setPlatformForm({ ...platformForm, platform_type: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                >
                  <option value="Shopify">Shopify</option>
                  <option value="WooCommerce">WooCommerce</option>
                  <option value="Magento">Magento</option>
                  <option value="BigCommerce">BigCommerce</option>
                  <option value="Amazon">Amazon</option>
                  <option value="EBay">eBay</option>
                </select>
                <input
                  type="url"
                  placeholder="Base URL (e.g., https://myshop.myshopify.com)"
                  value={platformForm.base_url}
                  onChange={(e) => setPlatformForm({ ...platformForm, base_url: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                  required
                />
                <input
                  type="text"
                  placeholder="Store ID"
                  value={platformForm.store_id}
                  onChange={(e) => setPlatformForm({ ...platformForm, store_id: e.target.value })}
                  className="border rounded-lg px-3 py-2"
                />
                <div className="col-span-2 flex gap-2">
                  <button type="submit" className="bg-blue-600 text-white px-4 py-2 rounded-lg">
                    Connect Platform
                  </button>
                  <button
                    type="button"
                    onClick={() => setShowPlatformForm(false)}
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
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Type</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Last Sync</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {platforms.map((platform) => (
                <tr key={platform.id}>
                  <td className="px-6 py-4 whitespace-nowrap font-medium">{platform.name}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${platformTypeColors[platform.platform_type] || 'bg-gray-100'}`}>
                      {platform.platform_type}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${statusColors[platform.status] || 'bg-gray-100'}`}>
                      {platform.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-gray-500">
                    {platform.last_sync_at ? new Date(platform.last_sync_at).toLocaleString() : 'Never'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'orders' && (
        <div className="bg-white rounded-lg shadow">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Imported Orders</h2>
          </div>
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Order #</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Total</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Sync</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {orders.map((order) => (
                <tr key={order.id}>
                  <td className="px-6 py-4 whitespace-nowrap font-medium">{order.order_number}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${statusColors[order.status] || 'bg-gray-100'}`}>
                      {order.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">${order.total.toFixed(2)}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${statusColors[order.sync_status] || 'bg-gray-100'}`}>
                      {order.sync_status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'listings' && (
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center gap-3 text-gray-500">
            <Package className="w-8 h-8" />
            <p>Product listing sync coming soon...</p>
          </div>
        </div>
      )}
    </div>
  );
}

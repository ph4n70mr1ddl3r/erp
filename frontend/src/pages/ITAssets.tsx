import { useState, useEffect } from 'react';
import { assets, type ITAsset, type SoftwareLicense, type CreateITAssetRequest, type CreateLicenseRequest } from '../api/client';

const assetTypeColors: Record<string, string> = {
  Hardware: 'bg-blue-100 text-blue-800',
  Software: 'bg-purple-100 text-purple-800',
  Network: 'bg-green-100 text-green-800',
  Server: 'bg-orange-100 text-orange-800',
  Mobile: 'bg-pink-100 text-pink-800',
};

const assetStatusColors: Record<string, string> = {
  Available: 'bg-green-100 text-green-800',
  InUse: 'bg-blue-100 text-blue-800',
  InMaintenance: 'bg-yellow-100 text-yellow-800',
  Reserved: 'bg-purple-100 text-purple-800',
  Retired: 'bg-gray-100 text-gray-800',
};

export default function ITAssets() {
  const [assetsList, setAssetsList] = useState<ITAsset[]>([]);
  const [licenses, setLicenses] = useState<SoftwareLicense[]>([]);
  const [assetStats, setAssetStats] = useState<Record<string, number>>({});
  const [loading, setLoading] = useState(true);
  const [showAssetForm, setShowAssetForm] = useState(false);
  const [showLicenseForm, setShowLicenseForm] = useState(false);
  const [activeTab, setActiveTab] = useState<'assets' | 'licenses'>('assets');
  const [assetForm, setAssetForm] = useState<CreateITAssetRequest>({
    asset_tag: '',
    name: '',
    asset_type: 'Hardware',
    purchase_cost: 0,
    currency: 'USD',
  });
  const [licenseForm, setLicenseForm] = useState<CreateLicenseRequest>({
    license_key: '',
    product_name: '',
    vendor: '',
    seats_purchased: 1,
    purchase_cost: 0,
    currency: 'USD',
    purchase_date: new Date().toISOString().split('T')[0],
    start_date: new Date().toISOString().split('T')[0],
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [assetsRes, licensesRes, statsRes] = await Promise.all([
        assets.getAssets(),
        assets.getLicenses(),
        assets.getAssetStats(),
      ]);
      setAssetsList(assetsRes.data.items || []);
      setLicenses(licensesRes.data.items || []);
      setAssetStats(statsRes.data.by_status || {});
    } catch (err) {
      console.error('Failed to load data:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleAssetSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await assets.createAsset(assetForm);
      setAssetForm({
        asset_tag: '',
        name: '',
        asset_type: 'Hardware',
        purchase_cost: 0,
        currency: 'USD',
      });
      setShowAssetForm(false);
      loadData();
    } catch (err) {
      console.error('Failed to create asset:', err);
    }
  };

  const handleLicenseSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await assets.createLicense(licenseForm);
      setLicenseForm({
        license_key: '',
        product_name: '',
        vendor: '',
        seats_purchased: 1,
        purchase_cost: 0,
        currency: 'USD',
        purchase_date: new Date().toISOString().split('T')[0],
        start_date: new Date().toISOString().split('T')[0],
      });
      setShowLicenseForm(false);
      loadData();
    } catch (err) {
      console.error('Failed to create license:', err);
    }
  };

  const updateAssetStatus = async (id: string, status: string) => {
    try {
      await assets.updateAssetStatus(id, status);
      loadData();
    } catch (err) {
      console.error('Failed to update status:', err);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-900">IT Asset Management</h1>
        <div className="space-x-2">
          <button
            onClick={() => setShowAssetForm(true)}
            className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700"
          >
            New Asset
          </button>
          <button
            onClick={() => setShowLicenseForm(true)}
            className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
          >
            New License
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
        <div className="bg-white p-4 rounded-lg shadow">
          <div className="text-2xl font-bold text-green-600">{assetStats['"Available"'] || 0}</div>
          <div className="text-sm text-gray-500">Available</div>
        </div>
        <div className="bg-white p-4 rounded-lg shadow">
          <div className="text-2xl font-bold text-blue-600">{assetStats['"InUse"'] || 0}</div>
          <div className="text-sm text-gray-500">In Use</div>
        </div>
        <div className="bg-white p-4 rounded-lg shadow">
          <div className="text-2xl font-bold text-yellow-600">{assetStats['"InMaintenance"'] || 0}</div>
          <div className="text-sm text-gray-500">In Maintenance</div>
        </div>
        <div className="bg-white p-4 rounded-lg shadow">
          <div className="text-2xl font-bold text-purple-600">{assetStats['"Reserved"'] || 0}</div>
          <div className="text-sm text-gray-500">Reserved</div>
        </div>
        <div className="bg-white p-4 rounded-lg shadow">
          <div className="text-2xl font-bold text-gray-600">{assetStats['"Retired"'] || 0}</div>
          <div className="text-sm text-gray-500">Retired</div>
        </div>
      </div>

      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          <button
            onClick={() => setActiveTab('assets')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'assets'
                ? 'border-indigo-500 text-indigo-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            Assets
          </button>
          <button
            onClick={() => setActiveTab('licenses')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'licenses'
                ? 'border-indigo-500 text-indigo-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            Software Licenses
          </button>
        </nav>
      </div>

      {activeTab === 'assets' && (
        <div className="bg-white shadow rounded-lg overflow-hidden">
          {loading ? (
            <div className="p-8 text-center text-gray-500">Loading...</div>
          ) : assetsList.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No assets found</div>
          ) : (
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Asset Tag</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Type</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Assigned To</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Actions</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {assetsList.map((asset) => (
                  <tr key={asset.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-indigo-600">
                      {asset.asset_tag}
                    </td>
                    <td className="px-6 py-4 text-sm text-gray-900">
                      {asset.name}
                      {asset.description && (
                        <p className="text-xs text-gray-500 truncate max-w-xs">{asset.description}</p>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${assetTypeColors[asset.asset_type] || 'bg-gray-100'}`}>
                        {asset.asset_type}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${assetStatusColors[asset.status] || 'bg-gray-100'}`}>
                        {asset.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {asset.assigned_to || '-'}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {asset.status === 'Available' && (
                        <button
                          onClick={() => updateAssetStatus(asset.id, 'InUse')}
                          className="text-blue-600 hover:text-blue-800 mr-3"
                        >
                          Deploy
                        </button>
                      )}
                      {asset.status === 'InUse' && (
                        <>
                          <button
                            onClick={() => updateAssetStatus(asset.id, 'InMaintenance')}
                            className="text-yellow-600 hover:text-yellow-800 mr-3"
                          >
                            Maintenance
                          </button>
                          <button
                            onClick={() => updateAssetStatus(asset.id, 'Available')}
                            className="text-green-600 hover:text-green-800"
                          >
                            Return
                          </button>
                        </>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {activeTab === 'licenses' && (
        <div className="bg-white shadow rounded-lg overflow-hidden">
          {loading ? (
            <div className="p-8 text-center text-gray-500">Loading...</div>
          ) : licenses.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No licenses found</div>
          ) : (
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Product</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Vendor</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Type</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Seats</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Expiry</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Actions</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {licenses.map((license) => (
                  <tr key={license.id} className="hover:bg-gray-50">
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                      {license.product_name}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {license.vendor}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {license.license_type}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      <span className="text-green-600">{license.seats_available}</span>
                      <span className="text-gray-400"> / </span>
                      <span className="text-gray-600">{license.seats_purchased}</span>
                      <div className="w-24 h-2 bg-gray-200 rounded-full mt-1">
                        <div
                          className="h-full bg-green-500 rounded-full"
                          style={{ width: `${(license.seats_available / license.seats_purchased) * 100}%` }}
                        />
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${
                        license.status === 'Active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
                      }`}>
                        {license.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {license.expiry_date ? new Date(license.expiry_date).toLocaleDateString() : 'Never'}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {license.seats_available > 0 && (
                        <button
                          onClick={async () => {
                            await assets.useLicenseSeat(license.id);
                            loadData();
                          }}
                          className="text-blue-600 hover:text-blue-800"
                        >
                          Use Seat
                        </button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {showAssetForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">New IT Asset</h2>
            <form onSubmit={handleAssetSubmit} className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Asset Tag *</label>
                  <input
                    type="text"
                    value={assetForm.asset_tag}
                    onChange={(e) => setAssetForm({ ...assetForm, asset_tag: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Asset Type</label>
                  <select
                    value={assetForm.asset_type}
                    onChange={(e) => setAssetForm({ ...assetForm, asset_type: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  >
                    <option value="Hardware">Hardware</option>
                    <option value="Software">Software</option>
                    <option value="Network">Network</option>
                    <option value="Server">Server</option>
                    <option value="Mobile">Mobile</option>
                    <option value="Peripheral">Peripheral</option>
                    <option value="Storage">Storage</option>
                    <option value="Security">Security</option>
                  </select>
                </div>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Name *</label>
                <input
                  type="text"
                  value={assetForm.name}
                  onChange={(e) => setAssetForm({ ...assetForm, name: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                  value={assetForm.description || ''}
                  onChange={(e) => setAssetForm({ ...assetForm, description: e.target.value })}
                  rows={2}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Purchase Cost *</label>
                  <input
                    type="number"
                    value={assetForm.purchase_cost}
                    onChange={(e) => setAssetForm({ ...assetForm, purchase_cost: parseInt(e.target.value) || 0 })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Currency</label>
                  <select
                    value={assetForm.currency}
                    onChange={(e) => setAssetForm({ ...assetForm, currency: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  >
                    <option value="USD">USD</option>
                    <option value="EUR">EUR</option>
                    <option value="GBP">GBP</option>
                  </select>
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Model</label>
                  <input
                    type="text"
                    value={assetForm.model || ''}
                    onChange={(e) => setAssetForm({ ...assetForm, model: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Serial Number</label>
                  <input
                    type="text"
                    value={assetForm.serial_number || ''}
                    onChange={(e) => setAssetForm({ ...assetForm, serial_number: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  />
                </div>
              </div>
              <div className="flex justify-end gap-3 pt-4">
                <button
                  type="button"
                  onClick={() => setShowAssetForm(false)}
                  className="px-4 py-2 text-gray-700 hover:text-gray-900"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="bg-indigo-600 text-white px-4 py-2 rounded-md hover:bg-indigo-700"
                >
                  Create Asset
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showLicenseForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-lg max-h-[90vh] overflow-y-auto">
            <h2 className="text-lg font-semibold mb-4">New Software License</h2>
            <form onSubmit={handleLicenseSubmit} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Product Name *</label>
                <input
                  type="text"
                  value={licenseForm.product_name}
                  onChange={(e) => setLicenseForm({ ...licenseForm, product_name: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  required
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Vendor *</label>
                  <input
                    type="text"
                    value={licenseForm.vendor}
                    onChange={(e) => setLicenseForm({ ...licenseForm, vendor: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">License Type</label>
                  <select
                    value={licenseForm.license_type || 'Perpetual'}
                    onChange={(e) => setLicenseForm({ ...licenseForm, license_type: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  >
                    <option value="Perpetual">Perpetual</option>
                    <option value="Subscription">Subscription</option>
                    <option value="Volume">Volume</option>
                    <option value="Site">Site</option>
                    <option value="NamedUser">Named User</option>
                    <option value="Concurrent">Concurrent</option>
                  </select>
                </div>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">License Key *</label>
                <input
                  type="text"
                  value={licenseForm.license_key}
                  onChange={(e) => setLicenseForm({ ...licenseForm, license_key: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                  required
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Seats Purchased *</label>
                  <input
                    type="number"
                    value={licenseForm.seats_purchased}
                    onChange={(e) => setLicenseForm({ ...licenseForm, seats_purchased: parseInt(e.target.value) || 1 })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    min="1"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Purchase Cost *</label>
                  <input
                    type="number"
                    value={licenseForm.purchase_cost}
                    onChange={(e) => setLicenseForm({ ...licenseForm, purchase_cost: parseInt(e.target.value) || 0 })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Purchase Date *</label>
                  <input
                    type="date"
                    value={licenseForm.purchase_date}
                    onChange={(e) => setLicenseForm({ ...licenseForm, purchase_date: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Start Date *</label>
                  <input
                    type="date"
                    value={licenseForm.start_date}
                    onChange={(e) => setLicenseForm({ ...licenseForm, start_date: e.target.value })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                    required
                  />
                </div>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Expiry Date</label>
                <input
                  type="date"
                  value={licenseForm.expiry_date || ''}
                  onChange={(e) => setLicenseForm({ ...licenseForm, expiry_date: e.target.value || undefined })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-indigo-500 focus:border-indigo-500"
                />
              </div>
              <div className="flex justify-end gap-3 pt-4">
                <button
                  type="button"
                  onClick={() => setShowLicenseForm(false)}
                  className="px-4 py-2 text-gray-700 hover:text-gray-900"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700"
                >
                  Create License
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

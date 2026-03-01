import { useEffect, useState } from 'react';
import { credit } from '../api/client';
import type { CreditProfile, CreditSummary, CreditTransaction, CreditHold } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { getErrorMessage } from '../utils/errors';

function formatCurrency(cents: number): string {
  return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(cents / 100);
}

function getRiskBadgeColor(level: string): string {
  switch (level) {
    case 'Low': return 'bg-green-100 text-green-800';
    case 'Medium': return 'bg-yellow-100 text-yellow-800';
    case 'High': return 'bg-orange-100 text-orange-800';
    case 'Critical': return 'bg-red-100 text-red-800';
    default: return 'bg-gray-100 text-gray-800';
  }
}

export default function CreditManagement() {
  const toast = useToast();
  const [loading, setLoading] = useState(true);
  const [summary, setSummary] = useState<CreditSummary | null>(null);
  const [profiles, setProfiles] = useState<CreditProfile[]>([]);
  const [onHold, setOnHold] = useState<CreditProfile[]>([]);
  const [highRisk, setHighRisk] = useState<CreditProfile[]>([]);
  const [selectedProfile, setSelectedProfile] = useState<CreditProfile | null>(null);
  const [transactions, setTransactions] = useState<CreditTransaction[]>([]);
  const [holds, setHolds] = useState<CreditHold[]>([]);
  const [activeTab, setActiveTab] = useState<'all' | 'onhold' | 'highrisk'>('all');
  
  const [showLimitModal, setShowLimitModal] = useState(false);
  const [showHoldModal, setShowHoldModal] = useState(false);
  const [showReleaseModal, setShowReleaseModal] = useState(false);
  const [newLimit, setNewLimit] = useState(0);
  const [limitReason, setLimitReason] = useState('');
  const [holdReason, setHoldReason] = useState('');
  const [releaseReason, setReleaseReason] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [summaryRes, profilesRes, onHoldRes, highRiskRes] = await Promise.all([
        credit.getSummary(),
        credit.getProfiles(1, 50),
        credit.getOnHold(),
        credit.getHighRisk(),
      ]);
      setSummary(summaryRes.data);
      setProfiles(profilesRes.data.data || []);
      setOnHold(onHoldRes.data.data || []);
      setHighRisk(highRiskRes.data.data || []);
    } catch {
      toast.error('Failed to load credit data');
    } finally {
      setLoading(false);
    }
  };

  const loadProfileDetails = async (profile: CreditProfile) => {
    try {
      setSelectedProfile(profile);
      const [txnsRes, holdsRes] = await Promise.all([
        credit.getTransactions(profile.customer_id, 20),
        credit.getHolds(profile.customer_id),
      ]);
      setTransactions(txnsRes.data.data || []);
      setHolds(holdsRes.data.data || []);
    } catch {
      toast.error('Failed to load profile details');
    }
  };

  const handleUpdateLimit = async () => {
    if (!selectedProfile || !limitReason.trim()) {
      toast.error('Please provide a reason for the limit change');
      return;
    }
    try {
      setSaving(true);
      await credit.updateLimit(selectedProfile.customer_id, {
        credit_limit: newLimit,
        reason: limitReason,
      });
      toast.success('Credit limit updated successfully');
      setShowLimitModal(false);
      setLimitReason('');
      loadData();
      if (selectedProfile) {
        const updated = profiles.find(p => p.customer_id === selectedProfile.customer_id);
        if (updated) loadProfileDetails({ ...updated, credit_limit: newLimit });
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to update credit limit'));
    } finally {
      setSaving(false);
    }
  };

  const handlePlaceHold = async () => {
    if (!selectedProfile || !holdReason.trim()) {
      toast.error('Please provide a reason for the hold');
      return;
    }
    try {
      setSaving(true);
      await credit.placeHold(selectedProfile.customer_id, { reason: holdReason });
      toast.success('Credit hold placed successfully');
      setShowHoldModal(false);
      setHoldReason('');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to place credit hold'));
    } finally {
      setSaving(false);
    }
  };

  const handleReleaseHold = async () => {
    if (!selectedProfile || !releaseReason.trim()) {
      toast.error('Please provide a reason for releasing the hold');
      return;
    }
    try {
      setSaving(true);
      await credit.releaseHold(selectedProfile.customer_id, { override_reason: releaseReason });
      toast.success('Credit hold released successfully');
      setShowReleaseModal(false);
      setReleaseReason('');
      loadData();
      if (selectedProfile) {
        loadProfileDetails({ ...selectedProfile, is_on_hold: false });
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to release credit hold'));
    } finally {
      setSaving(false);
    }
  };

  if (loading) return <LoadingPage />;

  const displayProfiles = activeTab === 'all' ? profiles : activeTab === 'onhold' ? onHold : highRisk;

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-900">Credit Management</h1>
      </div>

      {summary && (
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="bg-white rounded-lg shadow p-4">
            <p className="text-sm text-gray-500">Total Credit Limit</p>
            <p className="text-2xl font-bold text-gray-900">{formatCurrency(summary.total_credit_limit)}</p>
          </div>
          <div className="bg-white rounded-lg shadow p-4">
            <p className="text-sm text-gray-500">Credit Used</p>
            <p className="text-2xl font-bold text-blue-600">{formatCurrency(summary.total_credit_used)}</p>
            <p className="text-xs text-gray-500">{summary.avg_utilization_percent.toFixed(1)}% utilization</p>
          </div>
          <div className="bg-white rounded-lg shadow p-4">
            <p className="text-sm text-gray-500">Customers on Hold</p>
            <p className="text-2xl font-bold text-red-600">{summary.customers_on_hold}</p>
          </div>
          <div className="bg-white rounded-lg shadow p-4">
            <p className="text-sm text-gray-500">High Risk Customers</p>
            <p className="text-2xl font-bold text-orange-600">{summary.high_risk_customers}</p>
          </div>
        </div>
      )}

      <div className="flex space-x-4 border-b border-gray-200">
        <button
          onClick={() => setActiveTab('all')}
          className={`px-4 py-2 font-medium ${activeTab === 'all' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-gray-500'}`}
        >
          All Profiles ({profiles.length})
        </button>
        <button
          onClick={() => setActiveTab('onhold')}
          className={`px-4 py-2 font-medium ${activeTab === 'onhold' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-gray-500'}`}
        >
          On Hold ({onHold.length})
        </button>
        <button
          onClick={() => setActiveTab('highrisk')}
          className={`px-4 py-2 font-medium ${activeTab === 'highrisk' ? 'text-blue-600 border-b-2 border-blue-600' : 'text-gray-500'}`}
        >
          High Risk ({highRisk.length})
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-white rounded-lg shadow overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Customer</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Credit Limit</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Used</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Available</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Risk</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {displayProfiles.map((profile) => (
                <tr
                  key={profile.id}
                  onClick={() => loadProfileDetails(profile)}
                  className={`cursor-pointer hover:bg-gray-50 ${selectedProfile?.id === profile.id ? 'bg-blue-50' : ''}`}
                >
                  <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                    {profile.customer_id.slice(0, 8)}...
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    {formatCurrency(profile.credit_limit)}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm">
                    <div className="text-gray-900">{formatCurrency(profile.credit_used)}</div>
                    <div className="w-full bg-gray-200 rounded-full h-1.5 mt-1">
                      <div
                        className={`h-1.5 rounded-full ${profile.utilization_percent > 90 ? 'bg-red-600' : profile.utilization_percent > 70 ? 'bg-yellow-500' : 'bg-green-500'}`}
                        style={{ width: `${Math.min(profile.utilization_percent, 100)}%` }}
                      />
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    <span className={profile.available_credit < 0 ? 'text-red-600 font-medium' : ''}>
                      {formatCurrency(profile.available_credit)}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${getRiskBadgeColor(profile.risk_level)}`}>
                      {profile.risk_level}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    {profile.is_on_hold ? (
                      <span className="px-2 py-1 text-xs font-medium rounded-full bg-red-100 text-red-800">On Hold</span>
                    ) : (
                      <span className="px-2 py-1 text-xs font-medium rounded-full bg-green-100 text-green-800">Active</span>
                    )}
                  </td>
                </tr>
              ))}
              {displayProfiles.length === 0 && (
                <tr>
                  <td colSpan={6} className="px-6 py-4 text-center text-gray-500">
                    No credit profiles found
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>

        {selectedProfile && (
          <div className="bg-white rounded-lg shadow p-6 space-y-6">
            <div className="flex justify-between items-start">
              <h2 className="text-lg font-semibold">Profile Details</h2>
              <div className="flex space-x-2">
                <button
                  onClick={() => {
                    setNewLimit(selectedProfile.credit_limit);
                    setShowLimitModal(true);
                  }}
                  className="text-sm text-blue-600 hover:text-blue-800"
                >
                  Edit Limit
                </button>
                {selectedProfile.is_on_hold ? (
                  <button
                    onClick={() => setShowReleaseModal(true)}
                    className="text-sm text-green-600 hover:text-green-800"
                  >
                    Release Hold
                  </button>
                ) : (
                  <button
                    onClick={() => setShowHoldModal(true)}
                    className="text-sm text-red-600 hover:text-red-800"
                  >
                    Place Hold
                  </button>
                )}
              </div>
            </div>

            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-gray-500">Credit Limit</span>
                <span className="font-medium">{formatCurrency(selectedProfile.credit_limit)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Credit Used</span>
                <span className="font-medium">{formatCurrency(selectedProfile.credit_used)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Available Credit</span>
                <span className={`font-medium ${selectedProfile.available_credit < 0 ? 'text-red-600' : ''}`}>
                  {formatCurrency(selectedProfile.available_credit)}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Outstanding Invoices</span>
                <span className="font-medium">{formatCurrency(selectedProfile.outstanding_invoices)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Overdue Amount</span>
                <span className={`font-medium ${selectedProfile.overdue_amount > 0 ? 'text-red-600' : ''}`}>
                  {formatCurrency(selectedProfile.overdue_amount)}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Risk Level</span>
                <span className={`px-2 py-1 text-xs font-medium rounded-full ${getRiskBadgeColor(selectedProfile.risk_level)}`}>
                  {selectedProfile.risk_level}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-500">Auto Hold</span>
                <span className="font-medium">{selectedProfile.auto_hold_enabled ? 'Enabled' : 'Disabled'}</span>
              </div>
            </div>

            {holds.length > 0 && (
              <div>
                <h3 className="text-sm font-semibold mb-2">Credit Holds</h3>
                <div className="space-y-2">
                  {holds.slice(0, 3).map((hold) => (
                    <div key={hold.id} className="text-sm bg-gray-50 p-2 rounded">
                      <div className="font-medium">{hold.hold_type}</div>
                      <div className="text-gray-500">{hold.reason}</div>
                      <div className="text-xs text-gray-400">{new Date(hold.placed_at).toLocaleDateString()}</div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {transactions.length > 0 && (
              <div>
                <h3 className="text-sm font-semibold mb-2">Recent Transactions</h3>
                <div className="space-y-2 max-h-48 overflow-y-auto">
                  {transactions.slice(0, 5).map((txn) => (
                    <div key={txn.id} className="text-sm bg-gray-50 p-2 rounded">
                      <div className="flex justify-between">
                        <span className="font-medium">{txn.transaction_type}</span>
                        <span className={txn.amount >= 0 ? 'text-green-600' : 'text-red-600'}>
                          {txn.amount >= 0 ? '+' : ''}{formatCurrency(txn.amount)}
                        </span>
                      </div>
                      <div className="text-gray-500 text-xs">{txn.description}</div>
                      <div className="text-xs text-gray-400">{new Date(txn.created_at).toLocaleDateString()}</div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}
      </div>

      {showLimitModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-semibold mb-4">Update Credit Limit</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700">New Credit Limit ($)</label>
                <input
                  type="number"
                  value={newLimit / 100}
                  onChange={(e) => setNewLimit(Math.round(parseFloat(e.target.value || '0') * 100))}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Reason</label>
                <textarea
                  value={limitReason}
                  onChange={(e) => setLimitReason(e.target.value)}
                  rows={3}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                  placeholder="Explain the reason for this change..."
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end space-x-3">
              <button
                onClick={() => setShowLimitModal(false)}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Cancel
              </button>
              <button
                onClick={handleUpdateLimit}
                disabled={saving}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50"
              >
                {saving ? 'Updating...' : 'Update Limit'}
              </button>
            </div>
          </div>
        </div>
      )}

      {showHoldModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-semibold mb-4">Place Credit Hold</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700">Reason</label>
                <textarea
                  value={holdReason}
                  onChange={(e) => setHoldReason(e.target.value)}
                  rows={3}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                  placeholder="Explain why you're placing this hold..."
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end space-x-3">
              <button
                onClick={() => setShowHoldModal(false)}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Cancel
              </button>
              <button
                onClick={handlePlaceHold}
                disabled={saving}
                className="px-4 py-2 text-sm font-medium text-white bg-red-600 rounded-md hover:bg-red-700 disabled:opacity-50"
              >
                {saving ? 'Placing...' : 'Place Hold'}
              </button>
            </div>
          </div>
        </div>
      )}

      {showReleaseModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-semibold mb-4">Release Credit Hold</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700">Override Reason</label>
                <textarea
                  value={releaseReason}
                  onChange={(e) => setReleaseReason(e.target.value)}
                  rows={3}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
                  placeholder="Explain why you're releasing this hold..."
                />
              </div>
            </div>
            <div className="mt-6 flex justify-end space-x-3">
              <button
                onClick={() => setShowReleaseModal(false)}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200"
              >
                Cancel
              </button>
              <button
                onClick={handleReleaseHold}
                disabled={saving}
                className="px-4 py-2 text-sm font-medium text-white bg-green-600 rounded-md hover:bg-green-700 disabled:opacity-50"
              >
                {saving ? 'Releasing...' : 'Release Hold'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

import { useEffect, useState } from 'react';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import api from '../api/client';

interface CurrencyRevaluation {
  id: string;
  revaluation_number: string;
  revaluation_date: string;
  period_start: string;
  period_end: string;
  base_currency: string;
  status: string;
  total_unrealized_gain: number;
  total_unrealized_loss: number;
  net_unrealized: number;
  journal_entry_id?: string;
}

interface RevaluationLine {
  account_id: string;
  account_code: string;
  account_name: string;
  currency: string;
  original_balance: number;
  original_rate: number;
  revaluation_rate: number;
  base_currency_balance: number;
  revalued_balance: number;
  unrealized_gain: number;
  unrealized_loss: number;
}

interface RevaluationPreview {
  revaluation_date: string;
  base_currency: string;
  lines: RevaluationLine[];
  total_unrealized_gain: number;
  total_unrealized_loss: number;
  net_unrealized: number;
}

export default function CurrencyRevaluation() {
  const toast = useToast();
  const [revaluations, setRevaluations] = useState<CurrencyRevaluation[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showPreviewModal, setShowPreviewModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [preview, setPreview] = useState<RevaluationPreview | null>(null);
  const [selectedReval, setSelectedReval] = useState<CurrencyRevaluation | null>(null);
  const [detailLines, setDetailLines] = useState<RevaluationLine[]>([]);

  const [formData, setFormData] = useState({
    revaluation_date: new Date().toISOString().split('T')[0],
    period_start: new Date(new Date().getFullYear(), new Date().getMonth(), 1).toISOString().split('T')[0],
    period_end: new Date(new Date().getFullYear(), new Date().getMonth() + 1, 0).toISOString().split('T')[0],
    base_currency: 'USD',
  });

  useEffect(() => {
    loadRevaluations();
  }, []);

  const loadRevaluations = async () => {
    try {
      setLoading(true);
      const res = await api.get('/api/v1/finance/currency-revaluations');
      setRevaluations(res.data);
    } catch (err) {
      toast.error('Failed to load currency revaluations');
    } finally {
      setLoading(false);
    }
  };

  const handlePreview = async () => {
    try {
      setSaving(true);
      const res = await api.post('/api/v1/finance/currency-revaluations/preview', {
        revaluation_date: formData.revaluation_date,
        base_currency: formData.base_currency,
      });
      setPreview(res.data);
      setShowPreviewModal(true);
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to preview revaluation');
    } finally {
      setSaving(false);
    }
  };

  const handleCreate = async () => {
    try {
      setSaving(true);
      await api.post('/api/v1/finance/currency-revaluations', formData);
      toast.success('Currency revaluation created successfully');
      setShowCreateModal(false);
      setShowPreviewModal(false);
      setPreview(null);
      loadRevaluations();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to create revaluation');
    } finally {
      setSaving(false);
    }
  };

  const handlePost = async (id: string) => {
    try {
      await api.post(`/api/v1/finance/currency-revaluations/${id}/post`);
      toast.success('Revaluation posted successfully');
      loadRevaluations();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to post revaluation');
    }
  };

  const handleReverse = async (id: string) => {
    if (!confirm('Are you sure you want to reverse this revaluation?')) return;
    try {
      await api.post(`/api/v1/finance/currency-revaluations/${id}/reverse`);
      toast.success('Revaluation reversed successfully');
      loadRevaluations();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to reverse revaluation');
    }
  };

  const handleViewDetail = async (reval: CurrencyRevaluation) => {
    try {
      setSelectedReval(reval);
      const res = await api.get(`/api/v1/finance/currency-revaluations/${reval.id}/lines`);
      setDetailLines(res.data);
      setShowDetailModal(true);
    } catch (err) {
      toast.error('Failed to load revaluation details');
    }
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(amount);
  };

  const getStatusBadge = (status: string) => {
    const classes: Record<string, string> = {
      Draft: 'badge-warning',
      Completed: 'badge-success',
      Reversed: 'badge-error',
      Pending: 'badge-info',
    };
    return <span className={`badge ${classes[status] || 'badge-info'}`}>{status}</span>;
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Currency Revaluation</h1>
          <p className="text-sm text-gray-500 mt-1">IFRS/GAAP compliant foreign currency revaluation</p>
        </div>
        <button onClick={() => setShowCreateModal(true)} className="btn btn-primary">
          New Revaluation
        </button>
      </div>

      <div className="card mb-6">
        <div className="p-4 bg-blue-50 border-b">
          <h3 className="font-medium text-blue-900">About Currency Revaluation</h3>
          <p className="text-sm text-blue-700 mt-1">
            At period-end, foreign currency balances must be revalued at the current exchange rate.
            This creates journal entries for unrealized foreign exchange gains/losses to comply with
            IFRS and GAAP accounting standards.
          </p>
        </div>
      </div>

      <div className="card">
        <div className="p-4 border-b">
          <h2 className="text-lg font-semibold">Revaluation History</h2>
        </div>
        {revaluations.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            No currency revaluations found. Click "New Revaluation" to create one.
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Number</th>
                <th className="table-header">Date</th>
                <th className="table-header">Period</th>
                <th className="table-header">Base Currency</th>
                <th className="table-header text-right">Unrealized Gain</th>
                <th className="table-header text-right">Unrealized Loss</th>
                <th className="table-header text-right">Net</th>
                <th className="table-header">Status</th>
                <th className="table-header">Actions</th>
              </tr>
            </thead>
            <tbody>
              {revaluations.map((reval) => (
                <tr key={reval.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{reval.revaluation_number}</td>
                  <td className="table-cell">{new Date(reval.revaluation_date).toLocaleDateString()}</td>
                  <td className="table-cell text-sm">
                    {new Date(reval.period_start).toLocaleDateString()} - {new Date(reval.period_end).toLocaleDateString()}
                  </td>
                  <td className="table-cell">{reval.base_currency}</td>
                  <td className="table-cell text-right text-green-600">{formatCurrency(reval.total_unrealized_gain)}</td>
                  <td className="table-cell text-right text-red-600">{formatCurrency(reval.total_unrealized_loss)}</td>
                  <td className={`table-cell text-right font-medium ${reval.net_unrealized >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                    {formatCurrency(reval.net_unrealized)}
                  </td>
                  <td className="table-cell">{getStatusBadge(reval.status)}</td>
                  <td className="table-cell">
                    <div className="flex gap-1">
                      <button onClick={() => handleViewDetail(reval)} className="btn btn-secondary text-xs py-1">
                        View
                      </button>
                      {reval.status === 'Draft' && (
                        <button onClick={() => handlePost(reval.id)} className="btn btn-primary text-xs py-1">
                          Post
                        </button>
                      )}
                      {reval.status === 'Completed' && (
                        <button onClick={() => handleReverse(reval.id)} className="btn btn-secondary text-xs py-1">
                          Reverse
                        </button>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Currency Revaluation</h2>
            <form onSubmit={(e) => { e.preventDefault(); handlePreview(); }} className="space-y-4">
              <div>
                <label className="label">Revaluation Date</label>
                <input
                  type="date"
                  className="input"
                  value={formData.revaluation_date}
                  onChange={(e) => setFormData({ ...formData, revaluation_date: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">Period Start</label>
                <input
                  type="date"
                  className="input"
                  value={formData.period_start}
                  onChange={(e) => setFormData({ ...formData, period_start: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">Period End</label>
                <input
                  type="date"
                  className="input"
                  value={formData.period_end}
                  onChange={(e) => setFormData({ ...formData, period_end: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">Base Currency</label>
                <select
                  className="input"
                  value={formData.base_currency}
                  onChange={(e) => setFormData({ ...formData, base_currency: e.target.value })}
                >
                  <option value="USD">USD - US Dollar</option>
                  <option value="EUR">EUR - Euro</option>
                  <option value="GBP">GBP - British Pound</option>
                  <option value="JPY">JPY - Japanese Yen</option>
                  <option value="CAD">CAD - Canadian Dollar</option>
                  <option value="AUD">AUD - Australian Dollar</option>
                </select>
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowCreateModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Loading...' : 'Preview'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showPreviewModal && preview && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-4xl max-h-[90vh] overflow-y-auto">
            <h2 className="text-lg font-semibold mb-4">Preview Revaluation</h2>
            
            <div className="grid grid-cols-3 gap-4 mb-6">
              <div className="bg-green-50 p-4 rounded">
                <div className="text-sm text-green-600">Total Unrealized Gain</div>
                <div className="text-xl font-bold text-green-700">{formatCurrency(preview.total_unrealized_gain)}</div>
              </div>
              <div className="bg-red-50 p-4 rounded">
                <div className="text-sm text-red-600">Total Unrealized Loss</div>
                <div className="text-xl font-bold text-red-700">{formatCurrency(preview.total_unrealized_loss)}</div>
              </div>
              <div className={`p-4 rounded ${preview.net_unrealized >= 0 ? 'bg-green-50' : 'bg-red-50'}`}>
                <div className={`text-sm ${preview.net_unrealized >= 0 ? 'text-green-600' : 'text-red-600'}`}>Net Effect</div>
                <div className={`text-xl font-bold ${preview.net_unrealized >= 0 ? 'text-green-700' : 'text-red-700'}`}>
                  {formatCurrency(preview.net_unrealized)}
                </div>
              </div>
            </div>

            {preview.lines.length === 0 ? (
              <div className="text-center text-gray-500 py-8">
                No foreign currency accounts with balances found for revaluation.
              </div>
            ) : (
              <table className="w-full mb-6">
                <thead>
                  <tr className="border-b">
                    <th className="table-header">Account</th>
                    <th className="table-header">Currency</th>
                    <th className="table-header text-right">Original Balance</th>
                    <th className="table-header text-right">Orig. Rate</th>
                    <th className="table-header text-right">Reval. Rate</th>
                    <th className="table-header text-right">Base Balance</th>
                    <th className="table-header text-right">Reval. Balance</th>
                    <th className="table-header text-right">Gain</th>
                    <th className="table-header text-right">Loss</th>
                  </tr>
                </thead>
                <tbody>
                  {preview.lines.map((line, i) => (
                    <tr key={i} className="border-b">
                      <td className="table-cell">
                        <span className="font-mono text-xs">{line.account_code}</span>
                        <br />
                        <span className="text-sm">{line.account_name}</span>
                      </td>
                      <td className="table-cell">{line.currency}</td>
                      <td className="table-cell text-right">{formatCurrency(line.original_balance)}</td>
                      <td className="table-cell text-right">{line.original_rate.toFixed(4)}</td>
                      <td className="table-cell text-right">{line.revaluation_rate.toFixed(4)}</td>
                      <td className="table-cell text-right">{formatCurrency(line.base_currency_balance)}</td>
                      <td className="table-cell text-right">{formatCurrency(line.revalued_balance)}</td>
                      <td className="table-cell text-right text-green-600">
                        {line.unrealized_gain > 0 ? formatCurrency(line.unrealized_gain) : '-'}
                      </td>
                      <td className="table-cell text-right text-red-600">
                        {line.unrealized_loss > 0 ? formatCurrency(line.unrealized_loss) : '-'}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}

            <div className="flex gap-2 justify-end">
              <button onClick={() => setShowPreviewModal(false)} className="btn btn-secondary">
                Cancel
              </button>
              <button onClick={handleCreate} className="btn btn-primary" disabled={saving || preview.lines.length === 0}>
                {saving ? 'Creating...' : 'Create Revaluation'}
              </button>
            </div>
          </div>
        </div>
      )}

      {showDetailModal && selectedReval && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-4xl max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-start mb-4">
              <div>
                <h2 className="text-lg font-semibold">{selectedReval.revaluation_number}</h2>
                <p className="text-sm text-gray-500">
                  Revaluation Date: {new Date(selectedReval.revaluation_date).toLocaleDateString()}
                </p>
              </div>
              {getStatusBadge(selectedReval.status)}
            </div>

            <div className="grid grid-cols-3 gap-4 mb-6">
              <div className="bg-gray-50 p-4 rounded">
                <div className="text-sm text-gray-600">Total Unrealized Gain</div>
                <div className="text-xl font-bold text-green-600">{formatCurrency(selectedReval.total_unrealized_gain)}</div>
              </div>
              <div className="bg-gray-50 p-4 rounded">
                <div className="text-sm text-gray-600">Total Unrealized Loss</div>
                <div className="text-xl font-bold text-red-600">{formatCurrency(selectedReval.total_unrealized_loss)}</div>
              </div>
              <div className="bg-gray-50 p-4 rounded">
                <div className="text-sm text-gray-600">Net Effect</div>
                <div className={`text-xl font-bold ${selectedReval.net_unrealized >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                  {formatCurrency(selectedReval.net_unrealized)}
                </div>
              </div>
            </div>

            {selectedReval.journal_entry_id && (
              <div className="mb-4 p-3 bg-blue-50 rounded text-sm text-blue-700">
                Journal Entry: {selectedReval.journal_entry_id}
              </div>
            )}

            <table className="w-full mb-6">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Account</th>
                  <th className="table-header">Currency</th>
                  <th className="table-header text-right">Original</th>
                  <th className="table-header text-right">Revalued</th>
                  <th className="table-header text-right">Gain</th>
                  <th className="table-header text-right">Loss</th>
                </tr>
              </thead>
              <tbody>
                {detailLines.map((line, i) => (
                  <tr key={i} className="border-b">
                    <td className="table-cell">
                      <span className="font-mono text-xs">{line.account_code}</span>
                      <br />
                      <span className="text-sm">{line.account_name}</span>
                    </td>
                    <td className="table-cell">{line.currency}</td>
                    <td className="table-cell text-right">{formatCurrency(line.base_currency_balance)}</td>
                    <td className="table-cell text-right">{formatCurrency(line.revalued_balance)}</td>
                    <td className="table-cell text-right text-green-600">
                      {line.unrealized_gain > 0 ? formatCurrency(line.unrealized_gain) : '-'}
                    </td>
                    <td className="table-cell text-right text-red-600">
                      {line.unrealized_loss > 0 ? formatCurrency(line.unrealized_loss) : '-'}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            <div className="flex gap-2 justify-end">
              <button onClick={() => setShowDetailModal(false)} className="btn btn-secondary">
                Close
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

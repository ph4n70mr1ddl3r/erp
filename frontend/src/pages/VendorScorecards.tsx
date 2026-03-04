import { useEffect, useState, useCallback } from 'react';
import { purchasing, scorecards as scorecardsApi } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { getErrorMessage } from '../utils/errors';
import type { Vendor, SupplierScorecard } from '../types';

export default function VendorScorecards() {
  const toast = useToast();
  const [vendors, setVendors] = useState<Vendor[]>([]);
  const [selectedVendor, setSelectedVendor] = useState<Vendor | null>(null);
  const [scorecards, setScorecards] = useState<SupplierScorecard[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [newPeriod, setNewPeriod] = useState('');

  const loadVendors = useCallback(async () => {
    try {
      setLoading(true);
      const res = await purchasing.getVendors(1, 100);
      setVendors(res.data.items);
      if (res.data.items.length > 0 && !selectedVendor) {
        setSelectedVendor(res.data.items[0]);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load vendors'));
    } finally {
      setLoading(false);
    }
  }, [toast, selectedVendor]);

  const loadScorecards = useCallback(async () => {
    if (!selectedVendor) return;
    try {
      const res = await scorecardsApi.list(selectedVendor.id);
      setScorecards(res.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load scorecards'));
    }
  }, [selectedVendor, toast]);

  useEffect(() => { loadVendors(); }, [loadVendors]);
  useEffect(() => { if (selectedVendor) loadScorecards(); }, [selectedVendor, loadScorecards]);

  const handleCreateScorecard = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedVendor || !newPeriod) return;
    try {
      setSaving(true);
      await scorecardsApi.create(selectedVendor.id, newPeriod);
      toast.success('Scorecard created successfully');
      setShowModal(false);
      setNewPeriod('');
      loadScorecards();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create scorecard'));
    } finally {
      setSaving(false);
    }
  };

  const getScoreColor = (score: number) => {
    if (score >= 80) return 'text-green-600';
    if (score >= 60) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getScoreBadge = (score: number) => {
    if (score >= 80) return 'bg-green-100 text-green-800';
    if (score >= 60) return 'bg-yellow-100 text-yellow-800';
    return 'bg-red-100 text-red-800';
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Vendor Scorecards</h1>
        <button
          onClick={() => setShowModal(true)}
          disabled={!selectedVendor}
          className="btn btn-primary"
        >
          Create Scorecard
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Vendors</p>
          <p className="text-2xl font-bold">{vendors.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Scorecards</p>
          <p className="text-2xl font-bold">{scorecards.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Avg. On-Time</p>
          <p className={`text-2xl font-bold ${scorecards.length > 0 ? getScoreColor(Math.round(scorecards.reduce((a, s) => a + s.on_time_delivery, 0) / scorecards.length)) : ''}`}>
            {scorecards.length > 0 ? Math.round(scorecards.reduce((a, s) => a + s.on_time_delivery, 0) / scorecards.length) : 0}%
          </p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Avg. Quality</p>
          <p className={`text-2xl font-bold ${scorecards.length > 0 ? getScoreColor(Math.round(scorecards.reduce((a, s) => a + s.quality_score, 0) / scorecards.length)) : ''}`}>
            {scorecards.length > 0 ? Math.round(scorecards.reduce((a, s) => a + s.quality_score, 0) / scorecards.length) : 0}%
          </p>
        </div>
      </div>

      <div className="card mb-6">
        <div className="p-4 border-b">
          <label className="label mb-1">Select Vendor</label>
          <select
            className="input w-full md:w-64"
            value={selectedVendor?.id || ''}
            onChange={(e) => setSelectedVendor(vendors.find(v => v.id === e.target.value) || null)}
          >
            {vendors.map((v) => (
              <option key={v.id} value={v.id}>{v.name} ({v.code})</option>
            ))}
          </select>
        </div>
      </div>

      {selectedVendor && (
        <div className="card">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Performance Scorecards - {selectedVendor.name}</h2>
          </div>
          {scorecards.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              No scorecards found. Create one to start tracking vendor performance.
            </div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Period</th>
                  <th className="table-header">On-Time Delivery</th>
                  <th className="table-header">Quality Score</th>
                  <th className="table-header">Price Competitiveness</th>
                  <th className="table-header">Responsiveness</th>
                  <th className="table-header">Overall Score</th>
                  <th className="table-header">Total Orders</th>
                </tr>
              </thead>
              <tbody>
                {scorecards.map((s) => (
                  <tr key={s.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-mono">{s.period}</td>
                    <td className="table-cell">
                      <span className={`px-2 py-1 rounded text-sm font-medium ${getScoreBadge(s.on_time_delivery)}`}>
                        {s.on_time_delivery}%
                      </span>
                    </td>
                    <td className="table-cell">
                      <span className={`px-2 py-1 rounded text-sm font-medium ${getScoreBadge(s.quality_score)}`}>
                        {s.quality_score}%
                      </span>
                    </td>
                    <td className="table-cell">
                      <span className={`px-2 py-1 rounded text-sm font-medium ${getScoreBadge(s.price_competitiveness)}`}>
                        {s.price_competitiveness}%
                      </span>
                    </td>
                    <td className="table-cell">
                      <span className={`px-2 py-1 rounded text-sm font-medium ${getScoreBadge(s.responsiveness)}`}>
                        {s.responsiveness}%
                      </span>
                    </td>
                    <td className="table-cell">
                      <span className={`px-2 py-1 rounded text-sm font-bold ${getScoreBadge(s.overall_score)}`}>
                        {s.overall_score}%
                      </span>
                    </td>
                    <td className="table-cell">{s.total_orders}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {showModal && selectedVendor && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">Create Scorecard for {selectedVendor.name}</h2>
            <form onSubmit={handleCreateScorecard} className="space-y-4">
              <div>
                <label className="label">Period (YYYY-MM)</label>
                <input
                  type="month"
                  className="input"
                  value={newPeriod}
                  onChange={(e) => setNewPeriod(e.target.value)}
                  required
                />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

import { useCallback, useEffect, useState } from 'react';
import { inventoryAdjustments, type InventoryAdjustment, type InventoryAdjustmentAnalytics } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import { getErrorMessage } from '../utils/errors';

export default function InventoryAdjustmentsPage() {
  const toast = useToast();
  const [adjustments, setAdjustments] = useState<InventoryAdjustment[]>([]);
  const [analytics, setAnalytics] = useState<InventoryAdjustmentAnalytics | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [search, setSearch] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [selectedAdjustment, setSelectedAdjustment] = useState<InventoryAdjustment | null>(null);
  const [statusFilter, setStatusFilter] = useState<string>('');

  const [newAdjustment, setNewAdjustment] = useState({
    warehouse_id: '',
    adjustment_type: 'CountVariance',
    reason: '',
    notes: '',
    lines: [{ product_id: '', location_id: '', system_quantity: 0, counted_quantity: 0, unit_cost: 0 }],
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [adjRes, analyticsRes] = await Promise.all([
        inventoryAdjustments.list(undefined, statusFilter || undefined),
        inventoryAdjustments.getAnalytics(),
      ]);
      setAdjustments(adjRes.data.data);
      setAnalytics(analyticsRes.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load inventory adjustments'));
    } finally {
      setLoading(false);
    }
  }, [toast, statusFilter]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await inventoryAdjustments.create({
        warehouse_id: newAdjustment.warehouse_id,
        adjustment_type: newAdjustment.adjustment_type,
        reason: newAdjustment.reason,
        notes: newAdjustment.notes || undefined,
        lines: newAdjustment.lines.map(l => ({
          product_id: l.product_id,
          location_id: l.location_id,
          system_quantity: l.system_quantity,
          counted_quantity: l.counted_quantity,
          unit_cost: Math.round(l.unit_cost * 100),
        })),
      });
      toast.success('Adjustment created successfully');
      setShowModal(false);
      setNewAdjustment({
        warehouse_id: '',
        adjustment_type: 'CountVariance',
        reason: '',
        notes: '',
        lines: [{ product_id: '', location_id: '', system_quantity: 0, counted_quantity: 0, unit_cost: 0 }],
      });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create adjustment'));
    } finally {
      setSaving(false);
    }
  };

  const handleSubmit = async (adj: InventoryAdjustment) => {
    try {
      await inventoryAdjustments.submit(adj.id);
      toast.success('Adjustment submitted for approval');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to submit adjustment'));
    }
  };

  const handleApprove = async (adj: InventoryAdjustment) => {
    try {
      await inventoryAdjustments.approve(adj.id);
      toast.success('Adjustment approved');
      setSelectedAdjustment(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to approve adjustment'));
    }
  };

  const handleReject = async (adj: InventoryAdjustment) => {
    try {
      await inventoryAdjustments.reject(adj.id, 'Rejected by reviewer');
      toast.success('Adjustment rejected');
      setSelectedAdjustment(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to reject adjustment'));
    }
  };

  const handleComplete = async (adj: InventoryAdjustment) => {
    try {
      await inventoryAdjustments.complete(adj.id);
      toast.success('Adjustment completed');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to complete adjustment'));
    }
  };

  const handleCancel = async (adj: InventoryAdjustment) => {
    try {
      await inventoryAdjustments.cancel(adj.id);
      toast.success('Adjustment cancelled');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to cancel adjustment'));
    }
  };

  const handleDelete = async (adj: InventoryAdjustment) => {
    if (!confirm('Are you sure you want to delete this adjustment?')) return;
    try {
      await inventoryAdjustments.delete(adj.id);
      toast.success('Adjustment deleted');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to delete adjustment'));
    }
  };

  const addLine = () => {
    setNewAdjustment({
      ...newAdjustment,
      lines: [...newAdjustment.lines, { product_id: '', location_id: '', system_quantity: 0, counted_quantity: 0, unit_cost: 0 }],
    });
  };

  const removeLine = (index: number) => {
    if (newAdjustment.lines.length > 1) {
      setNewAdjustment({ ...newAdjustment, lines: newAdjustment.lines.filter((_, i) => i !== index) });
    }
  };

  const updateLine = (index: number, field: string, value: string | number) => {
    const lines = [...newAdjustment.lines];
    lines[index] = { ...lines[index], [field]: value };
    setNewAdjustment({ ...newAdjustment, lines });
  };

  const filteredAdjustments = adjustments.filter(a =>
    a.adjustment_number.toLowerCase().includes(search.toLowerCase()) ||
    a.reason.toLowerCase().includes(search.toLowerCase()) ||
    a.adjustment_type.toLowerCase().includes(search.toLowerCase())
  );

  const getStatusBadge = (status: string) => {
    const classes: Record<string, string> = {
      Draft: 'badge-default',
      Pending: 'badge-warning',
      Approved: 'badge-success',
      Rejected: 'badge-danger',
      Completed: 'badge-success',
      Cancelled: 'badge-default',
    };
    return <span className={`badge ${classes[status] || 'badge-default'}`}>{status}</span>;
  };

  const formatAmount = (amount: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(amount / 100);
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Inventory Adjustments</h1>
        <button onClick={() => setShowModal(true)} className="btn btn-primary">Create Adjustment</button>
      </div>

      {analytics && (
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div className="card p-4">
            <div className="text-sm text-gray-600">Total Adjustments</div>
            <div className="text-2xl font-bold">{analytics.total_adjustments}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Pending Approval</div>
            <div className="text-2xl font-bold text-yellow-600">{analytics.pending_adjustments}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Value Increase</div>
            <div className="text-2xl font-bold text-green-600">{formatAmount(analytics.total_value_increase)}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Value Decrease</div>
            <div className="text-2xl font-bold text-red-600">{formatAmount(analytics.total_value_decrease)}</div>
          </div>
        </div>
      )}

      <div className="flex gap-4 mb-4">
        <div className="flex-1">
          <SearchInput value={search} onChange={setSearch} placeholder="Search adjustments..." />
        </div>
        <select className="input w-40" value={statusFilter} onChange={(e) => setStatusFilter(e.target.value)}>
          <option value="">All Status</option>
          <option value="Draft">Draft</option>
          <option value="Pending">Pending</option>
          <option value="Approved">Approved</option>
          <option value="Completed">Completed</option>
          <option value="Rejected">Rejected</option>
          <option value="Cancelled">Cancelled</option>
        </select>
      </div>

      <div className="card">
        {filteredAdjustments.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {search || statusFilter ? 'No adjustments match your filters' : 'No adjustments found. Create one to get started.'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Adjustment #</th>
                <th className="table-header">Type</th>
                <th className="table-header">Reason</th>
                <th className="table-header">Value Change</th>
                <th className="table-header">Status</th>
                <th className="table-header">Created</th>
                <th className="table-header">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredAdjustments.map((adj) => (
                <tr key={adj.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{adj.adjustment_number}</td>
                  <td className="table-cell">{adj.adjustment_type}</td>
                  <td className="table-cell">{adj.reason}</td>
                  <td className={`table-cell font-semibold ${adj.total_value_change >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                    {formatAmount(adj.total_value_change)}
                  </td>
                  <td className="table-cell">{getStatusBadge(adj.status)}</td>
                  <td className="table-cell">{new Date(adj.created_at).toLocaleDateString()}</td>
                  <td className="table-cell">
                    <div className="flex gap-1">
                      <button onClick={() => setSelectedAdjustment(adj)} className="btn btn-secondary text-xs py-1">View</button>
                      {adj.status === 'Draft' && (
                        <>
                          <button onClick={() => handleSubmit(adj)} className="btn btn-primary text-xs py-1">Submit</button>
                          <button onClick={() => handleDelete(adj)} className="btn btn-danger text-xs py-1">Delete</button>
                        </>
                      )}
                      {adj.status === 'Approved' && (
                        <button onClick={() => handleComplete(adj)} className="btn btn-success text-xs py-1">Complete</button>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <h2 className="text-lg font-semibold mb-4">Create Inventory Adjustment</h2>
            <form onSubmit={handleCreate} className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Warehouse ID</label>
                  <input className="input" value={newAdjustment.warehouse_id} onChange={(e) => setNewAdjustment({ ...newAdjustment, warehouse_id: e.target.value })} required placeholder="Enter warehouse UUID" />
                </div>
                <div>
                  <label className="label">Adjustment Type</label>
                  <select className="input" value={newAdjustment.adjustment_type} onChange={(e) => setNewAdjustment({ ...newAdjustment, adjustment_type: e.target.value })}>
                    <option value="CountVariance">Count Variance</option>
                    <option value="Damage">Damage</option>
                    <option value="Theft">Theft</option>
                    <option value="Expired">Expired</option>
                    <option value="Obsolete">Obsolete</option>
                    <option value="Found">Found</option>
                    <option value="TransferCorrection">Transfer Correction</option>
                    <option value="Other">Other</option>
                  </select>
                </div>
              </div>

              <div>
                <label className="label">Reason</label>
                <input className="input" value={newAdjustment.reason} onChange={(e) => setNewAdjustment({ ...newAdjustment, reason: e.target.value })} required placeholder="e.g., Annual stock count discrepancy" />
              </div>

              <div>
                <label className="label">Notes</label>
                <input className="input" value={newAdjustment.notes} onChange={(e) => setNewAdjustment({ ...newAdjustment, notes: e.target.value })} placeholder="Optional notes" />
              </div>

              <div>
                <label className="label">Adjustment Lines</label>
                {newAdjustment.lines.map((line, i) => (
                  <div key={i} className="border rounded p-3 mb-2">
                    <div className="flex justify-between items-center mb-2">
                      <span className="font-medium">Line {i + 1}</span>
                      {newAdjustment.lines.length > 1 && (
                        <button type="button" onClick={() => removeLine(i)} className="text-red-600 text-sm">Remove</button>
                      )}
                    </div>
                    <div className="grid grid-cols-2 md:grid-cols-3 gap-2">
                      <input className="input" placeholder="Product ID" value={line.product_id} onChange={(e) => updateLine(i, 'product_id', e.target.value)} required />
                      <input className="input" placeholder="Location ID" value={line.location_id} onChange={(e) => updateLine(i, 'location_id', e.target.value)} required />
                      <input type="number" className="input" placeholder="System Qty" value={line.system_quantity} onChange={(e) => updateLine(i, 'system_quantity', parseInt(e.target.value) || 0)} required />
                      <input type="number" className="input" placeholder="Counted Qty" value={line.counted_quantity} onChange={(e) => updateLine(i, 'counted_quantity', parseInt(e.target.value) || 0)} required />
                      <input type="number" step="0.01" className="input" placeholder="Unit Cost ($)" value={line.unit_cost || ''} onChange={(e) => updateLine(i, 'unit_cost', parseFloat(e.target.value) || 0)} required />
                      <div className="flex items-center text-sm text-gray-600">
                        Adjustment: <span className={`font-bold ml-1 ${(line.counted_quantity - line.system_quantity) >= 0 ? 'text-green-600' : 'text-red-600'}`}>{line.counted_quantity - line.system_quantity}</span>
                      </div>
                    </div>
                  </div>
                ))}
                <button type="button" onClick={addLine} className="btn btn-secondary text-sm">Add Line</button>
              </div>

              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Creating...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {selectedAdjustment && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">Adjustment Details</h2>
            <div className="space-y-3 mb-4">
              <div className="flex justify-between">
                <span className="text-gray-600">Adjustment #:</span>
                <span className="font-mono">{selectedAdjustment.adjustment_number}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Type:</span>
                <span>{selectedAdjustment.adjustment_type}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Reason:</span>
                <span>{selectedAdjustment.reason}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Value Change:</span>
                <span className={`font-semibold ${selectedAdjustment.total_value_change >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                  {formatAmount(selectedAdjustment.total_value_change)}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Status:</span>
                {getStatusBadge(selectedAdjustment.status)}
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Created:</span>
                <span>{new Date(selectedAdjustment.created_at).toLocaleString()}</span>
              </div>
              {selectedAdjustment.notes && (
                <div className="flex justify-between">
                  <span className="text-gray-600">Notes:</span>
                  <span>{selectedAdjustment.notes}</span>
                </div>
              )}
            </div>

            <div className="flex gap-2 justify-end">
              <button onClick={() => setSelectedAdjustment(null)} className="btn btn-secondary">Close</button>
              {selectedAdjustment.status === 'Pending' && (
                <>
                  <button onClick={() => handleReject(selectedAdjustment)} className="btn btn-danger">Reject</button>
                  <button onClick={() => handleApprove(selectedAdjustment)} className="btn btn-primary">Approve</button>
                </>
              )}
              {selectedAdjustment.status === 'Draft' && (
                <>
                  <button onClick={() => handleCancel(selectedAdjustment)} className="btn btn-danger">Cancel</button>
                  <button onClick={() => { handleSubmit(selectedAdjustment); setSelectedAdjustment(null); }} className="btn btn-primary">Submit</button>
                </>
              )}
              {selectedAdjustment.status === 'Approved' && (
                <button onClick={() => { handleComplete(selectedAdjustment); setSelectedAdjustment(null); }} className="btn btn-success">Complete</button>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

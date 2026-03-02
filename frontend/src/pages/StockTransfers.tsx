import { useCallback, useEffect, useState } from 'react';
import { stockTransfers, type StockTransfer, type StockTransferAnalytics } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import { getErrorMessage } from '../utils/errors';

export default function StockTransfersPage() {
  const toast = useToast();
  const [transfers, setTransfers] = useState<StockTransfer[]>([]);
  const [analytics, setAnalytics] = useState<StockTransferAnalytics | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [search, setSearch] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [showShipModal, setShowShipModal] = useState(false);
  const [showReceiveModal, setShowReceiveModal] = useState(false);
  const [selectedTransfer, setSelectedTransfer] = useState<StockTransfer | null>(null);
  const [statusFilter, setStatusFilter] = useState<string>('');

  const [newTransfer, setNewTransfer] = useState({
    from_warehouse_id: '',
    to_warehouse_id: '',
    priority: 'Normal',
    notes: '',
    lines: [{ product_id: '', requested_quantity: 0, unit_cost: 0 }],
  });

  const [shipLines, setShipLines] = useState<{ product_id: string; shipped_quantity: number }[]>([]);
  const [receiveLines, setReceiveLines] = useState<{ product_id: string; received_quantity: number }[]>([]);

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [transRes, analyticsRes] = await Promise.all([
        stockTransfers.list(undefined, undefined, statusFilter || undefined),
        stockTransfers.getAnalytics(),
      ]);
      setTransfers(transRes.data.data);
      setAnalytics(analyticsRes.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load stock transfers'));
    } finally {
      setLoading(false);
    }
  }, [toast, statusFilter]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (newTransfer.from_warehouse_id === newTransfer.to_warehouse_id) {
      toast.error('Source and destination warehouses must be different');
      return;
    }
    try {
      setSaving(true);
      await stockTransfers.create({
        from_warehouse_id: newTransfer.from_warehouse_id,
        to_warehouse_id: newTransfer.to_warehouse_id,
        priority: newTransfer.priority,
        notes: newTransfer.notes || undefined,
        lines: newTransfer.lines.map(l => ({
          product_id: l.product_id,
          requested_quantity: l.requested_quantity,
          unit_cost: Math.round(l.unit_cost * 100),
        })),
      });
      toast.success('Transfer created successfully');
      setShowModal(false);
      setNewTransfer({
        from_warehouse_id: '',
        to_warehouse_id: '',
        priority: 'Normal',
        notes: '',
        lines: [{ product_id: '', requested_quantity: 0, unit_cost: 0 }],
      });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create transfer'));
    } finally {
      setSaving(false);
    }
  };

  const handleSubmit = async (t: StockTransfer) => {
    try {
      await stockTransfers.submit(t.id);
      toast.success('Transfer submitted for approval');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to submit transfer'));
    }
  };

  const handleApprove = async (t: StockTransfer) => {
    try {
      await stockTransfers.approve(t.id);
      toast.success('Transfer approved');
      setSelectedTransfer(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to approve transfer'));
    }
  };

  const handleReject = async (t: StockTransfer) => {
    try {
      await stockTransfers.reject(t.id, 'Rejected by reviewer');
      toast.success('Transfer rejected');
      setSelectedTransfer(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to reject transfer'));
    }
  };

  const handleCancel = async (t: StockTransfer) => {
    try {
      await stockTransfers.cancel(t.id);
      toast.success('Transfer cancelled');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to cancel transfer'));
    }
  };

  const handleDelete = async (t: StockTransfer) => {
    if (!confirm('Are you sure you want to delete this transfer?')) return;
    try {
      await stockTransfers.delete(t.id);
      toast.success('Transfer deleted');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to delete transfer'));
    }
  };

  const openShipModal = (t: StockTransfer) => {
    setSelectedTransfer(t);
    setShipLines((t.lines || []).map(l => ({ product_id: l.product_id, shipped_quantity: l.requested_quantity })));
    setShowShipModal(true);
  };

  const handleShip = async () => {
    if (!selectedTransfer) return;
    try {
      await stockTransfers.ship(selectedTransfer.id, shipLines);
      toast.success('Transfer shipped');
      setShowShipModal(false);
      setSelectedTransfer(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to ship transfer'));
    }
  };

  const openReceiveModal = (t: StockTransfer) => {
    setSelectedTransfer(t);
    setReceiveLines((t.lines || []).map(l => ({ product_id: l.product_id, received_quantity: l.shipped_quantity })));
    setShowReceiveModal(true);
  };

  const handleReceive = async () => {
    if (!selectedTransfer) return;
    try {
      await stockTransfers.receive(selectedTransfer.id, receiveLines);
      toast.success('Transfer received');
      setShowReceiveModal(false);
      setSelectedTransfer(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to receive transfer'));
    }
  };

  const addLine = () => {
    setNewTransfer({
      ...newTransfer,
      lines: [...newTransfer.lines, { product_id: '', requested_quantity: 0, unit_cost: 0 }],
    });
  };

  const removeLine = (index: number) => {
    if (newTransfer.lines.length > 1) {
      setNewTransfer({ ...newTransfer, lines: newTransfer.lines.filter((_, i) => i !== index) });
    }
  };

  const updateLine = (index: number, field: string, value: string | number) => {
    const lines = [...newTransfer.lines];
    lines[index] = { ...lines[index], [field]: value };
    setNewTransfer({ ...newTransfer, lines });
  };

  const filteredTransfers = transfers.filter(t =>
    t.transfer_number.toLowerCase().includes(search.toLowerCase()) ||
    t.status.toLowerCase().includes(search.toLowerCase())
  );

  const getStatusBadge = (status: string) => {
    const classes: Record<string, string> = {
      Draft: 'badge-default',
      Pending: 'badge-warning',
      Approved: 'badge-success',
      InTransit: 'badge-info',
      Received: 'badge-success',
      PartiallyReceived: 'badge-warning',
      Cancelled: 'badge-default',
    };
    return <span className={`badge ${classes[status] || 'badge-default'}`}>{status}</span>;
  };

  const getPriorityBadge = (priority: string) => {
    const classes: Record<string, string> = {
      Low: 'text-gray-600',
      Normal: 'text-gray-900',
      High: 'text-orange-600 font-semibold',
      Urgent: 'text-red-600 font-bold',
    };
    return <span className={classes[priority] || ''}>{priority}</span>;
  };

  const formatAmount = (amount: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(amount / 100);
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Stock Transfers</h1>
        <button onClick={() => setShowModal(true)} className="btn btn-primary">Create Transfer</button>
      </div>

      {analytics && (
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div className="card p-4">
            <div className="text-sm text-gray-600">Total Transfers</div>
            <div className="text-2xl font-bold">{analytics.total_transfers}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Pending Approval</div>
            <div className="text-2xl font-bold text-yellow-600">{analytics.pending_transfers}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">In Transit</div>
            <div className="text-2xl font-bold text-blue-600">{analytics.in_transit}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Completed</div>
            <div className="text-2xl font-bold text-green-600">{analytics.completed_transfers}</div>
          </div>
        </div>
      )}

      <div className="flex gap-4 mb-4">
        <div className="flex-1">
          <SearchInput value={search} onChange={setSearch} placeholder="Search transfers..." />
        </div>
        <select className="input w-40" value={statusFilter} onChange={(e) => setStatusFilter(e.target.value)}>
          <option value="">All Status</option>
          <option value="Draft">Draft</option>
          <option value="Pending">Pending</option>
          <option value="Approved">Approved</option>
          <option value="InTransit">In Transit</option>
          <option value="Received">Received</option>
          <option value="PartiallyReceived">Partially Received</option>
          <option value="Cancelled">Cancelled</option>
        </select>
      </div>

      <div className="card">
        {filteredTransfers.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {search || statusFilter ? 'No transfers match your filters' : 'No transfers found. Create one to get started.'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Transfer #</th>
                <th className="table-header">Priority</th>
                <th className="table-header">Status</th>
                <th className="table-header">Created</th>
                <th className="table-header">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredTransfers.map((t) => (
                <tr key={t.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{t.transfer_number}</td>
                  <td className="table-cell">{getPriorityBadge(t.priority)}</td>
                  <td className="table-cell">{getStatusBadge(t.status)}</td>
                  <td className="table-cell">{new Date(t.created_at).toLocaleDateString()}</td>
                  <td className="table-cell">
                    <div className="flex gap-1 flex-wrap">
                      <button onClick={() => setSelectedTransfer(t)} className="btn btn-secondary text-xs py-1">View</button>
                      {t.status === 'Draft' && (
                        <>
                          <button onClick={() => handleSubmit(t)} className="btn btn-primary text-xs py-1">Submit</button>
                          <button onClick={() => handleDelete(t)} className="btn btn-danger text-xs py-1">Delete</button>
                        </>
                      )}
                      {t.status === 'Approved' && (
                        <button onClick={() => openShipModal(t)} className="btn btn-primary text-xs py-1">Ship</button>
                      )}
                      {t.status === 'InTransit' && (
                        <button onClick={() => openReceiveModal(t)} className="btn btn-success text-xs py-1">Receive</button>
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
            <h2 className="text-lg font-semibold mb-4">Create Stock Transfer</h2>
            <form onSubmit={handleCreate} className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">From Warehouse ID</label>
                  <input className="input" value={newTransfer.from_warehouse_id} onChange={(e) => setNewTransfer({ ...newTransfer, from_warehouse_id: e.target.value })} required placeholder="Enter source warehouse UUID" />
                </div>
                <div>
                  <label className="label">To Warehouse ID</label>
                  <input className="input" value={newTransfer.to_warehouse_id} onChange={(e) => setNewTransfer({ ...newTransfer, to_warehouse_id: e.target.value })} required placeholder="Enter destination warehouse UUID" />
                </div>
              </div>

              <div>
                <label className="label">Priority</label>
                <select className="input" value={newTransfer.priority} onChange={(e) => setNewTransfer({ ...newTransfer, priority: e.target.value })}>
                  <option value="Low">Low</option>
                  <option value="Normal">Normal</option>
                  <option value="High">High</option>
                  <option value="Urgent">Urgent</option>
                </select>
              </div>

              <div>
                <label className="label">Notes</label>
                <input className="input" value={newTransfer.notes} onChange={(e) => setNewTransfer({ ...newTransfer, notes: e.target.value })} placeholder="Optional notes" />
              </div>

              <div>
                <label className="label">Transfer Lines</label>
                {newTransfer.lines.map((line, i) => (
                  <div key={i} className="border rounded p-3 mb-2">
                    <div className="flex justify-between items-center mb-2">
                      <span className="font-medium">Line {i + 1}</span>
                      {newTransfer.lines.length > 1 && (
                        <button type="button" onClick={() => removeLine(i)} className="text-red-600 text-sm">Remove</button>
                      )}
                    </div>
                    <div className="grid grid-cols-3 gap-2">
                      <input className="input" placeholder="Product ID" value={line.product_id} onChange={(e) => updateLine(i, 'product_id', e.target.value)} required />
                      <input type="number" className="input" placeholder="Quantity" value={line.requested_quantity || ''} onChange={(e) => updateLine(i, 'requested_quantity', parseInt(e.target.value) || 0)} required />
                      <input type="number" step="0.01" className="input" placeholder="Unit Cost ($)" value={line.unit_cost || ''} onChange={(e) => updateLine(i, 'unit_cost', parseFloat(e.target.value) || 0)} required />
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

      {selectedTransfer && !showShipModal && !showReceiveModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">Transfer Details</h2>
            <div className="space-y-3 mb-4">
              <div className="flex justify-between">
                <span className="text-gray-600">Transfer #:</span>
                <span className="font-mono">{selectedTransfer.transfer_number}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Priority:</span>
                {getPriorityBadge(selectedTransfer.priority)}
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Status:</span>
                {getStatusBadge(selectedTransfer.status)}
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Created:</span>
                <span>{new Date(selectedTransfer.created_at).toLocaleString()}</span>
              </div>
              {selectedTransfer.notes && (
                <div className="flex justify-between">
                  <span className="text-gray-600">Notes:</span>
                  <span>{selectedTransfer.notes}</span>
                </div>
              )}
            </div>

            {selectedTransfer.lines && selectedTransfer.lines.length > 0 && (
              <div className="mb-4">
                <h3 className="font-medium mb-2">Lines</h3>
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b">
                      <th className="text-left py-1">Product</th>
                      <th className="text-right py-1">Requested</th>
                      <th className="text-right py-1">Shipped</th>
                      <th className="text-right py-1">Received</th>
                    </tr>
                  </thead>
                  <tbody>
                    {selectedTransfer.lines.map((line, i) => (
                      <tr key={i} className="border-b">
                        <td className="py-1 font-mono text-xs">{line.product_id.slice(0, 8)}...</td>
                        <td className="text-right py-1">{line.requested_quantity}</td>
                        <td className="text-right py-1">{line.shipped_quantity}</td>
                        <td className="text-right py-1">{line.received_quantity}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}

            <div className="flex gap-2 justify-end">
              <button onClick={() => setSelectedTransfer(null)} className="btn btn-secondary">Close</button>
              {selectedTransfer.status === 'Pending' && (
                <>
                  <button onClick={() => handleReject(selectedTransfer)} className="btn btn-danger">Reject</button>
                  <button onClick={() => handleApprove(selectedTransfer)} className="btn btn-primary">Approve</button>
                </>
              )}
              {selectedTransfer.status === 'Draft' && (
                <>
                  <button onClick={() => handleCancel(selectedTransfer)} className="btn btn-danger">Cancel</button>
                  <button onClick={() => { handleSubmit(selectedTransfer); setSelectedTransfer(null); }} className="btn btn-primary">Submit</button>
                </>
              )}
            </div>
          </div>
        </div>
      )}

      {showShipModal && selectedTransfer && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">Ship Transfer: {selectedTransfer.transfer_number}</h2>
            <div className="space-y-4">
              {shipLines.map((line, i) => (
                <div key={i} className="flex gap-2 items-center">
                  <span className="flex-1 font-mono text-sm">{line.product_id.slice(0, 8)}...</span>
                  <input
                    type="number"
                    className="input w-24"
                    value={line.shipped_quantity}
                    onChange={(e) => {
                      const newLines = [...shipLines];
                      newLines[i] = { ...newLines[i], shipped_quantity: parseInt(e.target.value) || 0 };
                      setShipLines(newLines);
                    }}
                    min={0}
                  />
                </div>
              ))}
            </div>
            <div className="flex gap-2 justify-end mt-4">
              <button onClick={() => { setShowShipModal(false); setSelectedTransfer(null); }} className="btn btn-secondary">Cancel</button>
              <button onClick={handleShip} className="btn btn-primary">Ship</button>
            </div>
          </div>
        </div>
      )}

      {showReceiveModal && selectedTransfer && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">Receive Transfer: {selectedTransfer.transfer_number}</h2>
            <div className="space-y-4">
              {receiveLines.map((line, i) => (
                <div key={i} className="flex gap-2 items-center">
                  <span className="flex-1 font-mono text-sm">{line.product_id.slice(0, 8)}...</span>
                  <input
                    type="number"
                    className="input w-24"
                    value={line.received_quantity}
                    onChange={(e) => {
                      const newLines = [...receiveLines];
                      newLines[i] = { ...newLines[i], received_quantity: parseInt(e.target.value) || 0 };
                      setReceiveLines(newLines);
                    }}
                    min={0}
                  />
                </div>
              ))}
            </div>
            <div className="flex gap-2 justify-end mt-4">
              <button onClick={() => { setShowReceiveModal(false); setSelectedTransfer(null); }} className="btn btn-secondary">Cancel</button>
              <button onClick={handleReceive} className="btn btn-success">Receive</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

import { useEffect, useState, useCallback } from 'react';
import { payroll } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import type { PayrollRun, PayrollEntry } from '../types';
import { getErrorMessage } from '../utils/errors';
import { DollarSign } from 'lucide-react';

export default function PayrollManagement() {
  const toast = useToast();
  const [loading, setLoading] = useState(true);
  const [runs, setRuns] = useState<PayrollRun[]>([]);
  const [selectedRun, setSelectedRun] = useState<PayrollRun | null>(null);
  const [entries, setEntries] = useState<PayrollEntry[]>([]);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [creating, setCreating] = useState(false);

  const [newRun, setNewRun] = useState({
    pay_period_start: new Date().toISOString().split('T')[0],
    pay_period_end: new Date().toISOString().split('T')[0],
    pay_date: new Date().toISOString().split('T')[0],
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const runsRes = await payroll.listRuns();
      setRuns(runsRes.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load payroll data'));
    } finally {
      setLoading(false);
    }
  }, [toast]);
  useEffect(() => {
    loadData();
  }, [loadData]);
  const loadEntries = async (runId: string) => {
    try {
      const res = await payroll.listEntries(runId);
      setEntries(res.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load payroll entries'));
    }
  };
  const handleCreateRun = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setCreating(true);
      const res = await payroll.createRun(newRun);
      toast.success('Payroll run created successfully');
      setShowCreateModal(false);
      setRuns([res.data, ...runs]);
      setNewRun({
        pay_period_start: new Date().toISOString().split('T')[0],
        pay_period_end: new Date().toISOString().split('T')[0],
        pay_date: new Date().toISOString().split('T')[0],
      });
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create payroll run'));
    } finally {
      setCreating(false);
    }
  };
  const handleProcess = async (id: string) => {
    try {
      await payroll.processRun(id);
      toast.success('Payroll run processed');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to process payroll'));
    }
  };
  const handleApprove = async (id: string) => {
    try {
      await payroll.approveRun(id);
      toast.success('Payroll run approved');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to approve payroll'));
    }
  };
  const handlePay = async (id: string) => {
    try {
      await payroll.payRun(id);
      toast.success('Payroll run paid');
      loadData();
      if (selectedRun?.id === id) {
        setSelectedRun(null);
        setEntries([]);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to pay payroll'));
    }
  };
  const formatCurrency = (cents: number) => {
    return (cents / 100).toLocaleString('en-US', { style: 'currency', currency: 'USD' });
  };
  const formatDate = (dateStr: string) => {
    try {
      return new Date(dateStr).toLocaleDateString();
    } catch {
      return dateStr;
    }
  };
  const getStatusBadge = (status: string) => {
    const styles: Record<string, string> = {
      Draft: 'bg-gray-100 text-gray-800',
      Processing: 'bg-blue-100 text-blue-800',
      Approved: 'bg-green-100 text-green-800',
      Paid: 'bg-green-100 text-green-800',
    };
    return styles[status] || 'bg-gray-100 text-gray-800';
  };
  if (loading) return <LoadingPage />;
  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <div className="flex items-center gap-3">
          <DollarSign className="w-8 h-8 text-blue-600" />
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Payroll Management</h1>
          </div>
        </div>
        <button onClick={() => setShowCreateModal(true)} className="btn btn-primary">
          New Payroll Run
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 mb-6">
        <div className="card p-4">
          <div className="flex items-center gap-3">
            <DollarSign className="w-8 h-8 text-green-600" />
            <div>
              <div className="text-sm text-gray-500">Total Gross</div>
              <div className="text-2xl font-bold">
                {formatCurrency(runs.reduce((sum, r) => sum + r.total_gross, 0))}
              </div>
            </div>
          </div>
        </div>
        <div className="card p-4">
          <div className="flex items-center gap-3">
            <DollarSign className="w-8 h-8 text-red-600" />
            <div>
              <div className="text-sm text-gray-500">Total Deductions</div>
              <div className="text-2xl font-bold">
                {formatCurrency(runs.reduce((sum, r) => sum + r.total_deductions, 0))}
              </div>
            </div>
          </div>
        </div>
        <div className="card p-4">
          <div className="flex items-center gap-3">
            <DollarSign className="w-8 h-8 text-blue-600" />
            <div>
              <div className="text-sm text-gray-500">Total Net Pay</div>
              <div className="text-2xl font-bold">
                {formatCurrency(runs.reduce((sum, r) => sum + r.total_net, 0))}
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Payroll Runs</h2>
          </div>
          {runs.length === 0 ? (
            <div className="p-8 text-gray-500 text-center">No payroll runs found</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="text-left px-4 py-2">Run #</th>
                    <th className="text-left px-4 py-2">Period</th>
                    <th className="text-left px-4 py-2">Pay Date</th>
                    <th className="text-right px-4 py-2">Gross</th>
                    <th className="text-right px-4 py-2">Net</th>
                    <th className="text-left px-4 py-2">Status</th>
                    <th className="text-left px-4 py-2">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {runs.map((run) => (
                    <tr key={run.id} className="hover:bg-gray-50 border-t">
                      <td className="px-4 py-2 font-mono text-sm">{run.run_number}</td>
                      <td className="px-4 py-2">
                        {formatDate(run.pay_period_start)} - {formatDate(run.pay_period_end)}
                      </td>
                      <td className="px-4 py-2">{formatDate(run.pay_date)}</td>
                      <td className="px-4 py-2 text-right">{formatCurrency(run.total_gross)}</td>
                      <td className="px-4 py-2 text-right font-medium">{formatCurrency(run.total_net)}</td>
                      <td className="px-4 py-2">
                        <span className={`px-2 py-1 rounded text-xs ${getStatusBadge(run.status)}`}>
                          {run.status}
                        </span>
                      </td>
                      <td className="px-4 py-2">
                        <div className="flex gap-2">
                          {run.status === 'Draft' && (
                            <button
                              onClick={() => handleProcess(run.id)}
                              className="text-blue-600 hover:text-blue-800 text-sm"
                            >
                              Process
                            </button>
                          )}
                          {run.status === 'Processed' && (
                            <button
                              onClick={() => handleApprove(run.id)}
                              className="text-green-600 hover:text-green-800 text-sm"
                            >
                              Approve
                            </button>
                          )}
                          {run.status === 'Approved' && (
                            <button
                              onClick={() => handlePay(run.id)}
                              className="text-purple-600 hover:text-purple-800 text-sm"
                            >
                              Pay
                            </button>
                          )}
                          <button
                            onClick={() => {
                              setSelectedRun(run);
                              loadEntries(run.id);
                            }}
                            className="text-gray-600 hover:text-gray-800 text-sm"
                          >
                            View
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>

        <div className="card">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">
              {selectedRun ? `Entries - ${selectedRun.run_number}` : 'Payroll Entries'}
            </h2>
            {selectedRun && (
              <button
                onClick={() => {
                  setSelectedRun(null);
                  setEntries([]);
                }}
                className="text-sm text-gray-600"
              >
                Close
              </button>
            )}
          </div>
          {!selectedRun ? (
            <div className="p-8 text-gray-500 text-center">Select a payroll run to view entries</div>
          ) : entries.length === 0 ? (
            <div className="p-8 text-gray-500 text-center">No entries found</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="text-left px-4 py-2">Employee</th>
                    <th className="text-right px-4 py-2">Gross</th>
                    <th className="text-right px-4 py-2">Deductions</th>
                    <th className="text-right px-4 py-2 font-medium">Net Pay</th>
                    <th className="text-left px-4 py-2">Method</th>
                    <th className="text-left px-4 py-2">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {entries.map((entry) => (
                    <tr key={entry.id} className="hover:bg-gray-50 border-t">
                      <td className="px-4 py-2">{entry.employee_name}</td>
                      <td className="px-4 py-2 text-right">{formatCurrency(entry.gross_pay)}</td>
                      <td className="px-4 py-2 text-right">{formatCurrency(entry.total_deductions)}</td>
                      <td className="px-4 py-2 text-right font-medium">{formatCurrency(entry.net_pay)}</td>
                      <td className="px-4 py-2">{entry.payment_method}</td>
                      <td className="px-4 py-2">
                        <span className={`px-2 py-1 rounded text-xs ${entry.status === 'Paid' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'}`}>
                          {entry.status}
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>

      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Payroll Run</h2>
            <form onSubmit={handleCreateRun}>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Pay Period Start</label>
                  <input
                    type="date"
                    value={newRun.pay_period_start}
                    onChange={(e) => setNewRun({ ...newRun, pay_period_start: e.target.value })}
                    className="w-full border rounded px-3 py-2"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Pay Period End</label>
                  <input
                    type="date"
                    value={newRun.pay_period_end}
                    onChange={(e) => setNewRun({ ...newRun, pay_period_end: e.target.value })}
                    className="w-full border rounded px-3 py-2"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Pay Date</label>
                  <input
                    type="date"
                    value={newRun.pay_date}
                    onChange={(e) => setNewRun({ ...newRun, pay_date: e.target.value })}
                    className="w-full border rounded px-3 py-2"
                    required
                  />
                </div>
              </div>
              <div className="flex justify-end gap-2 mt-6">
                <button
                  type="button"
                  onClick={() => setShowCreateModal(false)}
                  className="btn btn-secondary"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={creating}
                  className="btn btn-primary"
                >
                  {creating ? 'Creating...' : 'Create'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

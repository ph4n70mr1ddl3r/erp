import { useEffect, useState, useCallback } from 'react';
import { expense, hr } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { getErrorMessage } from '../utils/errors';
import type { ExpenseReport, ExpenseCategory, Employee } from '../types';

export default function Expenses() {
  const toast = useToast();
  const [reports, setReports] = useState<ExpenseReport[]>([]);
  const [categories, setCategories] = useState<ExpenseCategory[]>([]);
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [rejectModal, setRejectModal] = useState<string | null>(null);
  const [rejectReason, setRejectReason] = useState('');
  const [newReport, setNewReport] = useState({
    employee_id: '',
    description: '',
    lines: [{ category_id: '', date: '', amount: 0, description: '' }]
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [reportsRes, categoriesRes, employeesRes] = await Promise.all([
        expense.listReports(),
        expense.listCategories(),
        hr.getEmployees(1, 100)
      ]);
      setReports(reportsRes.data);
      setCategories(categoriesRes.data);
      setEmployees(employeesRes.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load expense data'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => { loadData(); }, [loadData]);

  const handleAddLine = () => {
    setNewReport({
      ...newReport,
      lines: [...newReport.lines, { category_id: '', date: '', amount: 0, description: '' }]
    });
  };

  const handleRemoveLine = (index: number) => {
    if (newReport.lines.length > 1) {
      setNewReport({
        ...newReport,
        lines: newReport.lines.filter((_, i) => i !== index)
      });
    }
  };

  const handleLineChange = (index: number, field: string, value: string | number) => {
    const updatedLines = [...newReport.lines];
    updatedLines[index] = { ...updatedLines[index], [field]: value };
    setNewReport({ ...newReport, lines: updatedLines });
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newReport.employee_id) {
      toast.error('Please select an employee');
      return;
    }
    if (newReport.lines.some(l => !l.category_id || !l.date || l.amount <= 0)) {
      toast.error('Please fill in all expense line fields with valid amounts');
      return;
    }
    try {
      setSaving(true);
      await expense.createReport({
        employee_id: newReport.employee_id,
        title: newReport.description,
        lines: newReport.lines.map(l => ({
          category_id: l.category_id,
          expense_date: l.date,
          amount: Math.round(l.amount * 100),
          description: l.description || ''
        }))
      });
      toast.success('Expense report created successfully');
      setShowModal(false);
      setNewReport({
        employee_id: '',
        description: '',
        lines: [{ category_id: '', date: '', amount: 0, description: '' }]
      });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create expense report'));
    } finally {
      setSaving(false);
    }
  };

  const handleSubmit = async (id: string) => {
    try {
      await expense.submit(id);
      toast.success('Expense report submitted for approval');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to submit expense report'));
    }
  };

  const handleApprove = async (id: string) => {
    try {
      await expense.approve(id);
      toast.success('Expense report approved');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to approve expense report'));
    }
  };

  const handleReject = async (id: string) => {
    if (!rejectReason.trim()) {
      toast.error('Please provide a rejection reason');
      return;
    }
    try {
      await expense.reject(id, rejectReason);
      toast.success('Expense report rejected');
      setRejectModal(null);
      setRejectReason('');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to reject expense report'));
    }
  };

  const getStatusBadge = (status: string) => {
    const styles: Record<string, string> = {
      Draft: 'badge-default',
      Submitted: 'badge-warning',
      Approved: 'badge-success',
      Rejected: 'badge-danger',
      Paid: 'badge-info'
    };
    return styles[status] || 'badge-default';
  };

  const getEmployeeName = (id: string) => {
    const emp = employees.find(e => e.id === id);
    return emp ? `${emp.first_name} ${emp.last_name}` : id;
  };

  const formatCurrency = (cents: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(cents / 100);
  };

  const totalAmount = reports.reduce((sum, r) => sum + r.total_amount, 0);
  const pendingCount = reports.filter(r => r.status === 'Submitted').length;
  const approvedCount = reports.filter(r => r.status === 'Approved').length;

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Expense Management</h1>
        <button onClick={() => setShowModal(true)} className="btn btn-primary">New Expense Report</button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Expenses</p>
          <p className="text-2xl font-bold">{formatCurrency(totalAmount)}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Pending Approval</p>
          <p className="text-2xl font-bold text-yellow-600">{pendingCount}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Approved</p>
          <p className="text-2xl font-bold text-green-600">{approvedCount}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Reports</p>
          <p className="text-2xl font-bold">{reports.length}</p>
        </div>
      </div>

      <div className="card">
        <div className="p-4 border-b">
          <h2 className="text-lg font-semibold">Expense Reports</h2>
        </div>
        {reports.length === 0 ? (
          <div className="p-8 text-center text-gray-500">No expense reports found</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Report #</th>
                <th className="table-header">Employee</th>
                <th className="table-header">Description</th>
                <th className="table-header">Amount</th>
                <th className="table-header">Status</th>
                <th className="table-header">Actions</th>
              </tr>
            </thead>
            <tbody>
              {reports.map((report) => (
                <tr key={report.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono text-sm">{report.report_number}</td>
                  <td className="table-cell">{getEmployeeName(report.employee_id)}</td>
                  <td className="table-cell">{report.description}</td>
                  <td className="table-cell font-medium">{formatCurrency(report.total_amount)}</td>
                  <td className="table-cell">
                    <span className={`badge ${getStatusBadge(report.status)}`}>{report.status}</span>
                  </td>
                  <td className="table-cell">
                    {report.status === 'Draft' && (
                      <button
                        onClick={() => handleSubmit(report.id)}
                        className="btn btn-primary text-xs py-1"
                      >
                        Submit
                      </button>
                    )}
                    {report.status === 'Submitted' && (
                      <>
                        <button
                          onClick={() => handleApprove(report.id)}
                          className="btn btn-primary text-xs py-1 mr-1"
                        >
                          Approve
                        </button>
                        <button
                          onClick={() => setRejectModal(report.id)}
                          className="btn btn-secondary text-xs py-1"
                        >
                          Reject
                        </button>
                      </>
                    )}
                    {report.status !== 'Draft' && report.status !== 'Submitted' && (
                      <span className="text-gray-400 text-xs">No actions</span>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 overflow-y-auto">
          <div className="card p-6 w-full max-w-2xl my-8">
            <h2 className="text-lg font-semibold mb-4">New Expense Report</h2>
            <form onSubmit={handleCreate} className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Employee</label>
                  <select
                    className="input"
                    value={newReport.employee_id}
                    onChange={(e) => setNewReport({ ...newReport, employee_id: e.target.value })}
                    required
                  >
                    <option value="">Select employee</option>
                    {employees.map((emp) => (
                      <option key={emp.id} value={emp.id}>
                        {emp.first_name} {emp.last_name} ({emp.employee_number})
                      </option>
                    ))}
                  </select>
                </div>
                <div>
                  <label className="label">Description</label>
                  <input
                    type="text"
                    className="input"
                    value={newReport.description}
                    onChange={(e) => setNewReport({ ...newReport, description: e.target.value })}
                    placeholder="e.g., Business trip to NYC"
                    required
                  />
                </div>
              </div>

              <div>
                <div className="flex justify-between items-center mb-2">
                  <label className="label mb-0">Expense Lines</label>
                  <button type="button" onClick={handleAddLine} className="btn btn-secondary text-xs py-1">
                    Add Line
                  </button>
                </div>
                <div className="space-y-2">
                  {newReport.lines.map((line, index) => (
                    <div key={index} className="grid grid-cols-12 gap-2 items-start">
                      <div className="col-span-3">
                        <select
                          className="input text-sm"
                          value={line.category_id}
                          onChange={(e) => handleLineChange(index, 'category_id', e.target.value)}
                          required
                        >
                          <option value="">Category</option>
                          {categories.map((cat) => (
                            <option key={cat.id} value={cat.id}>
                              {cat.name}
                            </option>
                          ))}
                        </select>
                      </div>
                      <div className="col-span-2">
                        <input
                          type="date"
                          className="input text-sm"
                          value={line.date}
                          onChange={(e) => handleLineChange(index, 'date', e.target.value)}
                          required
                        />
                      </div>
                      <div className="col-span-2">
                        <input
                          type="number"
                          className="input text-sm"
                          placeholder="Amount"
                          min="0.01"
                          step="0.01"
                          value={line.amount || ''}
                          onChange={(e) => handleLineChange(index, 'amount', parseFloat(e.target.value) || 0)}
                          required
                        />
                      </div>
                      <div className="col-span-4">
                        <input
                          type="text"
                          className="input text-sm"
                          placeholder="Description"
                          value={line.description}
                          onChange={(e) => handleLineChange(index, 'description', e.target.value)}
                        />
                      </div>
                      <div className="col-span-1">
                        {newReport.lines.length > 1 && (
                          <button
                            type="button"
                            onClick={() => handleRemoveLine(index)}
                            className="btn btn-danger text-xs py-1 px-2"
                          >
                            X
                          </button>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              <div className="bg-gray-50 p-3 rounded">
                <p className="text-sm text-gray-600">
                  Total: <span className="font-bold">
                    {formatCurrency(newReport.lines.reduce((sum, l) => sum + Math.round(l.amount * 100), 0))}
                  </span>
                </p>
              </div>

              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create Report'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {rejectModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">Reject Expense Report</h2>
            <div className="space-y-4">
              <div>
                <label className="label">Rejection Reason</label>
                <textarea
                  className="input"
                  rows={3}
                  value={rejectReason}
                  onChange={(e) => setRejectReason(e.target.value)}
                  placeholder="Please provide a reason for rejection..."
                  required
                />
              </div>
              <div className="flex gap-2 justify-end">
                <button
                  onClick={() => { setRejectModal(null); setRejectReason(''); }}
                  className="btn btn-secondary"
                >
                  Cancel
                </button>
                <button
                  onClick={() => handleReject(rejectModal)}
                  className="btn btn-danger"
                >
                  Reject Report
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

import { useEffect, useState, useCallback } from 'react';
import { leave, hr } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { getErrorMessage } from '../utils/errors';
import type { LeaveType, LeaveRequest, Employee } from '../types';

export default function LeaveManagement() {
  const toast = useToast();
  const [leaveTypes, setLeaveTypes] = useState<LeaveType[]>([]);
  const [leaveRequests, setLeaveRequests] = useState<LeaveRequest[]>([]);
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [rejectModal, setRejectModal] = useState<string | null>(null);
  const [rejectReason, setRejectReason] = useState('');
  const [newRequest, setNewRequest] = useState({
    employee_id: '',
    leave_type_id: '',
    start_date: '',
    end_date: '',
    days: 1,
    reason: ''
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [typesRes, requestsRes, employeesRes] = await Promise.all([
        leave.listTypes(),
        leave.listRequests(),
        hr.getEmployees(1, 100)
      ]);
      setLeaveTypes(typesRes.data);
      setLeaveRequests(requestsRes.data);
      setEmployees(employeesRes.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load leave data'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => { loadData(); }, [loadData]);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await leave.createRequest({
        employee_id: newRequest.employee_id,
        leave_type_id: newRequest.leave_type_id,
        start_date: newRequest.start_date,
        end_date: newRequest.end_date,
        days: newRequest.days,
        reason: newRequest.reason || undefined
      });
      toast.success('Leave request submitted successfully');
      setShowModal(false);
      setNewRequest({ employee_id: '', leave_type_id: '', start_date: '', end_date: '', days: 1, reason: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to submit leave request'));
    } finally {
      setSaving(false);
    }
  };

  const handleApprove = async (id: string) => {
    try {
      await leave.approve(id);
      toast.success('Leave request approved');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to approve leave request'));
    }
  };

  const handleReject = async (id: string) => {
    if (!rejectReason.trim()) {
      toast.error('Please provide a rejection reason');
      return;
    }
    try {
      await leave.reject(id, rejectReason);
      toast.success('Leave request rejected');
      setRejectModal(null);
      setRejectReason('');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to reject leave request'));
    }
  };

  const getStatusBadge = (status: string) => {
    const styles: Record<string, string> = {
      Pending: 'badge-warning',
      Approved: 'badge-success',
      Rejected: 'badge-danger'
    };
    return styles[status] || 'badge-default';
  };

  const getEmployeeName = (id: string) => {
    const emp = employees.find(e => e.id === id);
    return emp ? `${emp.first_name} ${emp.last_name}` : id;
  };

  const getLeaveTypeName = (id: string) => {
    const type = leaveTypes.find(t => t.id === id);
    return type ? type.name : id;
  };

  const pendingCount = leaveRequests.filter(r => r.status === 'Pending').length;
  const approvedCount = leaveRequests.filter(r => r.status === 'Approved').length;

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Leave Management</h1>
        <button onClick={() => setShowModal(true)} className="btn btn-primary">New Leave Request</button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Leave Types</p>
          <p className="text-2xl font-bold">{leaveTypes.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Pending Requests</p>
          <p className="text-2xl font-bold text-yellow-600">{pendingCount}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Approved</p>
          <p className="text-2xl font-bold text-green-600">{approvedCount}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Requests</p>
          <p className="text-2xl font-bold">{leaveRequests.length}</p>
        </div>
      </div>

      <div className="card">
        <div className="p-4 border-b">
          <h2 className="text-lg font-semibold">Leave Requests</h2>
        </div>
        {leaveRequests.length === 0 ? (
          <div className="p-8 text-center text-gray-500">No leave requests found</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Employee</th>
                <th className="table-header">Leave Type</th>
                <th className="table-header">Start Date</th>
                <th className="table-header">End Date</th>
                <th className="table-header">Days</th>
                <th className="table-header">Status</th>
                <th className="table-header">Actions</th>
              </tr>
            </thead>
            <tbody>
              {leaveRequests.map((req) => (
                <tr key={req.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell">{getEmployeeName(req.employee_id)}</td>
                  <td className="table-cell">{getLeaveTypeName(req.leave_type_id)}</td>
                  <td className="table-cell">{req.start_date.split('T')[0]}</td>
                  <td className="table-cell">{req.end_date.split('T')[0]}</td>
                  <td className="table-cell">{req.days}</td>
                  <td className="table-cell">
                    <span className={`badge ${getStatusBadge(req.status)}`}>{req.status}</span>
                  </td>
                  <td className="table-cell">
                    {req.status === 'Pending' && (
                      <>
                        <button
                          onClick={() => handleApprove(req.id)}
                          className="btn btn-primary text-xs py-1 mr-1"
                        >
                          Approve
                        </button>
                        <button
                          onClick={() => setRejectModal(req.id)}
                          className="btn btn-secondary text-xs py-1"
                        >
                          Reject
                        </button>
                      </>
                    )}
                    {req.status !== 'Pending' && (
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
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Leave Request</h2>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="label">Employee</label>
                <select
                  className="input"
                  value={newRequest.employee_id}
                  onChange={(e) => setNewRequest({ ...newRequest, employee_id: e.target.value })}
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
                <label className="label">Leave Type</label>
                <select
                  className="input"
                  value={newRequest.leave_type_id}
                  onChange={(e) => setNewRequest({ ...newRequest, leave_type_id: e.target.value })}
                  required
                >
                  <option value="">Select leave type</option>
                  {leaveTypes.map((type) => (
                    <option key={type.id} value={type.id}>
                      {type.name} ({type.days_per_year} days/year)
                    </option>
                  ))}
                </select>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Start Date</label>
                  <input
                    type="date"
                    className="input"
                    value={newRequest.start_date}
                    onChange={(e) => setNewRequest({ ...newRequest, start_date: e.target.value })}
                    required
                  />
                </div>
                <div>
                  <label className="label">End Date</label>
                  <input
                    type="date"
                    className="input"
                    value={newRequest.end_date}
                    onChange={(e) => setNewRequest({ ...newRequest, end_date: e.target.value })}
                    required
                  />
                </div>
              </div>
              <div>
                <label className="label">Number of Days</label>
                <input
                  type="number"
                  className="input"
                  min="1"
                  value={newRequest.days}
                  onChange={(e) => setNewRequest({ ...newRequest, days: parseInt(e.target.value) || 1 })}
                  required
                />
              </div>
              <div>
                <label className="label">Reason (Optional)</label>
                <textarea
                  className="input"
                  rows={3}
                  value={newRequest.reason}
                  onChange={(e) => setNewRequest({ ...newRequest, reason: e.target.value })}
                />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Submitting...' : 'Submit Request'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {rejectModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">Reject Leave Request</h2>
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
                  Reject Request
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

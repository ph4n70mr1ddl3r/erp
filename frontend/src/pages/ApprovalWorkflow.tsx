import { useEffect, useState } from 'react';
import { approvalWorkflow, type ApprovalWorkflow as ApprovalWorkflowType, type ApprovalRequest } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';

export default function ApprovalWorkflowPage() {
  const toast = useToast();
  const [workflows, setWorkflows] = useState<ApprovalWorkflowType[]>([]);
  const [requests, setRequests] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [activeTab, setActiveTab] = useState<'workflows' | 'requests'>('workflows');
  const [search, setSearch] = useState('');
  const [showWorkflowModal, setShowWorkflowModal] = useState(false);
  const [selectedRequest, setSelectedRequest] = useState<ApprovalRequest | null>(null);

  const [newWorkflow, setNewWorkflow] = useState({
    code: '',
    name: '',
    description: '',
    document_type: 'PurchaseOrder',
    approval_type: 'Sequential',
    auto_approve_below: 0,
    levels: [{ name: 'Level 1', approver_type: 'SpecificUser', approver_ids: [] as string[], min_approvers: 1 }],
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [wfRes, reqRes] = await Promise.all([
        approvalWorkflow.listWorkflows(1, 50),
        approvalWorkflow.listRequests(1, 50),
      ]);
      setWorkflows(wfRes.data.items);
      setRequests(reqRes.data.items);
    } catch (err) {
      toast.error('Failed to load approval workflow data');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateWorkflow = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await approvalWorkflow.createWorkflow({
        code: newWorkflow.code,
        name: newWorkflow.name,
        description: newWorkflow.description || undefined,
        document_type: newWorkflow.document_type,
        approval_type: newWorkflow.approval_type,
        auto_approve_below: newWorkflow.auto_approve_below || undefined,
        levels: newWorkflow.levels.map((l) => ({
          name: l.name,
          approver_type: l.approver_type,
          approver_ids: l.approver_ids,
          min_approvers: l.min_approvers,
        })),
      });
      toast.success('Workflow created successfully');
      setShowWorkflowModal(false);
      setNewWorkflow({
        code: '',
        name: '',
        description: '',
        document_type: 'PurchaseOrder',
        approval_type: 'Sequential',
        auto_approve_below: 0,
        levels: [{ name: 'Level 1', approver_type: 'SpecificUser', approver_ids: [], min_approvers: 1 }],
      });
      loadData();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to create workflow');
    } finally {
      setSaving(false);
    }
  };

  const handleApproveRequest = async (request: ApprovalRequest) => {
    try {
      const userId = localStorage.getItem('userId') || '00000000-0000-0000-0000-000000000001';
      await approvalWorkflow.approveRequest(request.id, { approver_id: userId });
      toast.success('Request approved successfully');
      setSelectedRequest(null);
      loadData();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to approve request');
    }
  };

  const handleRejectRequest = async (request: ApprovalRequest, reason: string) => {
    try {
      const userId = localStorage.getItem('userId') || '00000000-0000-0000-0000-000000000001';
      await approvalWorkflow.rejectRequest(request.id, { approver_id: userId, reason });
      toast.success('Request rejected');
      setSelectedRequest(null);
      loadData();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to reject request');
    }
  };

  const addLevel = () => {
    setNewWorkflow({
      ...newWorkflow,
      levels: [...newWorkflow.levels, { name: `Level ${newWorkflow.levels.length + 1}`, approver_type: 'SpecificUser', approver_ids: [], min_approvers: 1 }],
    });
  };

  const removeLevel = (index: number) => {
    if (newWorkflow.levels.length > 1) {
      setNewWorkflow({ ...newWorkflow, levels: newWorkflow.levels.filter((_, i) => i !== index) });
    }
  };

  const updateLevel = (index: number, field: string, value: any) => {
    const levels = [...newWorkflow.levels];
    levels[index] = { ...levels[index], [field]: value };
    setNewWorkflow({ ...newWorkflow, levels });
  };

  const filteredWorkflows = workflows.filter(w =>
    w.name.toLowerCase().includes(search.toLowerCase()) ||
    w.code.toLowerCase().includes(search.toLowerCase()) ||
    w.document_type.toLowerCase().includes(search.toLowerCase())
  );

  const filteredRequests = requests.filter(r =>
    r.request_number.toLowerCase().includes(search.toLowerCase()) ||
    r.document_type.toLowerCase().includes(search.toLowerCase()) ||
    r.status.toLowerCase().includes(search.toLowerCase())
  );

  const getStatusBadge = (status: string) => {
    const classes: Record<string, string> = {
      Active: 'badge-success',
      Inactive: 'badge-warning',
      Pending: 'badge-warning',
      Approved: 'badge-success',
      Rejected: 'badge-danger',
      Cancelled: 'badge-default',
    };
    return <span className={`badge ${classes[status] || 'badge-default'}`}>{status}</span>;
  };

  const formatAmount = (amount: number, currency: string) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency }).format(amount / 100);
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Approval Workflows</h1>
        <button onClick={() => setShowWorkflowModal(true)} className="btn btn-primary">Create Workflow</button>
      </div>

      <div className="flex border-b mb-4">
        <button
          className={`px-4 py-2 font-medium ${activeTab === 'workflows' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-500'}`}
          onClick={() => setActiveTab('workflows')}
        >
          Workflows ({workflows.length})
        </button>
        <button
          className={`px-4 py-2 font-medium ${activeTab === 'requests' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-500'}`}
          onClick={() => setActiveTab('requests')}
        >
          Requests ({requests.length})
        </button>
      </div>

      <div className="mb-4">
        <SearchInput value={search} onChange={setSearch} placeholder={activeTab === 'workflows' ? 'Search workflows...' : 'Search requests...'} />
      </div>

      {activeTab === 'workflows' && (
        <div className="card">
          {filteredWorkflows.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              {search ? 'No workflows match your search' : 'No workflows found. Create one to get started.'}
            </div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Code</th>
                  <th className="table-header">Name</th>
                  <th className="table-header">Document Type</th>
                  <th className="table-header">Approval Type</th>
                  <th className="table-header">Levels</th>
                  <th className="table-header">Status</th>
                </tr>
              </thead>
              <tbody>
                {filteredWorkflows.map((wf) => (
                  <tr key={wf.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-mono">{wf.code}</td>
                    <td className="table-cell">{wf.name}</td>
                    <td className="table-cell">{wf.document_type}</td>
                    <td className="table-cell">{wf.approval_type}</td>
                    <td className="table-cell">{wf.levels.length}</td>
                    <td className="table-cell">{getStatusBadge(wf.status)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {activeTab === 'requests' && (
        <div className="card">
          {filteredRequests.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              {search ? 'No requests match your search' : 'No approval requests found.'}
            </div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Request #</th>
                  <th className="table-header">Document</th>
                  <th className="table-header">Type</th>
                  <th className="table-header">Amount</th>
                  <th className="table-header">Current Level</th>
                  <th className="table-header">Status</th>
                  <th className="table-header">Actions</th>
                </tr>
              </thead>
              <tbody>
                {filteredRequests.map((req) => (
                  <tr key={req.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-mono">{req.request_number}</td>
                    <td className="table-cell">{req.document_number}</td>
                    <td className="table-cell">{req.document_type}</td>
                    <td className="table-cell">{formatAmount(req.amount, req.currency)}</td>
                    <td className="table-cell">{req.current_level ?? '-'}</td>
                    <td className="table-cell">{getStatusBadge(req.status)}</td>
                    <td className="table-cell">
                      {req.status === 'Pending' && (
                        <button onClick={() => setSelectedRequest(req)} className="btn btn-primary text-xs py-1">
                          Review
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

      {showWorkflowModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <h2 className="text-lg font-semibold mb-4">Create Approval Workflow</h2>
            <form onSubmit={handleCreateWorkflow} className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Code</label>
                  <input className="input" value={newWorkflow.code} onChange={(e) => setNewWorkflow({ ...newWorkflow, code: e.target.value })} required />
                </div>
                <div>
                  <label className="label">Name</label>
                  <input className="input" value={newWorkflow.name} onChange={(e) => setNewWorkflow({ ...newWorkflow, name: e.target.value })} required />
                </div>
              </div>
              
              <div>
                <label className="label">Description</label>
                <input className="input" value={newWorkflow.description} onChange={(e) => setNewWorkflow({ ...newWorkflow, description: e.target.value })} />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Document Type</label>
                  <select className="input" value={newWorkflow.document_type} onChange={(e) => setNewWorkflow({ ...newWorkflow, document_type: e.target.value })}>
                    <option>PurchaseOrder</option>
                    <option>SalesOrder</option>
                    <option>Invoice</option>
                    <option>ExpenseReport</option>
                    <option>JournalEntry</option>
                    <option>Contract</option>
                  </select>
                </div>
                <div>
                  <label className="label">Approval Type</label>
                  <select className="input" value={newWorkflow.approval_type} onChange={(e) => setNewWorkflow({ ...newWorkflow, approval_type: e.target.value })}>
                    <option value="Sequential">Sequential</option>
                    <option value="AnyApprover">Any Approver</option>
                    <option value="AllApprovers">All Approvers</option>
                  </select>
                </div>
              </div>

              <div>
                <label className="label">Auto-approve Below Amount (cents)</label>
                <input type="number" className="input" value={newWorkflow.auto_approve_below} onChange={(e) => setNewWorkflow({ ...newWorkflow, auto_approve_below: parseInt(e.target.value) || 0 })} />
              </div>

              <div>
                <label className="label">Approval Levels</label>
                {newWorkflow.levels.map((level, i) => (
                  <div key={i} className="border rounded p-3 mb-2">
                    <div className="flex justify-between items-center mb-2">
                      <span className="font-medium">Level {i + 1}</span>
                      {newWorkflow.levels.length > 1 && (
                        <button type="button" onClick={() => removeLevel(i)} className="text-red-600 text-sm">Remove</button>
                      )}
                    </div>
                    <div className="grid grid-cols-2 gap-2">
                      <input className="input" placeholder="Level name" value={level.name} onChange={(e) => updateLevel(i, 'name', e.target.value)} required />
                      <select className="input" value={level.approver_type} onChange={(e) => updateLevel(i, 'approver_type', e.target.value)}>
                        <option value="SpecificUser">Specific User</option>
                        <option value="Role">Role</option>
                        <option value="Department">Department</option>
                        <option value="Supervisor">Supervisor</option>
                      </select>
                    </div>
                  </div>
                ))}
                <button type="button" onClick={addLevel} className="btn btn-secondary text-sm">Add Level</button>
              </div>

              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowWorkflowModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Creating...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {selectedRequest && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">Review Approval Request</h2>
            <div className="space-y-3 mb-4">
              <div className="flex justify-between">
                <span className="text-gray-600">Request #:</span>
                <span className="font-mono">{selectedRequest.request_number}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Document:</span>
                <span>{selectedRequest.document_number}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Type:</span>
                <span>{selectedRequest.document_type}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Amount:</span>
                <span className="font-semibold">{formatAmount(selectedRequest.amount, selectedRequest.currency)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-gray-600">Current Level:</span>
                <span>{selectedRequest.current_level ?? 'Final'}</span>
              </div>
            </div>

            {selectedRequest.approvals.length > 0 && (
              <div className="border-t pt-4 mb-4">
                <h3 className="font-medium mb-2">Approval History</h3>
                <div className="space-y-2">
                  {selectedRequest.approvals.map((a) => (
                    <div key={a.id} className="text-sm flex justify-between">
                      <span>Level {a.level_number}: {a.action}</span>
                      <span className="text-gray-500">{new Date(a.created_at).toLocaleDateString()}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            <div className="flex gap-2 justify-end">
              <button onClick={() => setSelectedRequest(null)} className="btn btn-secondary">Cancel</button>
              <button onClick={() => handleRejectRequest(selectedRequest, 'Rejected by reviewer')} className="btn btn-danger">Reject</button>
              <button onClick={() => handleApproveRequest(selectedRequest)} className="btn btn-primary">Approve</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

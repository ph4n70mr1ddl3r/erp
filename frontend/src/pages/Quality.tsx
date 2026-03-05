import { useCallback, useEffect, useState } from 'react';
import { quality, type QualityInspection, type NonConformanceReport, type QualityAnalytics } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import { getErrorMessage } from '../utils/errors';

type TabType = 'inspections' | 'ncrs';

export default function QualityPage() {
  const toast = useToast();
  const [activeTab, setActiveTab] = useState<TabType>('inspections');
  const [inspections, setInspections] = useState<QualityInspection[]>([]);
  const [ncrs, setNcrs] = useState<NonConformanceReport[]>([]);
  const [analytics, setAnalytics] = useState<QualityAnalytics | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [search, setSearch] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [statusFilter, setStatusFilter] = useState('');
  const [selectedItem, setSelectedItem] = useState<QualityInspection | NonConformanceReport | null>(null);

  const [newInspection, setNewInspection] = useState({
    inspection_type: 'Incoming',
    entity_type: 'Product',
    entity_id: '',
    inspector_id: '',
    inspection_date: new Date().toISOString().split('T')[0],
    notes: '',
    items: [{ criterion: '', expected_value: '', actual_value: '', pass_fail: undefined as boolean | undefined }],
  });

  const [newNCR, setNewNCR] = useState({
    source_type: 'IncomingInspection',
    source_id: '',
    product_id: '',
    description: '',
    severity: 'Minor',
    assigned_to: '',
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [inspRes, ncrRes, analyticsRes] = await Promise.all([
        quality.listInspections(statusFilter || undefined),
        quality.listNCRs(statusFilter || undefined),
        quality.getAnalytics(),
      ]);
      setInspections(inspRes.data.data);
      setNcrs(ncrRes.data.data);
      setAnalytics(analyticsRes.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load quality data'));
    } finally {
      setLoading(false);
    }
  }, [toast, statusFilter]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleCreateInspection = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await quality.createInspection({
        inspection_type: newInspection.inspection_type,
        entity_type: newInspection.entity_type,
        entity_id: newInspection.entity_id,
        inspector_id: newInspection.inspector_id || undefined,
        inspection_date: newInspection.inspection_date,
        notes: newInspection.notes || undefined,
        items: newInspection.items.map(i => ({
          criterion: i.criterion,
          expected_value: i.expected_value || undefined,
          actual_value: i.actual_value || undefined,
          pass_fail: i.pass_fail,
        })),
      });
      toast.success('Inspection created successfully');
      setShowModal(false);
      setNewInspection({
        inspection_type: 'Incoming',
        entity_type: 'Product',
        entity_id: '',
        inspector_id: '',
        inspection_date: new Date().toISOString().split('T')[0],
        notes: '',
        items: [{ criterion: '', expected_value: '', actual_value: '', pass_fail: undefined }],
      });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create inspection'));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateNCR = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await quality.createNCR({
        source_type: newNCR.source_type,
        source_id: newNCR.source_id || undefined,
        product_id: newNCR.product_id || undefined,
        description: newNCR.description,
        severity: newNCR.severity,
        assigned_to: newNCR.assigned_to || undefined,
      });
      toast.success('NCR created successfully');
      setShowModal(false);
      setNewNCR({
        source_type: 'IncomingInspection',
        source_id: '',
        product_id: '',
        description: '',
        severity: 'Minor',
        assigned_to: '',
      });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create NCR'));
    } finally {
      setSaving(false);
    }
  };

  const handleStartInspection = async (id: string) => {
    try {
      await quality.startInspection(id);
      toast.success('Inspection started');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to start inspection'));
    }
  };

  const handleCompleteInspection = async (id: string) => {
    try {
      await quality.completeInspection(id);
      toast.success('Inspection completed');
      setSelectedItem(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to complete inspection'));
    }
  };

  const handleCancelInspection = async (id: string) => {
    try {
      await quality.cancelInspection(id);
      toast.success('Inspection cancelled');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to cancel inspection'));
    }
  };

  const handleDeleteInspection = async (id: string) => {
    if (!confirm('Are you sure you want to delete this inspection?')) return;
    try {
      await quality.deleteInspection(id);
      toast.success('Inspection deleted');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to delete inspection'));
    }
  };

  const handleCloseNCR = async (id: string) => {
    try {
      await quality.closeNCR(id);
      toast.success('NCR closed');
      setSelectedItem(null);
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to close NCR'));
    }
  };

  const handleDeleteNCR = async (id: string) => {
    if (!confirm('Are you sure you want to delete this NCR?')) return;
    try {
      await quality.deleteNCR(id);
      toast.success('NCR deleted');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to delete NCR'));
    }
  };

  const addInspectionItem = () => {
    setNewInspection({
      ...newInspection,
      items: [...newInspection.items, { criterion: '', expected_value: '', actual_value: '', pass_fail: undefined }],
    });
  };

  const removeInspectionItem = (index: number) => {
    if (newInspection.items.length > 1) {
      setNewInspection({ ...newInspection, items: newInspection.items.filter((_, i) => i !== index) });
    }
  };

  const updateInspectionItem = (index: number, field: string, value: string | boolean) => {
    const items = [...newInspection.items];
    items[index] = { ...items[index], [field]: value };
    setNewInspection({ ...newInspection, items });
  };

  const filteredInspections = inspections.filter(i =>
    i.inspection_number.toLowerCase().includes(search.toLowerCase()) ||
    i.entity_type.toLowerCase().includes(search.toLowerCase()) ||
    i.status.toLowerCase().includes(search.toLowerCase())
  );

  const filteredNCRs = ncrs.filter(n =>
    n.ncr_number.toLowerCase().includes(search.toLowerCase()) ||
    n.description.toLowerCase().includes(search.toLowerCase()) ||
    n.status.toLowerCase().includes(search.toLowerCase())
  );

  const getStatusBadge = (status: string) => {
    const classes: Record<string, string> = {
      Pending: 'badge-warning',
      InProgress: 'badge-info',
      Passed: 'badge-success',
      Failed: 'badge-danger',
      Open: 'badge-warning',
      UnderInvestigation: 'badge-info',
      CorrectiveAction: 'badge-info',
      Closed: 'badge-success',
      Cancelled: 'badge-default',
    };
    return <span className={`badge ${classes[status] || 'badge-default'}`}>{status}</span>;
  };

  const getSeverityBadge = (severity: string) => {
    const classes: Record<string, string> = {
      Minor: 'badge-default',
      Major: 'badge-warning',
      Critical: 'badge-danger',
    };
    return <span className={`badge ${classes[severity] || 'badge-default'}`}>{severity}</span>;
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Quality Management</h1>
        <button onClick={() => setShowModal(true)} className="btn btn-primary">
          {activeTab === 'inspections' ? 'Create Inspection' : 'Create NCR'}
        </button>
      </div>

      {analytics && (
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div className="card p-4">
            <div className="text-sm text-gray-600">Total Inspections</div>
            <div className="text-2xl font-bold">{analytics.total_inspections}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Pass Rate</div>
            <div className="text-2xl font-bold text-green-600">{analytics.pass_rate.toFixed(1)}%</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Open NCRs</div>
            <div className="text-2xl font-bold text-yellow-600">{analytics.open_ncrs}</div>
          </div>
          <div className="card p-4">
            <div className="text-sm text-gray-600">Failed Inspections</div>
            <div className="text-2xl font-bold text-red-600">{analytics.failed_inspections}</div>
          </div>
        </div>
      )}

      <div className="flex gap-4 mb-4">
        <div className="flex border-b">
          <button
            onClick={() => setActiveTab('inspections')}
            className={`px-4 py-2 ${activeTab === 'inspections' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-600'}`}
          >
            Inspections ({inspections.length})
          </button>
          <button
            onClick={() => setActiveTab('ncrs')}
            className={`px-4 py-2 ${activeTab === 'ncrs' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-600'}`}
          >
            NCRs ({ncrs.length})
          </button>
        </div>
        <div className="flex-1">
          <SearchInput value={search} onChange={setSearch} placeholder={`Search ${activeTab}...`} />
        </div>
        <select className="input w-40" value={statusFilter} onChange={(e) => setStatusFilter(e.target.value)}>
          <option value="">All Status</option>
          {activeTab === 'inspections' ? (
            <>
              <option value="Pending">Pending</option>
              <option value="InProgress">In Progress</option>
              <option value="Passed">Passed</option>
              <option value="Failed">Failed</option>
            </>
          ) : (
            <>
              <option value="Open">Open</option>
              <option value="UnderInvestigation">Under Investigation</option>
              <option value="CorrectiveAction">Corrective Action</option>
              <option value="Closed">Closed</option>
            </>
          )}
        </select>
      </div>

      <div className="card">
        {activeTab === 'inspections' ? (
          filteredInspections.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              {search || statusFilter ? 'No inspections match your filters' : 'No inspections found. Create one to get started.'}
            </div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Inspection #</th>
                  <th className="table-header">Type</th>
                  <th className="table-header">Entity</th>
                  <th className="table-header">Date</th>
                  <th className="table-header">Status</th>
                  <th className="table-header">Result</th>
                  <th className="table-header">Actions</th>
                </tr>
              </thead>
              <tbody>
                {filteredInspections.map((insp) => (
                  <tr key={insp.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-mono">{insp.inspection_number}</td>
                    <td className="table-cell">{insp.inspection_type}</td>
                    <td className="table-cell">{insp.entity_type}</td>
                    <td className="table-cell">{new Date(insp.inspection_date).toLocaleDateString()}</td>
                    <td className="table-cell">{getStatusBadge(insp.status)}</td>
                    <td className="table-cell">{insp.result ? getStatusBadge(insp.result) : '-'}</td>
                    <td className="table-cell">
                      <div className="flex gap-1">
                        <button onClick={() => setSelectedItem(insp)} className="btn btn-secondary text-xs py-1">View</button>
                        {insp.status === 'Pending' && (
                          <>
                            <button onClick={() => handleStartInspection(insp.id)} className="btn btn-primary text-xs py-1">Start</button>
                            <button onClick={() => handleDeleteInspection(insp.id)} className="btn btn-danger text-xs py-1">Delete</button>
                          </>
                        )}
                        {insp.status === 'InProgress' && (
                          <button onClick={() => handleCompleteInspection(insp.id)} className="btn btn-success text-xs py-1">Complete</button>
                        )}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        ) : (
          filteredNCRs.length === 0 ? (
            <div className="p-8 text-center text-gray-500">
              {search || statusFilter ? 'No NCRs match your filters' : 'No NCRs found. Create one to get started.'}
            </div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">NCR #</th>
                  <th className="table-header">Source</th>
                  <th className="table-header">Description</th>
                  <th className="table-header">Severity</th>
                  <th className="table-header">Status</th>
                  <th className="table-header">Created</th>
                  <th className="table-header">Actions</th>
                </tr>
              </thead>
              <tbody>
                {filteredNCRs.map((ncr) => (
                  <tr key={ncr.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-mono">{ncr.ncr_number}</td>
                    <td className="table-cell">{ncr.source_type}</td>
                    <td className="table-cell">{ncr.description.substring(0, 50)}...</td>
                    <td className="table-cell">{getSeverityBadge(ncr.severity)}</td>
                    <td className="table-cell">{getStatusBadge(ncr.status)}</td>
                    <td className="table-cell">{new Date(ncr.created_at).toLocaleDateString()}</td>
                    <td className="table-cell">
                      <div className="flex gap-1">
                        <button onClick={() => setSelectedItem(ncr)} className="btn btn-secondary text-xs py-1">View</button>
                        {ncr.status === 'Open' && (
                          <button onClick={() => handleDeleteNCR(ncr.id)} className="btn btn-danger text-xs py-1">Delete</button>
                        )}
                        {ncr.status !== 'Closed' && ncr.corrective_action && (
                          <button onClick={() => handleCloseNCR(ncr.id)} className="btn btn-success text-xs py-1">Close</button>
                        )}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )
        )}
      </div>

      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-lg font-semibold">
                {activeTab === 'inspections' ? 'Create Quality Inspection' : 'Create Non-Conformance Report'}
              </h2>
              <div className="flex gap-2">
                <button
                  onClick={() => setActiveTab('inspections')}
                  className={`px-3 py-1 text-sm rounded ${activeTab === 'inspections' ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
                >
                  Inspection
                </button>
                <button
                  onClick={() => setActiveTab('ncrs')}
                  className={`px-3 py-1 text-sm rounded ${activeTab === 'ncrs' ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
                >
                  NCR
                </button>
              </div>
            </div>

            {activeTab === 'inspections' ? (
              <form onSubmit={handleCreateInspection} className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="label">Inspection Type</label>
                    <select className="input" value={newInspection.inspection_type} onChange={(e) => setNewInspection({ ...newInspection, inspection_type: e.target.value })}>
                      <option value="Incoming">Incoming</option>
                      <option value="InProcess">In-Process</option>
                      <option value="Final">Final</option>
                      <option value="Outgoing">Outgoing</option>
                      <option value="Supplier">Supplier</option>
                      <option value="Customer">Customer</option>
                    </select>
                  </div>
                  <div>
                    <label className="label">Entity Type</label>
                    <select className="input" value={newInspection.entity_type} onChange={(e) => setNewInspection({ ...newInspection, entity_type: e.target.value })}>
                      <option value="Product">Product</option>
                      <option value="Order">Order</option>
                      <option value="Shipment">Shipment</option>
                      <option value="Receipt">Receipt</option>
                    </select>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="label">Entity ID</label>
                    <input className="input" value={newInspection.entity_id} onChange={(e) => setNewInspection({ ...newInspection, entity_id: e.target.value })} required placeholder="Enter entity UUID" />
                  </div>
                  <div>
                    <label className="label">Inspector ID</label>
                    <input className="input" value={newInspection.inspector_id} onChange={(e) => setNewInspection({ ...newInspection, inspector_id: e.target.value })} placeholder="Optional employee UUID" />
                  </div>
                </div>

                <div>
                  <label className="label">Inspection Date</label>
                  <input type="date" className="input" value={newInspection.inspection_date} onChange={(e) => setNewInspection({ ...newInspection, inspection_date: e.target.value })} required />
                </div>

                <div>
                  <label className="label">Notes</label>
                  <input className="input" value={newInspection.notes} onChange={(e) => setNewInspection({ ...newInspection, notes: e.target.value })} placeholder="Optional notes" />
                </div>

                <div>
                  <label className="label">Inspection Items</label>
                  {newInspection.items.map((item, i) => (
                    <div key={i} className="border rounded p-3 mb-2">
                      <div className="flex justify-between items-center mb-2">
                        <span className="font-medium">Item {i + 1}</span>
                        {newInspection.items.length > 1 && (
                          <button type="button" onClick={() => removeInspectionItem(i)} className="text-red-600 text-sm">Remove</button>
                        )}
                      </div>
                      <div className="grid grid-cols-2 gap-2">
                        <input className="input" placeholder="Criterion" value={item.criterion} onChange={(e) => updateInspectionItem(i, 'criterion', e.target.value)} required />
                        <input className="input" placeholder="Expected Value" value={item.expected_value} onChange={(e) => updateInspectionItem(i, 'expected_value', e.target.value)} />
                      </div>
                    </div>
                  ))}
                  <button type="button" onClick={addInspectionItem} className="btn btn-secondary text-sm">Add Item</button>
                </div>

                <div className="flex gap-2 justify-end">
                  <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                  <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Creating...' : 'Create'}</button>
                </div>
              </form>
            ) : (
              <form onSubmit={handleCreateNCR} className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="label">Source Type</label>
                    <select className="input" value={newNCR.source_type} onChange={(e) => setNewNCR({ ...newNCR, source_type: e.target.value })}>
                      <option value="IncomingInspection">Incoming Inspection</option>
                      <option value="InProcessInspection">In-Process Inspection</option>
                      <option value="FinalInspection">Final Inspection</option>
                      <option value="CustomerComplaint">Customer Complaint</option>
                      <option value="InternalAudit">Internal Audit</option>
                      <option value="SupplierIssue">Supplier Issue</option>
                      <option value="ProductionIssue">Production Issue</option>
                      <option value="Other">Other</option>
                    </select>
                  </div>
                  <div>
                    <label className="label">Severity</label>
                    <select className="input" value={newNCR.severity} onChange={(e) => setNewNCR({ ...newNCR, severity: e.target.value })}>
                      <option value="Minor">Minor</option>
                      <option value="Major">Major</option>
                      <option value="Critical">Critical</option>
                    </select>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="label">Source ID (Optional)</label>
                    <input className="input" value={newNCR.source_id} onChange={(e) => setNewNCR({ ...newNCR, source_id: e.target.value })} placeholder="Related entity UUID" />
                  </div>
                  <div>
                    <label className="label">Product ID (Optional)</label>
                    <input className="input" value={newNCR.product_id} onChange={(e) => setNewNCR({ ...newNCR, product_id: e.target.value })} placeholder="Product UUID" />
                  </div>
                </div>

                <div>
                  <label className="label">Description</label>
                  <textarea className="input" rows={3} value={newNCR.description} onChange={(e) => setNewNCR({ ...newNCR, description: e.target.value })} required placeholder="Describe the non-conformance..." />
                </div>

                <div>
                  <label className="label">Assigned To (Optional)</label>
                  <input className="input" value={newNCR.assigned_to} onChange={(e) => setNewNCR({ ...newNCR, assigned_to: e.target.value })} placeholder="Employee UUID" />
                </div>

                <div className="flex gap-2 justify-end">
                  <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                  <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Creating...' : 'Create'}</button>
                </div>
              </form>
            )}
          </div>
        </div>
      )}

      {selectedItem && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-lg">
            <h2 className="text-lg font-semibold mb-4">
              {'inspection_number' in selectedItem ? 'Inspection Details' : 'NCR Details'}
            </h2>
            {'inspection_number' in selectedItem ? (
              <div className="space-y-3 mb-4">
                <div className="flex justify-between">
                  <span className="text-gray-600">Inspection #:</span>
                  <span className="font-mono">{(selectedItem as QualityInspection).inspection_number}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Type:</span>
                  <span>{(selectedItem as QualityInspection).inspection_type}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Entity:</span>
                  <span>{(selectedItem as QualityInspection).entity_type}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Date:</span>
                  <span>{new Date((selectedItem as QualityInspection).inspection_date).toLocaleDateString()}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Status:</span>
                  {getStatusBadge((selectedItem as QualityInspection).status)}
                </div>
                {(selectedItem as QualityInspection).result && (
                  <div className="flex justify-between">
                    <span className="text-gray-600">Result:</span>
                    {getStatusBadge((selectedItem as QualityInspection).result!)}
                  </div>
                )}
                {(selectedItem as QualityInspection).notes && (
                  <div className="flex justify-between">
                    <span className="text-gray-600">Notes:</span>
                    <span>{(selectedItem as QualityInspection).notes}</span>
                  </div>
                )}
              </div>
            ) : (
              <div className="space-y-3 mb-4">
                <div className="flex justify-between">
                  <span className="text-gray-600">NCR #:</span>
                  <span className="font-mono">{(selectedItem as NonConformanceReport).ncr_number}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Source:</span>
                  <span>{(selectedItem as NonConformanceReport).source_type}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Severity:</span>
                  {getSeverityBadge((selectedItem as NonConformanceReport).severity)}
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-600">Status:</span>
                  {getStatusBadge((selectedItem as NonConformanceReport).status)}
                </div>
                <div>
                  <span className="text-gray-600 block mb-1">Description:</span>
                  <p className="text-sm">{(selectedItem as NonConformanceReport).description}</p>
                </div>
                {(selectedItem as NonConformanceReport).root_cause && (
                  <div>
                    <span className="text-gray-600 block mb-1">Root Cause:</span>
                    <p className="text-sm">{(selectedItem as NonConformanceReport).root_cause}</p>
                  </div>
                )}
                {(selectedItem as NonConformanceReport).corrective_action && (
                  <div>
                    <span className="text-gray-600 block mb-1">Corrective Action:</span>
                    <p className="text-sm">{(selectedItem as NonConformanceReport).corrective_action}</p>
                  </div>
                )}
              </div>
            )}

            <div className="flex gap-2 justify-end">
              <button onClick={() => setSelectedItem(null)} className="btn btn-secondary">Close</button>
              {'inspection_number' in selectedItem && (selectedItem as QualityInspection).status === 'InProgress' && (
                <button onClick={() => handleCompleteInspection(selectedItem.id)} className="btn btn-success">Complete</button>
              )}
              {'inspection_number' in selectedItem && (selectedItem as QualityInspection).status === 'Pending' && (
                <>
                  <button onClick={() => { handleCancelInspection(selectedItem.id); setSelectedItem(null); }} className="btn btn-danger">Cancel</button>
                  <button onClick={() => { handleStartInspection(selectedItem.id); setSelectedItem(null); }} className="btn btn-primary">Start</button>
                </>
              )}
              {'ncr_number' in selectedItem && (selectedItem as NonConformanceReport).status !== 'Closed' && (selectedItem as NonConformanceReport).corrective_action && (
                <button onClick={() => handleCloseNCR(selectedItem.id)} className="btn btn-success">Close NCR</button>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

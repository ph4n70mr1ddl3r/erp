import { useEffect, useState } from 'react';
import { audit } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import { getErrorMessage } from '../utils/errors';

interface AuditLog {
  id: string;
  entity_type: string;
  entity_id: string;
  action: string;
  old_values: Record<string, unknown>;
  new_values: Record<string, unknown>;
  user_id: string | null;
  username: string | null;
  created_at: string;
}

export default function AuditLogs() {
  const toast = useToast();
  const [logs, setLogs] = useState<AuditLog[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [selectedLog, setSelectedLog] = useState<AuditLog | null>(null);
  const [entityTypeFilter, setEntityTypeFilter] = useState('');

  useEffect(() => { loadLogs(); }, [entityTypeFilter]);

  const loadLogs = async () => {
    try {
      setLoading(true);
      const params: { per_page: number; entity_type?: string } = { per_page: 100 };
      if (entityTypeFilter) params.entity_type = entityTypeFilter;
      const res = await audit.getLogs(params);
      setLogs(res.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load audit logs'));
    } finally {
      setLoading(false);
    }
  };

  const filteredLogs = logs.filter(l =>
    l.entity_type.toLowerCase().includes(search.toLowerCase()) ||
    l.action.toLowerCase().includes(search.toLowerCase()) ||
    (l.username && l.username.toLowerCase().includes(search.toLowerCase()))
  );

  const formatDate = (date: string) => {
    return new Date(date).toLocaleString();
  };

  const getActionColor = (action: string) => {
    switch (action) {
      case 'Create': return 'badge-success';
      case 'Update': return 'badge-info';
      case 'Delete': return 'badge-danger';
      case 'Post': return 'badge-success';
      case 'Approve': return 'badge-success';
      case 'Confirm': return 'badge-success';
      case 'Cancel': return 'badge-warning';
      default: return 'badge-info';
    }
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 mb-6">Audit Trail</h1>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Logs</p>
          <p className="text-2xl font-bold">{logs.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Creates</p>
          <p className="text-2xl font-bold text-green-600">{logs.filter(l => l.action === 'Create').length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Updates</p>
          <p className="text-2xl font-bold text-blue-600">{logs.filter(l => l.action === 'Update').length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Deletes</p>
          <p className="text-2xl font-bold text-red-600">{logs.filter(l => l.action === 'Delete').length}</p>
        </div>
      </div>

      <div className="card">
        <div className="p-4 border-b flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
          <div className="flex gap-4">
            <SearchInput value={search} onChange={setSearch} placeholder="Search logs..." />
            <select
              className="input w-40"
              value={entityTypeFilter}
              onChange={(e) => setEntityTypeFilter(e.target.value)}
            >
              <option value="">All Types</option>
              <option value="Account">Account</option>
              <option value="JournalEntry">Journal Entry</option>
              <option value="Product">Product</option>
              <option value="Warehouse">Warehouse</option>
              <option value="Customer">Customer</option>
              <option value="SalesOrder">Sales Order</option>
              <option value="Vendor">Vendor</option>
              <option value="PurchaseOrder">Purchase Order</option>
              <option value="Employee">Employee</option>
            </select>
          </div>
        </div>

        {filteredLogs.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {search || entityTypeFilter ? 'No logs match your filters' : 'No audit logs found. Activity will be logged as you use the system.'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Date/Time</th>
                <th className="table-header">Action</th>
                <th className="table-header">Entity</th>
                <th className="table-header">User</th>
                <th className="table-header">Details</th>
              </tr>
            </thead>
            <tbody>
              {filteredLogs.map((log) => (
                <tr key={log.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell text-sm">{formatDate(log.created_at)}</td>
                  <td className="table-cell">
                    <span className={`badge ${getActionColor(log.action)}`}>{log.action}</span>
                  </td>
                  <td className="table-cell">
                    <span className="font-medium">{log.entity_type}</span>
                    <span className="text-gray-400 text-xs ml-2">{log.entity_id.substring(0, 8)}...</span>
                  </td>
                  <td className="table-cell">{log.username || 'System'}</td>
                  <td className="table-cell">
                    <button
                      onClick={() => setSelectedLog(log)}
                      className="text-blue-600 hover:text-blue-800 text-sm"
                    >
                      View Changes
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Detail Modal */}
      {selectedLog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-lg font-semibold">Change Details</h2>
              <button onClick={() => setSelectedLog(null)} className="text-gray-400 hover:text-gray-600">&times;</button>
            </div>
            
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span className="text-gray-500">Entity Type:</span>
                  <span className="ml-2 font-medium">{selectedLog.entity_type}</span>
                </div>
                <div>
                  <span className="text-gray-500">Action:</span>
                  <span className={`ml-2 badge ${getActionColor(selectedLog.action)}`}>{selectedLog.action}</span>
                </div>
                <div>
                  <span className="text-gray-500">User:</span>
                  <span className="ml-2">{selectedLog.username || 'System'}</span>
                </div>
                <div>
                  <span className="text-gray-500">Time:</span>
                  <span className="ml-2">{formatDate(selectedLog.created_at)}</span>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                {selectedLog.old_values && (
                  <div>
                    <h3 className="font-medium text-red-700 mb-2">Old Values</h3>
                    <pre className="bg-red-50 p-3 rounded text-xs overflow-x-auto">
                      {JSON.stringify(selectedLog.old_values, null, 2)}
                    </pre>
                  </div>
                )}
                {selectedLog.new_values && (
                  <div>
                    <h3 className="font-medium text-green-700 mb-2">New Values</h3>
                    <pre className="bg-green-50 p-3 rounded text-xs overflow-x-auto">
                      {JSON.stringify(selectedLog.new_values, null, 2)}
                    </pre>
                  </div>
                )}
                {!selectedLog.old_values && !selectedLog.new_values && (
                  <div className="col-span-2 text-center text-gray-400 py-4">
                    No detailed change data available
                  </div>
                )}
              </div>
            </div>

            <div className="mt-6 flex justify-end">
              <button onClick={() => setSelectedLog(null)} className="btn btn-secondary">Close</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

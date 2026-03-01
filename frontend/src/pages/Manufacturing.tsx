import { useEffect, useState } from 'react';
import api from '../api/client';

interface BOM {
  id: string;
  bom_number: string;
  product_name: string;
  status: string;
  name: string;
  quantity: number;
  components?: unknown[];
}

interface WorkOrder {
  id: string;
  order_number: string;
  product_name: string;
  status: string;
  quantity: number;
}

export default function Manufacturing() {
  const [boms, setBoms] = useState<BOM[]>([]);
  const [workOrders, setWorkOrders] = useState<WorkOrder[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    Promise.all([
      api.get('/api/v1/manufacturing/boms?page=1&per_page=20'),
      api.get('/api/v1/manufacturing/work-orders?page=1&per_page=20'),
    ]).then(([bomsData, woData]) => {
      setBoms(bomsData.data.items || []);
      setWorkOrders(woData.data.items || []);
      setLoading(false);
    });
  }, []);

  const handleStart = async (id: string) => {
    await api.post(`/api/v1/manufacturing/work-orders/${id}/start`);
    window.location.reload();
  };

  const handleComplete = async (id: string) => {
    await api.post(`/api/v1/manufacturing/work-orders/${id}/complete`);
    window.location.reload();
  };

  if (loading) return <div className="text-center py-10">Loading...</div>;

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 mb-6">Manufacturing</h1>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4"><p className="text-sm text-gray-500">BOMs</p><p className="text-2xl font-bold">{boms.length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Work Orders</p><p className="text-2xl font-bold">{workOrders.length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">In Progress</p><p className="text-2xl font-bold">{workOrders.filter(o => o.status === 'Pending').length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Completed</p><p className="text-2xl font-bold">{workOrders.filter(o => o.status === 'Completed').length}</p></div>
      </div>

      <div className="card mb-6">
        <h2 className="text-lg font-semibold p-4 border-b">Work Orders</h2>
        <table className="w-full">
          <thead>
            <tr className="border-b">
              <th className="table-header">Order #</th>
              <th className="table-header">Quantity</th>
              <th className="table-header">Status</th>
              <th className="table-header">Action</th>
            </tr>
          </thead>
          <tbody>
            {workOrders.map((wo) => (
              <tr key={wo.id} className="border-b hover:bg-gray-50">
                <td className="table-cell font-mono">{wo.order_number}</td>
                <td className="table-cell">{wo.quantity}</td>
                <td className="table-cell">
                  <span className={`badge ${wo.status === 'Draft' ? 'badge-warning' : wo.status === 'Pending' ? 'badge-info' : 'badge-success'}`}>{wo.status}</span>
                </td>
                <td className="table-cell">
                  {wo.status === 'Draft' && <button onClick={() => handleStart(wo.id)} className="btn btn-primary text-xs py-1 mr-1">Start</button>}
                  {wo.status === 'Pending' && <button onClick={() => handleComplete(wo.id)} className="btn btn-secondary text-xs py-1">Complete</button>}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="card">
        <h2 className="text-lg font-semibold p-4 border-b">Bills of Materials</h2>
        <table className="w-full">
          <thead>
            <tr className="border-b">
              <th className="table-header">Name</th>
              <th className="table-header">Quantity</th>
              <th className="table-header">Components</th>
              <th className="table-header">Status</th>
            </tr>
          </thead>
          <tbody>
            {boms.map((bom) => (
              <tr key={bom.id} className="border-b hover:bg-gray-50">
                <td className="table-cell">{bom.name}</td>
                <td className="table-cell">{bom.quantity}</td>
                <td className="table-cell">{bom.components?.length || 0} items</td>
                <td className="table-cell"><span className={`badge ${bom.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>{bom.status}</span></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

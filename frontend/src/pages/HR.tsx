import { useEffect, useState } from 'react';
import { hr } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import type { Employee } from '../types';

export default function HR() {
  const toast = useToast();
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [search, setSearch] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [newEmployee, setNewEmployee] = useState({
    employee_number: '',
    first_name: '',
    last_name: '',
    email: '',
    hire_date: new Date().toISOString().split('T')[0]
  });

  useEffect(() => { loadData(); }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const res = await hr.getEmployees(1, 50);
      setEmployees(res.data.items);
    } catch (err) {
      toast.error('Failed to load employee data');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await hr.createEmployee(newEmployee);
      toast.success('Employee created successfully');
      setShowModal(false);
      setNewEmployee({ employee_number: '', first_name: '', last_name: '', email: '', hire_date: new Date().toISOString().split('T')[0] });
      loadData();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to create employee');
    } finally {
      setSaving(false);
    }
  };

  const handleCheckIn = async (id: string) => {
    try {
      await hr.checkIn(id);
      toast.success('Checked in successfully');
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to check in');
    }
  };

  const handleCheckOut = async (id: string) => {
    try {
      await hr.checkOut(id);
      toast.success('Checked out successfully');
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to check out');
    }
  };

  const filteredEmployees = employees.filter(e =>
    e.employee_number.toLowerCase().includes(search.toLowerCase()) ||
    e.first_name.toLowerCase().includes(search.toLowerCase()) ||
    e.last_name.toLowerCase().includes(search.toLowerCase())
  );

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Human Resources</h1>
        <button onClick={() => setShowModal(true)} className="btn btn-primary">Add Employee</button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4"><p className="text-sm text-gray-500">Total Employees</p><p className="text-2xl font-bold">{employees.length}</p></div>
        <div className="card p-4"><p className="text-sm text-gray-500">Active</p><p className="text-2xl font-bold">{employees.filter(e => e.status === 'Active').length}</p></div>
        <div className="card p-4 col-span-2"><p className="text-sm text-gray-500">Quick Actions</p><p className="text-sm mt-2">Use employee list below to check in/out</p></div>
      </div>

      <div className="card">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Employees</h2>
          <SearchInput value={search} onChange={setSearch} placeholder="Search employees..." />
        </div>
        {filteredEmployees.length === 0 ? (
          <div className="p-8 text-center text-gray-500">{search ? 'No employees match your search' : 'No employees found'}</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Employee #</th>
                <th className="table-header">Name</th>
                <th className="table-header">Email</th>
                <th className="table-header">Status</th>
                <th className="table-header">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredEmployees.map((emp) => (
                <tr key={emp.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{emp.employee_number}</td>
                  <td className="table-cell">{emp.first_name} {emp.last_name}</td>
                  <td className="table-cell">{emp.email}</td>
                  <td className="table-cell"><span className={`badge ${emp.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>{emp.status}</span></td>
                  <td className="table-cell">
                    <button onClick={() => handleCheckIn(emp.id)} className="btn btn-secondary text-xs py-1 mr-1">Check In</button>
                    <button onClick={() => handleCheckOut(emp.id)} className="btn btn-secondary text-xs py-1">Check Out</button>
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
            <h2 className="text-lg font-semibold mb-4">New Employee</h2>
            <form onSubmit={handleCreate} className="space-y-4">
              <div>
                <label className="label">Employee Number</label>
                <input className="input" value={newEmployee.employee_number} onChange={(e) => setNewEmployee({ ...newEmployee, employee_number: e.target.value })} required />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">First Name</label>
                  <input className="input" value={newEmployee.first_name} onChange={(e) => setNewEmployee({ ...newEmployee, first_name: e.target.value })} required />
                </div>
                <div>
                  <label className="label">Last Name</label>
                  <input className="input" value={newEmployee.last_name} onChange={(e) => setNewEmployee({ ...newEmployee, last_name: e.target.value })} required />
                </div>
              </div>
              <div>
                <label className="label">Email</label>
                <input type="email" className="input" value={newEmployee.email} onChange={(e) => setNewEmployee({ ...newEmployee, email: e.target.value })} required />
              </div>
              <div>
                <label className="label">Hire Date</label>
                <input type="date" className="input" value={newEmployee.hire_date} onChange={(e) => setNewEmployee({ ...newEmployee, hire_date: e.target.value })} required />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

import { useEffect, useState, useCallback } from 'react';
import { budget, finance } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import type { Budget, Account } from '../types';
import { getErrorMessage } from '../utils/errors';
import { PiggyBank } from 'lucide-react';

export default function Budgets() {
  const toast = useToast();
  const [budgets, setBudgets] = useState<Budget[]>([]);
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [search, setSearch] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [selectedBudget, setSelectedBudget] = useState<Budget | null>(null);

  const [newBudget, setNewBudget] = useState({
    name: '',
    start_date: '',
    end_date: '',
    lines: [{ account_id: '', period: 1, amount: 0 }],
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [budgetRes, accRes] = await Promise.all([
        budget.list(),
        finance.getAccounts(1, 100),
      ]);
      setBudgets(budgetRes.data);
      setAccounts(accRes.data.items.filter((a: Account) => a.account_type === 'Expense' || a.account_type === 'Revenue'));
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load budgets'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleCreateBudget = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newBudget.start_date || !newBudget.end_date) {
      toast.error('Please select start and end dates');
      return;
    }
    const validLines = newBudget.lines.filter(l => l.account_id && l.amount > 0);
    if (validLines.length === 0) {
      toast.error('Please add at least one budget line with amount');
      return;
    }
    try {
      setSaving(true);
      await budget.create({
        name: newBudget.name,
        start_date: newBudget.start_date,
        end_date: newBudget.end_date,
        lines: validLines.map(l => ({
          account_id: l.account_id,
          period: l.period,
          amount: Math.round(l.amount * 100),
        })),
      });
      toast.success('Budget created successfully');
      setShowModal(false);
      setNewBudget({ name: '', start_date: '', end_date: '', lines: [{ account_id: '', period: 1, amount: 0 }] });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const addLine = () => {
    setNewBudget({ ...newBudget, lines: [...newBudget.lines, { account_id: '', period: 1, amount: 0 }] });
  };

  const removeLine = (index: number) => {
    if (newBudget.lines.length > 1) {
      const lines = newBudget.lines.filter((_, i) => i !== index);
      setNewBudget({ ...newBudget, lines });
    }
  };

  const updateLine = (index: number, field: string, value: string | number) => {
    const lines = [...newBudget.lines];
    lines[index] = { ...lines[index], [field]: value };
    setNewBudget({ ...newBudget, lines });
  };

  const filteredBudgets = budgets.filter(b =>
    b.name.toLowerCase().includes(search.toLowerCase())
  );

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(amount);
  };

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString();
  };

  const getVarianceColor = (percent: number) => {
    if (percent >= 0) return 'text-green-600';
    if (percent >= -10) return 'text-yellow-600';
    return 'text-red-600';
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Budgets</h1>
        <button onClick={() => setShowModal(true)} className="btn btn-primary flex items-center gap-2">
          <PiggyBank className="w-4 h-4" />
          New Budget
        </button>
      </div>

      <div className="card mb-6">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Budget Overview</h2>
          <SearchInput value={search} onChange={setSearch} placeholder="Search budgets..." />
        </div>
        {filteredBudgets.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {search ? 'No budgets match your search' : 'No budgets found. Create your first budget to get started.'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Name</th>
                <th className="table-header">Period</th>
                <th className="table-header text-right">Budget</th>
                <th className="table-header text-right">Actual</th>
                <th className="table-header text-right">Variance</th>
                <th className="table-header">Status</th>
                <th className="table-header">Action</th>
              </tr>
            </thead>
            <tbody>
              {filteredBudgets.map((b) => (
                <tr key={b.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-medium">{b.name}</td>
                  <td className="table-cell text-sm">
                    {formatDate(b.start_date)} - {formatDate(b.end_date)}
                  </td>
                  <td className="table-cell text-right">{formatCurrency(b.total_amount)}</td>
                  <td className="table-cell text-right">{formatCurrency(b.total_actual)}</td>
                  <td className={`table-cell text-right font-medium ${getVarianceColor(b.variance_percent)}`}>
                    {formatCurrency(b.total_variance)}
                    <span className="text-xs ml-1">({b.variance_percent.toFixed(1)}%)</span>
                  </td>
                  <td className="table-cell">
                    <span className={`badge ${b.status === 'Active' ? 'badge-success' : b.status === 'Draft' ? 'badge-warning' : 'badge-info'}`}>
                      {b.status}
                    </span>
                  </td>
                  <td className="table-cell">
                    <button onClick={() => setSelectedBudget(b)} className="btn btn-secondary text-xs py-1">
                      View Details
                    </button>
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
            <h2 className="text-lg font-semibold mb-4">New Budget</h2>
            <form onSubmit={handleCreateBudget} className="space-y-4">
              <div>
                <label className="label">Budget Name</label>
                <input className="input" value={newBudget.name} onChange={(e) => setNewBudget({ ...newBudget, name: e.target.value })} placeholder="e.g., FY 2025 Operating Budget" required />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Start Date</label>
                  <input type="date" className="input" value={newBudget.start_date} onChange={(e) => setNewBudget({ ...newBudget, start_date: e.target.value })} required />
                </div>
                <div>
                  <label className="label">End Date</label>
                  <input type="date" className="input" value={newBudget.end_date} onChange={(e) => setNewBudget({ ...newBudget, end_date: e.target.value })} required />
                </div>
              </div>
              
              <div>
                <label className="label">Budget Lines</label>
                {newBudget.lines.map((line, i) => (
                  <div key={i} className="flex gap-2 mb-2 items-center">
                    <select className="input flex-1" value={line.account_id} onChange={(e) => updateLine(i, 'account_id', e.target.value)} required>
                      <option value="">Select Account</option>
                      {accounts.map((a) => <option key={a.id} value={a.id}>{a.code} - {a.name}</option>)}
                    </select>
                    <input type="number" className="input w-20" placeholder="Period" value={line.period} onChange={(e) => updateLine(i, 'period', parseInt(e.target.value) || 1)} min="1" max="12" />
                    <input type="number" className="input w-32" placeholder="Amount" value={line.amount || ''} onChange={(e) => updateLine(i, 'amount', parseFloat(e.target.value) || 0)} step="0.01" min="0" />
                    <button type="button" onClick={() => removeLine(i)} className="text-red-600 hover:text-red-800 px-2 text-xl">×</button>
                  </div>
                ))}
                <button type="button" onClick={addLine} className="btn btn-secondary text-sm mt-2">Add Line</button>
              </div>

              <div className="flex justify-between items-center text-sm text-gray-600 bg-gray-50 p-3 rounded">
                <span>Total Budget: {formatCurrency(newBudget.lines.reduce((s, l) => s + (l.amount || 0), 0))}</span>
              </div>

              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Creating...' : 'Create Budget'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {selectedBudget && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-4xl max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-lg font-semibold">{selectedBudget.name}</h2>
              <button onClick={() => setSelectedBudget(null)} className="text-gray-500 hover:text-gray-700 text-xl">×</button>
            </div>
            
            <div className="grid grid-cols-4 gap-4 mb-6">
              <div className="bg-gray-50 p-3 rounded">
                <p className="text-xs text-gray-500">Period</p>
                <p className="font-medium">{formatDate(selectedBudget.start_date)} - {formatDate(selectedBudget.end_date)}</p>
              </div>
              <div className="bg-gray-50 p-3 rounded">
                <p className="text-xs text-gray-500">Total Budget</p>
                <p className="font-medium">{formatCurrency(selectedBudget.total_amount)}</p>
              </div>
              <div className="bg-gray-50 p-3 rounded">
                <p className="text-xs text-gray-500">Actual</p>
                <p className="font-medium">{formatCurrency(selectedBudget.total_actual)}</p>
              </div>
              <div className="bg-gray-50 p-3 rounded">
                <p className="text-xs text-gray-500">Variance</p>
                <p className={`font-medium ${getVarianceColor(selectedBudget.variance_percent)}`}>
                  {formatCurrency(selectedBudget.total_variance)} ({selectedBudget.variance_percent.toFixed(1)}%)
                </p>
              </div>
            </div>

            {selectedBudget.lines.length > 0 ? (
              <table className="w-full">
                <thead>
                  <tr className="border-b">
                    <th className="table-header">Account</th>
                    <th className="table-header text-center">Period</th>
                    <th className="table-header text-right">Budget</th>
                    <th className="table-header text-right">Actual</th>
                    <th className="table-header text-right">Variance</th>
                    <th className="table-header text-right">Var %</th>
                  </tr>
                </thead>
                <tbody>
                  {selectedBudget.lines.map((line, i) => (
                    <tr key={i} className="border-b hover:bg-gray-50">
                      <td className="table-cell">
                        <span className="font-mono text-xs text-gray-500">{line.account_code}</span>
                        <span className="ml-2">{line.account_name}</span>
                      </td>
                      <td className="table-cell text-center">{line.period}</td>
                      <td className="table-cell text-right">{formatCurrency(line.budget_amount)}</td>
                      <td className="table-cell text-right">{formatCurrency(line.actual_amount)}</td>
                      <td className={`table-cell text-right ${getVarianceColor(line.variance_percent)}`}>
                        {formatCurrency(line.variance)}
                      </td>
                      <td className={`table-cell text-right ${getVarianceColor(line.variance_percent)}`}>
                        {line.variance_percent.toFixed(1)}%
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <p className="text-center text-gray-500 py-4">No budget lines</p>
            )}

            <div className="flex justify-end mt-4">
              <button onClick={() => setSelectedBudget(null)} className="btn btn-secondary">Close</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

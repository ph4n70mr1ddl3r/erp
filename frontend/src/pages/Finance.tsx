import { useEffect, useState } from 'react';
import { finance } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import type { Account, JournalEntry } from '../types';

export default function Finance() {
  const toast = useToast();
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [entries, setEntries] = useState<JournalEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [accountSearch, setAccountSearch] = useState('');
  const [entrySearch, setEntrySearch] = useState('');
  const [showAccountModal, setShowAccountModal] = useState(false);
  const [showJEModal, setShowJEModal] = useState(false);

  const [newAccount, setNewAccount] = useState({ code: '', name: '', account_type: 'Asset' });
  const [newEntry, setNewEntry] = useState({ 
    description: '', 
    reference: '', 
    lines: [{ account_id: '', debit: 0, credit: 0 }] 
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [accRes, jeRes] = await Promise.all([
        finance.getAccounts(1, 50),
        finance.getJournalEntries(1, 20),
      ]);
      setAccounts(accRes.data.items);
      setEntries(jeRes.data.items);
    } catch (err) {
      toast.error('Failed to load finance data');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateAccount = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await finance.createAccount(newAccount);
      toast.success('Account created successfully');
      setShowAccountModal(false);
      setNewAccount({ code: '', name: '', account_type: 'Asset' });
      loadData();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to create account');
    } finally {
      setSaving(false);
    }
  };

  const handleCreateEntry = async (e: React.FormEvent) => {
    e.preventDefault();
    const totalDebits = newEntry.lines.reduce((sum, l) => sum + (l.debit || 0), 0);
    const totalCredits = newEntry.lines.reduce((sum, l) => sum + (l.credit || 0), 0);
    if (Math.abs(totalDebits - totalCredits) > 0.01) {
      toast.error('Journal entry must balance (debits must equal credits)');
      return;
    }
    try {
      setSaving(true);
      await finance.createJournalEntry({
        description: newEntry.description,
        reference: newEntry.reference,
        lines: newEntry.lines.map(l => ({
          ...l,
          debit: l.debit * 100,
          credit: l.credit * 100,
        })),
      });
      toast.success('Journal entry created successfully');
      setShowJEModal(false);
      setNewEntry({ description: '', reference: '', lines: [{ account_id: '', debit: 0, credit: 0 }] });
      loadData();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to create journal entry');
    } finally {
      setSaving(false);
    }
  };

  const handlePostEntry = async (id: string) => {
    try {
      await finance.postJournalEntry(id);
      toast.success('Journal entry posted successfully');
      loadData();
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to post journal entry');
    }
  };

  const addLine = () => {
    setNewEntry({ ...newEntry, lines: [...newEntry.lines, { account_id: '', debit: 0, credit: 0 }] });
  };

  const removeLine = (index: number) => {
    if (newEntry.lines.length > 1) {
      const lines = newEntry.lines.filter((_, i) => i !== index);
      setNewEntry({ ...newEntry, lines });
    }
  };

  const updateLine = (index: number, field: string, value: any) => {
    const lines = [...newEntry.lines];
    lines[index] = { ...lines[index], [field]: value };
    setNewEntry({ ...newEntry, lines });
  };

  const filteredAccounts = accounts.filter(a => 
    a.code.toLowerCase().includes(accountSearch.toLowerCase()) ||
    a.name.toLowerCase().includes(accountSearch.toLowerCase())
  );

  const filteredEntries = entries.filter(e => 
    e.entry_number.toLowerCase().includes(entrySearch.toLowerCase()) ||
    e.description.toLowerCase().includes(entrySearch.toLowerCase())
  );

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Finance</h1>
        <div className="flex gap-2">
          <button onClick={() => setShowAccountModal(true)} className="btn btn-primary">Add Account</button>
          <button onClick={() => setShowJEModal(true)} className="btn btn-secondary">Journal Entry</button>
        </div>
      </div>

      {/* Chart of Accounts */}
      <div className="card mb-6">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Chart of Accounts</h2>
          <SearchInput value={accountSearch} onChange={setAccountSearch} placeholder="Search accounts..." />
        </div>
        {filteredAccounts.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {accountSearch ? 'No accounts match your search' : 'No accounts found'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Code</th>
                <th className="table-header">Name</th>
                <th className="table-header">Type</th>
                <th className="table-header">Status</th>
              </tr>
            </thead>
            <tbody>
              {filteredAccounts.map((acc) => (
                <tr key={acc.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{acc.code}</td>
                  <td className="table-cell">{acc.name}</td>
                  <td className="table-cell">{acc.account_type}</td>
                  <td className="table-cell">
                    <span className={`badge ${acc.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>
                      {acc.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Journal Entries */}
      <div className="card">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Journal Entries</h2>
          <SearchInput value={entrySearch} onChange={setEntrySearch} placeholder="Search entries..." />
        </div>
        {filteredEntries.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {entrySearch ? 'No entries match your search' : 'No journal entries found'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Number</th>
                <th className="table-header">Date</th>
                <th className="table-header">Description</th>
                <th className="table-header">Amount</th>
                <th className="table-header">Status</th>
                <th className="table-header">Action</th>
              </tr>
            </thead>
            <tbody>
              {filteredEntries.map((entry) => (
                <tr key={entry.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{entry.entry_number}</td>
                  <td className="table-cell">{new Date(entry.date).toLocaleDateString()}</td>
                  <td className="table-cell">{entry.description}</td>
                  <td className="table-cell">${entry.total_debit.toFixed(2)}</td>
                  <td className="table-cell">
                    <span className={`badge ${entry.status === 'Draft' ? 'badge-warning' : entry.status === 'Completed' ? 'badge-success' : 'badge-info'}`}>
                      {entry.status}
                    </span>
                  </td>
                  <td className="table-cell">
                    {entry.status === 'Draft' && (
                      <button onClick={() => handlePostEntry(entry.id)} className="btn btn-primary text-xs py-1">
                        Post
                      </button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Account Modal */}
      {showAccountModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Account</h2>
            <form onSubmit={handleCreateAccount} className="space-y-4">
              <div>
                <label className="label">Code</label>
                <input className="input" value={newAccount.code} onChange={(e) => setNewAccount({ ...newAccount, code: e.target.value })} required />
              </div>
              <div>
                <label className="label">Name</label>
                <input className="input" value={newAccount.name} onChange={(e) => setNewAccount({ ...newAccount, name: e.target.value })} required />
              </div>
              <div>
                <label className="label">Type</label>
                <select className="input" value={newAccount.account_type} onChange={(e) => setNewAccount({ ...newAccount, account_type: e.target.value })}>
                  <option>Asset</option>
                  <option>Liability</option>
                  <option>Equity</option>
                  <option>Revenue</option>
                  <option>Expense</option>
                </select>
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowAccountModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Journal Entry Modal */}
      {showJEModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <h2 className="text-lg font-semibold mb-4">New Journal Entry</h2>
            <form onSubmit={handleCreateEntry} className="space-y-4">
              <div>
                <label className="label">Description</label>
                <input className="input" value={newEntry.description} onChange={(e) => setNewEntry({ ...newEntry, description: e.target.value })} required />
              </div>
              <div>
                <label className="label">Reference</label>
                <input className="input" value={newEntry.reference} onChange={(e) => setNewEntry({ ...newEntry, reference: e.target.value })} />
              </div>
              
              <div>
                <label className="label">Lines</label>
                {newEntry.lines.map((line, i) => (
                  <div key={i} className="flex gap-2 mb-2">
                    <select className="input flex-1" value={line.account_id} onChange={(e) => updateLine(i, 'account_id', e.target.value)} required>
                      <option value="">Select Account</option>
                      {accounts.map((a) => <option key={a.id} value={a.id}>{a.code} - {a.name}</option>)}
                    </select>
                    <input type="number" className="input w-24" placeholder="Debit" value={line.debit || ''} onChange={(e) => updateLine(i, 'debit', parseFloat(e.target.value) || 0)} step="0.01" min="0" />
                    <input type="number" className="input w-24" placeholder="Credit" value={line.credit || ''} onChange={(e) => updateLine(i, 'credit', parseFloat(e.target.value) || 0)} step="0.01" min="0" />
                    <button type="button" onClick={() => removeLine(i)} className="text-red-600 hover:text-red-800 px-2">×</button>
                  </div>
                ))}
                <button type="button" onClick={addLine} className="btn btn-secondary text-sm">Add Line</button>
              </div>

              <div className="flex justify-between items-center text-sm text-gray-600 bg-gray-50 p-2 rounded">
                <span>Total Debits: ${newEntry.lines.reduce((s, l) => s + (l.debit || 0), 0).toFixed(2)}</span>
                <span>Total Credits: ${newEntry.lines.reduce((s, l) => s + (l.credit || 0), 0).toFixed(2)}</span>
              </div>

              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowJEModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

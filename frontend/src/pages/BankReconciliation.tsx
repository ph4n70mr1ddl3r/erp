import { useEffect, useState, useCallback } from 'react';
import { bank } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import type { BankAccount, BankStatement, BankTransaction, ReconciliationSession, ReconciliationSummary } from '../types';
import { getErrorMessage } from '../utils/errors';

export default function BankReconciliation() {
  const toast = useToast();
  const [loading, setLoading] = useState(true);
  const [accounts, setAccounts] = useState<BankAccount[]>([]);
  const [selectedAccount, setSelectedAccount] = useState<BankAccount | null>(null);
  const [statements, setStatements] = useState<BankStatement[]>([]);
  const [transactions, setTransactions] = useState<BankTransaction[]>([]);
  const [reconciliations, setReconciliations] = useState<ReconciliationSession[]>([]);
  const [summary, setSummary] = useState<ReconciliationSummary | null>(null);
  const [showImportModal, setShowImportModal] = useState(false);
  const [activeTab, setActiveTab] = useState<'accounts' | 'statements' | 'transactions' | 'reconciliations'>('accounts');
  
  const [importData, setImportData] = useState({
    bank_account_id: '',
    statement_date: new Date().toISOString().split('T')[0],
    currency: 'USD',
    opening_balance: 0,
    closing_balance: 0,
    transactions: [] as Array<{
      transaction_date: string;
      transaction_type: string;
      amount: number;
      description: string;
      reference_number?: string;
      payee_name?: string;
    }>
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const accountsRes = await bank.listAccounts();
      setAccounts(accountsRes.data);
      if (accountsRes.data.length > 0 && !selectedAccount) {
        setSelectedAccount(accountsRes.data[0]);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load bank data'));
    } finally {
      setLoading(false);
    }
  }, [toast, selectedAccount]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  useEffect(() => {
    if (selectedAccount) {
      bank.listStatements({ account_id: selectedAccount.id }).then(res => setStatements(res.data)).catch(() => setStatements([]));
      bank.getSummary(selectedAccount.id).then(res => setSummary(res.data)).catch(() => setSummary(null));
      bank.listReconciliations({ account_id: selectedAccount.id }).then(res => setReconciliations(res.data)).catch(() => setReconciliations([]));
      bank.listTransactions({ account_id: selectedAccount.id }).then(res => setTransactions(res.data)).catch(() => setTransactions([]));
    }
  }, [selectedAccount]);

  const handleImportStatement = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!importData.transactions.length) {
      toast.error('Please add at least one transaction');
      return;
    }
    try {
      await bank.importStatement({ ...importData, bank_account_id: selectedAccount?.id || '' });
      toast.success('Statement imported successfully');
      setShowImportModal(false);
      setImportData({
        bank_account_id: '',
        statement_date: new Date().toISOString().split('T')[0],
        currency: 'USD',
        opening_balance: 0,
        closing_balance: 0,
        transactions: []
      });
      if (selectedAccount) {
        bank.listStatements({ account_id: selectedAccount.id }).then(res => setStatements(res.data));
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to import statement'));
    }
  };

  const handleReconcile = async (id: string) => {
    try {
      await bank.reconcileTransaction(id, {});
      toast.success('Transaction reconciled');
      if (selectedAccount) {
        const res = await bank.listTransactions({ account_id: selectedAccount.id });
        setTransactions(res.data);
        const sumRes = await bank.getSummary(selectedAccount.id);
        setSummary(sumRes.data);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to reconcile'));
    }
  };

  const handleAutoMatch = async (sessionId: string) => {
    try {
      const result = await bank.autoMatchTransactions(sessionId);
      toast.success(`Auto-matched ${result.matched_count} transactions`);
      if (selectedAccount) {
        const [txRes, sumRes, recRes] = await Promise.all([
          bank.listTransactions({ account_id: selectedAccount.id }),
          bank.getSummary(selectedAccount.id),
          bank.getReconciliation(sessionId),
        ]);
        setTransactions(txRes.data);
        setSummary(sumRes.data);
        setReconciliations([recRes.data, ...reconciliations.filter(r => r.id !== sessionId)]);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Auto-match failed'));
    }
  };

  const handleStartReconciliation = async () => {
    if (!selectedAccount) return;
    try {
      const today = new Date();
      const thirtyDaysAgo = new Date(today);
      thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30);
      
      await bank.startReconciliation({
        bank_account_id: selectedAccount.id,
        period_start: thirtyDaysAgo.toISOString().split('T')[0],
        period_end: today.toISOString().split('T')[0],
        opening_balance: summary?.gl_balance || 0,
        closing_balance: summary?.bank_balance || 0,
      });
      toast.success('Reconciliation session started');
      if (selectedAccount) {
        bank.listReconciliations({ account_id: selectedAccount.id }).then(res => setReconciliations(res.data));
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to start reconciliation'));
    }
  };

  const handleCompleteReconciliation = async (id: string) => {
    try {
      await bank.completeReconciliation(id);
      toast.success('Reconciliation completed');
      if (selectedAccount) {
        bank.listReconciliations({ account_id: selectedAccount.id }).then(res => setReconciliations(res.data));
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to complete reconciliation'));
    }
  };

  const addTransactionToImport = () => {
    setImportData({
      ...importData,
      transactions: [
        ...importData.transactions,
        {
          transaction_date: new Date().toISOString().split('T')[0],
          transaction_type: 'Credit',
          amount: 0,
          description: '',
          reference_number: '',
          payee_name: '',
        }
      ]
    });
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

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Bank Reconciliation</h1>
        <button onClick={() => setShowImportModal(true)} className="btn btn-primary">
          Import Statement
        </button>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4 mb-6">
        <div className="card p-4">
          <h3 className="text-sm font-medium text-gray-500 mb-2">Select Account</h3>
          <select
            value={selectedAccount?.id || ''}
            onChange={(e) => {
              const acct = accounts.find(a => a.id === e.target.value);
              setSelectedAccount(acct || null);
            }}
            className="w-full border rounded px-3 py-2"
          >
            {accounts.map(a => (
              <option key={a.id} value={a.id}>
                {a.account_name} ({a.masked_account_number})
              </option>
            ))}
          </select>
        </div>
        
        {summary && (
          <div className="card p-4 col-span-3">
            <h3 className="text-sm font-medium text-gray-500 mb-2">Reconciliation Summary</h3>
            <div className="grid grid-cols-4 gap-4 mt-2">
              <div>
                <span className="text-gray-600 text-sm">GL Balance:</span>
                <div className="font-medium">{formatCurrency(summary.gl_balance)}</div>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Bank Balance:</span>
                <div className="font-medium">{formatCurrency(summary.bank_balance || 0)}</div>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Unreconciled:</span>
                <div className="font-medium">{summary.unreconciled_count}</div>
              </div>
              <div>
                <span className="text-gray-600 text-sm">Variance:</span>
                <div className={`font-medium ${summary.variance !== 0 ? 'text-red-600' : 'text-green-600'}`}>
                  {formatCurrency(summary.variance)}
                </div>
              </div>
            </div>
          </div>
        )}
      </div>

      <div className="mb-4">
        <div className="flex border-b">
          {(['accounts', 'statements', 'transactions', 'reconciliations'] as const).map(tab => (
            <button
              key={tab}
              onClick={() => setActiveTab(tab)}
              className={`px-4 py-2 font-medium capitalize ${activeTab === tab ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-500'}`}
            >
              {tab}
            </button>
          ))}
        </div>
      </div>

      {activeTab === 'accounts' && (
        <div className="card">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Bank Accounts</h2>
          </div>
          {accounts.length === 0 ? (
            <div className="p-8 text-gray-500 text-center">No accounts configured</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="text-left px-4 py-2">Name</th>
                    <th className="text-left px-4 py-2">Number</th>
                    <th className="text-left px-4 py-2">Currency</th>
                    <th className="text-left px-4 py-2">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {accounts.map(a => (
                    <tr key={a.id} className="hover:bg-gray-50 border-t">
                      <td className="px-4 py-2 font-medium">{a.account_name}</td>
                      <td className="px-4 py-2 text-gray-500">{a.masked_account_number}</td>
                      <td className="px-4 py-2">{a.currency}</td>
                      <td className="px-4 py-2">
                        <span className={`px-2 py-1 rounded text-xs ${
                          a.status === 'Active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
                        }`}>
                          {a.status}
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}

      {activeTab === 'statements' && selectedAccount && (
        <div className="card">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Bank Statements</h2>
          </div>
          {statements.length === 0 ? (
            <div className="p-8 text-gray-500 text-center">No statements found</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="text-left px-4 py-2">Statement #</th>
                    <th className="text-left px-4 py-2">Date</th>
                    <th className="text-left px-4 py-2">Opening</th>
                    <th className="text-left px-4 py-2">Closing</th>
                    <th className="text-left px-4 py-2">Status</th>
                  </tr>
                </thead>
                <tbody>
                  {statements.map(s => (
                    <tr key={s.id} className="hover:bg-gray-50 border-t">
                      <td className="px-4 py-2 font-mono text-sm">{s.statement_number}</td>
                      <td className="px-4 py-2">{formatDate(s.statement_date)}</td>
                      <td className="px-4 py-2">{formatCurrency(s.opening_balance)}</td>
                      <td className="px-4 py-2">{formatCurrency(s.closing_balance)}</td>
                      <td className="px-4 py-2">
                        <span className={`px-2 py-1 rounded text-xs ${
                          s.status === 'Reconciled' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'
                        }`}>
                          {s.status}
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}

      {activeTab === 'transactions' && selectedAccount && (
        <div className="card">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">
              Unreconciled Transactions ({transactions.filter(t => t.reconciliation_status === 'Unreconciled').length})
            </h2>
          </div>
          <div className="overflow-x-auto">
            <table className="min-w-full">
              <thead className="bg-gray-50">
                <tr>
                  <th className="text-left px-4 py-2">Date</th>
                  <th className="text-left px-4 py-2">Description</th>
                  <th className="text-left px-4 py-2">Amount</th>
                  <th className="text-left px-4 py-2">Status</th>
                  <th className="text-left px-4 py-2">Actions</th>
                </tr>
              </thead>
              <tbody>
                {transactions.filter(t => t.reconciliation_status === 'Unreconciled').map(t => (
                  <tr key={t.id} className="hover:bg-gray-50 border-t">
                    <td className="px-4 py-2">{formatDate(t.transaction_date)}</td>
                    <td className="px-4 py-2">{t.description}</td>
                    <td className={`px-4 py-2 font-medium ${t.amount >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                      {formatCurrency(t.amount)}
                    </td>
                    <td className="px-4 py-2">
                      <span className="px-2 py-1 rounded text-xs bg-yellow-100 text-yellow-800">
                        Unreconciled
                      </span>
                    </td>
                    <td className="px-4 py-2">
                      <button
                        onClick={() => handleReconcile(t.id)}
                        className="text-blue-600 hover:text-blue-800"
                      >
                        Reconcile
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {activeTab === 'reconciliations' && (
        <div className="card">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Reconciliation Sessions</h2>
            {selectedAccount && (
              <button onClick={handleStartReconciliation} className="btn btn-secondary">
                New Reconciliation
              </button>
            )}
          </div>
          {reconciliations.length === 0 ? (
            <div className="p-8 text-gray-500 text-center">No reconciliation sessions</div>
          ) : (
            <div className="overflow-x-auto">
              <table className="min-w-full">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="text-left px-4 py-2">Session #</th>
                    <th className="text-left px-4 py-2">Period</th>
                    <th className="text-left px-4 py-2">Matched</th>
                    <th className="text-left px-4 py-2">Variance</th>
                    <th className="text-left px-4 py-2">Status</th>
                    <th className="text-left px-4 py-2">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {reconciliations.map(r => (
                    <tr key={r.id} className="hover:bg-gray-50 border-t">
                      <td className="px-4 py-2 font-mono text-sm">{r.session_number}</td>
                      <td className="px-4 py-2">
                        {formatDate(r.period_start)} - {formatDate(r.period_end)}
                      </td>
                      <td className="px-4 py-2">
                        {r.matched_count} / {r.total_transactions}
                      </td>
                      <td className={`px-4 py-2 ${r.variance !== 0 ? 'text-red-600' : 'text-green-600'}`}>
                        {formatCurrency(r.variance)}
                      </td>
                      <td className="px-4 py-2">
                        <span className={`px-2 py-1 rounded text-xs ${
                          r.status === 'Completed' ? 'bg-green-100 text-green-800' :
                          r.status === 'InProgress' ? 'bg-blue-100 text-blue-800' :
                          'bg-gray-100 text-gray-800'
                        }`}>
                          {r.status}
                        </span>
                      </td>
                      <td className="px-4 py-2">
                        {r.status === 'InProgress' && (
                          <div className="flex gap-2">
                            <button
                              onClick={() => handleAutoMatch(r.id)}
                              className="text-blue-600 hover:text-blue-800"
                            >
                              Auto-Match
                            </button>
                            <button
                              onClick={() => handleCompleteReconciliation(r.id)}
                              className="text-green-600 hover:text-green-800"
                            >
                              Complete
                            </button>
                          </div>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      )}

      {showImportModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl max-h-[80vh] overflow-y-auto">
            <h2 className="text-lg font-semibold mb-4">Import Bank Statement</h2>
            <form onSubmit={handleImportStatement}>
              <div className="grid grid-cols-2 gap-4 mb-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Statement Date</label>
                  <input
                    type="date"
                    value={importData.statement_date}
                    onChange={(e) => setImportData({ ...importData, statement_date: e.target.value })}
                    className="w-full border rounded px-3 py-2"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Currency</label>
                  <select
                    value={importData.currency}
                    onChange={(e) => setImportData({ ...importData, currency: e.target.value })}
                    className="w-full border rounded px-3 py-2"
                  >
                    <option value="USD">USD</option>
                    <option value="EUR">EUR</option>
                    <option value="GBP">GBP</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Opening Balance (cents)</label>
                  <input
                    type="number"
                    value={importData.opening_balance}
                    onChange={(e) => setImportData({ ...importData, opening_balance: parseInt(e.target.value) || 0 })}
                    className="w-full border rounded px-3 py-2"
                    required
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Closing Balance (cents)</label>
                  <input
                    type="number"
                    value={importData.closing_balance}
                    onChange={(e) => setImportData({ ...importData, closing_balance: parseInt(e.target.value) || 0 })}
                    className="w-full border rounded px-3 py-2"
                    required
                  />
                </div>
              </div>

              <div className="mb-4">
                <div className="flex justify-between items-center mb-2">
                  <label className="block text-sm font-medium text-gray-700">Transactions</label>
                  <button type="button" onClick={addTransactionToImport} className="text-sm text-blue-600 hover:text-blue-800">
                    + Add Transaction
                  </button>
                </div>
                {importData.transactions.map((t, idx) => (
                  <div key={idx} className="border rounded p-3 mb-2">
                    <div className="grid grid-cols-4 gap-2">
                      <div>
                        <label className="block text-xs text-gray-500">Date</label>
                        <input
                          type="date"
                          value={t.transaction_date}
                          onChange={(e) => {
                            const newTx = [...importData.transactions];
                            newTx[idx] = { ...newTx[idx], transaction_date: e.target.value };
                            setImportData({ ...importData, transactions: newTx });
                          }}
                          className="w-full border rounded px-2 py-1 text-sm"
                        />
                      </div>
                      <div>
                        <label className="block text-xs text-gray-500">Type</label>
                        <select
                          value={t.transaction_type}
                          onChange={(e) => {
                            const newTx = [...importData.transactions];
                            newTx[idx] = { ...newTx[idx], transaction_type: e.target.value };
                            setImportData({ ...importData, transactions: newTx });
                          }}
                          className="w-full border rounded px-2 py-1 text-sm"
                        >
                          <option value="Credit">Credit</option>
                          <option value="Debit">Debit</option>
                        </select>
                      </div>
                      <div>
                        <label className="block text-xs text-gray-500">Amount (cents)</label>
                        <input
                          type="number"
                          value={t.amount}
                          onChange={(e) => {
                            const newTx = [...importData.transactions];
                            newTx[idx] = { ...newTx[idx], amount: parseInt(e.target.value) || 0 };
                            setImportData({ ...importData, transactions: newTx });
                          }}
                          className="w-full border rounded px-2 py-1 text-sm"
                        />
                      </div>
                      <div>
                        <label className="block text-xs text-gray-500">Description</label>
                        <input
                          type="text"
                          value={t.description}
                          onChange={(e) => {
                            const newTx = [...importData.transactions];
                            newTx[idx] = { ...newTx[idx], description: e.target.value };
                            setImportData({ ...importData, transactions: newTx });
                          }}
                          className="w-full border rounded px-2 py-1 text-sm"
                        />
                      </div>
                    </div>
                  </div>
                ))}
              </div>

              <div className="flex justify-end gap-2 mt-4">
                <button type="button" onClick={() => setShowImportModal(false)} className="btn btn-secondary">
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Import Statement
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

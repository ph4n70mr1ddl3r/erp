import { useState } from 'react';
import { finance } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import type { AccountBalanceResponse } from '../types';

interface BalanceSheetResponse {
  as_of_date: string;
  assets: AccountBalanceResponse[];
  total_assets: number;
  liabilities: AccountBalanceResponse[];
  total_liabilities: number;
  equity: AccountBalanceResponse[];
  total_equity: number;
}

interface ProfitAndLossResponse {
  from_date: string;
  to_date: string;
  revenue: AccountBalanceResponse[];
  total_revenue: number;
  expenses: AccountBalanceResponse[];
  total_expenses: number;
  net_income: number;
}

interface TrialBalanceResponse {
  as_of_date: string;
  accounts: { account_id: string; account_code: string; account_name: string; debit: number; credit: number }[];
  total_debits: number;
  total_credits: number;
}

export default function Reports() {
  const toast = useToast();
  const [loading, setLoading] = useState(false);
  const [activeReport, setActiveReport] = useState<'balance-sheet' | 'profit-loss' | 'trial-balance' | null>(null);
  const [balanceSheet, setBalanceSheet] = useState<BalanceSheetResponse | null>(null);
  const [profitLoss, setProfitLoss] = useState<ProfitAndLossResponse | null>(null);
  const [trialBalance, setTrialBalance] = useState<TrialBalanceResponse | null>(null);

  const loadBalanceSheet = async () => {
    try {
      setLoading(true);
      const res = await finance.getBalanceSheet();
      setBalanceSheet(res.data);
      setActiveReport('balance-sheet');
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to load balance sheet');
    } finally {
      setLoading(false);
    }
  };

  const loadProfitLoss = async () => {
    try {
      setLoading(true);
      const res = await finance.getProfitAndLoss();
      setProfitLoss(res.data);
      setActiveReport('profit-loss');
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to load P&L');
    } finally {
      setLoading(false);
    }
  };

  const loadTrialBalance = async () => {
    try {
      setLoading(true);
      const res = await finance.getTrialBalance();
      setTrialBalance(res.data);
      setActiveReport('trial-balance');
    } catch (err: any) {
      toast.error(err.response?.data?.error || 'Failed to load trial balance');
    } finally {
      setLoading(false);
    }
  };

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(amount);
  };

  const formatDate = (date: string) => {
    return new Date(date).toLocaleDateString();
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 mb-6">Financial Reports</h1>

      {/* Report Selection */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <button
          onClick={loadBalanceSheet}
          className={`card p-6 text-left hover:shadow-md transition-shadow ${activeReport === 'balance-sheet' ? 'ring-2 ring-blue-500' : ''}`}
        >
          <h2 className="text-lg font-semibold">Balance Sheet</h2>
          <p className="text-sm text-gray-500 mt-1">Assets, Liabilities & Equity</p>
        </button>
        <button
          onClick={loadProfitLoss}
          className={`card p-6 text-left hover:shadow-md transition-shadow ${activeReport === 'profit-loss' ? 'ring-2 ring-blue-500' : ''}`}
        >
          <h2 className="text-lg font-semibold">Profit & Loss</h2>
          <p className="text-sm text-gray-500 mt-1">Revenue & Expenses</p>
        </button>
        <button
          onClick={loadTrialBalance}
          className={`card p-6 text-left hover:shadow-md transition-shadow ${activeReport === 'trial-balance' ? 'ring-2 ring-blue-500' : ''}`}
        >
          <h2 className="text-lg font-semibold">Trial Balance</h2>
          <p className="text-sm text-gray-500 mt-1">Debits & Credits Summary</p>
        </button>
      </div>

      {/* Balance Sheet Report */}
      {activeReport === 'balance-sheet' && balanceSheet && (
        <div className="card">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Balance Sheet</h2>
            <span className="text-sm text-gray-500">As of {formatDate(balanceSheet.as_of_date)}</span>
          </div>
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 p-6">
            {/* Assets */}
            <div>
              <h3 className="font-semibold text-blue-700 mb-3">Assets</h3>
              <table className="w-full">
                <tbody>
                  {balanceSheet.assets.map((a) => (
                    <tr key={a.account_id} className="border-b">
                      <td className="py-2 text-sm">{a.account_code} - {a.account_name}</td>
                      <td className="py-2 text-sm text-right">{formatCurrency(a.balance)}</td>
                    </tr>
                  ))}
                </tbody>
                <tfoot>
                  <tr className="font-semibold bg-blue-50">
                    <td className="py-2 px-2">Total Assets</td>
                    <td className="py-2 px-2 text-right">{formatCurrency(balanceSheet.total_assets)}</td>
                  </tr>
                </tfoot>
              </table>
            </div>

            {/* Liabilities */}
            <div>
              <h3 className="font-semibold text-red-700 mb-3">Liabilities</h3>
              <table className="w-full">
                <tbody>
                  {balanceSheet.liabilities.map((a) => (
                    <tr key={a.account_id} className="border-b">
                      <td className="py-2 text-sm">{a.account_code} - {a.account_name}</td>
                      <td className="py-2 text-sm text-right">{formatCurrency(Math.abs(a.balance))}</td>
                    </tr>
                  ))}
                </tbody>
                <tfoot>
                  <tr className="font-semibold bg-red-50">
                    <td className="py-2 px-2">Total Liabilities</td>
                    <td className="py-2 px-2 text-right">{formatCurrency(Math.abs(balanceSheet.total_liabilities))}</td>
                  </tr>
                </tfoot>
              </table>
            </div>

            {/* Equity */}
            <div>
              <h3 className="font-semibold text-green-700 mb-3">Equity</h3>
              <table className="w-full">
                <tbody>
                  {balanceSheet.equity.map((a) => (
                    <tr key={a.account_id} className="border-b">
                      <td className="py-2 text-sm">{a.account_code} - {a.account_name}</td>
                      <td className="py-2 text-sm text-right">{formatCurrency(Math.abs(a.balance))}</td>
                    </tr>
                  ))}
                </tbody>
                <tfoot>
                  <tr className="font-semibold bg-green-50">
                    <td className="py-2 px-2">Total Equity</td>
                    <td className="py-2 px-2 text-right">{formatCurrency(Math.abs(balanceSheet.total_equity))}</td>
                  </tr>
                </tfoot>
              </table>
            </div>
          </div>
        </div>
      )}

      {/* Profit & Loss Report */}
      {activeReport === 'profit-loss' && profitLoss && (
        <div className="card">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Profit & Loss Statement</h2>
            <span className="text-sm text-gray-500">
              {formatDate(profitLoss.from_date)} - {formatDate(profitLoss.to_date)}
            </span>
          </div>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 p-6">
            {/* Revenue */}
            <div>
              <h3 className="font-semibold text-green-700 mb-3">Revenue</h3>
              <table className="w-full">
                <tbody>
                  {profitLoss.revenue.map((a) => (
                    <tr key={a.account_id} className="border-b">
                      <td className="py-2 text-sm">{a.account_code} - {a.account_name}</td>
                      <td className="py-2 text-sm text-right">{formatCurrency(Math.abs(a.balance))}</td>
                    </tr>
                  ))}
                </tbody>
                <tfoot>
                  <tr className="font-semibold bg-green-50">
                    <td className="py-2 px-2">Total Revenue</td>
                    <td className="py-2 px-2 text-right">{formatCurrency(profitLoss.total_revenue)}</td>
                  </tr>
                </tfoot>
              </table>
            </div>

            {/* Expenses */}
            <div>
              <h3 className="font-semibold text-red-700 mb-3">Expenses</h3>
              <table className="w-full">
                <tbody>
                  {profitLoss.expenses.map((a) => (
                    <tr key={a.account_id} className="border-b">
                      <td className="py-2 text-sm">{a.account_code} - {a.account_name}</td>
                      <td className="py-2 text-sm text-right">{formatCurrency(Math.abs(a.balance))}</td>
                    </tr>
                  ))}
                </tbody>
                <tfoot>
                  <tr className="font-semibold bg-red-50">
                    <td className="py-2 px-2">Total Expenses</td>
                    <td className="py-2 px-2 text-right">{formatCurrency(profitLoss.total_expenses)}</td>
                  </tr>
                </tfoot>
              </table>
            </div>
          </div>
          <div className="border-t p-4">
            <div className="flex justify-between items-center text-xl">
              <span className="font-semibold">Net Income</span>
              <span className={`font-bold ${profitLoss.net_income >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                {formatCurrency(profitLoss.net_income)}
              </span>
            </div>
          </div>
        </div>
      )}

      {/* Trial Balance Report */}
      {activeReport === 'trial-balance' && trialBalance && (
        <div className="card">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Trial Balance</h2>
            <span className="text-sm text-gray-500">As of {formatDate(trialBalance.as_of_date)}</span>
          </div>
          <table className="w-full">
            <thead>
              <tr className="border-b bg-gray-50">
                <th className="table-header">Account</th>
                <th className="table-header text-right">Debit</th>
                <th className="table-header text-right">Credit</th>
              </tr>
            </thead>
            <tbody>
              {trialBalance.accounts.map((a) => (
                <tr key={a.account_id} className="border-b hover:bg-gray-50">
                  <td className="table-cell">{a.account_code} - {a.account_name}</td>
                  <td className="table-cell text-right">{a.debit > 0 ? formatCurrency(a.debit) : ''}</td>
                  <td className="table-cell text-right">{a.credit > 0 ? formatCurrency(a.credit) : ''}</td>
                </tr>
              ))}
            </tbody>
            <tfoot className="bg-gray-100 font-semibold">
              <tr>
                <td className="py-2 px-4">Totals</td>
                <td className="py-2 px-4 text-right">{formatCurrency(trialBalance.total_debits)}</td>
                <td className="py-2 px-4 text-right">{formatCurrency(trialBalance.total_credits)}</td>
              </tr>
            </tfoot>
          </table>
        </div>
      )}

      {!activeReport && (
        <div className="card p-8 text-center text-gray-500">
          Select a report type above to view financial statements
        </div>
      )}
    </div>
  );
}

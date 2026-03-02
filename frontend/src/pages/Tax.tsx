import { useEffect, useState, useCallback } from 'react';
import { tax, sales } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import type { TaxJurisdiction, TaxRate } from '../api/client';
import type { Customer } from '../types';
import { getErrorMessage } from '../utils/errors';

export default function Tax() {
  const toast = useToast();
  const [jurisdictions, setJurisdictions] = useState<TaxJurisdiction[]>([]);
  const [rates, setRates] = useState<TaxRate[]>([]);
  const [customers, setCustomers] = useState<Customer[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [jurisdictionSearch, setJurisdictionSearch] = useState('');
  const [rateSearch, setRateSearch] = useState('');
  const [showJurisdictionModal, setShowJurisdictionModal] = useState(false);
  const [showRateModal, setShowRateModal] = useState(false);
  const [showCalculatorModal, setShowCalculatorModal] = useState(false);
  const [showExemptionModal, setShowExemptionModal] = useState(false);

  const [newJurisdiction, setNewJurisdiction] = useState({
    code: '',
    name: '',
    country_code: 'US',
    state_code: '',
    county: '',
    city: '',
  });

  const [newRate, setNewRate] = useState({
    jurisdiction_id: '',
    name: '',
    code: '',
    rate: 0,
    tax_type: 'SalesTax',
    is_compound: false,
    is_recoverable: false,
  });

  const [calculator, setCalculator] = useState({
    jurisdiction_id: '',
    amount: 0,
    customer_id: '',
    result: null as { taxable_amount: number; total_tax: number } | null,
  });

  const [newExemption, setNewExemption] = useState({
    customer_id: '',
    exemption_type: 'Resale',
    certificate_number: '',
    jurisdiction_id: '',
    issue_date: new Date().toISOString().split('T')[0],
    expiry_date: '',
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [jurRes, rateRes, custRes] = await Promise.all([
        tax.getJurisdictions(1, 100),
        tax.getRates(1, 100),
        sales.getCustomers(1, 100),
      ]);
      setJurisdictions(jurRes.data.items);
      setRates(rateRes.data.items);
      setCustomers(custRes.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load tax data'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleCreateJurisdiction = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await tax.createJurisdiction(newJurisdiction);
      toast.success('Jurisdiction created successfully');
      setShowJurisdictionModal(false);
      setNewJurisdiction({ code: '', name: '', country_code: 'US', state_code: '', county: '', city: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateRate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await tax.createRate({
        ...newRate,
        rate: newRate.rate,
      });
      toast.success('Tax rate created successfully');
      setShowRateModal(false);
      setNewRate({ jurisdiction_id: '', name: '', code: '', rate: 0, tax_type: 'SalesTax', is_compound: false, is_recoverable: false });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const handleCalculateTax = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      const result = await tax.calculateTax({
        jurisdiction_id: calculator.jurisdiction_id,
        amount: Math.round(calculator.amount * 100),
        customer_id: calculator.customer_id || undefined,
      });
      setCalculator({ ...calculator, result: result.data });
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateExemption = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await tax.createExemption({
        ...newExemption,
        issue_date: new Date(newExemption.issue_date).toISOString(),
        expiry_date: newExemption.expiry_date ? new Date(newExemption.expiry_date).toISOString() : undefined,
      });
      toast.success('Tax exemption created successfully');
      setShowExemptionModal(false);
      setNewExemption({ customer_id: '', exemption_type: 'Resale', certificate_number: '', jurisdiction_id: '', issue_date: new Date().toISOString().split('T')[0], expiry_date: '' });
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const filteredJurisdictions = jurisdictions.filter(j =>
    j.code.toLowerCase().includes(jurisdictionSearch.toLowerCase()) ||
    j.name.toLowerCase().includes(jurisdictionSearch.toLowerCase()) ||
    j.country_code.toLowerCase().includes(jurisdictionSearch.toLowerCase())
  );

  const filteredRates = rates.filter(r =>
    r.name.toLowerCase().includes(rateSearch.toLowerCase()) ||
    r.code.toLowerCase().includes(rateSearch.toLowerCase())
  );

  const getJurisdictionName = (id: string) => {
    const j = jurisdictions.find(j => j.id === id);
    return j ? j.name : id;
  };

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Tax Management</h1>
        <div className="flex gap-2 flex-wrap">
          <button onClick={() => setShowJurisdictionModal(true)} className="btn btn-primary">Add Jurisdiction</button>
          <button onClick={() => setShowRateModal(true)} className="btn btn-secondary">Add Tax Rate</button>
          <button onClick={() => setShowCalculatorModal(true)} className="btn btn-secondary">Tax Calculator</button>
          <button onClick={() => setShowExemptionModal(true)} className="btn btn-secondary">Add Exemption</button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
        <div className="card p-4">
          <div className="text-sm text-gray-500">Jurisdictions</div>
          <div className="text-2xl font-bold">{jurisdictions.length}</div>
        </div>
        <div className="card p-4">
          <div className="text-sm text-gray-500">Tax Rates</div>
          <div className="text-2xl font-bold">{rates.length}</div>
        </div>
        <div className="card p-4">
          <div className="text-sm text-gray-500">Active Rates</div>
          <div className="text-2xl font-bold">{rates.filter(r => r.status === 'Active').length}</div>
        </div>
      </div>

      <div className="card mb-6">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Tax Jurisdictions</h2>
          <SearchInput value={jurisdictionSearch} onChange={setJurisdictionSearch} placeholder="Search jurisdictions..." />
        </div>
        {filteredJurisdictions.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {jurisdictionSearch ? 'No jurisdictions match your search' : 'No jurisdictions found. Create one to get started.'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Code</th>
                <th className="table-header">Name</th>
                <th className="table-header">Country</th>
                <th className="table-header">Status</th>
              </tr>
            </thead>
            <tbody>
              {filteredJurisdictions.map((j) => (
                <tr key={j.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{j.code}</td>
                  <td className="table-cell">{j.name}</td>
                  <td className="table-cell">{j.country_code}</td>
                  <td className="table-cell">
                    <span className={`badge ${j.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>
                      {j.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      <div className="card">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Tax Rates</h2>
          <SearchInput value={rateSearch} onChange={setRateSearch} placeholder="Search rates..." />
        </div>
        {filteredRates.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {rateSearch ? 'No rates match your search' : 'No tax rates found. Create a jurisdiction first, then add rates.'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Code</th>
                <th className="table-header">Name</th>
                <th className="table-header">Jurisdiction</th>
                <th className="table-header">Type</th>
                <th className="table-header">Rate</th>
                <th className="table-header">Status</th>
              </tr>
            </thead>
            <tbody>
              {filteredRates.map((r) => (
                <tr key={r.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{r.code}</td>
                  <td className="table-cell">{r.name}</td>
                  <td className="table-cell">{getJurisdictionName(r.jurisdiction_id)}</td>
                  <td className="table-cell">{r.tax_type}</td>
                  <td className="table-cell">{(r.rate * 100).toFixed(2)}%</td>
                  <td className="table-cell">
                    <span className={`badge ${r.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>
                      {r.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {showJurisdictionModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Tax Jurisdiction</h2>
            <form onSubmit={handleCreateJurisdiction} className="space-y-4">
              <div>
                <label className="label">Code</label>
                <input className="input" value={newJurisdiction.code} onChange={(e) => setNewJurisdiction({ ...newJurisdiction, code: e.target.value })} required placeholder="e.g., US-CA" />
              </div>
              <div>
                <label className="label">Name</label>
                <input className="input" value={newJurisdiction.name} onChange={(e) => setNewJurisdiction({ ...newJurisdiction, name: e.target.value })} required placeholder="e.g., California" />
              </div>
              <div>
                <label className="label">Country Code</label>
                <input className="input" value={newJurisdiction.country_code} onChange={(e) => setNewJurisdiction({ ...newJurisdiction, country_code: e.target.value })} required placeholder="US" maxLength={2} />
              </div>
              <div>
                <label className="label">State Code (optional)</label>
                <input className="input" value={newJurisdiction.state_code} onChange={(e) => setNewJurisdiction({ ...newJurisdiction, state_code: e.target.value })} placeholder="e.g., CA" maxLength={2} />
              </div>
              <div>
                <label className="label">County (optional)</label>
                <input className="input" value={newJurisdiction.county} onChange={(e) => setNewJurisdiction({ ...newJurisdiction, county: e.target.value })} />
              </div>
              <div>
                <label className="label">City (optional)</label>
                <input className="input" value={newJurisdiction.city} onChange={(e) => setNewJurisdiction({ ...newJurisdiction, city: e.target.value })} />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowJurisdictionModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showRateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Tax Rate</h2>
            <form onSubmit={handleCreateRate} className="space-y-4">
              <div>
                <label className="label">Jurisdiction</label>
                <select className="input" value={newRate.jurisdiction_id} onChange={(e) => setNewRate({ ...newRate, jurisdiction_id: e.target.value })} required>
                  <option value="">Select Jurisdiction</option>
                  {jurisdictions.map((j) => <option key={j.id} value={j.id}>{j.code} - {j.name}</option>)}
                </select>
              </div>
              <div>
                <label className="label">Code</label>
                <input className="input" value={newRate.code} onChange={(e) => setNewRate({ ...newRate, code: e.target.value })} required placeholder="e.g., STATE-TAX" />
              </div>
              <div>
                <label className="label">Name</label>
                <input className="input" value={newRate.name} onChange={(e) => setNewRate({ ...newRate, name: e.target.value })} required placeholder="e.g., State Sales Tax" />
              </div>
              <div>
                <label className="label">Tax Type</label>
                <select className="input" value={newRate.tax_type} onChange={(e) => setNewRate({ ...newRate, tax_type: e.target.value })}>
                  <option value="SalesTax">Sales Tax</option>
                  <option value="VAT">VAT</option>
                  <option value="GST">GST</option>
                  <option value="PST">PST</option>
                  <option value="HST">HST</option>
                  <option value="Withholding">Withholding</option>
                  <option value="Excise">Excise</option>
                </select>
              </div>
              <div>
                <label className="label">Rate (decimal, e.g., 0.0825 for 8.25%)</label>
                <input type="number" className="input" value={newRate.rate} onChange={(e) => setNewRate({ ...newRate, rate: parseFloat(e.target.value) || 0 })} required step="0.0001" min="0" max="1" />
              </div>
              <div className="flex gap-4">
                <label className="flex items-center gap-2">
                  <input type="checkbox" checked={newRate.is_compound} onChange={(e) => setNewRate({ ...newRate, is_compound: e.target.checked })} />
                  <span className="text-sm">Compound</span>
                </label>
                <label className="flex items-center gap-2">
                  <input type="checkbox" checked={newRate.is_recoverable} onChange={(e) => setNewRate({ ...newRate, is_recoverable: e.target.checked })} />
                  <span className="text-sm">Recoverable</span>
                </label>
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowRateModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showCalculatorModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">Tax Calculator</h2>
            <form onSubmit={handleCalculateTax} className="space-y-4">
              <div>
                <label className="label">Jurisdiction</label>
                <select className="input" value={calculator.jurisdiction_id} onChange={(e) => setCalculator({ ...calculator, jurisdiction_id: e.target.value, result: null })} required>
                  <option value="">Select Jurisdiction</option>
                  {jurisdictions.map((j) => <option key={j.id} value={j.id}>{j.code} - {j.name}</option>)}
                </select>
              </div>
              <div>
                <label className="label">Amount ($)</label>
                <input type="number" className="input" value={calculator.amount || ''} onChange={(e) => setCalculator({ ...calculator, amount: parseFloat(e.target.value) || 0, result: null })} required step="0.01" min="0" />
              </div>
              <div>
                <label className="label">Customer (optional, for exemptions)</label>
                <select className="input" value={calculator.customer_id} onChange={(e) => setCalculator({ ...calculator, customer_id: e.target.value, result: null })}>
                  <option value="">Select Customer</option>
                  {customers.map((c) => <option key={c.id} value={c.id}>{c.code} - {c.name}</option>)}
                </select>
              </div>
              {calculator.result && (
                <div className="bg-gray-50 p-4 rounded-lg space-y-2">
                  <div className="flex justify-between">
                    <span className="text-gray-600">Taxable Amount:</span>
                    <span className="font-medium">${calculator.result.taxable_amount.toFixed(2)}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">Total Tax:</span>
                    <span className="font-bold text-blue-600">${calculator.result.total_tax.toFixed(2)}</span>
                  </div>
                  <div className="flex justify-between border-t pt-2">
                    <span className="text-gray-600">Total:</span>
                    <span className="font-bold">${(calculator.result.taxable_amount + calculator.result.total_tax).toFixed(2)}</span>
                  </div>
                </div>
              )}
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowCalculatorModal(false)} className="btn btn-secondary">Close</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Calculating...' : 'Calculate'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showExemptionModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Tax Exemption</h2>
            <form onSubmit={handleCreateExemption} className="space-y-4">
              <div>
                <label className="label">Customer</label>
                <select className="input" value={newExemption.customer_id} onChange={(e) => setNewExemption({ ...newExemption, customer_id: e.target.value })} required>
                  <option value="">Select Customer</option>
                  {customers.map((c) => <option key={c.id} value={c.id}>{c.code} - {c.name}</option>)}
                </select>
              </div>
              <div>
                <label className="label">Exemption Type</label>
                <select className="input" value={newExemption.exemption_type} onChange={(e) => setNewExemption({ ...newExemption, exemption_type: e.target.value })}>
                  <option value="Resale">Resale</option>
                  <option value="Manufacturing">Manufacturing</option>
                  <option value="Agricultural">Agricultural</option>
                  <option value="Government">Government</option>
                  <option value="NonProfit">Non-Profit</option>
                  <option value="Educational">Educational</option>
                  <option value="DirectPay">Direct Pay</option>
                  <option value="Other">Other</option>
                </select>
              </div>
              <div>
                <label className="label">Certificate Number</label>
                <input className="input" value={newExemption.certificate_number} onChange={(e) => setNewExemption({ ...newExemption, certificate_number: e.target.value })} required />
              </div>
              <div>
                <label className="label">Jurisdiction (optional)</label>
                <select className="input" value={newExemption.jurisdiction_id} onChange={(e) => setNewExemption({ ...newExemption, jurisdiction_id: e.target.value })}>
                  <option value="">All Jurisdictions</option>
                  {jurisdictions.map((j) => <option key={j.id} value={j.id}>{j.code} - {j.name}</option>)}
                </select>
              </div>
              <div>
                <label className="label">Issue Date</label>
                <input type="date" className="input" value={newExemption.issue_date} onChange={(e) => setNewExemption({ ...newExemption, issue_date: e.target.value })} required />
              </div>
              <div>
                <label className="label">Expiry Date (optional)</label>
                <input type="date" className="input" value={newExemption.expiry_date} onChange={(e) => setNewExemption({ ...newExemption, expiry_date: e.target.value })} />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowExemptionModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

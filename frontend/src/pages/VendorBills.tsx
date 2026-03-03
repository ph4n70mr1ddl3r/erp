import { useState, useEffect, useCallback } from 'react';
import { vendorBills, type VendorBill, type ThreeWayMatchResult } from '../api/client';
import { useToast } from '../components/Toast';
import { getErrorMessage } from '../utils/errors';
import { LoadingPage } from '../components/Spinner';

export default function VendorBillsPage() {
  const toast = useToast();
  const [bills, setBills] = useState<VendorBill[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);
  const [selectedBill, setSelectedBill] = useState<VendorBill | null>(null);
  const [matchResult, setMatchResult] = useState<ThreeWayMatchResult | null>(null);
  const [formData, setFormData] = useState({
    vendor_invoice_number: '',
    vendor_id: '',
    purchase_order_id: '',
    bill_date: new Date().toISOString().split('T')[0],
    due_date: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
    notes: '',
    lines: [{ description: '', quantity: 1, unit_price: 0, tax_rate: 0 }],
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const res = await vendorBills.list(1, 100);
      setBills(res.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load vendor bills'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => {
    void loadData();
  }, [loadData]);

  const resetForm = () => {
    setFormData({
      vendor_invoice_number: '',
      vendor_id: '',
      purchase_order_id: '',
      bill_date: new Date().toISOString().split('T')[0],
      due_date: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
      notes: '',
      lines: [{ description: '', quantity: 1, unit_price: 0, tax_rate: 0 }],
    });
    setShowForm(false);
  };

  const addLine = () => {
    setFormData({
      ...formData,
      lines: [...formData.lines, { description: '', quantity: 1, unit_price: 0, tax_rate: 0 }],
    });
  };

  const removeLine = (index: number) => {
    if (formData.lines.length > 1) {
      setFormData({
        ...formData,
        lines: formData.lines.filter((_, i) => i !== index),
      });
    }
  };

  const updateLine = (index: number, field: string, value: string | number) => {
    const newLines = [...formData.lines];
    newLines[index] = { ...newLines[index], [field]: value };
    setFormData({ ...formData, lines: newLines });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await vendorBills.create({
        vendor_invoice_number: formData.vendor_invoice_number,
        vendor_id: formData.vendor_id,
        purchase_order_id: formData.purchase_order_id || undefined,
        bill_date: new Date(formData.bill_date).toISOString(),
        due_date: new Date(formData.due_date).toISOString(),
        notes: formData.notes || undefined,
        lines: formData.lines.map(l => ({
          description: l.description,
          quantity: l.quantity,
          unit_price: Math.round(l.unit_price * 100),
          tax_rate: l.tax_rate,
        })),
      });
      toast.success('Vendor bill created');
      resetForm();
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create vendor bill'));
    }
  };

  const handleAction = async (action: string, id: string) => {
    try {
      switch (action) {
        case 'submit':
          await vendorBills.submit(id);
          toast.success('Bill submitted for approval');
          break;
        case 'approve':
          await vendorBills.approve(id);
          toast.success('Bill approved');
          break;
        case 'void':
          await vendorBills.void(id);
          toast.success('Bill voided');
          break;
        case 'match': {
          const result = await vendorBills.performThreeWayMatch(id);
          setMatchResult(result.data);
          toast.success('Three-way match performed');
          break;
        }
        case 'delete':
          if (!window.confirm('Are you sure you want to delete this bill?')) return;
          await vendorBills.delete(id);
          toast.success('Bill deleted');
          break;
      }
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, `Failed to ${action} bill`));
    }
  };

  const formatCurrency = (cents: number) => `$${(cents / 100).toFixed(2)}`;

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Draft': return 'bg-gray-100 text-gray-800';
      case 'Pending': return 'bg-yellow-100 text-yellow-800';
      case 'Approved': return 'bg-blue-100 text-blue-800';
      case 'PartiallyPaid': return 'bg-purple-100 text-purple-800';
      case 'Paid': return 'bg-green-100 text-green-800';
      case 'Void': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getMatchStatusColor = (status: string) => {
    switch (status) {
      case 'FullyMatched': return 'bg-green-100 text-green-800';
      case 'PartiallyMatched': return 'bg-yellow-100 text-yellow-800';
      case 'Unmatched': return 'bg-gray-100 text-gray-800';
      case 'Exception': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  if (loading) return <LoadingPage />;

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Vendor Bills</h1>
          <p className="text-gray-600 mt-1">Manage accounts payable - vendor invoices and three-way matching</p>
        </div>
        <button
          onClick={() => setShowForm(true)}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 flex items-center gap-2"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          New Vendor Bill
        </button>
      </div>

      {showForm && (
        <div className="bg-white rounded-lg shadow p-6 mb-6">
          <h2 className="text-lg font-semibold mb-4">Create Vendor Bill</h2>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Vendor Invoice Number *</label>
                <input
                  type="text"
                  value={formData.vendor_invoice_number}
                  onChange={(e) => setFormData({ ...formData, vendor_invoice_number: e.target.value })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Vendor ID *</label>
                <input
                  type="text"
                  value={formData.vendor_id}
                  onChange={(e) => setFormData({ ...formData, vendor_id: e.target.value })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
                  placeholder="UUID"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Bill Date *</label>
                <input
                  type="date"
                  value={formData.bill_date}
                  onChange={(e) => setFormData({ ...formData, bill_date: e.target.value })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Due Date *</label>
                <input
                  type="date"
                  value={formData.due_date}
                  onChange={(e) => setFormData({ ...formData, due_date: e.target.value })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
                  required
                />
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Purchase Order ID (Optional)</label>
              <input
                type="text"
                value={formData.purchase_order_id}
                onChange={(e) => setFormData({ ...formData, purchase_order_id: e.target.value })}
                className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
                placeholder="UUID for three-way matching"
              />
            </div>

            <div>
              <div className="flex justify-between items-center mb-2">
                <label className="block text-sm font-medium text-gray-700">Bill Lines</label>
                <button
                  type="button"
                  onClick={addLine}
                  className="text-sm text-blue-600 hover:text-blue-800"
                >
                  + Add Line
                </button>
              </div>
              <div className="space-y-2">
                {formData.lines.map((line, index) => (
                  <div key={index} className="grid grid-cols-12 gap-2 items-end">
                    <div className="col-span-5">
                      <input
                        type="text"
                        placeholder="Description"
                        value={line.description}
                        onChange={(e) => updateLine(index, 'description', e.target.value)}
                        className="w-full border rounded px-2 py-1 text-sm"
                        required
                      />
                    </div>
                    <div className="col-span-2">
                      <input
                        type="number"
                        placeholder="Qty"
                        value={line.quantity}
                        onChange={(e) => updateLine(index, 'quantity', parseFloat(e.target.value) || 0)}
                        className="w-full border rounded px-2 py-1 text-sm"
                        min="0"
                        required
                      />
                    </div>
                    <div className="col-span-2">
                      <input
                        type="number"
                        placeholder="Unit Price"
                        value={line.unit_price}
                        onChange={(e) => updateLine(index, 'unit_price', parseFloat(e.target.value) || 0)}
                        className="w-full border rounded px-2 py-1 text-sm"
                        min="0"
                        step="0.01"
                        required
                      />
                    </div>
                    <div className="col-span-2">
                      <input
                        type="number"
                        placeholder="Tax %"
                        value={line.tax_rate}
                        onChange={(e) => updateLine(index, 'tax_rate', parseFloat(e.target.value) || 0)}
                        className="w-full border rounded px-2 py-1 text-sm"
                        min="0"
                        max="100"
                      />
                    </div>
                    <div className="col-span-1">
                      {formData.lines.length > 1 && (
                        <button
                          type="button"
                          onClick={() => removeLine(index)}
                          className="text-red-500 hover:text-red-700"
                        >
                          ×
                        </button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Notes</label>
              <textarea
                value={formData.notes}
                onChange={(e) => setFormData({ ...formData, notes: e.target.value })}
                className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500"
                rows={2}
              />
            </div>

            <div className="flex gap-2">
              <button
                type="submit"
                className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700"
              >
                Create Bill
              </button>
              <button
                type="button"
                onClick={resetForm}
                className="bg-gray-200 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-300"
              >
                Cancel
              </button>
            </div>
          </form>
        </div>
      )}

      {selectedBill && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-2xl w-full m-4 max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-start mb-4">
              <div>
                <h2 className="text-xl font-bold">{selectedBill.bill_number}</h2>
                <p className="text-gray-600">Vendor Invoice: {selectedBill.vendor_invoice_number}</p>
              </div>
              <button
                onClick={() => { setSelectedBill(null); setMatchResult(null); }}
                className="text-gray-400 hover:text-gray-600"
              >
                ✕
              </button>
            </div>

            <div className="grid grid-cols-2 gap-4 mb-4">
              <div>
                <span className="text-sm text-gray-500">Status</span>
                <p><span className={`px-2 py-1 text-xs rounded-full ${getStatusColor(selectedBill.status)}`}>
                  {selectedBill.status}
                </span></p>
              </div>
              <div>
                <span className="text-sm text-gray-500">Match Status</span>
                <p><span className={`px-2 py-1 text-xs rounded-full ${getMatchStatusColor(selectedBill.match_status)}`}>
                  {selectedBill.match_status}
                </span></p>
              </div>
              <div>
                <span className="text-sm text-gray-500">Total</span>
                <p className="font-semibold">{formatCurrency(selectedBill.total)}</p>
              </div>
              <div>
                <span className="text-sm text-gray-500">Amount Paid</span>
                <p className="font-semibold">{formatCurrency(selectedBill.amount_paid)}</p>
              </div>
            </div>

            <table className="w-full text-sm mb-4">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-3 py-2 text-left">Description</th>
                  <th className="px-3 py-2 text-right">Qty</th>
                  <th className="px-3 py-2 text-right">Price</th>
                  <th className="px-3 py-2 text-right">Total</th>
                </tr>
              </thead>
              <tbody>
                {selectedBill.lines.map((line) => (
                  <tr key={line.id} className="border-b">
                    <td className="px-3 py-2">{line.description}</td>
                    <td className="px-3 py-2 text-right">{line.quantity}</td>
                    <td className="px-3 py-2 text-right">{formatCurrency(line.unit_price)}</td>
                    <td className="px-3 py-2 text-right">{formatCurrency(line.line_total)}</td>
                  </tr>
                ))}
              </tbody>
            </table>

            {matchResult && (
              <div className="bg-gray-50 rounded-lg p-4 mb-4">
                <h3 className="font-semibold mb-2">Three-Way Match Result</h3>
                <div className="grid grid-cols-3 gap-2 text-sm mb-2">
                  <div>Matched: {matchResult.total_matched_lines}</div>
                  <div>Unmatched: {matchResult.total_unmatched_lines}</div>
                  <div>Exceptions: {matchResult.total_exceptions}</div>
                </div>
                {matchResult.exceptions.length > 0 && (
                  <div className="text-sm text-red-600">
                    {matchResult.exceptions.map((e, i) => (
                      <div key={i}>{e.message}</div>
                    ))}
                  </div>
                )}
              </div>
            )}

            <div className="flex gap-2 flex-wrap">
              {selectedBill.status === 'Draft' && (
                <>
                  <button
                    onClick={() => handleAction('submit', selectedBill.id)}
                    className="bg-yellow-500 text-white px-3 py-1 rounded hover:bg-yellow-600"
                  >
                    Submit
                  </button>
                  <button
                    onClick={() => handleAction('delete', selectedBill.id)}
                    className="bg-red-500 text-white px-3 py-1 rounded hover:bg-red-600"
                  >
                    Delete
                  </button>
                </>
              )}
              {selectedBill.status === 'Pending' && (
                <>
                  <button
                    onClick={() => handleAction('approve', selectedBill.id)}
                    className="bg-green-500 text-white px-3 py-1 rounded hover:bg-green-600"
                  >
                    Approve
                  </button>
                  <button
                    onClick={() => handleAction('void', selectedBill.id)}
                    className="bg-red-500 text-white px-3 py-1 rounded hover:bg-red-600"
                  >
                    Void
                  </button>
                </>
              )}
              {selectedBill.purchase_order_id && (
                <button
                  onClick={() => handleAction('match', selectedBill.id)}
                  className="bg-purple-500 text-white px-3 py-1 rounded hover:bg-purple-600"
                >
                  Run 3-Way Match
                </button>
              )}
            </div>
          </div>
        </div>
      )}

      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Bill #</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Vendor Invoice</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Bill Date</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Due Date</th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">Total</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Match</th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase">Actions</th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {bills.map((bill) => (
              <tr key={bill.id} className="hover:bg-gray-50">
                <td className="px-6 py-4 whitespace-nowrap font-medium text-blue-600 cursor-pointer"
                    onClick={() => setSelectedBill(bill)}>
                  {bill.bill_number}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">{bill.vendor_invoice_number}</td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  {new Date(bill.bill_date).toLocaleDateString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
                  {new Date(bill.due_date).toLocaleDateString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-right font-medium">
                  {formatCurrency(bill.total)}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`px-2 py-1 text-xs rounded-full ${getStatusColor(bill.status)}`}>
                    {bill.status}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`px-2 py-1 text-xs rounded-full ${getMatchStatusColor(bill.match_status)}`}>
                    {bill.match_status}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-right">
                  <button
                    onClick={() => setSelectedBill(bill)}
                    className="text-blue-600 hover:text-blue-900 text-sm"
                  >
                    View
                  </button>
                </td>
              </tr>
            ))}
            {bills.length === 0 && (
              <tr>
                <td colSpan={8} className="px-6 py-12 text-center text-gray-500">
                  No vendor bills found. Click "New Vendor Bill" to create one.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      <div className="mt-6 bg-blue-50 rounded-lg p-4">
        <h3 className="font-medium text-blue-900 mb-2">About Vendor Bills</h3>
        <ul className="text-sm text-blue-800 space-y-1">
          <li><strong>Three-Way Matching</strong> - Automatically match bills against POs and goods receipts</li>
          <li><strong>Status Flow</strong> - Draft → Pending → Approved → Paid</li>
          <li><strong>Payment Tracking</strong> - Record partial and full payments against bills</li>
          <li><strong>Exception Handling</strong> - Identify quantity and price variances automatically</li>
        </ul>
      </div>
    </div>
  );
}

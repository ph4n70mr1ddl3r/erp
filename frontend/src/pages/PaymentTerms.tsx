import React, { useState, useEffect, useCallback } from 'react';
import { paymentTerms, type PaymentTerm } from '../api/client';
import { useToast } from '../components/Toast';
import { getErrorMessage } from '../utils/errors';
import { LoadingPage } from '../components/Spinner';

const PaymentTermsPage: React.FC = () => {
  const toast = useToast();
  const [terms, setTerms] = useState<PaymentTerm[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);
  const [editingTerm, setEditingTerm] = useState<PaymentTerm | null>(null);
  const [formData, setFormData] = useState({
    code: '',
    name: '',
    description: '',
    due_days: 30,
    discount_days: null as number | null,
    discount_percent: null as number | null,
    is_default: false,
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const res = await paymentTerms.list(1, 100);
      setTerms(res.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load payment terms'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => {
    void loadData();
  }, [loadData]);

  const resetForm = () => {
    setFormData({
      code: '',
      name: '',
      description: '',
      due_days: 30,
      discount_days: null,
      discount_percent: null,
      is_default: false,
    });
    setEditingTerm(null);
    setShowForm(false);
  };

  const handleEdit = (term: PaymentTerm) => {
    setFormData({
      code: term.code,
      name: term.name,
      description: term.description || '',
      due_days: term.due_days,
      discount_days: term.discount_days,
      discount_percent: term.discount_percent,
      is_default: term.is_default,
    });
    setEditingTerm(term);
    setShowForm(true);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (editingTerm) {
        await paymentTerms.update(editingTerm.id, {
          ...formData,
          description: formData.description || undefined,
          discount_days: formData.discount_days || undefined,
          discount_percent: formData.discount_percent || undefined,
        });
        toast.success('Payment term updated');
      } else {
        await paymentTerms.create({
          ...formData,
          description: formData.description || undefined,
          discount_days: formData.discount_days || undefined,
          discount_percent: formData.discount_percent || undefined,
        });
        toast.success('Payment term created');
      }
      resetForm();
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to save payment term'));
    }
  };

  const handleDelete = async (id: string) => {
    if (!window.confirm('Are you sure you want to delete this payment term?')) return;
    try {
      await paymentTerms.delete(id);
      toast.success('Payment term deleted');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to delete payment term'));
    }
  };

  const handleSetDefault = async (id: string) => {
    try {
      await paymentTerms.setDefault(id);
      toast.success('Default payment term updated');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to set default'));
    }
  };

  if (loading) return <LoadingPage />;

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Payment Terms</h1>
          <p className="text-gray-600 mt-1">Manage payment terms for customers and vendors</p>
        </div>
        <button
          onClick={() => setShowForm(true)}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 flex items-center gap-2"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          Add Payment Term
        </button>
      </div>

      {showForm && (
        <div className="bg-white rounded-lg shadow p-6 mb-6">
          <h2 className="text-lg font-semibold mb-4">
            {editingTerm ? 'Edit Payment Term' : 'Create Payment Term'}
          </h2>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Code *</label>
                <input
                  type="text"
                  value={formData.code}
                  onChange={(e) => setFormData({ ...formData, code: e.target.value.toUpperCase() })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="e.g., NET30"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Name *</label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="e.g., Net 30 Days"
                  required
                />
              </div>
              <div className="md:col-span-2">
                <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <input
                  type="text"
                  value={formData.description}
                  onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="e.g., Payment due within 30 days"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Due Days *</label>
                <input
                  type="number"
                  value={formData.due_days}
                  onChange={(e) => setFormData({ ...formData, due_days: parseInt(e.target.value) || 0 })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  min="0"
                  required
                />
              </div>
              <div className="flex items-center">
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={formData.is_default}
                    onChange={(e) => setFormData({ ...formData, is_default: e.target.checked })}
                    className="w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
                  />
                  <span className="text-sm font-medium text-gray-700">Set as Default</span>
                </label>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Discount Days</label>
                <input
                  type="number"
                  value={formData.discount_days ?? ''}
                  onChange={(e) => setFormData({ ...formData, discount_days: e.target.value ? parseInt(e.target.value) : null })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  min="0"
                  placeholder="Days for early payment discount"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Discount Percent</label>
                <input
                  type="number"
                  step="0.01"
                  value={formData.discount_percent ?? ''}
                  onChange={(e) => setFormData({ ...formData, discount_percent: e.target.value ? parseFloat(e.target.value) : null })}
                  className="w-full border rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  min="0"
                  max="100"
                  placeholder="e.g., 2.0"
                />
              </div>
            </div>
            <div className="flex gap-2">
              <button
                type="submit"
                className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700"
              >
                {editingTerm ? 'Update' : 'Create'}
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

      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Code</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Name</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Due Days</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Early Payment</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {terms.map((term) => (
              <tr key={term.id} className="hover:bg-gray-50">
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="flex items-center gap-2">
                    <span className="font-medium text-gray-900">{term.code}</span>
                    {term.is_default && (
                      <span className="px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded-full">Default</span>
                    )}
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <div>
                    <div className="text-sm font-medium text-gray-900">{term.name}</div>
                    {term.description && (
                      <div className="text-sm text-gray-500">{term.description}</div>
                    )}
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {term.due_days === 0 ? 'Immediate' : `${term.due_days} days`}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {term.discount_days && term.discount_percent ? (
                    <span className="text-green-600">
                      {term.discount_percent}% discount if paid within {term.discount_days} days
                    </span>
                  ) : (
                    <span className="text-gray-400">None</span>
                  )}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <span className={`px-2 py-1 text-xs rounded-full ${
                    term.status === 'Active' 
                      ? 'bg-green-100 text-green-800' 
                      : 'bg-red-100 text-red-800'
                  }`}>
                    {term.status}
                  </span>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                  <div className="flex justify-end gap-2">
                    {!term.is_default && term.status === 'Active' && (
                      <button
                        onClick={() => handleSetDefault(term.id)}
                        className="text-blue-600 hover:text-blue-900"
                        title="Set as default"
                      >
                        Set Default
                      </button>
                    )}
                    <button
                      onClick={() => handleEdit(term)}
                      className="text-indigo-600 hover:text-indigo-900"
                    >
                      Edit
                    </button>
                    <button
                      onClick={() => handleDelete(term.id)}
                      className="text-red-600 hover:text-red-900"
                    >
                      Delete
                    </button>
                  </div>
                </td>
              </tr>
            ))}
            {terms.length === 0 && (
              <tr>
                <td colSpan={6} className="px-6 py-12 text-center text-gray-500">
                  No payment terms found. Click "Add Payment Term" to create one.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      <div className="mt-6 bg-blue-50 rounded-lg p-4">
        <h3 className="font-medium text-blue-900 mb-2">Common Payment Terms Examples</h3>
        <ul className="text-sm text-blue-800 space-y-1">
          <li><strong>Net 30</strong> - Payment due within 30 days of invoice date</li>
          <li><strong>2/10 Net 30</strong> - 2% discount if paid within 10 days, otherwise due in 30 days</li>
          <li><strong>COD</strong> - Cash on delivery, payment required upon receipt</li>
          <li><strong>EOM</strong> - End of Month, payment due at the end of the invoice month</li>
          <li><strong>Prepaid</strong> - Payment required before shipment</li>
        </ul>
      </div>
    </div>
  );
};

export default PaymentTermsPage;

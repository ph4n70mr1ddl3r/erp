import { useState, useEffect } from 'react';
import { stripe, payments, StripeCheckoutSession, StripePaymentIntent } from '../api/client';

export default function StripePayments() {
  const [activeTab, setActiveTab] = useState<'checkout' | 'intent' | 'history'>('checkout');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  
  const [checkoutForm, setCheckoutForm] = useState({
    customer_id: '',
    amount: '',
    currency: 'USD',
    description: '',
    customer_email: '',
  });
  
  const [intentForm, setIntentForm] = useState({
    customer_id: '',
    amount: '',
    currency: 'USD',
    description: '',
  });
  
  const [checkoutSession, setCheckoutSession] = useState<StripeCheckoutSession | null>(null);
  const [paymentIntent, setPaymentIntent] = useState<StripePaymentIntent | null>(null);
  const [paymentHistory, setPaymentHistory] = useState<unknown[]>([]);
  
  const handleCreateCheckout = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    setSuccess(null);
    
    try {
      const response = await stripe.createCheckoutSession({
        customer_id: checkoutForm.customer_id,
        amount: Math.round(parseFloat(checkoutForm.amount) * 100),
        currency: checkoutForm.currency,
        description: checkoutForm.description || 'Payment',
        success_url: `${window.location.origin}/payments?session_id={CHECKOUT_SESSION_ID}`,
        cancel_url: `${window.location.origin}/payments?canceled=true`,
        customer_email: checkoutForm.customer_email || undefined,
      });
      
      setCheckoutSession(response.data);
      
      if (response.data.checkout_url) {
        window.open(response.data.checkout_url, '_blank');
      }
      
      setSuccess('Checkout session created! Redirecting to Stripe...');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create checkout session');
    } finally {
      setLoading(false);
    }
  };
  
  const handleCreateIntent = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    setSuccess(null);
    
    try {
      const response = await stripe.createPaymentIntent({
        customer_id: intentForm.customer_id,
        amount: Math.round(parseFloat(intentForm.amount) * 100),
        currency: intentForm.currency,
        description: intentForm.description || undefined,
      });
      
      setPaymentIntent(response.data);
      setSuccess('Payment intent created! Use the client secret to complete payment.');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create payment intent');
    } finally {
      setLoading(false);
    }
  };
  
  const handleLoadHistory = async () => {
    if (!checkoutForm.customer_id) {
      setError('Please enter a customer ID');
      return;
    }
    
    setLoading(true);
    try {
      const response = await payments.getCustomerPayments(checkoutForm.customer_id);
      setPaymentHistory(response.data.items || []);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load payment history');
    } finally {
      setLoading(false);
    }
  };
  
  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    const sessionId = params.get('session_id');
    const canceled = params.get('canceled');
    
    if (sessionId) {
      setSuccess('Payment completed successfully!');
      setActiveTab('history');
    } else if (canceled) {
      setError('Payment was canceled.');
    }
  }, []);
  
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-gray-900">Stripe Payments</h1>
      </div>
      
      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-md">
          {error}
        </div>
      )}
      
      {success && (
        <div className="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-md">
          {success}
        </div>
      )}
      
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex space-x-8">
          <button
            onClick={() => setActiveTab('checkout')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'checkout'
                ? 'border-indigo-500 text-indigo-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            Checkout Session
          </button>
          <button
            onClick={() => setActiveTab('intent')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'intent'
                ? 'border-indigo-500 text-indigo-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            Payment Intent
          </button>
          <button
            onClick={() => setActiveTab('history')}
            className={`py-4 px-1 border-b-2 font-medium text-sm ${
              activeTab === 'history'
                ? 'border-indigo-500 text-indigo-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            Payment History
          </button>
        </nav>
      </div>
      
      {activeTab === 'checkout' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-lg font-medium text-gray-900 mb-4">Create Checkout Session</h2>
            <form onSubmit={handleCreateCheckout} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700">Customer ID</label>
                <input
                  type="text"
                  value={checkoutForm.customer_id}
                  onChange={(e) => setCheckoutForm({ ...checkoutForm, customer_id: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Customer Email (optional)</label>
                <input
                  type="email"
                  value={checkoutForm.customer_email}
                  onChange={(e) => setCheckoutForm({ ...checkoutForm, customer_email: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Amount</label>
                <input
                  type="number"
                  step="0.01"
                  value={checkoutForm.amount}
                  onChange={(e) => setCheckoutForm({ ...checkoutForm, amount: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Currency</label>
                <select
                  value={checkoutForm.currency}
                  onChange={(e) => setCheckoutForm({ ...checkoutForm, currency: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                >
                  <option value="USD">USD</option>
                  <option value="EUR">EUR</option>
                  <option value="GBP">GBP</option>
                  <option value="CAD">CAD</option>
                  <option value="AUD">AUD</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Description</label>
                <input
                  type="text"
                  value={checkoutForm.description}
                  onChange={(e) => setCheckoutForm({ ...checkoutForm, description: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                />
              </div>
              <button
                type="submit"
                disabled={loading}
                className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
              >
                {loading ? 'Creating...' : 'Create Checkout Session'}
              </button>
            </form>
          </div>
          
          {checkoutSession && (
            <div className="bg-white shadow rounded-lg p-6">
              <h2 className="text-lg font-medium text-gray-900 mb-4">Session Details</h2>
              <dl className="space-y-3">
                <div>
                  <dt className="text-sm font-medium text-gray-500">Session ID</dt>
                  <dd className="mt-1 text-sm text-gray-900 font-mono">{checkoutSession.id}</dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">Stripe Session ID</dt>
                  <dd className="mt-1 text-sm text-gray-900 font-mono text-xs">{checkoutSession.stripe_session_id}</dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">Amount</dt>
                  <dd className="mt-1 text-sm text-gray-900">
                    {(checkoutSession.amount / 100).toFixed(2)} {checkoutSession.currency.toUpperCase()}
                  </dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">Status</dt>
                  <dd className="mt-1">
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                      {checkoutSession.status}
                    </span>
                  </dd>
                </div>
                {checkoutSession.checkout_url && (
                  <div>
                    <dt className="text-sm font-medium text-gray-500">Checkout URL</dt>
                    <dd className="mt-1">
                      <a
                        href={checkoutSession.checkout_url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-indigo-600 hover:text-indigo-500 text-sm"
                      >
                        Open Checkout Page
                      </a>
                    </dd>
                  </div>
                )}
              </dl>
            </div>
          )}
        </div>
      )}
      
      {activeTab === 'intent' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="bg-white shadow rounded-lg p-6">
            <h2 className="text-lg font-medium text-gray-900 mb-4">Create Payment Intent</h2>
            <form onSubmit={handleCreateIntent} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700">Customer ID</label>
                <input
                  type="text"
                  value={intentForm.customer_id}
                  onChange={(e) => setIntentForm({ ...intentForm, customer_id: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Amount</label>
                <input
                  type="number"
                  step="0.01"
                  value={intentForm.amount}
                  onChange={(e) => setIntentForm({ ...intentForm, amount: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Currency</label>
                <select
                  value={intentForm.currency}
                  onChange={(e) => setIntentForm({ ...intentForm, currency: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                >
                  <option value="USD">USD</option>
                  <option value="EUR">EUR</option>
                  <option value="GBP">GBP</option>
                  <option value="CAD">CAD</option>
                  <option value="AUD">AUD</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700">Description</label>
                <input
                  type="text"
                  value={intentForm.description}
                  onChange={(e) => setIntentForm({ ...intentForm, description: e.target.value })}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                />
              </div>
              <button
                type="submit"
                disabled={loading}
                className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50"
              >
                {loading ? 'Creating...' : 'Create Payment Intent'}
              </button>
            </form>
          </div>
          
          {paymentIntent && (
            <div className="bg-white shadow rounded-lg p-6">
              <h2 className="text-lg font-medium text-gray-900 mb-4">Intent Details</h2>
              <dl className="space-y-3">
                <div>
                  <dt className="text-sm font-medium text-gray-500">Intent ID</dt>
                  <dd className="mt-1 text-sm text-gray-900 font-mono">{paymentIntent.id}</dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">Stripe Intent ID</dt>
                  <dd className="mt-1 text-sm text-gray-900 font-mono text-xs">{paymentIntent.stripe_intent_id}</dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">Client Secret</dt>
                  <dd className="mt-1 text-sm text-gray-900 font-mono text-xs break-all">{paymentIntent.client_secret}</dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">Amount</dt>
                  <dd className="mt-1 text-sm text-gray-900">
                    {(paymentIntent.amount / 100).toFixed(2)} {paymentIntent.currency.toUpperCase()}
                  </dd>
                </div>
                <div>
                  <dt className="text-sm font-medium text-gray-500">Status</dt>
                  <dd className="mt-1">
                    <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                      {paymentIntent.status}
                    </span>
                  </dd>
                </div>
              </dl>
              <div className="mt-6 p-4 bg-gray-50 rounded-md">
                <h3 className="text-sm font-medium text-gray-900 mb-2">Integration Code</h3>
                <pre className="text-xs text-gray-600 overflow-x-auto">
{`const stripe = Stripe('pk_test_...');
const elements = stripe.elements({
  clientSecret: '${paymentIntent.client_secret}'
});
// Use elements to build your payment form`}
                </pre>
              </div>
            </div>
          )}
        </div>
      )}
      
      {activeTab === 'history' && (
        <div className="bg-white shadow rounded-lg">
          <div className="p-6 border-b border-gray-200">
            <div className="flex items-center justify-between">
              <h2 className="text-lg font-medium text-gray-900">Payment History</h2>
              <button
                onClick={handleLoadHistory}
                disabled={loading}
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-indigo-700 bg-indigo-100 hover:bg-indigo-200"
              >
                {loading ? 'Loading...' : 'Load History'}
              </button>
            </div>
            <p className="mt-1 text-sm text-gray-500">
              Enter a Customer ID in the Checkout tab first, then click Load History.
            </p>
          </div>
          
          {paymentHistory.length > 0 ? (
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Payment #</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Amount</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Method</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Status</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Date</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {(paymentHistory as { payment_number: string; amount: number; currency: string; payment_method: string; status: string; paid_at: string }[]).map((payment, idx) => (
                  <tr key={idx}>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                      {payment.payment_number}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {(payment.amount / 100).toFixed(2)} {payment.currency?.toUpperCase()}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {payment.payment_method}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                        payment.status === 'Completed' ? 'bg-green-100 text-green-800' :
                        payment.status === 'Pending' ? 'bg-yellow-100 text-yellow-800' :
                        'bg-gray-100 text-gray-800'
                      }`}>
                        {payment.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {payment.paid_at ? new Date(payment.paid_at).toLocaleDateString() : '-'}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          ) : (
            <div className="p-6 text-center text-gray-500">
              No payment history to display
            </div>
          )}
        </div>
      )}
    </div>
  );
}

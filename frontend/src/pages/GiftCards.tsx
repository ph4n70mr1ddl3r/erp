import React, { useState, useEffect, useCallback } from 'react';
import { giftcards } from '../api/client';
import { useToast } from '../components/Toast';
import { getErrorMessage } from '../utils/errors';

interface GiftCard {
  id: string;
  card_number: string;
  gift_card_type: 'Physical' | 'Digital' | 'ECode';
  initial_balance: number;
  current_balance: number;
  currency: string;
  status: 'Active' | 'Inactive' | 'Redeemed' | 'Expired' | 'Cancelled';
  issued_date: string;
  expiry_date?: string;
  recipient_name?: string;
  recipient_email?: string;
}

interface GiftCardTransaction {
  id: string;
  transaction_number: string;
  gift_card_id: string;
  transaction_type: 'Issue' | 'Reload' | 'Redeem' | 'Refund' | 'Adjust' | 'Expire';
  amount: number;
  balance_before: number;
  balance_after: number;
  created_at: string;
}

const GiftCards: React.FC = () => {
  const toast = useToast();
  const [giftCards, setGiftCards] = useState<GiftCard[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreate, setShowCreate] = useState(false);
  const [showRedeem, setShowRedeem] = useState<string | null>(null);
  const [showReload, setShowReload] = useState<string | null>(null);
  const [selectedCard, setSelectedCard] = useState<GiftCard | null>(null);
  const [transactions, setTransactions] = useState<GiftCardTransaction[]>([]);

  const [newCard, setNewCard] = useState({
    gift_card_type: 'Digital' as const,
    initial_balance: 10000,
    currency: 'USD',
    recipient_name: '',
    recipient_email: '',
    message: '',
  });

  const [redeemAmount, setRedeemAmount] = useState(0);
  const [reloadAmount, setReloadAmount] = useState(0);

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const res = await giftcards.list();
      setGiftCards(res.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load gift cards'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => {
    void loadData();
  }, [loadData]);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await giftcards.create({
        ...newCard,
        initial_balance: Math.round(newCard.initial_balance * 100),
      });
      setNewCard({
        gift_card_type: 'Digital',
        initial_balance: 10000,
        currency: 'USD',
        recipient_name: '',
        recipient_email: '',
        message: '',
      });
      setShowCreate(false);
      loadData();
      toast.success('Gift card created successfully');
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create gift card'));
    }
  };

  const handleRedeem = async (id: string) => {
    try {
      await giftcards.redeem(id, { amount: Math.round(redeemAmount * 100) });
      setShowRedeem(null);
      setRedeemAmount(0);
      loadData();
      toast.success('Gift card redeemed successfully');
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to redeem gift card'));
    }
  };

  const handleReload = async (id: string) => {
    try {
      await giftcards.reload(id, { amount: Math.round(reloadAmount * 100) });
      setShowReload(null);
      setReloadAmount(0);
      loadData();
      toast.success('Gift card reloaded successfully');
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to reload gift card'));
    }
  };

  const handleCancel = async (id: string) => {
    if (!window.confirm('Are you sure you want to cancel this gift card?')) return;
    try {
      await giftcards.cancel(id, { reason: 'Cancelled by admin' });
      loadData();
      toast.success('Gift card cancelled');
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to cancel gift card'));
    }
  };

  const viewTransactions = async (card: GiftCard) => {
    try {
      const res = await giftcards.getTransactions(card.id);
      setSelectedCard(card);
      setTransactions(res.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load transactions'));
    }
  };

  const formatCurrency = (cents: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
    }).format(cents / 100);
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Active':
        return 'bg-green-100 text-green-800';
      case 'Redeemed':
        return 'bg-blue-100 text-blue-800';
      case 'Expired':
        return 'bg-yellow-100 text-yellow-800';
      case 'Cancelled':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Gift Cards</h1>
        <button
          onClick={() => setShowCreate(true)}
          className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
        >
          Issue Gift Card
        </button>
      </div>

      {showCreate && (
        <Modal title="Issue Gift Card" onClose={() => setShowCreate(false)}>
          <form onSubmit={handleCreate}>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-1">Type</label>
              <select
                value={newCard.gift_card_type}
                onChange={(e) => setNewCard({ ...newCard, gift_card_type: e.target.value as 'Physical' | 'Digital' | 'ECode' })}
                className="w-full border rounded px-3 py-2"
              >
                <option value="Digital">Digital</option>
                <option value="Physical">Physical</option>
                <option value="ECode">E-Code</option>
              </select>
            </div>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-1">Initial Balance ($)</label>
              <input
                type="number"
                value={newCard.initial_balance / 100}
                onChange={(e) => setNewCard({ ...newCard, initial_balance: parseFloat(e.target.value) * 100 })}
                className="w-full border rounded px-3 py-2"
                min="1"
                step="0.01"
                required
              />
            </div>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-1">Currency</label>
              <select
                value={newCard.currency}
                onChange={(e) => setNewCard({ ...newCard, currency: e.target.value })}
                className="w-full border rounded px-3 py-2"
              >
                <option value="USD">USD</option>
                <option value="EUR">EUR</option>
                <option value="GBP">GBP</option>
              </select>
            </div>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-1">Recipient Name</label>
              <input
                type="text"
                value={newCard.recipient_name}
                onChange={(e) => setNewCard({ ...newCard, recipient_name: e.target.value })}
                className="w-full border rounded px-3 py-2"
              />
            </div>
            <div className="mb-4">
              <label className="block text-sm font-medium mb-1">Recipient Email</label>
              <input
                type="email"
                value={newCard.recipient_email}
                onChange={(e) => setNewCard({ ...newCard, recipient_email: e.target.value })}
                className="w-full border rounded px-3 py-2"
              />
            </div>
            <div className="flex justify-end gap-2">
              <button type="button" onClick={() => setShowCreate(false)} className="px-4 py-2 border rounded">
                Cancel
              </button>
              <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded">
                Issue
              </button>
            </div>
          </form>
        </Modal>
      )}

      {showRedeem && (
        <Modal title="Redeem Gift Card" onClose={() => setShowRedeem(null)}>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-1">Amount ($)</label>
            <input
              type="number"
              value={redeemAmount}
              onChange={(e) => setRedeemAmount(parseFloat(e.target.value))}
              className="w-full border rounded px-3 py-2"
              min="0.01"
              step="0.01"
            />
          </div>
          <div className="flex justify-end gap-2">
            <button onClick={() => setShowRedeem(null)} className="px-4 py-2 border rounded">
              Cancel
            </button>
            <button onClick={() => handleRedeem(showRedeem)} className="px-4 py-2 bg-green-600 text-white rounded">
              Redeem
            </button>
          </div>
        </Modal>
      )}

      {showReload && (
        <Modal title="Reload Gift Card" onClose={() => setShowReload(null)}>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-1">Amount ($)</label>
            <input
              type="number"
              value={reloadAmount}
              onChange={(e) => setReloadAmount(parseFloat(e.target.value))}
              className="w-full border rounded px-3 py-2"
              min="0.01"
              step="0.01"
            />
          </div>
          <div className="flex justify-end gap-2">
            <button onClick={() => setShowReload(null)} className="px-4 py-2 border rounded">
              Cancel
            </button>
            <button onClick={() => handleReload(showReload)} className="px-4 py-2 bg-blue-600 text-white rounded">
              Reload
            </button>
          </div>
        </Modal>
      )}

      {selectedCard && (
        <Modal title={`Transactions - ${selectedCard.card_number}`} onClose={() => setSelectedCard(null)}>
          <div className="max-h-96 overflow-y-auto">
            {transactions.length === 0 ? (
              <p className="text-gray-500 text-center py-4">No transactions found</p>
            ) : (
              <table className="w-full text-sm">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-2 py-2 text-left">Type</th>
                    <th className="px-2 py-2 text-right">Amount</th>
                    <th className="px-2 py-2 text-right">Balance</th>
                    <th className="px-2 py-2 text-left">Date</th>
                  </tr>
                </thead>
                <tbody>
                  {transactions.map((tx) => (
                    <tr key={tx.id} className="border-t">
                      <td className="px-2 py-2">{tx.transaction_type}</td>
                      <td className="px-2 py-2 text-right">
                        {formatCurrency(Math.abs(tx.amount), selectedCard.currency)}
                      </td>
                      <td className="px-2 py-2 text-right">{formatCurrency(tx.balance_after, selectedCard.currency)}</td>
                      <td className="px-2 py-2 text-xs">{new Date(tx.created_at).toLocaleDateString()}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </div>
        </Modal>
      )}

      <div className="bg-white rounded-lg shadow overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-4 py-3 text-left">Card Number</th>
              <th className="px-4 py-3 text-left">Type</th>
              <th className="px-4 py-3 text-right">Balance</th>
              <th className="px-4 py-3 text-left">Recipient</th>
              <th className="px-4 py-3 text-left">Status</th>
              <th className="px-4 py-3 text-left">Issued</th>
              <th className="px-4 py-3 text-left">Actions</th>
            </tr>
          </thead>
          <tbody>
            {giftCards.length === 0 ? (
              <tr>
                <td colSpan={7} className="px-4 py-8 text-center text-gray-500">
                  No gift cards found. Click "Issue Gift Card" to create one.
                </td>
              </tr>
            ) : (
              giftCards.map((card) => (
                <tr key={card.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3 font-mono">{card.card_number}</td>
                  <td className="px-4 py-3">{card.gift_card_type}</td>
                  <td className="px-4 py-3 text-right font-medium">
                    {formatCurrency(card.current_balance, card.currency)}
                    {card.current_balance !== card.initial_balance && (
                      <span className="text-gray-400 text-xs ml-1">
                        / {formatCurrency(card.initial_balance, card.currency)}
                      </span>
                    )}
                  </td>
                  <td className="px-4 py-3">
                    {card.recipient_name || card.recipient_email || '-'}
                  </td>
                  <td className="px-4 py-3">
                    <span className={`px-2 py-1 rounded text-xs ${getStatusColor(card.status)}`}>
                      {card.status}
                    </span>
                  </td>
                  <td className="px-4 py-3 text-sm">{new Date(card.issued_date).toLocaleDateString()}</td>
                  <td className="px-4 py-3">
                    <div className="flex gap-2">
                      <button
                        onClick={() => viewTransactions(card)}
                        className="text-blue-600 hover:text-blue-800 text-sm"
                      >
                        History
                      </button>
                      {card.status === 'Active' && card.current_balance > 0 && (
                        <button
                          onClick={() => setShowRedeem(card.id)}
                          className="text-green-600 hover:text-green-800 text-sm"
                        >
                          Redeem
                        </button>
                      )}
                      {card.status === 'Active' && (
                        <button
                          onClick={() => setShowReload(card.id)}
                          className="text-blue-600 hover:text-blue-800 text-sm"
                        >
                          Reload
                        </button>
                      )}
                      {card.status === 'Active' && (
                        <button
                          onClick={() => handleCancel(card.id)}
                          className="text-red-600 hover:text-red-800 text-sm"
                        >
                          Cancel
                        </button>
                      )}
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

const Modal: React.FC<{ title: string; onClose: () => void; children: React.ReactNode }> = ({
  title,
  onClose,
  children,
}) => (
  <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onClick={onClose}>
    <div className="bg-white p-6 rounded-lg shadow-lg w-96" onClick={(e) => e.stopPropagation()}>
      <h2 className="text-lg font-semibold mb-4">{title}</h2>
      {children}
    </div>
  </div>
);

export default GiftCards;

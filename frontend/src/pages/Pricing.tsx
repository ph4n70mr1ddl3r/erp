import React, { useState, useEffect } from 'react';
import { pricing } from '../api/client';

interface PriceBook {
  id: string;
  name: string;
  code: string;
  currency: string;
  is_default: boolean;
}

interface Discount {
  id: string;
  name: string;
  code: string;
  discount_type: string;
  value: number;
}

interface Promotion {
  id: string;
  name: string;
  code: string;
  status: string;
}

const Pricing: React.FC = () => {
  const [activeTab, setActiveTab] = useState('pricebooks');
  const [priceBooks, setPriceBooks] = useState<PriceBook[]>([]);
  const [discounts, setDiscounts] = useState<Discount[]>([]);
  const [promotions, setPromotions] = useState<Promotion[]>([]);
  
  const [showCreatePriceBook, setShowCreatePriceBook] = useState(false);
  const [showCreateDiscount, setShowCreateDiscount] = useState(false);
  const [showCreatePromotion, setShowCreatePromotion] = useState(false);

  const [newPriceBook, setNewPriceBook] = useState({ name: '', code: '', currency: 'USD' });
  const [newDiscount, setNewDiscount] = useState({ name: '', code: '', discount_type: 'Percentage', value: 0 });
  const [newPromotion, setNewPromotion] = useState({ name: '', code: '', start_date: '', end_date: '', rules: '{}', rewards: '{}' });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [pbRes, discRes, promoRes] = await Promise.all([
        pricing.listPriceBooks(),
        pricing.listDiscounts(),
        pricing.listPromotions(),
      ]);
      setPriceBooks(pbRes.data);
      setDiscounts(discRes.data);
      setPromotions(promoRes.data);
    } catch (error) {
      console.error('Failed to load pricing data:', error);
    }
  };

  const handleCreatePriceBook = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await pricing.createPriceBook(newPriceBook);
      setNewPriceBook({ name: '', code: '', currency: 'USD' });
      setShowCreatePriceBook(false);
      loadData();
    } catch (error) {
      console.error('Failed to create price book:', error);
    }
  };

  const handleCreateDiscount = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await pricing.createDiscount({ ...newDiscount, requires_code: true });
      setNewDiscount({ name: '', code: '', discount_type: 'Percentage', value: 0 });
      setShowCreateDiscount(false);
      loadData();
    } catch (error) {
      console.error('Failed to create discount:', error);
    }
  };

  const handleCreatePromotion = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await pricing.createPromotion(newPromotion);
      setNewPromotion({ name: '', code: '', start_date: '', end_date: '', rules: '{}', rewards: '{}' });
      setShowCreatePromotion(false);
      loadData();
    } catch (error) {
      console.error('Failed to create promotion:', error);
    }
  };

  const tabs = [
    { id: 'pricebooks', label: 'Price Books' },
    { id: 'discounts', label: 'Discounts' },
    { id: 'promotions', label: 'Promotions' },
  ];

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Pricing Engine</h1>
        {activeTab === 'pricebooks' && (
          <button
            onClick={() => setShowCreatePriceBook(true)}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            New Price Book
          </button>
        )}
        {activeTab === 'discounts' && (
          <button
            onClick={() => setShowCreateDiscount(true)}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            New Discount
          </button>
        )}
        {activeTab === 'promotions' && (
          <button
            onClick={() => setShowCreatePromotion(true)}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            New Promotion
          </button>
        )}
      </div>

      <div className="flex space-x-4 mb-6">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`px-4 py-2 rounded ${activeTab === tab.id ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {showCreatePriceBook && (
        <Modal title="Create Price Book" onClose={() => setShowCreatePriceBook(false)}>
          <form onSubmit={handleCreatePriceBook}>
            <input
              type="text"
              value={newPriceBook.name}
              onChange={(e) => setNewPriceBook({ ...newPriceBook, name: e.target.value })}
              placeholder="Name"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={newPriceBook.code}
              onChange={(e) => setNewPriceBook({ ...newPriceBook, code: e.target.value })}
              placeholder="Code"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <select
              value={newPriceBook.currency}
              onChange={(e) => setNewPriceBook({ ...newPriceBook, currency: e.target.value })}
              className="w-full border rounded px-3 py-2 mb-4"
            >
              <option value="USD">USD</option>
              <option value="EUR">EUR</option>
              <option value="GBP">GBP</option>
            </select>
            <div className="flex justify-end">
              <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded">Create</button>
            </div>
          </form>
        </Modal>
      )}

      {showCreateDiscount && (
        <Modal title="Create Discount" onClose={() => setShowCreateDiscount(false)}>
          <form onSubmit={handleCreateDiscount}>
            <input
              type="text"
              value={newDiscount.name}
              onChange={(e) => setNewDiscount({ ...newDiscount, name: e.target.value })}
              placeholder="Name"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={newDiscount.code}
              onChange={(e) => setNewDiscount({ ...newDiscount, code: e.target.value })}
              placeholder="Code"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <select
              value={newDiscount.discount_type}
              onChange={(e) => setNewDiscount({ ...newDiscount, discount_type: e.target.value })}
              className="w-full border rounded px-3 py-2 mb-4"
            >
              <option value="Percentage">Percentage</option>
              <option value="FixedAmount">Fixed Amount</option>
            </select>
            <input
              type="number"
              value={newDiscount.value}
              onChange={(e) => setNewDiscount({ ...newDiscount, value: parseFloat(e.target.value) })}
              placeholder="Value"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <div className="flex justify-end">
              <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded">Create</button>
            </div>
          </form>
        </Modal>
      )}

      {showCreatePromotion && (
        <Modal title="Create Promotion" onClose={() => setShowCreatePromotion(false)}>
          <form onSubmit={handleCreatePromotion}>
            <input
              type="text"
              value={newPromotion.name}
              onChange={(e) => setNewPromotion({ ...newPromotion, name: e.target.value })}
              placeholder="Name"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={newPromotion.code}
              onChange={(e) => setNewPromotion({ ...newPromotion, code: e.target.value })}
              placeholder="Code"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="datetime-local"
              value={newPromotion.start_date}
              onChange={(e) => setNewPromotion({ ...newPromotion, start_date: e.target.value })}
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="datetime-local"
              value={newPromotion.end_date}
              onChange={(e) => setNewPromotion({ ...newPromotion, end_date: e.target.value })}
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <div className="flex justify-end">
              <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded">Create</button>
            </div>
          </form>
        </Modal>
      )}

      <div className="bg-white rounded-lg shadow">
        {activeTab === 'pricebooks' && (
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left">Name</th>
                <th className="px-4 py-3 text-left">Code</th>
                <th className="px-4 py-3 text-left">Currency</th>
                <th className="px-4 py-3 text-left">Default</th>
              </tr>
            </thead>
            <tbody>
              {priceBooks.map((pb) => (
                <tr key={pb.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3">{pb.name}</td>
                  <td className="px-4 py-3">{pb.code}</td>
                  <td className="px-4 py-3">{pb.currency}</td>
                  <td className="px-4 py-3">{pb.is_default ? 'Yes' : 'No'}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {activeTab === 'discounts' && (
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left">Name</th>
                <th className="px-4 py-3 text-left">Code</th>
                <th className="px-4 py-3 text-left">Type</th>
                <th className="px-4 py-3 text-left">Value</th>
              </tr>
            </thead>
            <tbody>
              {discounts.map((d) => (
                <tr key={d.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3">{d.name}</td>
                  <td className="px-4 py-3">{d.code}</td>
                  <td className="px-4 py-3">{d.discount_type}</td>
                  <td className="px-4 py-3">
                    {d.discount_type === 'Percentage' ? `${d.value}%` : `$${d.value}`}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {activeTab === 'promotions' && (
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left">Name</th>
                <th className="px-4 py-3 text-left">Code</th>
                <th className="px-4 py-3 text-left">Status</th>
              </tr>
            </thead>
            <tbody>
              {promotions.map((p) => (
                <tr key={p.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3">{p.name}</td>
                  <td className="px-4 py-3">{p.code}</td>
                  <td className="px-4 py-3">
                    <span className={`px-2 py-1 rounded text-xs ${
                      p.status === 'Active' ? 'bg-green-100 text-green-800' :
                      p.status === 'Draft' ? 'bg-gray-100 text-gray-800' :
                      'bg-yellow-100 text-yellow-800'
                    }`}>
                      {p.status}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
};

const Modal: React.FC<{ title: string; onClose: () => void; children: React.ReactNode }> = ({ title, onClose, children }) => (
  <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div className="bg-white p-6 rounded-lg shadow-lg w-96">
      <h2 className="text-lg font-semibold mb-4">{title}</h2>
      {children}
      <button onClick={onClose} className="mt-4 text-gray-500 hover:text-gray-700">Close</button>
    </div>
  </div>
);

export default Pricing;

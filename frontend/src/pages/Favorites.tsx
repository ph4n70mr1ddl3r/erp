import React, { useState, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import { Star, Trash2, Package, Users, ShoppingCart, FileText, Briefcase, Ticket, Folder } from 'lucide-react';
import { favorites as favoritesApi } from '../api/client';
import { useToast } from '../components/Toast';
import { getErrorMessage } from '../utils/errors';
import { Spinner } from '../components/Spinner';

interface Favorite {
  id: string;
  user_id: string;
  favorite_type: string;
  entity_id: string | null;
  entity_name: string;
  entity_code: string | null;
  notes: string | null;
  created_at: string;
}

interface FavoriteListResponse {
  items: Favorite[];
  total: number;
}

const typeIcons: Record<string, React.FC<{className?: string}>> = {
  Product: Package,
  Customer: Users,
  Vendor: Users,
  Order: ShoppingCart,
  Invoice: FileText,
  Quote: FileText,
  PurchaseOrder: ShoppingCart,
  Employee: Users,
  Project: Folder,
  Ticket: Ticket,
  Report: FileText,
  Page: Briefcase,
};

const typeRoutes: Record<string, string> = {
  Product: '/inventory',
  Customer: '/sales',
  Vendor: '/purchasing',
  Order: '/sales',
  Invoice: '/sales',
  Quote: '/sales',
  PurchaseOrder: '/purchasing',
  Employee: '/hr',
  Project: '/projects',
  Ticket: '/service',
  Report: '/reports',
  Page: '/',
};

const Favorites: React.FC = () => {
  const toast = useToast();
  const [loading, setLoading] = useState(true);
  const [favorites, setFavorites] = useState<Favorite[]>([]);
  const [total, setTotal] = useState(0);
  const [filterType, setFilterType] = useState<string | null>(null);

  const loadFavorites = useCallback(async () => {
    try {
      setLoading(true);
      const params = filterType ? `?favorite_type=${filterType}` : '';
      const res = await favoritesApi.list(params);
      const data: FavoriteListResponse = res.data;
      setFavorites(data.items);
      setTotal(data.total);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load favorites'));
    } finally {
      setLoading(false);
    }
  }, [filterType, toast]);

  useEffect(() => {
    void loadFavorites();
  }, [loadFavorites]);

  const handleDelete = async (id: string) => {
    try {
      await favoritesApi.delete(id);
      setFavorites(favorites.filter(f => f.id !== id));
      setTotal(total - 1);
      toast.success('Removed from favorites');
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to remove favorite'));
    }
  };

  const groupedFavorites = favorites.reduce((acc, fav) => {
    const type = fav.favorite_type;
    if (!acc[type]) acc[type] = [];
    acc[type].push(fav);
    return acc;
  }, {} as Record<string, Favorite[]>);

  const types = Object.keys(groupedFavorites);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <Spinner size="lg" />
      </div>
    );
  }

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-2xl font-bold">Favorites</h1>
          <p className="text-gray-500 mt-1">{total} items saved</p>
        </div>
        <div className="flex items-center gap-2">
          <select
            value={filterType || ''}
            onChange={(e) => setFilterType(e.target.value || null)}
            className="border rounded px-3 py-2"
          >
            <option value="">All Types</option>
            <option value="Product">Products</option>
            <option value="Customer">Customers</option>
            <option value="Vendor">Vendors</option>
            <option value="Order">Orders</option>
            <option value="Invoice">Invoices</option>
            <option value="Project">Projects</option>
            <option value="Ticket">Tickets</option>
          </select>
        </div>
      </div>

      {favorites.length === 0 ? (
        <div className="text-center py-12">
          <Star className="w-16 h-16 text-gray-300 mx-auto mb-4" />
          <h3 className="text-lg font-medium text-gray-900">No favorites yet</h3>
          <p className="text-gray-500 mt-2">Star items throughout the system to add them here</p>
        </div>
      ) : (
        <div className="space-y-6">
          {types.map((type) => {
            const Icon = typeIcons[type] || Star;
            const items = groupedFavorites[type];
            
            return (
              <div key={type} className="bg-white rounded-lg shadow">
                <div className="px-4 py-3 border-b bg-gray-50 rounded-t-lg flex items-center gap-2">
                  <Icon className="w-5 h-5 text-gray-500" />
                  <h2 className="font-semibold">{type}s</h2>
                  <span className="text-sm text-gray-500">({items.length})</span>
                </div>
                <div className="divide-y">
                  {items.map((fav) => (
                    <div key={fav.id} className="px-4 py-3 flex items-center justify-between hover:bg-gray-50">
                      <div className="flex items-center gap-3">
                        <Star className="w-5 h-5 text-yellow-400 fill-yellow-400" />
                        <div>
                          <Link
                            to={`${typeRoutes[type] || '/'}${fav.entity_id ? `/${fav.entity_id}` : ''}`}
                            className="font-medium text-blue-600 hover:underline"
                          >
                            {fav.entity_name}
                          </Link>
                          {fav.entity_code && (
                            <span className="ml-2 text-sm text-gray-500">({fav.entity_code})</span>
                          )}
                          {fav.notes && (
                            <p className="text-sm text-gray-500 mt-0.5">{fav.notes}</p>
                          )}
                        </div>
                      </div>
                      <div className="flex items-center gap-4">
                        <span className="text-sm text-gray-400">
                          Added {new Date(fav.created_at).toLocaleDateString()}
                        </span>
                        <button
                          onClick={() => handleDelete(fav.id)}
                          className="p-1 text-gray-400 hover:text-red-500"
                          title="Remove from favorites"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
};

export default Favorites;

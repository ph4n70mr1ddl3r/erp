import { useEffect, useState } from 'react';
import { inventory } from '../api/client';
import type { CreateProductRequest } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { SearchInput } from '../components/SearchInput';
import { ConfirmDialog } from '../components/ConfirmDialog';
import type { Product, Warehouse } from '../types';
import { getErrorMessage } from '../types';

export default function Inventory() {
  const toast = useToast();
  const [products, setProducts] = useState<Product[]>([]);
  const [warehouses, setWarehouses] = useState<Warehouse[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [search, setSearch] = useState('');
  const [showProductModal, setShowProductModal] = useState(false);
  const [showWarehouseModal, setShowWarehouseModal] = useState(false);
  const [deleteConfirm, setDeleteConfirm] = useState<{ type: 'product' | 'warehouse'; id: string; name: string } | null>(null);
  const [editingProduct, setEditingProduct] = useState<Product | null>(null);
  const [newProduct, setNewProduct] = useState<CreateProductRequest>({ sku: '', name: '', product_type: 'Goods', unit_of_measure: 'PCS' });
  const [newWarehouse, setNewWarehouse] = useState({ code: '', name: '' });

  useEffect(() => { loadData(); }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [prodRes, whRes] = await Promise.all([inventory.getProducts(1, 50), inventory.getWarehouses()]);
      setProducts(prodRes.data.items);
      setWarehouses(whRes.data);
    } catch (err) {
      toast.error('Failed to load inventory data');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateProduct = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await inventory.createProduct(newProduct);
      toast.success('Product created successfully');
      setShowProductModal(false);
      setNewProduct({ sku: '', name: '', product_type: 'Goods', unit_of_measure: 'PCS' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const handleUpdateProduct = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!editingProduct) return;
    try {
      setSaving(true);
      await inventory.updateProduct(editingProduct.id, newProduct);
      toast.success('Product updated successfully');
      setShowProductModal(false);
      setEditingProduct(null);
      setNewProduct({ sku: '', name: '', product_type: 'Goods', unit_of_measure: 'PCS' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateWarehouse = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await inventory.createWarehouse(newWarehouse);
      toast.success('Warehouse created successfully');
      setShowWarehouseModal(false);
      setNewWarehouse({ code: '', name: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setSaving(false);
    }
  };

  const handleDeleteProduct = async () => {
    if (!deleteConfirm || deleteConfirm.type !== 'product') return;
    try {
      await inventory.deleteProduct(deleteConfirm.id);
      toast.success('Product deleted successfully');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err));
    } finally {
      setDeleteConfirm(null);
    }
  };

  const openEditProduct = (product: Product) => {
    setEditingProduct(product);
    setNewProduct({ 
      sku: product.sku, 
      name: product.name, 
      product_type: (product.product_type as CreateProductRequest['product_type']) || 'Goods', 
      unit_of_measure: product.unit_of_measure 
    });
    setShowProductModal(true);
  };

  const closeProductModal = () => {
    setShowProductModal(false);
    setEditingProduct(null);
    setNewProduct({ sku: '', name: '', product_type: 'Goods', unit_of_measure: 'PCS' });
  };

  const filteredProducts = products.filter(p => 
    p.sku.toLowerCase().includes(search.toLowerCase()) ||
    p.name.toLowerCase().includes(search.toLowerCase())
  );

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Inventory</h1>
        <div className="flex gap-2">
          <button onClick={() => setShowProductModal(true)} className="btn btn-primary">Add Product</button>
          <button onClick={() => setShowWarehouseModal(true)} className="btn btn-secondary">Add Warehouse</button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Products</p>
          <p className="text-2xl font-bold">{products.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Warehouses</p>
          <p className="text-2xl font-bold">{warehouses.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Active Products</p>
          <p className="text-2xl font-bold">{products.filter(p => p.status === 'Active').length}</p>
        </div>
      </div>

      <div className="card">
        <div className="p-4 border-b flex justify-between items-center">
          <h2 className="text-lg font-semibold">Products</h2>
          <SearchInput value={search} onChange={setSearch} placeholder="Search products..." />
        </div>
        {filteredProducts.length === 0 ? (
          <div className="p-8 text-center text-gray-500">
            {search ? 'No products match your search' : 'No products found'}
          </div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">SKU</th>
                <th className="table-header">Name</th>
                <th className="table-header">Type</th>
                <th className="table-header">UoM</th>
                <th className="table-header">Status</th>
                <th className="table-header">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredProducts.map((p) => (
                <tr key={p.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{p.sku}</td>
                  <td className="table-cell">{p.name}</td>
                  <td className="table-cell">{p.product_type}</td>
                  <td className="table-cell">{p.unit_of_measure}</td>
                  <td className="table-cell">
                    <span className={`badge ${p.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>{p.status}</span>
                  </td>
                  <td className="table-cell">
                    <div className="flex gap-2">
                      <button onClick={() => openEditProduct(p)} className="text-blue-600 hover:text-blue-800 text-sm">Edit</button>
                      <button onClick={() => setDeleteConfirm({ type: 'product', id: p.id, name: p.name })} className="text-red-600 hover:text-red-800 text-sm">Delete</button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      <div className="card mt-6">
        <h2 className="text-lg font-semibold p-4 border-b">Warehouses</h2>
        {warehouses.length === 0 ? (
          <div className="p-8 text-center text-gray-500">No warehouses found</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Code</th>
                <th className="table-header">Name</th>
                <th className="table-header">Status</th>
              </tr>
            </thead>
            <tbody>
              {warehouses.map((w) => (
                <tr key={w.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono">{w.code}</td>
                  <td className="table-cell">{w.name}</td>
                  <td className="table-cell">
                    <span className={`badge ${w.status === 'Active' ? 'badge-success' : 'badge-warning'}`}>{w.status}</span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Product Modal */}
      {showProductModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">{editingProduct ? 'Edit Product' : 'New Product'}</h2>
            <form onSubmit={editingProduct ? handleUpdateProduct : handleCreateProduct} className="space-y-4">
              <div>
                <label className="label">SKU</label>
                <input className="input" value={newProduct.sku} onChange={(e) => setNewProduct({ ...newProduct, sku: e.target.value })} required />
              </div>
              <div>
                <label className="label">Name</label>
                <input className="input" value={newProduct.name} onChange={(e) => setNewProduct({ ...newProduct, name: e.target.value })} required />
              </div>
              <div>
                <label className="label">Unit of Measure</label>
                <input className="input" value={newProduct.unit_of_measure} onChange={(e) => setNewProduct({ ...newProduct, unit_of_measure: e.target.value })} required />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={closeProductModal} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Saving...' : editingProduct ? 'Update' : 'Create'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Warehouse Modal */}
      {showWarehouseModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Warehouse</h2>
            <form onSubmit={handleCreateWarehouse} className="space-y-4">
              <div>
                <label className="label">Code</label>
                <input className="input" value={newWarehouse.code} onChange={(e) => setNewWarehouse({ ...newWarehouse, code: e.target.value })} required />
              </div>
              <div>
                <label className="label">Name</label>
                <input className="input" value={newWarehouse.name} onChange={(e) => setNewWarehouse({ ...newWarehouse, name: e.target.value })} required />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowWarehouseModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Delete Confirmation */}
      <ConfirmDialog
        isOpen={!!deleteConfirm}
        title="Delete Product"
        message={`Are you sure you want to delete "${deleteConfirm?.name}"? This action cannot be undone.`}
        confirmText="Delete"
        onConfirm={handleDeleteProduct}
        onCancel={() => setDeleteConfirm(null)}
        variant="danger"
      />
    </div>
  );
}

import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Package, ShoppingCart, Users, DollarSign } from 'lucide-react';
import { finance, inventory, sales, purchasing, hr } from '../api/client';

interface Stats {
  accounts: number;
  products: number;
  customers: number;
  orders: number;
  vendors: number;
  employees: number;
}

export default function Dashboard() {
  const [stats, setStats] = useState<Stats>({
    accounts: 0,
    products: 0,
    customers: 0,
    orders: 0,
    vendors: 0,
    employees: 0,
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      finance.getAccounts(1, 1),
      inventory.getProducts(1, 1),
      sales.getCustomers(1, 1),
      sales.getOrders(1, 1),
      purchasing.getVendors(1, 1),
      hr.getEmployees(1, 1),
    ])
      .then(([accounts, products, customers, orders, vendors, employees]) => {
        setStats({
          accounts: accounts.data.total,
          products: products.data.total,
          customers: customers.data.total,
          orders: orders.data.total,
          vendors: vendors.data.total,
          employees: employees.data.total,
        });
      })
      .finally(() => setLoading(false));
  }, []);

  const statCards = [
    { label: 'Products', value: stats.products, icon: Package, color: 'bg-blue-500', href: '/inventory' },
    { label: 'Sales Orders', value: stats.orders, icon: ShoppingCart, color: 'bg-green-500', href: '/sales' },
    { label: 'Customers', value: stats.customers, icon: Users, color: 'bg-purple-500', href: '/sales' },
    { label: 'Vendors', value: stats.vendors, icon: Users, color: 'bg-orange-500', href: '/purchasing' },
    { label: 'Accounts', value: stats.accounts, icon: DollarSign, color: 'bg-indigo-500', href: '/finance' },
    { label: 'Employees', value: stats.employees, icon: Users, color: 'bg-pink-500', href: '/hr' },
  ];

  if (loading) {
    return <div className="text-center py-10">Loading...</div>;
  }

  return (
    <div>
      <h1 className="text-2xl font-bold text-gray-900 mb-6">Dashboard</h1>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
        {statCards.map((stat) => (
          <Link
            key={stat.label}
            to={stat.href}
            className="card p-6 hover:shadow-md transition-shadow"
          >
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm text-gray-500">{stat.label}</p>
                <p className="text-3xl font-bold text-gray-900">{stat.value}</p>
              </div>
              <div className={`${stat.color} p-3 rounded-lg`}>
                <stat.icon className="w-6 h-6 text-white" />
              </div>
            </div>
          </Link>
        ))}
      </div>

      {/* Quick Actions */}
      <div className="card p-6">
        <h2 className="text-lg font-semibold mb-4">Quick Actions</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <Link to="/inventory" className="btn btn-secondary text-center">Add Product</Link>
          <Link to="/sales" className="btn btn-secondary text-center">New Order</Link>
          <Link to="/finance" className="btn btn-secondary text-center">Journal Entry</Link>
          <Link to="/hr" className="btn btn-secondary text-center">Add Employee</Link>
        </div>
      </div>

      {/* Getting Started */}
      <div className="mt-6 card p-6">
        <h2 className="text-lg font-semibold mb-4">Getting Started</h2>
        <div className="space-y-3 text-gray-600">
          <p>1. Create Chart of Accounts in <Link to="/finance" className="text-blue-600 hover:underline">Finance</Link></p>
          <p>2. Add Products in <Link to="/inventory" className="text-blue-600 hover:underline">Inventory</Link></p>
          <p>3. Create Customers in <Link to="/sales" className="text-blue-600 hover:underline">Sales</Link></p>
          <p>4. Start processing orders!</p>
        </div>
      </div>
    </div>
  );
}

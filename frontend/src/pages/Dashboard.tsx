import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Package, ShoppingCart, Users, DollarSign, TrendingUp, TrendingDown, Activity, FileText, Factory, UserCheck } from 'lucide-react';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, PieChart, Pie, Cell, Legend } from 'recharts';
import { finance, inventory, sales, purchasing, hr } from '../api/client';
import { LoadingPage } from '../components/Spinner';

interface Stats {
  accounts: number;
  products: number;
  customers: number;
  orders: number;
  vendors: number;
  employees: number;
  journalEntries: number;
  purchaseOrders: number;
  warehouses: number;
}

interface OrderData {
  id: string;
  order_number: string;
  total: number;
  status: string;
  date?: string;
}

interface JournalEntry {
  id: string;
  entry_number: string;
  total_debit: number;
  status: string;
}

export default function Dashboard() {
  const [stats, setStats] = useState<Stats>({
    accounts: 0,
    products: 0,
    customers: 0,
    orders: 0,
    vendors: 0,
    employees: 0,
    journalEntries: 0,
    purchaseOrders: 0,
    warehouses: 0,
  });
  const [recentOrders, setRecentOrders] = useState<OrderData[]>([]);
  const [recentEntries, setRecentEntries] = useState<JournalEntry[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([
      finance.getAccounts(1, 1),
      inventory.getProducts(1, 1),
      sales.getCustomers(1, 1),
      sales.getOrders(1, 5),
      purchasing.getVendors(1, 1),
      purchasing.getOrders(1, 1),
      hr.getEmployees(1, 1),
      finance.getJournalEntries(1, 5),
      inventory.getWarehouses(),
    ])
      .then(([accounts, products, customers, orders, vendors, pos, employees, entries, warehouses]) => {
        setStats({
          accounts: accounts.data.total,
          products: products.data.total,
          customers: customers.data.total,
          orders: orders.data.total,
          vendors: vendors.data.total,
          employees: employees.data.total,
          journalEntries: entries.data.total,
          purchaseOrders: pos.data.total,
          warehouses: warehouses.data.length,
        });
        setRecentOrders(orders.data.items || []);
        setRecentEntries(entries.data.items || []);
      })
      .finally(() => setLoading(false));
  }, []);

  const statusData = [
    { name: 'Draft', value: recentOrders.filter(o => o.status === 'Draft').length, color: '#F59E0B' },
    { name: 'Confirmed', value: recentOrders.filter(o => o.status !== 'Draft').length, color: '#10B981' },
  ].filter(d => d.value > 0);

  const moduleData = [
    { name: 'Finance', value: stats.accounts + stats.journalEntries },
    { name: 'Sales', value: stats.customers + stats.orders },
    { name: 'Purchasing', value: stats.vendors + stats.purchaseOrders },
    { name: 'Inventory', value: stats.products + stats.warehouses },
    { name: 'HR', value: stats.employees },
  ];

  const statCards = [
    { label: 'Products', value: stats.products, icon: Package, color: 'bg-blue-500', href: '/inventory', trend: '+12%', trendUp: true },
    { label: 'Sales Orders', value: stats.orders, icon: ShoppingCart, color: 'bg-green-500', href: '/sales', trend: '+8%', trendUp: true },
    { label: 'Customers', value: stats.customers, icon: Users, color: 'bg-purple-500', href: '/sales', trend: '+5%', trendUp: true },
    { label: 'Vendors', value: stats.vendors, icon: Users, color: 'bg-orange-500', href: '/purchasing', trend: '+3%', trendUp: true },
    { label: 'Accounts', value: stats.accounts, icon: DollarSign, color: 'bg-indigo-500', href: '/finance' },
    { label: 'Employees', value: stats.employees, icon: UserCheck, color: 'bg-pink-500', href: '/hr' },
  ];

  if (loading) return <LoadingPage />;

  const totalRevenue = recentOrders.reduce((sum, o) => sum + (o.total || 0), 0);
  const totalJournalAmount = recentEntries.reduce((sum, e) => sum + (e.total_debit || 0), 0);

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
        <div className="text-sm text-gray-500">
          Last updated: {new Date().toLocaleString()}
        </div>
      </div>

      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-6 gap-4 mb-8">
        {statCards.map((stat) => (
          <Link
            key={stat.label}
            to={stat.href}
            className="card p-4 hover:shadow-md transition-shadow"
          >
            <div className="flex items-center justify-between mb-2">
              <div className={`${stat.color} p-2 rounded-lg`}>
                <stat.icon className="w-4 h-4 text-white" />
              </div>
              {stat.trend && (
                <span className={`text-xs flex items-center ${stat.trendUp ? 'text-green-600' : 'text-red-600'}`}>
                  {stat.trendUp ? <TrendingUp className="w-3 h-3 mr-1" /> : <TrendingDown className="w-3 h-3 mr-1" />}
                  {stat.trend}
                </span>
              )}
            </div>
            <p className="text-2xl font-bold text-gray-900">{stat.value}</p>
            <p className="text-xs text-gray-500">{stat.label}</p>
          </Link>
        ))}
      </div>

      {/* Metrics Row */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="card p-6">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm text-gray-500">Total Revenue (Recent)</span>
            <DollarSign className="w-4 h-4 text-green-500" />
          </div>
          <p className="text-3xl font-bold text-gray-900">${totalRevenue.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}</p>
        </div>
        <div className="card p-6">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm text-gray-500">Journal Activity</span>
            <FileText className="w-4 h-4 text-blue-500" />
          </div>
          <p className="text-3xl font-bold text-gray-900">${totalJournalAmount.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}</p>
        </div>
        <div className="card p-6">
          <div className="flex items-center justify-between mb-2">
            <span className="text-sm text-gray-500">Active Modules</span>
            <Activity className="w-4 h-4 text-purple-500" />
          </div>
          <p className="text-3xl font-bold text-gray-900">7</p>
        </div>
      </div>

      {/* Charts Row */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        {/* Module Overview */}
        <div className="card p-6">
          <h2 className="text-lg font-semibold mb-4">Data Distribution by Module</h2>
          {moduleData.some(m => m.value > 0) ? (
            <ResponsiveContainer width="100%" height={250}>
              <BarChart data={moduleData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#E5E7EB" />
                <XAxis dataKey="name" tick={{ fontSize: 12 }} />
                <YAxis tick={{ fontSize: 12 }} />
                <Tooltip />
                <Bar dataKey="value" fill="#3B82F6" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="h-[250px] flex items-center justify-center text-gray-400">
              No data available. Start adding records to see charts.
            </div>
          )}
        </div>

        {/* Order Status */}
        <div className="card p-6">
          <h2 className="text-lg font-semibold mb-4">Order Status Distribution</h2>
          {statusData.length > 0 ? (
            <ResponsiveContainer width="100%" height={250}>
              <PieChart>
                <Pie
                  data={statusData}
                  cx="50%"
                  cy="50%"
                  innerRadius={60}
                  outerRadius={100}
                  paddingAngle={5}
                  dataKey="value"
                >
                  {statusData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip />
                <Legend />
              </PieChart>
            </ResponsiveContainer>
          ) : (
            <div className="h-[250px] flex items-center justify-center text-gray-400">
              No orders yet. Create your first order to see this chart.
            </div>
          )}
        </div>
      </div>

      {/* Recent Activity & Quick Actions */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        {/* Recent Orders */}
        <div className="card lg:col-span-2">
          <div className="p-4 border-b flex justify-between items-center">
            <h2 className="text-lg font-semibold">Recent Sales Orders</h2>
            <Link to="/sales" className="text-sm text-blue-600 hover:underline">View all</Link>
          </div>
          {recentOrders.length > 0 ? (
            <table className="w-full">
              <thead>
                <tr className="border-b bg-gray-50">
                  <th className="table-header">Order #</th>
                  <th className="table-header">Total</th>
                  <th className="table-header">Status</th>
                </tr>
              </thead>
              <tbody>
                {recentOrders.slice(0, 5).map((order) => (
                  <tr key={order.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-mono">{order.order_number}</td>
                    <td className="table-cell">${order.total.toFixed(2)}</td>
                    <td className="table-cell">
                      <span className={`badge ${order.status === 'Draft' ? 'badge-warning' : 'badge-success'}`}>
                        {order.status}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          ) : (
            <div className="p-8 text-center text-gray-400">
              No orders yet. <Link to="/sales" className="text-blue-600 hover:underline">Create your first order</Link>
            </div>
          )}
        </div>

        {/* Quick Actions */}
        <div className="card p-6">
          <h2 className="text-lg font-semibold mb-4">Quick Actions</h2>
          <div className="space-y-3">
            <Link to="/inventory" className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors">
              <Package className="w-5 h-5 text-blue-500" />
              <span>Add Product</span>
            </Link>
            <Link to="/sales" className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors">
              <ShoppingCart className="w-5 h-5 text-green-500" />
              <span>New Sales Order</span>
            </Link>
            <Link to="/finance" className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors">
              <FileText className="w-5 h-5 text-purple-500" />
              <span>Journal Entry</span>
            </Link>
            <Link to="/purchasing" className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors">
              <Users className="w-5 h-5 text-orange-500" />
              <span>Add Vendor</span>
            </Link>
            <Link to="/hr" className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors">
              <UserCheck className="w-5 h-5 text-pink-500" />
              <span>Add Employee</span>
            </Link>
            <Link to="/manufacturing" className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors">
              <Factory className="w-5 h-5 text-indigo-500" />
              <span>Create BOM</span>
            </Link>
          </div>
        </div>
      </div>

      {/* Getting Started */}
      <div className="card p-6">
        <h2 className="text-lg font-semibold mb-4">Getting Started with ERP</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div className="flex gap-3">
            <div className="bg-blue-100 text-blue-600 rounded-full w-8 h-8 flex items-center justify-center font-bold shrink-0">1</div>
            <div>
              <h3 className="font-medium">Set up Finance</h3>
              <p className="text-sm text-gray-500">Create your Chart of Accounts</p>
              <Link to="/finance" className="text-sm text-blue-600 hover:underline">Go to Finance →</Link>
            </div>
          </div>
          <div className="flex gap-3">
            <div className="bg-green-100 text-green-600 rounded-full w-8 h-8 flex items-center justify-center font-bold shrink-0">2</div>
            <div>
              <h3 className="font-medium">Add Products</h3>
              <p className="text-sm text-gray-500">Set up your inventory</p>
              <Link to="/inventory" className="text-sm text-blue-600 hover:underline">Go to Inventory →</Link>
            </div>
          </div>
          <div className="flex gap-3">
            <div className="bg-purple-100 text-purple-600 rounded-full w-8 h-8 flex items-center justify-center font-bold shrink-0">3</div>
            <div>
              <h3 className="font-medium">Add Customers</h3>
              <p className="text-sm text-gray-500">Start managing sales</p>
              <Link to="/sales" className="text-sm text-blue-600 hover:underline">Go to Sales →</Link>
            </div>
          </div>
          <div className="flex gap-3">
            <div className="bg-orange-100 text-orange-600 rounded-full w-8 h-8 flex items-center justify-center font-bold shrink-0">4</div>
            <div>
              <h3 className="font-medium">Process Orders</h3>
              <p className="text-sm text-gray-500">Start selling!</p>
              <Link to="/sales" className="text-sm text-blue-600 hover:underline">Create Order →</Link>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

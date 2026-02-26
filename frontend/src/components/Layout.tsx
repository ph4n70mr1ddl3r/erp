import { Link, useLocation } from 'react-router-dom';
import { 
  LayoutDashboard, 
  Building2, 
  Package, 
  ShoppingCart, 
  Users, 
  Factory, 
  UserCog,
  LogOut,
  Menu,
  X,
  FileBarChart,
  ClipboardList,
  Headphones,
  Monitor,
  Folder,
  Shield,
  CreditCard,
  Globe,
  FileText,
  DollarSign,
  Gavel,
  Settings,
  Zap,
  RefreshCw,
  CheckSquare
} from 'lucide-react';
import { useState } from 'react';
import { useAuth } from '../hooks/useAuth';
import NotificationCenter from './NotificationCenter';
import GlobalSearch from './GlobalSearch';

const navItems = [
  { path: '/', icon: LayoutDashboard, label: 'Dashboard' },
  { path: '/finance', icon: Building2, label: 'Finance' },
  { path: '/reports', icon: FileBarChart, label: 'Reports' },
  { path: '/audit', icon: ClipboardList, label: 'Audit Trail' },
  { path: '/inventory', icon: Package, label: 'Inventory' },
  { path: '/sales', icon: ShoppingCart, label: 'Sales' },
  { path: '/purchasing', icon: Users, label: 'Purchasing' },
  { path: '/manufacturing', icon: Factory, label: 'Manufacturing' },
  { path: '/hr', icon: UserCog, label: 'HR' },
  { path: '/projects', icon: Folder, label: 'Projects' },
  { path: '/service', icon: Headphones, label: 'Service Desk' },
  { path: '/assets', icon: Monitor, label: 'IT Assets' },
  { path: '/compliance', icon: Shield, label: 'Compliance' },
  { path: '/pos', icon: CreditCard, label: 'POS' },
  { path: '/ecommerce', icon: Globe, label: 'E-Commerce' },
  { path: '/documents', icon: FileText, label: 'Documents' },
  { path: '/pricing', icon: DollarSign, label: 'Pricing' },
  { path: '/sourcing', icon: Gavel, label: 'Sourcing' },
  { path: '/config', icon: Settings, label: 'Settings' },
  { path: '/rules', icon: Zap, label: 'Rules' },
  { path: '/approval-workflow', icon: CheckSquare, label: 'Approvals' },
  { path: '/currency-revaluation', icon: RefreshCw, label: 'FX Revaluation' },
];

export default function Layout({ children }: { children: React.ReactNode }) {
  const location = useLocation();
  const { user, logout } = useAuth();
  const [sidebarOpen, setSidebarOpen] = useState(false);

  return (
    <div className="min-h-screen flex">
      {/* Mobile sidebar backdrop */}
      {sidebarOpen && (
        <div 
          className="fixed inset-0 bg-black bg-opacity-50 z-20 lg:hidden"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar */}
      <aside className={`fixed lg:static inset-y-0 left-0 z-30 w-64 bg-white border-r border-gray-200 transform transition-transform duration-200 ${sidebarOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}`}>
        <div className="flex items-center justify-between h-16 px-4 border-b border-gray-200">
          <Link to="/" className="text-xl font-bold text-blue-600">ERP System</Link>
          <button onClick={() => setSidebarOpen(false)} className="lg:hidden">
            <X className="w-6 h-6" />
          </button>
        </div>
        
        <nav className="p-4 space-y-1">
          {navItems.map((item) => (
            <Link
              key={item.path}
              to={item.path}
              onClick={() => setSidebarOpen(false)}
              className={`sidebar-link ${location.pathname === item.path ? 'active' : ''}`}
            >
              <item.icon className="w-5 h-5" />
              {item.label}
            </Link>
          ))}
        </nav>

        <div className="absolute bottom-0 left-0 right-0 p-4 border-t border-gray-200">
          <div className="flex items-center gap-3 mb-3">
            <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center">
              <span className="text-blue-600 font-medium text-sm">
                {user?.full_name?.charAt(0) || user?.username?.charAt(0) || '?'}
              </span>
            </div>
            <div className="flex-1 min-w-0">
              <p className="text-sm font-medium truncate">{user?.full_name || user?.username}</p>
              <p className="text-xs text-gray-500">{user?.role}</p>
            </div>
          </div>
          <button
            onClick={logout}
            className="flex items-center gap-2 w-full px-3 py-2 text-sm text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg"
          >
            <LogOut className="w-4 h-4" />
            Sign out
          </button>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 min-w-0">
        {/* Header */}
        <header className="hidden lg:flex items-center justify-between h-16 px-6 border-b border-gray-200 bg-white">
          <GlobalSearch />
          <NotificationCenter />
        </header>

        {/* Mobile header */}
        <header className="lg:hidden flex items-center gap-4 h-16 px-4 border-b border-gray-200 bg-white">
          <button onClick={() => setSidebarOpen(true)}>
            <Menu className="w-6 h-6" />
          </button>
          <span className="font-bold text-blue-600">ERP System</span>
          <div className="ml-auto flex items-center gap-2">
            <NotificationCenter />
          </div>
        </header>

        <div className="p-6">
          {children}
        </div>
      </main>
    </div>
  );
}

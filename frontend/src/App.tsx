import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './hooks/useAuth';
import { ToastProvider } from './components/Toast';
import { Spinner } from './components/Spinner';
import ErrorBoundary from './components/ErrorBoundary';
import Layout from './components/Layout';
import Login from './pages/Login';
import Dashboard from './pages/Dashboard';
import Finance from './pages/Finance';
import Inventory from './pages/Inventory';
import Sales from './pages/Sales';
import Purchasing from './pages/Purchasing';
import Manufacturing from './pages/Manufacturing';
import HR from './pages/HR';
import Reports from './pages/Reports';
import AuditLogs from './pages/AuditLogs';
import ServiceDesk from './pages/ServiceDesk';
import ITAssets from './pages/ITAssets';
import Compliance from './pages/Compliance';
import Projects from './pages/Projects';
import POS from './pages/POS';
import Ecommerce from './pages/Ecommerce';
import Documents from './pages/Documents';
import Pricing from './pages/Pricing';
import Sourcing from './pages/Sourcing';
import ConfigPage from './pages/Config';
import RulesPage from './pages/Rules';
import CurrencyRevaluation from './pages/CurrencyRevaluation';
import ApprovalWorkflow from './pages/ApprovalWorkflow';
import CreditManagement from './pages/CreditManagement';
import StripePayments from './pages/StripePayments';
import CRM from './pages/CRM';

function PrivateRoute({ children }: { children: React.ReactNode }) {
  const { token, isLoading } = useAuth();
  
  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Spinner size="lg" />
      </div>
    );
  }
  
  return token ? <Layout>{children}</Layout> : <Navigate to="/login" />;
}

function AppRoutes() {
  const { token } = useAuth();
  
  return (
    <Routes>
      <Route path="/login" element={token ? <Navigate to="/" /> : <Login />} />
      <Route path="/" element={<PrivateRoute><Dashboard /></PrivateRoute>} />
      <Route path="/finance" element={<PrivateRoute><Finance /></PrivateRoute>} />
      <Route path="/reports" element={<PrivateRoute><Reports /></PrivateRoute>} />
      <Route path="/audit" element={<PrivateRoute><AuditLogs /></PrivateRoute>} />
      <Route path="/inventory" element={<PrivateRoute><Inventory /></PrivateRoute>} />
      <Route path="/sales" element={<PrivateRoute><Sales /></PrivateRoute>} />
      <Route path="/purchasing" element={<PrivateRoute><Purchasing /></PrivateRoute>} />
      <Route path="/manufacturing" element={<PrivateRoute><Manufacturing /></PrivateRoute>} />
      <Route path="/hr" element={<PrivateRoute><HR /></PrivateRoute>} />
      <Route path="/service" element={<PrivateRoute><ServiceDesk /></PrivateRoute>} />
      <Route path="/assets" element={<PrivateRoute><ITAssets /></PrivateRoute>} />
      <Route path="/compliance" element={<PrivateRoute><Compliance /></PrivateRoute>} />
      <Route path="/projects" element={<PrivateRoute><Projects /></PrivateRoute>} />
      <Route path="/pos" element={<PrivateRoute><POS /></PrivateRoute>} />
      <Route path="/ecommerce" element={<PrivateRoute><Ecommerce /></PrivateRoute>} />
      <Route path="/documents" element={<PrivateRoute><Documents /></PrivateRoute>} />
      <Route path="/pricing" element={<PrivateRoute><Pricing /></PrivateRoute>} />
      <Route path="/sourcing" element={<PrivateRoute><Sourcing /></PrivateRoute>} />
      <Route path="/config" element={<PrivateRoute><ConfigPage /></PrivateRoute>} />
      <Route path="/rules" element={<PrivateRoute><RulesPage /></PrivateRoute>} />
      <Route path="/currency-revaluation" element={<PrivateRoute><CurrencyRevaluation /></PrivateRoute>} />
      <Route path="/approval-workflow" element={<PrivateRoute><ApprovalWorkflow /></PrivateRoute>} />
      <Route path="/credit" element={<PrivateRoute><CreditManagement /></PrivateRoute>} />
      <Route path="/payments" element={<PrivateRoute><StripePayments /></PrivateRoute>} />
      <Route path="/crm" element={<PrivateRoute><CRM /></PrivateRoute>} />
    </Routes>
  );
}

function App() {
  return (
    <BrowserRouter>
      <ErrorBoundary>
        <AuthProvider>
          <ToastProvider>
            <AppRoutes />
          </ToastProvider>
        </AuthProvider>
      </ErrorBoundary>
    </BrowserRouter>
  );
}

export default App;

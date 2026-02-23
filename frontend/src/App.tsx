import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './hooks/useAuth';
import { ToastProvider } from './components/Toast';
import { Spinner } from './components/Spinner';
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
    </Routes>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <ToastProvider>
          <AppRoutes />
        </ToastProvider>
      </AuthProvider>
    </BrowserRouter>
  );
}

export default App;

import React, { useState, useEffect } from 'react';
import { config } from '../api/client';

interface ConfigItem {
  id: string;
  category: string;
  key: string;
  value: string;
}

interface CompanySettings {
  company_name: string;
  currency: string;
  timezone: string;
}

interface AuditSettings {
  log_retention_days: number;
  max_login_attempts: number;
  require_mfa: boolean;
}

const ConfigPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState('general');
  const [configs, setConfigs] = useState<ConfigItem[]>([]);
  const [companySettings, setCompanySettings] = useState<CompanySettings | null>(null);
  const [auditSettings, setAuditSettings] = useState<AuditSettings | null>(null);
  
  const [category, setCategory] = useState('');
  const [key, setKey] = useState('');
  const [value, setValue] = useState('');

  const loadData = async () => {
    try {
      const [configsRes, companyRes, auditRes] = await Promise.all([
        config.listConfigs(),
        config.getCompanySettings(),
        config.getAuditSettings(),
      ]);
      setConfigs(configsRes.data);
      setCompanySettings(companyRes.data);
      setAuditSettings(auditRes.data);
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  };

  useEffect(() => {
    void loadData();
  }, []);

  const handleSetConfig = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await config.setConfig({ category, key, value });
      setCategory('');
      setKey('');
      setValue('');
      loadData();
    } catch (error) {
      console.error('Failed to set config:', error);
    }
  };

  const handleUpdateCompany = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (companySettings) {
        await config.updateCompanySettings(companySettings);
        alert('Company settings updated');
      }
    } catch (error) {
      console.error('Failed to update company settings:', error);
    }
  };

  const handleUpdateAudit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      if (auditSettings) {
        await config.updateAuditSettings(auditSettings);
        alert('Audit settings updated');
      }
    } catch (error) {
      console.error('Failed to update audit settings:', error);
    }
  };

  const tabs = [
    { id: 'general', label: 'General' },
    { id: 'configs', label: 'System Config' },
    { id: 'audit', label: 'Security' },
  ];

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6">System Configuration</h1>

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

      {activeTab === 'general' && companySettings && (
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold mb-4">Company Settings</h2>
          <form onSubmit={handleUpdateCompany}>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">Company Name</label>
                <input
                  type="text"
                  value={companySettings.company_name}
                  onChange={(e) => setCompanySettings({ ...companySettings, company_name: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Currency</label>
                <select
                  value={companySettings.currency}
                  onChange={(e) => setCompanySettings({ ...companySettings, currency: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                >
                  <option value="USD">USD</option>
                  <option value="EUR">EUR</option>
                  <option value="GBP">GBP</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Timezone</label>
                <select
                  value={companySettings.timezone}
                  onChange={(e) => setCompanySettings({ ...companySettings, timezone: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                >
                  <option value="UTC">UTC</option>
                  <option value="America/New_York">America/New_York</option>
                  <option value="Europe/London">Europe/London</option>
                  <option value="Asia/Tokyo">Asia/Tokyo</option>
                </select>
              </div>
            </div>
            <button
              type="submit"
              className="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Save Changes
            </button>
          </form>
        </div>
      )}

      {activeTab === 'configs' && (
        <div className="space-y-6">
          <div className="bg-white rounded-lg shadow p-6">
            <h2 className="text-lg font-semibold mb-4">Add Configuration</h2>
            <form onSubmit={handleSetConfig} className="flex space-x-4">
              <input
                type="text"
                value={category}
                onChange={(e) => setCategory(e.target.value)}
                placeholder="Category"
                className="flex-1 border rounded px-3 py-2"
                required
              />
              <input
                type="text"
                value={key}
                onChange={(e) => setKey(e.target.value)}
                placeholder="Key"
                className="flex-1 border rounded px-3 py-2"
                required
              />
              <input
                type="text"
                value={value}
                onChange={(e) => setValue(e.target.value)}
                placeholder="Value"
                className="flex-1 border rounded px-3 py-2"
                required
              />
              <button
                type="submit"
                className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
              >
                Add
              </button>
            </form>
          </div>

          <div className="bg-white rounded-lg shadow">
            <table className="w-full">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-4 py-3 text-left">Category</th>
                  <th className="px-4 py-3 text-left">Key</th>
                  <th className="px-4 py-3 text-left">Value</th>
                </tr>
              </thead>
              <tbody>
                {configs.map((c) => (
                  <tr key={c.id} className="border-t hover:bg-gray-50">
                    <td className="px-4 py-3">{c.category}</td>
                    <td className="px-4 py-3">{c.key}</td>
                    <td className="px-4 py-3 font-mono text-sm">{c.value}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {activeTab === 'audit' && auditSettings && (
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold mb-4">Security & Audit Settings</h2>
          <form onSubmit={handleUpdateAudit}>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">Log Retention (days)</label>
                <input
                  type="number"
                  value={auditSettings.log_retention_days}
                  onChange={(e) => setAuditSettings({ ...auditSettings, log_retention_days: parseInt(e.target.value) })}
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">Max Login Attempts</label>
                <input
                  type="number"
                  value={auditSettings.max_login_attempts}
                  onChange={(e) => setAuditSettings({ ...auditSettings, max_login_attempts: parseInt(e.target.value) })}
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div className="flex items-center">
                <input
                  type="checkbox"
                  checked={auditSettings.require_mfa}
                  onChange={(e) => setAuditSettings({ ...auditSettings, require_mfa: e.target.checked })}
                  className="mr-2"
                />
                <label className="text-sm font-medium">Require MFA</label>
              </div>
            </div>
            <button
              type="submit"
              className="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Save Changes
            </button>
          </form>
        </div>
      )}
    </div>
  );
};

export default ConfigPage;

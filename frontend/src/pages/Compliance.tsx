import { useState, useEffect } from 'react';
import { Shield, Users, FileCheck, AlertTriangle, Clock, Building2, Plus, X } from 'lucide-react';
import { compliance, type ComplianceStats, type DataSubject, type ConsentRecord, type DSARRequest, type DataBreach } from '../api/client';
import { useToast } from '../components/Toast';
import { getErrorMessage } from '../utils/errors';

type Tab = 'overview' | 'subjects' | 'consents' | 'dsars' | 'breaches';

export default function Compliance() {
  const toast = useToast();
  const [activeTab, setActiveTab] = useState<Tab>('overview');
  const [stats, setStats] = useState<ComplianceStats | null>(null);
  const [subjects, setSubjects] = useState<DataSubject[]>([]);
  const [consents, setConsents] = useState<ConsentRecord[]>([]);
  const [dsars, setDsars] = useState<DSARRequest[]>([]);
  const [breaches, setBreaches] = useState<DataBreach[]>([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState<string | null>(null);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [statsRes, subjectsRes, consentsRes, dsarsRes, breachesRes] = await Promise.all([
        compliance.getStats(),
        compliance.getDataSubjects(),
        compliance.getConsents(),
        compliance.getDSARs(),
        compliance.getBreaches(),
      ]);
      setStats(statsRes.data);
      setSubjects(subjectsRes.data);
      setConsents(consentsRes.data);
      setDsars(dsarsRes.data);
      setBreaches(breachesRes.data);
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to load compliance data'));
    } finally {
      setLoading(false);
    }
  };

  const handleCreateSubject = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const formData = new FormData(form);
    try {
      await compliance.createDataSubject({
        email: formData.get('email') as string,
        first_name: formData.get('first_name') as string || undefined,
        last_name: formData.get('last_name') as string || undefined,
      });
      setShowModal(null);
      loadData();
      form.reset();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to create subject'));
    }
  };

  const handleCreateDSAR = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const formData = new FormData(form);
    try {
      await compliance.createDSAR({
        data_subject_id: formData.get('data_subject_id') as string,
        request_type: formData.get('request_type') as string,
        description: formData.get('description') as string || undefined,
      });
      setShowModal(null);
      loadData();
      form.reset();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to create DSAR'));
    }
  };

  const handleWithdrawConsent = async (id: string) => {
    try {
      await compliance.withdrawConsent(id);
      loadData();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to withdraw consent'));
    }
  };

  const handleCompleteDSAR = async (id: string) => {
    const response = prompt('Enter response:');
    if (response) {
      try {
        await compliance.completeDSAR(id, response);
        loadData();
      } catch (error: unknown) {
        toast.error(getErrorMessage(error, 'Failed to complete DSAR'));
      }
    }
  };

  const handleCreateBreach = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const formData = new FormData(form);
    try {
      await compliance.createBreach({
        title: formData.get('title') as string,
        description: formData.get('description') as string,
        breach_type: formData.get('breach_type') as string,
        severity: formData.get('severity') as string,
      });
      setShowModal(null);
      loadData();
      form.reset();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to create breach'));
    }
  };

  if (loading) {
    return <div className="text-center py-8">Loading...</div>;
  }

  const tabs: { id: Tab; label: string; icon: React.ReactNode }[] = [
    { id: 'overview', label: 'Overview', icon: <Shield className="w-4 h-4" /> },
    { id: 'subjects', label: 'Data Subjects', icon: <Users className="w-4 h-4" /> },
    { id: 'consents', label: 'Consents', icon: <FileCheck className="w-4 h-4" /> },
    { id: 'dsars', label: 'DSARs', icon: <Clock className="w-4 h-4" /> },
    { id: 'breaches', label: 'Breaches', icon: <AlertTriangle className="w-4 h-4" /> },
  ];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Compliance & GDPR</h1>
          <p className="text-gray-500">Data privacy and compliance management</p>
        </div>
      </div>

      <div className="border-b border-gray-200 mb-6">
        <nav className="flex gap-4">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={`flex items-center gap-2 px-4 py-2 border-b-2 transition-colors ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700'
              }`}
            >
              {tab.icon}
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      {activeTab === 'overview' && stats && (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center gap-3">
              <Users className="w-8 h-8 text-blue-500" />
              <div>
                <p className="text-2xl font-bold">{stats.data_subjects}</p>
                <p className="text-sm text-gray-500">Data Subjects</p>
              </div>
            </div>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center gap-3">
              <FileCheck className="w-8 h-8 text-green-500" />
              <div>
                <p className="text-2xl font-bold">{stats.active_consents}</p>
                <p className="text-sm text-gray-500">Active Consents</p>
              </div>
            </div>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center gap-3">
              <Clock className="w-8 h-8 text-yellow-500" />
              <div>
                <p className="text-2xl font-bold">{stats.pending_dsars}</p>
                <p className="text-sm text-gray-500">Pending DSARs</p>
              </div>
            </div>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center gap-3">
              <AlertTriangle className="w-8 h-8 text-red-500" />
              <div>
                <p className="text-2xl font-bold">{stats.active_breaches}</p>
                <p className="text-sm text-gray-500">Active Breaches</p>
              </div>
            </div>
          </div>
          <div className="bg-white p-6 rounded-lg shadow">
            <div className="flex items-center gap-3">
              <Building2 className="w-8 h-8 text-purple-500" />
              <div>
                <p className="text-2xl font-bold">{stats.active_processors}</p>
                <p className="text-sm text-gray-500">Processors</p>
              </div>
            </div>
          </div>
        </div>
      )}

      {activeTab === 'subjects' && (
        <div>
          <div className="flex justify-end mb-4">
            <button
              onClick={() => setShowModal('subject')}
              className="btn-primary flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              Add Data Subject
            </button>
          </div>
          <div className="bg-white rounded-lg shadow overflow-hidden">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Email</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Name</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Created</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {subjects.map((subject) => (
                  <tr key={subject.id}>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">{subject.email}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {subject.first_name} {subject.last_name}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${
                        subject.verification_status === 'Verified' ? 'bg-green-100 text-green-800' :
                        subject.verification_status === 'Pending' ? 'bg-yellow-100 text-yellow-800' :
                        'bg-gray-100 text-gray-800'
                      }`}>
                        {subject.verification_status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                      {new Date(subject.created_at).toLocaleDateString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {activeTab === 'consents' && (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Type</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Purpose</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Legal Basis</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Actions</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {consents.map((consent) => (
                <tr key={consent.id}>
                  <td className="px-6 py-4 whitespace-nowrap text-sm">{consent.consent_type}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm">{consent.purpose}</td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm">{consent.legal_basis}</td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${
                      consent.status === 'Granted' ? 'bg-green-100 text-green-800' :
                      consent.status === 'Withdrawn' ? 'bg-red-100 text-red-800' :
                      'bg-gray-100 text-gray-800'
                    }`}>
                      {consent.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm">
                    {consent.status === 'Granted' && (
                      <button
                        onClick={() => handleWithdrawConsent(consent.id)}
                        className="text-red-600 hover:text-red-800"
                      >
                        Withdraw
                      </button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'dsars' && (
        <div>
          <div className="flex justify-end mb-4">
            <button
              onClick={() => setShowModal('dsar')}
              className="btn-primary flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              New DSAR
            </button>
          </div>
          <div className="bg-white rounded-lg shadow overflow-hidden">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Number</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Type</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Due Date</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Actions</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {dsars.map((dsar) => (
                  <tr key={dsar.id}>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">{dsar.request_number}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">{dsar.request_type}</td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${
                        dsar.status === 'Completed' ? 'bg-green-100 text-green-800' :
                        dsar.status === 'InProgress' ? 'bg-blue-100 text-blue-800' :
                        'bg-yellow-100 text-yellow-800'
                      }`}>
                        {dsar.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">{new Date(dsar.due_date).toLocaleDateString()}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {dsar.status !== 'Completed' && dsar.status !== 'Rejected' && (
                        <button
                          onClick={() => handleCompleteDSAR(dsar.id)}
                          className="text-blue-600 hover:text-blue-800"
                        >
                          Complete
                        </button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {activeTab === 'breaches' && (
        <div>
          <div className="flex justify-end mb-4">
            <button
              onClick={() => setShowModal('breach')}
              className="btn-primary flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              Report Breach
            </button>
          </div>
          <div className="bg-white rounded-lg shadow overflow-hidden">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Number</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Title</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Severity</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Discovered</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {breaches.map((breach) => (
                  <tr key={breach.id}>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">{breach.breach_number}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">{breach.title}</td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${
                        breach.severity === 'Critical' ? 'bg-red-100 text-red-800' :
                        breach.severity === 'High' ? 'bg-orange-100 text-orange-800' :
                        breach.severity === 'Medium' ? 'bg-yellow-100 text-yellow-800' :
                        'bg-green-100 text-green-800'
                      }`}>
                        {breach.severity}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${
                        breach.status === 'Closed' ? 'bg-green-100 text-green-800' :
                        breach.status === 'Resolved' ? 'bg-blue-100 text-blue-800' :
                        'bg-yellow-100 text-yellow-800'
                      }`}>
                        {breach.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {new Date(breach.discovered_at).toLocaleDateString()}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-lg font-bold">
                {showModal === 'subject' && 'Add Data Subject'}
                {showModal === 'dsar' && 'New DSAR Request'}
                {showModal === 'breach' && 'Report Data Breach'}
              </h2>
              <button onClick={() => setShowModal(null)}>
                <X className="w-5 h-5" />
              </button>
            </div>
            
            {showModal === 'subject' && (
              <form onSubmit={handleCreateSubject} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Email *</label>
                  <input name="email" type="email" required className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">First Name</label>
                  <input name="first_name" className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Last Name</label>
                  <input name="last_name" className="input-field" />
                </div>
                <button type="submit" className="btn-primary w-full">Create</button>
              </form>
            )}

            {showModal === 'dsar' && (
              <form onSubmit={handleCreateDSAR} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Data Subject *</label>
                  <select name="data_subject_id" required className="input-field">
                    <option value="">Select...</option>
                    {subjects.map((s) => (
                      <option key={s.id} value={s.id}>{s.email}</option>
                    ))}
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Request Type *</label>
                  <select name="request_type" required className="input-field">
                    <option value="Access">Access</option>
                    <option value="Rectification">Rectification</option>
                    <option value="Erasure">Erasure</option>
                    <option value="Portability">Portability</option>
                    <option value="Restriction">Restriction</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Description</label>
                  <textarea name="description" className="input-field" rows={3} />
                </div>
                <button type="submit" className="btn-primary w-full">Create Request</button>
              </form>
            )}

            {showModal === 'breach' && (
              <form onSubmit={handleCreateBreach} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Title *</label>
                  <input name="title" required className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Description *</label>
                  <textarea name="description" required className="input-field" rows={3} />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Type *</label>
                  <select name="breach_type" required className="input-field">
                    <option value="Confidentiality">Confidentiality</option>
                    <option value="Integrity">Integrity</option>
                    <option value="Availability">Availability</option>
                    <option value="UnauthorizedAccess">Unauthorized Access</option>
                    <option value="Loss">Data Loss</option>
                  </select>
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Severity *</label>
                  <select name="severity" required className="input-field">
                    <option value="Low">Low</option>
                    <option value="Medium">Medium</option>
                    <option value="High">High</option>
                    <option value="Critical">Critical</option>
                  </select>
                </div>
                <button type="submit" className="btn-primary w-full">Report Breach</button>
              </form>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

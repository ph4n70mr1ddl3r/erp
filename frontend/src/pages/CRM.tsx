import { useEffect, useState } from 'react';
import { crm, type CreateLeadRequest, type CreateOpportunityRequest } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { getErrorMessage } from '../utils/errors';
import type { Lead, Opportunity } from '../types';

const STAGES = ['Prospecting', 'Qualification', 'Proposal', 'Negotiation', 'ClosedWon', 'ClosedLost'];
const STAGE_LABELS: Record<string, string> = {
  Prospecting: 'Prospecting',
  Qualification: 'Qualification',
  Proposal: 'Proposal',
  Negotiation: 'Negotiation',
  ClosedWon: 'Closed Won',
  ClosedLost: 'Closed Lost',
};

export default function CRM() {
  const toast = useToast();
  const [leads, setLeads] = useState<Lead[]>([]);
  const [opportunities, setOpportunities] = useState<Opportunity[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [showLeadModal, setShowLeadModal] = useState(false);
  const [showOppModal, setShowOppModal] = useState(false);
  const [newLead, setNewLead] = useState<CreateLeadRequest>({ company_name: '', contact_name: '', email: '', phone: '', source: '', industry: '', estimated_value: 0 });
  const [newOpp, setNewOpp] = useState<CreateOpportunityRequest>({ name: '', amount: 0, expected_close_date: '', description: '' });

  useEffect(() => { loadData(); }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      const [leadsRes, oppsRes] = await Promise.all([crm.getLeads(), crm.getOpportunities()]);
      setLeads(leadsRes.data);
      setOpportunities(oppsRes.data);
    } catch (err) {
      toast.error('Failed to load CRM data');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateLead = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await crm.createLead(newLead);
      toast.success('Lead created successfully');
      setShowLeadModal(false);
      setNewLead({ company_name: '', contact_name: '', email: '', phone: '', source: '', industry: '', estimated_value: 0 });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create lead'));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateOpportunity = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await crm.createOpportunity(newOpp);
      toast.success('Opportunity created successfully');
      setShowOppModal(false);
      setNewOpp({ name: '', amount: 0, expected_close_date: '', description: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create opportunity'));
    } finally {
      setSaving(false);
    }
  };

  const handleMoveStage = async (opp: Opportunity, newStage: string) => {
    try {
      await crm.updateOpportunityStage(opp.id, newStage);
      toast.success('Stage updated');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to update stage'));
    }
  };

  const getOpportunitiesByStage = (stage: string) => opportunities.filter(o => o.stage === stage);
  
  const totalPipeline = opportunities.filter(o => o.stage !== 'ClosedWon' && o.stage !== 'ClosedLost').reduce((sum, o) => sum + o.amount, 0);
  const wonValue = opportunities.filter(o => o.stage === 'ClosedWon').reduce((sum, o) => sum + o.amount, 0);

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">CRM</h1>
        <div className="flex gap-2">
          <button onClick={() => setShowLeadModal(true)} className="btn btn-secondary">Add Lead</button>
          <button onClick={() => setShowOppModal(true)} className="btn btn-primary">Add Opportunity</button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Leads</p>
          <p className="text-2xl font-bold">{leads.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Open Opportunities</p>
          <p className="text-2xl font-bold">{opportunities.filter(o => o.stage !== 'ClosedWon' && o.stage !== 'ClosedLost').length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Pipeline Value</p>
          <p className="text-2xl font-bold">${totalPipeline.toLocaleString()}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Won (Closed)</p>
          <p className="text-2xl font-bold text-green-600">${wonValue.toLocaleString()}</p>
        </div>
      </div>

      <div className="card mb-6">
        <div className="p-4 border-b">
          <h2 className="text-lg font-semibold">Sales Pipeline</h2>
          <p className="text-sm text-gray-500">Drag opportunities between stages (click to move)</p>
        </div>
        <div className="p-4 overflow-x-auto">
          <div className="flex gap-4 min-w-max">
            {STAGES.map(stage => {
              const stageOpps = getOpportunitiesByStage(stage);
              const stageValue = stageOpps.reduce((sum, o) => sum + o.amount, 0);
              const isWon = stage === 'ClosedWon';
              const isLost = stage === 'ClosedLost';
              return (
                <div key={stage} className={`w-64 flex-shrink-0 ${isWon ? 'bg-green-50' : isLost ? 'bg-red-50' : 'bg-gray-50'} rounded-lg p-3`}>
                  <div className="flex justify-between items-center mb-3">
                    <h3 className={`font-medium ${isWon ? 'text-green-700' : isLost ? 'text-red-700' : 'text-gray-700'}`}>{STAGE_LABELS[stage]}</h3>
                    <span className="text-xs bg-white px-2 py-1 rounded-full">{stageOpps.length}</span>
                  </div>
                  <p className="text-sm text-gray-500 mb-3">${stageValue.toLocaleString()}</p>
                  <div className="space-y-2">
                    {stageOpps.map(opp => (
                      <div key={opp.id} className="bg-white p-3 rounded shadow-sm border">
                        <div className="font-medium text-sm truncate">{opp.name}</div>
                        <div className="flex justify-between items-center mt-2">
                          <span className="text-sm font-semibold">${opp.amount.toLocaleString()}</span>
                          <span className="text-xs text-gray-500">{opp.probability}%</span>
                        </div>
                        <div className="flex gap-1 mt-2 flex-wrap">
                          {STAGES.filter(s => s !== stage).slice(0, 4).map(s => (
                            <button
                              key={s}
                              onClick={() => handleMoveStage(opp, s)}
                              className="text-xs px-2 py-1 bg-gray-100 hover:bg-gray-200 rounded truncate"
                            >
                              {s === 'ClosedWon' ? 'Won' : s === 'ClosedLost' ? 'Lost' : s.slice(0, 4)}
                            </button>
                          ))}
                        </div>
                      </div>
                    ))}
                    {stageOpps.length === 0 && (
                      <div className="text-center text-gray-400 text-sm py-4">No opportunities</div>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </div>

      <div className="card">
        <div className="p-4 border-b">
          <h2 className="text-lg font-semibold">Leads</h2>
        </div>
        {leads.length === 0 ? (
          <div className="p-8 text-center text-gray-500">No leads found. Click "Add Lead" to create one.</div>
        ) : (
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="table-header">Lead #</th>
                <th className="table-header">Company</th>
                <th className="table-header">Contact</th>
                <th className="table-header">Email</th>
                <th className="table-header">Est. Value</th>
                <th className="table-header">Status</th>
              </tr>
            </thead>
            <tbody>
              {leads.map((lead) => (
                <tr key={lead.id} className="border-b hover:bg-gray-50">
                  <td className="table-cell font-mono text-sm">{lead.lead_number}</td>
                  <td className="table-cell">{lead.company_name}</td>
                  <td className="table-cell">{lead.contact_name || '-'}</td>
                  <td className="table-cell">{lead.email || '-'}</td>
                  <td className="table-cell">${lead.estimated_value.toLocaleString()}</td>
                  <td className="table-cell">
                    <span className={`badge ${lead.status === 'Converted' ? 'badge-success' : lead.status === 'Lost' ? 'badge-error' : lead.status === 'Qualified' ? 'badge-info' : 'badge-warning'}`}>{lead.status}</span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {showLeadModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Lead</h2>
            <form onSubmit={handleCreateLead} className="space-y-4">
              <div>
                <label className="label">Company Name *</label>
                <input className="input" value={newLead.company_name} onChange={(e) => setNewLead({ ...newLead, company_name: e.target.value })} required />
              </div>
              <div>
                <label className="label">Contact Name</label>
                <input className="input" value={newLead.contact_name || ''} onChange={(e) => setNewLead({ ...newLead, contact_name: e.target.value })} />
              </div>
              <div>
                <label className="label">Email</label>
                <input type="email" className="input" value={newLead.email || ''} onChange={(e) => setNewLead({ ...newLead, email: e.target.value })} />
              </div>
              <div>
                <label className="label">Phone</label>
                <input className="input" value={newLead.phone || ''} onChange={(e) => setNewLead({ ...newLead, phone: e.target.value })} />
              </div>
              <div>
                <label className="label">Source</label>
                <select className="input" value={newLead.source || ''} onChange={(e) => setNewLead({ ...newLead, source: e.target.value })}>
                  <option value="">Select source</option>
                  <option value="Website">Website</option>
                  <option value="Referral">Referral</option>
                  <option value="Trade Show">Trade Show</option>
                  <option value="Cold Call">Cold Call</option>
                  <option value="Social Media">Social Media</option>
                  <option value="Other">Other</option>
                </select>
              </div>
              <div>
                <label className="label">Industry</label>
                <input className="input" value={newLead.industry || ''} onChange={(e) => setNewLead({ ...newLead, industry: e.target.value })} />
              </div>
              <div>
                <label className="label">Estimated Value ($)</label>
                <input type="number" className="input" value={newLead.estimated_value || ''} onChange={(e) => setNewLead({ ...newLead, estimated_value: parseFloat(e.target.value) || 0 })} min="0" step="0.01" />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowLeadModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showOppModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Opportunity</h2>
            <form onSubmit={handleCreateOpportunity} className="space-y-4">
              <div>
                <label className="label">Opportunity Name *</label>
                <input className="input" value={newOpp.name} onChange={(e) => setNewOpp({ ...newOpp, name: e.target.value })} required />
              </div>
              <div>
                <label className="label">Amount ($) *</label>
                <input type="number" className="input" value={newOpp.amount || ''} onChange={(e) => setNewOpp({ ...newOpp, amount: parseFloat(e.target.value) || 0 })} min="0" step="0.01" required />
              </div>
              <div>
                <label className="label">Expected Close Date</label>
                <input type="date" className="input" value={newOpp.expected_close_date || ''} onChange={(e) => setNewOpp({ ...newOpp, expected_close_date: e.target.value })} />
              </div>
              <div>
                <label className="label">Description</label>
                <textarea className="input" rows={3} value={newOpp.description || ''} onChange={(e) => setNewOpp({ ...newOpp, description: e.target.value })} />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowOppModal(false)} className="btn btn-secondary" disabled={saving}>Cancel</button>
                <button type="submit" className="btn btn-primary" disabled={saving}>{saving ? 'Saving...' : 'Create'}</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

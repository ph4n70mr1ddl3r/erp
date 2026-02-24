import React, { useState, useEffect } from 'react';
import { rules } from '../api/client';

interface Rule {
  id: string;
  name: string;
  code: string;
  entity_type: string;
  status: string;
}

interface Ruleset {
  id: string;
  name: string;
  code: string;
  entity_type: string;
}

const RulesPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState('rules');
  const [rulesList, setRulesList] = useState<Rule[]>([]);
  const [rulesets, setRulesets] = useState<Ruleset[]>([]);
  
  const [showCreateRule, setShowCreateRule] = useState(false);
  const [showCreateRuleset, setShowCreateRuleset] = useState(false);
  const [showExecute, setShowExecute] = useState(false);
  
  const [newRule, setNewRule] = useState({
    name: '',
    code: '',
    entity_type: '',
    conditions: '{}',
    actions: '{}',
    rule_type: 'Validation',
  });
  
  const [newRuleset, setNewRuleset] = useState({
    name: '',
    code: '',
    entity_type: '',
    execution_mode: 'Sequential',
  });
  
  const [execution, setExecution] = useState({
    entity_type: '',
    entity_id: '',
    context: '{}',
  });

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [rulesRes, rulesetsRes] = await Promise.all([
        rules.listRules(),
        rules.listRulesets(),
      ]);
      setRulesList(rulesRes.data);
      setRulesets(rulesetsRes.data);
    } catch (error) {
      console.error('Failed to load rules:', error);
    }
  };

  const handleCreateRule = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await rules.createRule(newRule);
      setNewRule({ name: '', code: '', entity_type: '', conditions: '{}', actions: '{}', rule_type: 'Validation' });
      setShowCreateRule(false);
      loadData();
    } catch (error) {
      console.error('Failed to create rule:', error);
    }
  };

  const handleCreateRuleset = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await rules.createRuleset(newRuleset);
      setNewRuleset({ name: '', code: '', entity_type: '', execution_mode: 'Sequential' });
      setShowCreateRuleset(false);
      loadData();
    } catch (error) {
      console.error('Failed to create ruleset:', error);
    }
  };

  const handleExecute = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const result = await rules.executeRules({
        entity_type: execution.entity_type,
        entity_id: execution.entity_id,
        context: JSON.parse(execution.context),
      });
      alert(`Executed ${result.data.length} rules`);
    } catch (error) {
      console.error('Failed to execute rules:', error);
      alert('Failed to execute rules');
    }
  };

  const handleDelete = async (ruleId: string) => {
    if (confirm('Are you sure you want to delete this rule?')) {
      try {
        await rules.deleteRule(ruleId);
        loadData();
      } catch (error) {
        console.error('Failed to delete rule:', error);
      }
    }
  };

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Business Rules Engine</h1>
        <div className="space-x-2">
          <button
            onClick={() => setShowExecute(true)}
            className="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700"
          >
            Execute Rules
          </button>
          {activeTab === 'rules' && (
            <button
              onClick={() => setShowCreateRule(true)}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              New Rule
            </button>
          )}
          {activeTab === 'rulesets' && (
            <button
              onClick={() => setShowCreateRuleset(true)}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              New Ruleset
            </button>
          )}
        </div>
      </div>

      <div className="flex space-x-4 mb-6">
        <button
          onClick={() => setActiveTab('rules')}
          className={`px-4 py-2 rounded ${activeTab === 'rules' ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
        >
          Rules
        </button>
        <button
          onClick={() => setActiveTab('rulesets')}
          className={`px-4 py-2 rounded ${activeTab === 'rulesets' ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
        >
          Rulesets
        </button>
      </div>

      {showCreateRule && (
        <Modal title="Create Rule" onClose={() => setShowCreateRule(false)}>
          <form onSubmit={handleCreateRule}>
            <input
              type="text"
              value={newRule.name}
              onChange={(e) => setNewRule({ ...newRule, name: e.target.value })}
              placeholder="Name"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={newRule.code}
              onChange={(e) => setNewRule({ ...newRule, code: e.target.value })}
              placeholder="Code"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={newRule.entity_type}
              onChange={(e) => setNewRule({ ...newRule, entity_type: e.target.value })}
              placeholder="Entity Type (e.g., Order, Invoice)"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <select
              value={newRule.rule_type}
              onChange={(e) => setNewRule({ ...newRule, rule_type: e.target.value })}
              className="w-full border rounded px-3 py-2 mb-4"
            >
              <option value="Validation">Validation</option>
              <option value="Calculation">Calculation</option>
              <option value="Transformation">Transformation</option>
              <option value="Routing">Routing</option>
              <option value="Approval">Approval</option>
              <option value="Discount">Discount</option>
            </select>
            <textarea
              value={newRule.conditions}
              onChange={(e) => setNewRule({ ...newRule, conditions: e.target.value })}
              placeholder="Conditions (JSON)"
              className="w-full border rounded px-3 py-2 mb-4 h-24 font-mono text-sm"
            />
            <textarea
              value={newRule.actions}
              onChange={(e) => setNewRule({ ...newRule, actions: e.target.value })}
              placeholder="Actions (JSON)"
              className="w-full border rounded px-3 py-2 mb-4 h-24 font-mono text-sm"
            />
            <div className="flex justify-end">
              <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded">Create</button>
            </div>
          </form>
        </Modal>
      )}

      {showCreateRuleset && (
        <Modal title="Create Ruleset" onClose={() => setShowCreateRuleset(false)}>
          <form onSubmit={handleCreateRuleset}>
            <input
              type="text"
              value={newRuleset.name}
              onChange={(e) => setNewRuleset({ ...newRuleset, name: e.target.value })}
              placeholder="Name"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={newRuleset.code}
              onChange={(e) => setNewRuleset({ ...newRuleset, code: e.target.value })}
              placeholder="Code"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={newRuleset.entity_type}
              onChange={(e) => setNewRuleset({ ...newRuleset, entity_type: e.target.value })}
              placeholder="Entity Type"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <select
              value={newRuleset.execution_mode}
              onChange={(e) => setNewRuleset({ ...newRuleset, execution_mode: e.target.value })}
              className="w-full border rounded px-3 py-2 mb-4"
            >
              <option value="Sequential">Sequential</option>
              <option value="Parallel">Parallel</option>
              <option value="FirstMatch">First Match</option>
              <option value="AllMatches">All Matches</option>
            </select>
            <div className="flex justify-end">
              <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded">Create</button>
            </div>
          </form>
        </Modal>
      )}

      {showExecute && (
        <Modal title="Execute Rules" onClose={() => setShowExecute(false)}>
          <form onSubmit={handleExecute}>
            <input
              type="text"
              value={execution.entity_type}
              onChange={(e) => setExecution({ ...execution, entity_type: e.target.value })}
              placeholder="Entity Type"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <input
              type="text"
              value={execution.entity_id}
              onChange={(e) => setExecution({ ...execution, entity_id: e.target.value })}
              placeholder="Entity ID"
              className="w-full border rounded px-3 py-2 mb-4"
              required
            />
            <textarea
              value={execution.context}
              onChange={(e) => setExecution({ ...execution, context: e.target.value })}
              placeholder="Context (JSON)"
              className="w-full border rounded px-3 py-2 mb-4 h-32 font-mono text-sm"
            />
            <div className="flex justify-end">
              <button type="submit" className="px-4 py-2 bg-green-600 text-white rounded">Execute</button>
            </div>
          </form>
        </Modal>
      )}

      <div className="bg-white rounded-lg shadow">
        {activeTab === 'rules' && (
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left">Name</th>
                <th className="px-4 py-3 text-left">Code</th>
                <th className="px-4 py-3 text-left">Entity Type</th>
                <th className="px-4 py-3 text-left">Status</th>
                <th className="px-4 py-3 text-left">Actions</th>
              </tr>
            </thead>
            <tbody>
              {rulesList.map((rule) => (
                <tr key={rule.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3">{rule.name}</td>
                  <td className="px-4 py-3 font-mono text-sm">{rule.code}</td>
                  <td className="px-4 py-3">{rule.entity_type}</td>
                  <td className="px-4 py-3">
                    <span className="px-2 py-1 rounded text-xs bg-green-100 text-green-800">
                      {rule.status}
                    </span>
                  </td>
                  <td className="px-4 py-3">
                    <button
                      onClick={() => handleDelete(rule.id)}
                      className="text-red-600 hover:underline"
                    >
                      Delete
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {activeTab === 'rulesets' && (
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left">Name</th>
                <th className="px-4 py-3 text-left">Code</th>
                <th className="px-4 py-3 text-left">Entity Type</th>
              </tr>
            </thead>
            <tbody>
              {rulesets.map((rs) => (
                <tr key={rs.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3">{rs.name}</td>
                  <td className="px-4 py-3 font-mono text-sm">{rs.code}</td>
                  <td className="px-4 py-3">{rs.entity_type}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </div>
  );
};

const Modal: React.FC<{ title: string; onClose: () => void; children: React.ReactNode }> = ({ title, onClose, children }) => (
  <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div className="bg-white p-6 rounded-lg shadow-lg w-[500px] max-h-[80vh] overflow-y-auto">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-semibold">{title}</h2>
        <button onClick={onClose} className="text-gray-500 hover:text-gray-700">&times;</button>
      </div>
      {children}
    </div>
  </div>
);

export default RulesPage;

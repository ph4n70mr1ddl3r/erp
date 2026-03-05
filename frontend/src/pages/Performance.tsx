import { useEffect, useState, useCallback } from 'react';
import { performance, hr } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { getErrorMessage } from '../utils/errors';
import type { PerformanceCycle, PerformanceGoal, PerformanceReview, Employee } from '../types';

export default function Performance() {
  const toast = useToast();
  const [cycles, setCycles] = useState<PerformanceCycle[]>([]);
  const [goals, setGoals] = useState<PerformanceGoal[]>([]);
  const [reviews, setReviews] = useState<PerformanceReview[]>([]);
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [activeTab, setActiveTab] = useState<'cycles' | 'goals' | 'reviews'>('cycles');
  const [showCycleModal, setShowCycleModal] = useState(false);
  const [showGoalModal, setShowGoalModal] = useState(false);
  const [showReviewModal, setShowReviewModal] = useState(false);
  
  const [newCycle, setNewCycle] = useState({
    name: '',
    cycle_type: 'Annual',
    start_date: '',
    end_date: '',
    review_due_date: ''
  });
  
  const [newGoal, setNewGoal] = useState({
    employee_id: '',
    cycle_id: '',
    title: '',
    description: '',
    weight: 1,
    target_value: ''
  });
  
  const [newReview, setNewReview] = useState({
    employee_id: '',
    reviewer_id: '',
    cycle_id: '',
    review_type: 'SelfReview'
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [cyclesRes, goalsRes, reviewsRes, employeesRes] = await Promise.all([
        performance.listCycles(),
        performance.listGoals(),
        performance.listReviews(),
        hr.getEmployees(1, 100)
      ]);
      setCycles(cyclesRes.data);
      setGoals(goalsRes.data);
      setReviews(reviewsRes.data);
      setEmployees(employeesRes.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load performance data'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => { loadData(); }, [loadData]);

  const handleCreateCycle = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await performance.createCycle(newCycle);
      toast.success('Performance cycle created successfully');
      setShowCycleModal(false);
      setNewCycle({ name: '', cycle_type: 'Annual', start_date: '', end_date: '', review_due_date: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create cycle'));
    } finally {
      setSaving(false);
    }
  };

  const handleActivateCycle = async (id: string) => {
    try {
      await performance.activateCycle(id);
      toast.success('Cycle activated successfully');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to activate cycle'));
    }
  };

  const handleCloseCycle = async (id: string) => {
    try {
      await performance.closeCycle(id);
      toast.success('Cycle closed successfully');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to close cycle'));
    }
  };

  const handleCreateGoal = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await performance.createGoal({
        employee_id: newGoal.employee_id,
        cycle_id: newGoal.cycle_id,
        title: newGoal.title,
        description: newGoal.description || undefined,
        weight: newGoal.weight,
        target_value: newGoal.target_value || undefined
      });
      toast.success('Goal created successfully');
      setShowGoalModal(false);
      setNewGoal({ employee_id: '', cycle_id: '', title: '', description: '', weight: 1, target_value: '' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create goal'));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateReview = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await performance.createReview(newReview);
      toast.success('Review created successfully');
      setShowReviewModal(false);
      setNewReview({ employee_id: '', reviewer_id: '', cycle_id: '', review_type: 'SelfReview' });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create review'));
    } finally {
      setSaving(false);
    }
  };

  const handleSubmitReview = async (id: string) => {
    try {
      await performance.submitReview(id, { overall_rating: 3 });
      toast.success('Review submitted successfully');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to submit review'));
    }
  };

  const getStatusBadge = (status: string) => {
    const styles: Record<string, string> = {
      Draft: 'badge-default',
      Active: 'badge-success',
      Pending: 'badge-warning',
      Submitted: 'badge-info',
      Closed: 'badge-danger',
      Approved: 'badge-success'
    };
    return styles[status] || 'badge-default';
  };

  const getEmployeeName = (id: string) => {
    const emp = employees.find(e => e.id === id);
    return emp ? `${emp.first_name} ${emp.last_name}` : id;
  };

  const activeCycles = cycles.filter(c => c.status === 'Active').length;
  const completedGoals = goals.filter(g => g.status === 'Approved').length;
  const pendingReviews = reviews.filter(r => r.status === 'Draft').length;

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Performance Management</h1>
        <div className="flex gap-2">
          {activeTab === 'cycles' && (
            <button onClick={() => setShowCycleModal(true)} className="btn btn-primary">New Cycle</button>
          )}
          {activeTab === 'goals' && (
            <button onClick={() => setShowGoalModal(true)} className="btn btn-primary">New Goal</button>
          )}
          {activeTab === 'reviews' && (
            <button onClick={() => setShowReviewModal(true)} className="btn btn-primary">New Review</button>
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Active Cycles</p>
          <p className="text-2xl font-bold text-green-600">{activeCycles}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Goals</p>
          <p className="text-2xl font-bold">{goals.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Completed Goals</p>
          <p className="text-2xl font-bold text-green-600">{completedGoals}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Pending Reviews</p>
          <p className="text-2xl font-bold text-yellow-600">{pendingReviews}</p>
        </div>
      </div>

      <div className="flex gap-4 mb-4 border-b">
        <button
          onClick={() => setActiveTab('cycles')}
          className={`pb-2 px-1 ${activeTab === 'cycles' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-500'}`}
        >
          Cycles
        </button>
        <button
          onClick={() => setActiveTab('goals')}
          className={`pb-2 px-1 ${activeTab === 'goals' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-500'}`}
        >
          Goals
        </button>
        <button
          onClick={() => setActiveTab('reviews')}
          className={`pb-2 px-1 ${activeTab === 'reviews' ? 'border-b-2 border-blue-600 text-blue-600' : 'text-gray-500'}`}
        >
          Reviews
        </button>
      </div>

      {activeTab === 'cycles' && (
        <div className="card">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Performance Cycles</h2>
          </div>
          {cycles.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No performance cycles found</div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Name</th>
                  <th className="table-header">Type</th>
                  <th className="table-header">Start Date</th>
                  <th className="table-header">End Date</th>
                  <th className="table-header">Review Due</th>
                  <th className="table-header">Status</th>
                  <th className="table-header">Actions</th>
                </tr>
              </thead>
              <tbody>
                {cycles.map((cycle) => (
                  <tr key={cycle.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell">{cycle.name}</td>
                    <td className="table-cell">{cycle.cycle_type}</td>
                    <td className="table-cell">{cycle.start_date}</td>
                    <td className="table-cell">{cycle.end_date}</td>
                    <td className="table-cell">{cycle.review_due_date}</td>
                    <td className="table-cell">
                      <span className={`badge ${getStatusBadge(cycle.status)}`}>{cycle.status}</span>
                    </td>
                    <td className="table-cell">
                      {cycle.status === 'Draft' && (
                        <button onClick={() => handleActivateCycle(cycle.id)} className="btn btn-primary text-xs py-1">
                          Activate
                        </button>
                      )}
                      {cycle.status === 'Active' && (
                        <button onClick={() => handleCloseCycle(cycle.id)} className="btn btn-secondary text-xs py-1">
                          Close
                        </button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {activeTab === 'goals' && (
        <div className="card">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Performance Goals</h2>
          </div>
          {goals.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No performance goals found</div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Employee</th>
                  <th className="table-header">Title</th>
                  <th className="table-header">Weight</th>
                  <th className="table-header">Target</th>
                  <th className="table-header">Self Rating</th>
                  <th className="table-header">Manager Rating</th>
                  <th className="table-header">Status</th>
                </tr>
              </thead>
              <tbody>
                {goals.map((goal) => (
                  <tr key={goal.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell">{getEmployeeName(goal.employee_id)}</td>
                    <td className="table-cell">{goal.title}</td>
                    <td className="table-cell">{goal.weight}%</td>
                    <td className="table-cell">{goal.target_value || '-'}</td>
                    <td className="table-cell">{goal.self_rating || '-'}</td>
                    <td className="table-cell">{goal.manager_rating || '-'}</td>
                    <td className="table-cell">
                      <span className={`badge ${getStatusBadge(goal.status)}`}>{goal.status}</span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {activeTab === 'reviews' && (
        <div className="card">
          <div className="p-4 border-b">
            <h2 className="text-lg font-semibold">Performance Reviews</h2>
          </div>
          {reviews.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No performance reviews found</div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Employee</th>
                  <th className="table-header">Reviewer</th>
                  <th className="table-header">Type</th>
                  <th className="table-header">Rating</th>
                  <th className="table-header">Status</th>
                  <th className="table-header">Actions</th>
                </tr>
              </thead>
              <tbody>
                {reviews.map((review) => (
                  <tr key={review.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell">{getEmployeeName(review.employee_id)}</td>
                    <td className="table-cell">{getEmployeeName(review.reviewer_id)}</td>
                    <td className="table-cell">{review.review_type}</td>
                    <td className="table-cell">{review.overall_rating || '-'}</td>
                    <td className="table-cell">
                      <span className={`badge ${getStatusBadge(review.status)}`}>{review.status}</span>
                    </td>
                    <td className="table-cell">
                      {review.status === 'Draft' && (
                        <button onClick={() => handleSubmitReview(review.id)} className="btn btn-primary text-xs py-1">
                          Submit
                        </button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {showCycleModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Performance Cycle</h2>
            <form onSubmit={handleCreateCycle} className="space-y-4">
              <div>
                <label className="label">Name</label>
                <input
                  type="text"
                  className="input"
                  value={newCycle.name}
                  onChange={(e) => setNewCycle({ ...newCycle, name: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">Cycle Type</label>
                <select
                  className="input"
                  value={newCycle.cycle_type}
                  onChange={(e) => setNewCycle({ ...newCycle, cycle_type: e.target.value })}
                >
                  <option value="Annual">Annual</option>
                  <option value="MidYear">Mid-Year</option>
                  <option value="Quarterly">Quarterly</option>
                </select>
              </div>
              <div>
                <label className="label">Start Date</label>
                <input
                  type="date"
                  className="input"
                  value={newCycle.start_date}
                  onChange={(e) => setNewCycle({ ...newCycle, start_date: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">End Date</label>
                <input
                  type="date"
                  className="input"
                  value={newCycle.end_date}
                  onChange={(e) => setNewCycle({ ...newCycle, end_date: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">Review Due Date</label>
                <input
                  type="date"
                  className="input"
                  value={newCycle.review_due_date}
                  onChange={(e) => setNewCycle({ ...newCycle, review_due_date: e.target.value })}
                  required
                />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowCycleModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create Cycle'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showGoalModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Performance Goal</h2>
            <form onSubmit={handleCreateGoal} className="space-y-4">
              <div>
                <label className="label">Employee</label>
                <select
                  className="input"
                  value={newGoal.employee_id}
                  onChange={(e) => setNewGoal({ ...newGoal, employee_id: e.target.value })}
                  required
                >
                  <option value="">Select employee</option>
                  {employees.map((emp) => (
                    <option key={emp.id} value={emp.id}>
                      {emp.first_name} {emp.last_name}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Cycle</label>
                <select
                  className="input"
                  value={newGoal.cycle_id}
                  onChange={(e) => setNewGoal({ ...newGoal, cycle_id: e.target.value })}
                  required
                >
                  <option value="">Select cycle</option>
                  {cycles.filter(c => c.status === 'Active').map((cycle) => (
                    <option key={cycle.id} value={cycle.id}>
                      {cycle.name}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Title</label>
                <input
                  type="text"
                  className="input"
                  value={newGoal.title}
                  onChange={(e) => setNewGoal({ ...newGoal, title: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">Description</label>
                <textarea
                  className="input"
                  rows={2}
                  value={newGoal.description}
                  onChange={(e) => setNewGoal({ ...newGoal, description: e.target.value })}
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Weight (%)</label>
                  <input
                    type="number"
                    className="input"
                    min="1"
                    max="100"
                    value={newGoal.weight}
                    onChange={(e) => setNewGoal({ ...newGoal, weight: parseInt(e.target.value) || 1 })}
                    required
                  />
                </div>
                <div>
                  <label className="label">Target Value</label>
                  <input
                    type="text"
                    className="input"
                    value={newGoal.target_value}
                    onChange={(e) => setNewGoal({ ...newGoal, target_value: e.target.value })}
                  />
                </div>
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowGoalModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create Goal'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showReviewModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Performance Review</h2>
            <form onSubmit={handleCreateReview} className="space-y-4">
              <div>
                <label className="label">Employee</label>
                <select
                  className="input"
                  value={newReview.employee_id}
                  onChange={(e) => setNewReview({ ...newReview, employee_id: e.target.value })}
                  required
                >
                  <option value="">Select employee</option>
                  {employees.map((emp) => (
                    <option key={emp.id} value={emp.id}>
                      {emp.first_name} {emp.last_name}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Reviewer</label>
                <select
                  className="input"
                  value={newReview.reviewer_id}
                  onChange={(e) => setNewReview({ ...newReview, reviewer_id: e.target.value })}
                  required
                >
                  <option value="">Select reviewer</option>
                  {employees.map((emp) => (
                    <option key={emp.id} value={emp.id}>
                      {emp.first_name} {emp.last_name}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Cycle</label>
                <select
                  className="input"
                  value={newReview.cycle_id}
                  onChange={(e) => setNewReview({ ...newReview, cycle_id: e.target.value })}
                  required
                >
                  <option value="">Select cycle</option>
                  {cycles.filter(c => c.status === 'Active').map((cycle) => (
                    <option key={cycle.id} value={cycle.id}>
                      {cycle.name}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Review Type</label>
                <select
                  className="input"
                  value={newReview.review_type}
                  onChange={(e) => setNewReview({ ...newReview, review_type: e.target.value })}
                >
                  <option value="SelfReview">Self Review</option>
                  <option value="ManagerReview">Manager Review</option>
                  <option value="PeerReview">Peer Review</option>
                </select>
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowReviewModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create Review'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

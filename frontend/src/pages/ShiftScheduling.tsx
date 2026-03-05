import { useEffect, useState, useCallback } from 'react';
import { shiftScheduling, hr } from '../api/client';
import { useToast } from '../components/Toast';
import { LoadingPage } from '../components/Spinner';
import { getErrorMessage } from '../utils/errors';
import type { Shift, Schedule, ShiftAssignment, Employee } from '../types';

export default function ShiftScheduling() {
  const toast = useToast();
  const [shifts, setShifts] = useState<Shift[]>([]);
  const [schedules, setSchedules] = useState<Schedule[]>([]);
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [assignments, setAssignments] = useState<ShiftAssignment[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [activeTab, setActiveTab] = useState<'shifts' | 'schedules' | 'assignments'>('shifts');
  const [showShiftModal, setShowShiftModal] = useState(false);
  const [showScheduleModal, setShowScheduleModal] = useState(false);
  const [showAssignmentModal, setShowAssignmentModal] = useState(false);
  const [selectedScheduleId, setSelectedScheduleId] = useState<string>('');
  const [newShift, setNewShift] = useState({
    code: '',
    name: '',
    description: '',
    start_time: '09:00',
    end_time: '17:00',
    break_minutes: 30,
    grace_period_minutes: 15,
    color_code: '#3B82F6'
  });
  const [newSchedule, setNewSchedule] = useState({
    code: '',
    name: '',
    description: '',
    department_id: '',
    start_date: '',
    end_date: ''
  });
  const [newAssignment, setNewAssignment] = useState({
    schedule_id: '',
    shift_id: '',
    employee_id: '',
    assignment_date: '',
    notes: ''
  });

  const loadData = useCallback(async () => {
    try {
      setLoading(true);
      const [shiftsRes, schedulesRes, employeesRes] = await Promise.all([
        shiftScheduling.listShifts(1, 100),
        shiftScheduling.listSchedules(1, 100),
        hr.getEmployees(1, 100)
      ]);
      setShifts(shiftsRes.data.items);
      setSchedules(schedulesRes.data.items);
      setEmployees(employeesRes.data.items);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load shift scheduling data'));
    } finally {
      setLoading(false);
    }
  }, [toast]);

  useEffect(() => { loadData(); }, [loadData]);

  const loadAssignments = async (scheduleId: string) => {
    if (!scheduleId) return;
    try {
      const res = await shiftScheduling.listAssignments(scheduleId);
      setAssignments(res.data);
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to load assignments'));
    }
  };;

  const handleCreateShift = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await shiftScheduling.createShift({
        code: newShift.code,
        name: newShift.name,
        description: newShift.description || undefined,
        start_time: newShift.start_time,
        end_time: newShift.end_time,
        break_minutes: newShift.break_minutes,
        grace_period_minutes: newShift.grace_period_minutes,
        color_code: newShift.color_code
      });
      toast.success('Shift created successfully');
      setShowShiftModal(false);
      setNewShift({
        code: '',
        name: '',
        description: '',
        start_time: '09:00',
        end_time: '17:00',
        break_minutes: 30,
        grace_period_minutes: 15,
        color_code: '#3B82F6'
      });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create shift'));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateSchedule = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await shiftScheduling.createSchedule({
        code: newSchedule.code,
        name: newSchedule.name,
        description: newSchedule.description || undefined,
        department_id: newSchedule.department_id || undefined,
        start_date: newSchedule.start_date,
        end_date: newSchedule.end_date
      });
      toast.success('Schedule created successfully');
      setShowScheduleModal(false);
      setNewSchedule({
        code: '',
        name: '',
        description: '',
        department_id: '',
        start_date: '',
        end_date: ''
      });
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create schedule'));
    } finally {
      setSaving(false);
    }
  };

  const handleCreateAssignment = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      setSaving(true);
      await shiftScheduling.createAssignment({
        schedule_id: newAssignment.schedule_id,
        shift_id: newAssignment.shift_id,
        employee_id: newAssignment.employee_id,
        assignment_date: newAssignment.assignment_date,
        notes: newAssignment.notes || undefined
      });
      toast.success('Assignment created successfully');
      setShowAssignmentModal(false);
      setNewAssignment({
        schedule_id: '',
        shift_id: '',
        employee_id: '',
        assignment_date: '',
        notes: ''
      });
      if (newAssignment.schedule_id) {
        loadAssignments(newAssignment.schedule_id);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to create assignment'));
    } finally {
      setSaving(false);
    }
  };

  const handlePublishSchedule = async (id: string) => {
    try {
      await shiftScheduling.publishSchedule(id);
      toast.success('Schedule published successfully');
      loadData();
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to publish schedule'));
    }
  };

  const handleClockIn = async (id: string) => {
    try {
      await shiftScheduling.clockIn(id);
      toast.success('Clocked in successfully');
      if (selectedScheduleId) {
        loadAssignments(selectedScheduleId);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to clock in'));
    }
  };

  const handleClockOut = async (id: string) => {
    try {
      await shiftScheduling.clockOut(id);
      toast.success('Clocked out successfully');
      if (selectedScheduleId) {
        loadAssignments(selectedScheduleId);
      }
    } catch (err: unknown) {
      toast.error(getErrorMessage(err, 'Failed to clock out'));
    }
  };

  const getStatusBadge = (status: string) => {
    const styles: Record<string, string> = {
      Draft: 'badge-default',
      Active: 'badge-success',
      Inactive: 'badge-danger',
      Published: 'badge-success',
      Archived: 'badge-default',
      Scheduled: 'badge-info',
      Confirmed: 'badge-success',
      InProgress: 'badge-warning',
      Completed: 'badge-success',
      Absent: 'badge-danger',
      Cancelled: 'badge-danger'
    };
    return styles[status] || 'badge-default';
  };

  const getEmployeeName = (id: string) => {
    const emp = employees.find(e => e.id === id);
    return emp ? `${emp.first_name} ${emp.last_name}` : id;
  };

  const getShiftName = (id: string) => {
    const shift = shifts.find(s => s.id === id);
    return shift ? `${shift.name} (${shift.start_time}-${shift.end_time})` : id;
  };

  const activeShifts = shifts.filter(s => s.status === 'Active').length;
  const publishedSchedules = schedules.filter(s => s.status === 'Published').length;
  const todayAssignments = assignments.filter(a => a.assignment_date === new Date().toISOString().split('T')[0]).length;

  if (loading) return <LoadingPage />;

  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Shift Scheduling</h1>
        <div className="flex gap-2">
          {activeTab === 'shifts' && (
            <button onClick={() => setShowShiftModal(true)} className="btn btn-primary">New Shift</button>
          )}
          {activeTab === 'schedules' && (
            <button onClick={() => setShowScheduleModal(true)} className="btn btn-primary">New Schedule</button>
          )}
          {activeTab === 'assignments' && (
            <button onClick={() => setShowAssignmentModal(true)} className="btn btn-primary">New Assignment</button>
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-6">
        <div className="card p-4">
          <p className="text-sm text-gray-500">Active Shifts</p>
          <p className="text-2xl font-bold">{activeShifts}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Total Schedules</p>
          <p className="text-2xl font-bold">{schedules.length}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Published</p>
          <p className="text-2xl font-bold text-green-600">{publishedSchedules}</p>
        </div>
        <div className="card p-4">
          <p className="text-sm text-gray-500">Today's Assignments</p>
          <p className="text-2xl font-bold">{todayAssignments}</p>
        </div>
      </div>

      <div className="flex gap-4 mb-6 border-b">
        <button
          className={`pb-2 px-1 ${activeTab === 'shifts' ? 'border-b-2 border-blue-500 text-blue-600' : 'text-gray-500'}`}
          onClick={() => setActiveTab('shifts')}
        >
          Shifts
        </button>
        <button
          className={`pb-2 px-1 ${activeTab === 'schedules' ? 'border-b-2 border-blue-500 text-blue-600' : 'text-gray-500'}`}
          onClick={() => setActiveTab('schedules')}
        >
          Schedules
        </button>
        <button
          className={`pb-2 px-1 ${activeTab === 'assignments' ? 'border-b-2 border-blue-500 text-blue-600' : 'text-gray-500'}`}
          onClick={() => setActiveTab('assignments')}
        >
          Assignments
        </button>
      </div>

      {activeTab === 'shifts' && (
        <div className="card">
          {shifts.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No shifts found. Create a shift to get started.</div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Code</th>
                  <th className="table-header">Name</th>
                  <th className="table-header">Start Time</th>
                  <th className="table-header">End Time</th>
                  <th className="table-header">Break (min)</th>
                  <th className="table-header">Status</th>
                </tr>
              </thead>
              <tbody>
                {shifts.map((shift) => (
                  <tr key={shift.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-medium">{shift.code}</td>
                    <td className="table-cell">{shift.name}</td>
                    <td className="table-cell">{shift.start_time}</td>
                    <td className="table-cell">{shift.end_time}</td>
                    <td className="table-cell">{shift.break_minutes}</td>
                    <td className="table-cell">
                      <span className={`badge ${getStatusBadge(shift.status)}`}>{shift.status}</span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      )}

      {activeTab === 'schedules' && (
        <div className="card">
          {schedules.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No schedules found. Create a schedule to get started.</div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Code</th>
                  <th className="table-header">Name</th>
                  <th className="table-header">Start Date</th>
                  <th className="table-header">End Date</th>
                  <th className="table-header">Status</th>
                  <th className="table-header">Actions</th>
                </tr>
              </thead>
              <tbody>
                {schedules.map((schedule) => (
                  <tr key={schedule.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-medium">{schedule.code}</td>
                    <td className="table-cell">{schedule.name}</td>
                    <td className="table-cell">{schedule.start_date}</td>
                    <td className="table-cell">{schedule.end_date}</td>
                    <td className="table-cell">
                      <span className={`badge ${getStatusBadge(schedule.status)}`}>{schedule.status}</span>
                    </td>
                    <td className="table-cell">
                      {schedule.status === 'Draft' && (
                        <button
                          onClick={() => handlePublishSchedule(schedule.id)}
                          className="btn btn-primary text-xs py-1"
                        >
                          Publish
                        </button>
                      )}
                      {schedule.status === 'Published' && (
                        <button
                          onClick={() => {
                            setSelectedScheduleId(schedule.id);
                            loadAssignments(schedule.id);
                            setActiveTab('assignments');
                          }}
                          className="btn btn-secondary text-xs py-1"
                        >
                          View Assignments
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

      {activeTab === 'assignments' && (
        <div className="card">
          <div className="p-4 border-b flex items-center gap-4">
            <label className="text-sm font-medium">Schedule:</label>
            <select
              className="input w-64"
              value={selectedScheduleId}
              onChange={(e) => {
                setSelectedScheduleId(e.target.value);
                loadAssignments(e.target.value);
              }}
            >
              <option value="">Select a schedule</option>
              {schedules.filter(s => s.status === 'Published').map((s) => (
                <option key={s.id} value={s.id}>{s.name}</option>
              ))}
            </select>
          </div>
          {!selectedScheduleId ? (
            <div className="p-8 text-center text-gray-500">Select a schedule to view assignments</div>
          ) : assignments.length === 0 ? (
            <div className="p-8 text-center text-gray-500">No assignments found for this schedule</div>
          ) : (
            <table className="w-full">
              <thead>
                <tr className="border-b">
                  <th className="table-header">Employee</th>
                  <th className="table-header">Shift</th>
                  <th className="table-header">Date</th>
                  <th className="table-header">Status</th>
                  <th className="table-header">Notes</th>
                  <th className="table-header">Actions</th>
                </tr>
              </thead>
              <tbody>
                {assignments.map((assignment) => (
                  <tr key={assignment.id} className="border-b hover:bg-gray-50">
                    <td className="table-cell font-medium">{getEmployeeName(assignment.employee_id)}</td>
                    <td className="table-cell">{getShiftName(assignment.shift_id)}</td>
                    <td className="table-cell">{assignment.assignment_date}</td>
                    <td className="table-cell">
                      <span className={`badge ${getStatusBadge(assignment.status)}`}>{assignment.status}</span>
                    </td>
                    <td className="table-cell">{assignment.notes || '-'}</td>
                    <td className="table-cell">
                      {assignment.status === 'Scheduled' && (
                        <button
                          onClick={() => handleClockIn(assignment.id)}
                          className="btn btn-primary text-xs py-1"
                        >
                          Clock In
                        </button>
                      )}
                      {assignment.status === 'InProgress' && (
                        <button
                          onClick={() => handleClockOut(assignment.id)}
                          className="btn btn-secondary text-xs py-1"
                        >
                          Clock Out
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

      {showShiftModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Shift</h2>
            <form onSubmit={handleCreateShift} className="space-y-4">
              <div>
                <label className="label">Code</label>
                <input
                  type="text"
                  className="input"
                  value={newShift.code}
                  onChange={(e) => setNewShift({ ...newShift, code: e.target.value })}
                  placeholder="e.g., MORNING, NIGHT"
                  required
                />
              </div>
              <div>
                <label className="label">Name</label>
                <input
                  type="text"
                  className="input"
                  value={newShift.name}
                  onChange={(e) => setNewShift({ ...newShift, name: e.target.value })}
                  placeholder="e.g., Morning Shift"
                  required
                />
              </div>
              <div>
                <label className="label">Description</label>
                <input
                  type="text"
                  className="input"
                  value={newShift.description}
                  onChange={(e) => setNewShift({ ...newShift, description: e.target.value })}
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Start Time</label>
                  <input
                    type="time"
                    className="input"
                    value={newShift.start_time}
                    onChange={(e) => setNewShift({ ...newShift, start_time: e.target.value })}
                    required
                  />
                </div>
                <div>
                  <label className="label">End Time</label>
                  <input
                    type="time"
                    className="input"
                    value={newShift.end_time}
                    onChange={(e) => setNewShift({ ...newShift, end_time: e.target.value })}
                    required
                  />
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Break (minutes)</label>
                  <input
                    type="number"
                    className="input"
                    min="0"
                    value={newShift.break_minutes}
                    onChange={(e) => setNewShift({ ...newShift, break_minutes: parseInt(e.target.value) || 0 })}
                  />
                </div>
                <div>
                  <label className="label">Grace Period (min)</label>
                  <input
                    type="number"
                    className="input"
                    min="0"
                    value={newShift.grace_period_minutes}
                    onChange={(e) => setNewShift({ ...newShift, grace_period_minutes: parseInt(e.target.value) || 0 })}
                  />
                </div>
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowShiftModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create Shift'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showScheduleModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Schedule</h2>
            <form onSubmit={handleCreateSchedule} className="space-y-4">
              <div>
                <label className="label">Code</label>
                <input
                  type="text"
                  className="input"
                  value={newSchedule.code}
                  onChange={(e) => setNewSchedule({ ...newSchedule, code: e.target.value })}
                  placeholder="e.g., WEEK-2024-01"
                  required
                />
              </div>
              <div>
                <label className="label">Name</label>
                <input
                  type="text"
                  className="input"
                  value={newSchedule.name}
                  onChange={(e) => setNewSchedule({ ...newSchedule, name: e.target.value })}
                  placeholder="e.g., Week 1 January 2024"
                  required
                />
              </div>
              <div>
                <label className="label">Description</label>
                <input
                  type="text"
                  className="input"
                  value={newSchedule.description}
                  onChange={(e) => setNewSchedule({ ...newSchedule, description: e.target.value })}
                />
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="label">Start Date</label>
                  <input
                    type="date"
                    className="input"
                    value={newSchedule.start_date}
                    onChange={(e) => setNewSchedule({ ...newSchedule, start_date: e.target.value })}
                    required
                  />
                </div>
                <div>
                  <label className="label">End Date</label>
                  <input
                    type="date"
                    className="input"
                    value={newSchedule.end_date}
                    onChange={(e) => setNewSchedule({ ...newSchedule, end_date: e.target.value })}
                    required
                  />
                </div>
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowScheduleModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create Schedule'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showAssignmentModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="card p-6 w-full max-w-md">
            <h2 className="text-lg font-semibold mb-4">New Assignment</h2>
            <form onSubmit={handleCreateAssignment} className="space-y-4">
              <div>
                <label className="label">Schedule</label>
                <select
                  className="input"
                  value={newAssignment.schedule_id}
                  onChange={(e) => setNewAssignment({ ...newAssignment, schedule_id: e.target.value })}
                  required
                >
                  <option value="">Select schedule</option>
                  {schedules.filter(s => s.status === 'Published').map((s) => (
                    <option key={s.id} value={s.id}>{s.name}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Employee</label>
                <select
                  className="input"
                  value={newAssignment.employee_id}
                  onChange={(e) => setNewAssignment({ ...newAssignment, employee_id: e.target.value })}
                  required
                >
                  <option value="">Select employee</option>
                  {employees.map((emp) => (
                    <option key={emp.id} value={emp.id}>
                      {emp.first_name} {emp.last_name} ({emp.employee_number})
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Shift</label>
                <select
                  className="input"
                  value={newAssignment.shift_id}
                  onChange={(e) => setNewAssignment({ ...newAssignment, shift_id: e.target.value })}
                  required
                >
                  <option value="">Select shift</option>
                  {shifts.filter(s => s.status === 'Active').map((s) => (
                    <option key={s.id} value={s.id}>
                      {s.name} ({s.start_time} - {s.end_time})
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="label">Assignment Date</label>
                <input
                  type="date"
                  className="input"
                  value={newAssignment.assignment_date}
                  onChange={(e) => setNewAssignment({ ...newAssignment, assignment_date: e.target.value })}
                  required
                />
              </div>
              <div>
                <label className="label">Notes</label>
                <textarea
                  className="input"
                  rows={2}
                  value={newAssignment.notes}
                  onChange={(e) => setNewAssignment({ ...newAssignment, notes: e.target.value })}
                />
              </div>
              <div className="flex gap-2 justify-end">
                <button type="button" onClick={() => setShowAssignmentModal(false)} className="btn btn-secondary" disabled={saving}>
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary" disabled={saving}>
                  {saving ? 'Creating...' : 'Create Assignment'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

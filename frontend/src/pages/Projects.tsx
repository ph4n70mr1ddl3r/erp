import { useState, useEffect } from 'react';
import { Folder, ListTodo, Flag, Clock, Plus, X, Check, Play, Pause, CheckCircle } from 'lucide-react';
import { projects, type Project, type ProjectTask, type ProjectMilestone, type Timesheet } from '../api/client';
import { useToast } from '../components/Toast';
import { getErrorMessage } from '../utils/errors';

type Tab = 'projects' | 'tasks' | 'timesheets';

export default function Projects() {
  const toast = useToast();
  const [activeTab, setActiveTab] = useState<Tab>('projects');
  const [projectList, setProjectList] = useState<Project[]>([]);
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);
  const [tasks, setTasks] = useState<ProjectTask[]>([]);
  const [milestones, setMilestones] = useState<ProjectMilestone[]>([]);
  const [timesheets, setTimesheets] = useState<Timesheet[]>([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState<string | null>(null);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    try {
      const [projectsRes, timesheetsRes] = await Promise.all([
        projects.getProjects(),
        projects.getTimesheets(),
      ]);
      setProjectList(projectsRes.data);
      setTimesheets(timesheetsRes.data);
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to load data'));
    } finally {
      setLoading(false);
    }
  };

  const loadProjectDetails = async (project: Project) => {
    try {
      const [tasksRes, milestonesRes] = await Promise.all([
        projects.getTasks(project.id),
        projects.getMilestones(project.id),
      ]);
      setTasks(tasksRes.data);
      setMilestones(milestonesRes.data);
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to load project details'));
    }
  };

  const handleSelectProject = (project: Project) => {
    setSelectedProject(project);
    loadProjectDetails(project);
    setActiveTab('tasks');
  };

  const handleCreateProject = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const form = e.currentTarget;
    const formData = new FormData(form);
    try {
      await projects.createProject({
        name: formData.get('name') as string,
        description: formData.get('description') as string || undefined,
        start_date: formData.get('start_date') as string,
        end_date: formData.get('end_date') as string || undefined,
        budget: formData.get('budget') ? parseInt(formData.get('budget') as string) : undefined,
      });
      setShowModal(null);
      loadData();
      form.reset();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to create project'));
    }
  };

  const handleCreateTask = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!selectedProject) return;
    const form = e.currentTarget;
    const formData = new FormData(form);
    try {
      await projects.createTask({
        project_id: selectedProject.id,
        name: formData.get('name') as string,
        start_date: formData.get('start_date') as string,
      });
      setShowModal(null);
      loadProjectDetails(selectedProject);
      form.reset();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to create task'));
    }
  };

  const handleCompleteTask = async (taskId: string) => {
    try {
      await projects.completeTask(taskId);
      if (selectedProject) {
        loadProjectDetails(selectedProject);
      }
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to complete task'));
    }
  };

  const handleCreateMilestone = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!selectedProject) return;
    const form = e.currentTarget;
    const formData = new FormData(form);
    try {
      await projects.createMilestone({
        project_id: selectedProject.id,
        name: formData.get('name') as string,
        planned_date: formData.get('planned_date') as string,
      });
      setShowModal(null);
      loadProjectDetails(selectedProject);
      form.reset();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to create milestone'));
    }
  };

  const handleCompleteMilestone = async (milestoneId: string) => {
    try {
      await projects.completeMilestone(milestoneId);
      if (selectedProject) {
        loadProjectDetails(selectedProject);
      }
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to complete milestone'));
    }
  };

  const handleUpdateStatus = async (projectId: string, status: string) => {
    try {
      await projects.updateStatus(projectId, status);
      loadData();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to update status'));
    }
  };

  const handleApproveTimesheet = async (id: string) => {
    try {
      await projects.approveTimesheet(id);
      loadData();
    } catch (error: unknown) {
      toast.error(getErrorMessage(error, 'Failed to approve timesheet'));
    }
  };

  if (loading) {
    return <div className="text-center py-8">Loading...</div>;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Project Management</h1>
          <p className="text-gray-500">Manage projects, tasks, and timesheets</p>
        </div>
      </div>

      <div className="border-b border-gray-200 mb-6">
        <nav className="flex gap-4">
          <button
            onClick={() => setActiveTab('projects')}
            className={`flex items-center gap-2 px-4 py-2 border-b-2 transition-colors ${
              activeTab === 'projects'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            <Folder className="w-4 h-4" />
            Projects
          </button>
          <button
            onClick={() => setActiveTab('tasks')}
            className={`flex items-center gap-2 px-4 py-2 border-b-2 transition-colors ${
              activeTab === 'tasks'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            <ListTodo className="w-4 h-4" />
            Tasks
          </button>
          <button
            onClick={() => setActiveTab('timesheets')}
            className={`flex items-center gap-2 px-4 py-2 border-b-2 transition-colors ${
              activeTab === 'timesheets'
                ? 'border-blue-500 text-blue-600'
                : 'border-transparent text-gray-500 hover:text-gray-700'
            }`}
          >
            <Clock className="w-4 h-4" />
            Timesheets
          </button>
        </nav>
      </div>

      {activeTab === 'projects' && (
        <div>
          <div className="flex justify-end mb-4">
            <button
              onClick={() => setShowModal('project')}
              className="btn-primary flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              New Project
            </button>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {projectList.map((project) => (
              <div
                key={project.id}
                className="bg-white rounded-lg shadow p-6 cursor-pointer hover:shadow-md transition-shadow"
                onClick={() => handleSelectProject(project)}
              >
                <div className="flex items-start justify-between mb-2">
                  <h3 className="font-semibold text-lg">{project.name}</h3>
                  <span className={`px-2 py-1 text-xs rounded-full ${
                    project.status === 'Active' ? 'bg-green-100 text-green-800' :
                    project.status === 'Completed' ? 'bg-blue-100 text-blue-800' :
                    project.status === 'OnHold' ? 'bg-yellow-100 text-yellow-800' :
                    'bg-gray-100 text-gray-800'
                  }`}>
                    {project.status}
                  </span>
                </div>
                <p className="text-sm text-gray-500 mb-3">{project.description || 'No description'}</p>
                <div className="mb-3">
                  <div className="flex justify-between text-xs text-gray-500 mb-1">
                    <span>Progress</span>
                    <span>{project.percent_complete}%</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-blue-500 h-2 rounded-full"
                      style={{ width: `${project.percent_complete}%` }}
                    />
                  </div>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-500">{project.project_number}</span>
                  <div className="flex gap-2">
                    {project.status === 'Active' && (
                      <button
                        onClick={(e) => { e.stopPropagation(); handleUpdateStatus(project.id, 'OnHold'); }}
                        className="text-yellow-600 hover:text-yellow-800"
                        title="Put on hold"
                      >
                        <Pause className="w-4 h-4" />
                      </button>
                    )}
                    {project.status === 'OnHold' && (
                      <button
                        onClick={(e) => { e.stopPropagation(); handleUpdateStatus(project.id, 'Active'); }}
                        className="text-green-600 hover:text-green-800"
                        title="Resume"
                      >
                        <Play className="w-4 h-4" />
                      </button>
                    )}
                    {project.status !== 'Completed' && (
                      <button
                        onClick={(e) => { e.stopPropagation(); handleUpdateStatus(project.id, 'Completed'); }}
                        className="text-blue-600 hover:text-blue-800"
                        title="Mark complete"
                      >
                        <CheckCircle className="w-4 h-4" />
                      </button>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'tasks' && (
        <div>
          {selectedProject ? (
            <>
              <div className="flex items-center gap-2 mb-4">
                <button
                  onClick={() => { setSelectedProject(null); setTasks([]); setMilestones([]); }}
                  className="text-blue-600 hover:text-blue-800"
                >
                  &larr; Back to Projects
                </button>
                <span className="text-gray-400">|</span>
                <h2 className="font-semibold">{selectedProject.name}</h2>
              </div>
              
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div>
                  <div className="flex items-center justify-between mb-3">
                    <h3 className="font-semibold flex items-center gap-2">
                      <ListTodo className="w-5 h-5" />
                      Tasks
                    </h3>
                    <button
                      onClick={() => setShowModal('task')}
                      className="text-blue-600 hover:text-blue-800 text-sm"
                    >
                      + Add Task
                    </button>
                  </div>
                  <div className="bg-white rounded-lg shadow divide-y">
                    {tasks.length === 0 ? (
                      <p className="p-4 text-gray-500 text-center">No tasks yet</p>
                    ) : (
                      tasks.map((task) => (
                        <div key={task.id} className="p-4 flex items-center justify-between">
                          <div>
                            <p className="font-medium">{task.name}</p>
                            <div className="flex items-center gap-2 mt-1">
                              <span className={`px-2 py-0.5 text-xs rounded-full ${
                                task.status === 'Completed' ? 'bg-green-100 text-green-800' :
                                task.status === 'InProgress' ? 'bg-blue-100 text-blue-800' :
                                'bg-gray-100 text-gray-800'
                              }`}>
                                {task.status}
                              </span>
                              <span className="text-xs text-gray-500">{task.percent_complete}%</span>
                            </div>
                          </div>
                          {task.status !== 'Completed' && (
                            <button
                              onClick={() => handleCompleteTask(task.id)}
                              className="text-green-600 hover:text-green-800"
                            >
                              <Check className="w-5 h-5" />
                            </button>
                          )}
                        </div>
                      ))
                    )}
                  </div>
                </div>

                <div>
                  <div className="flex items-center justify-between mb-3">
                    <h3 className="font-semibold flex items-center gap-2">
                      <Flag className="w-5 h-5" />
                      Milestones
                    </h3>
                    <button
                      onClick={() => setShowModal('milestone')}
                      className="text-blue-600 hover:text-blue-800 text-sm"
                    >
                      + Add Milestone
                    </button>
                  </div>
                  <div className="bg-white rounded-lg shadow divide-y">
                    {milestones.length === 0 ? (
                      <p className="p-4 text-gray-500 text-center">No milestones yet</p>
                    ) : (
                      milestones.map((milestone) => (
                        <div key={milestone.id} className="p-4 flex items-center justify-between">
                          <div>
                            <p className="font-medium">{milestone.name}</p>
                            <div className="flex items-center gap-2 mt-1">
                              <span className={`px-2 py-0.5 text-xs rounded-full ${
                                milestone.status === 'Completed' ? 'bg-green-100 text-green-800' :
                                'bg-gray-100 text-gray-800'
                              }`}>
                                {milestone.status}
                              </span>
                              {milestone.planned_date && (
                                <span className="text-xs text-gray-500">
                                  Due: {new Date(milestone.planned_date).toLocaleDateString()}
                                </span>
                              )}
                            </div>
                          </div>
                          {milestone.status !== 'Completed' && (
                            <button
                              onClick={() => handleCompleteMilestone(milestone.id)}
                              className="text-green-600 hover:text-green-800"
                            >
                              <Check className="w-5 h-5" />
                            </button>
                          )}
                        </div>
                      ))
                    )}
                  </div>
                </div>
              </div>
            </>
          ) : (
            <div className="text-center py-8 text-gray-500">
              Select a project to view tasks and milestones
            </div>
          )}
        </div>
      )}

      {activeTab === 'timesheets' && (
        <div className="bg-white rounded-lg shadow overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Number</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Period</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Hours</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Status</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Actions</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {timesheets.length === 0 ? (
                <tr>
                  <td colSpan={5} className="px-6 py-4 text-center text-gray-500">No timesheets</td>
                </tr>
              ) : (
                timesheets.map((ts) => (
                  <tr key={ts.id}>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">{ts.timesheet_number}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {new Date(ts.period_start).toLocaleDateString()} - {new Date(ts.period_end).toLocaleDateString()}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">{ts.total_hours}</td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`px-2 py-1 text-xs rounded-full ${
                        ts.status === 'Approved' ? 'bg-green-100 text-green-800' :
                        ts.status === 'Submitted' ? 'bg-blue-100 text-blue-800' :
                        ts.status === 'Rejected' ? 'bg-red-100 text-red-800' :
                        'bg-gray-100 text-gray-800'
                      }`}>
                        {ts.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {ts.status === 'Submitted' && (
                        <button
                          onClick={() => handleApproveTimesheet(ts.id)}
                          className="text-green-600 hover:text-green-800"
                        >
                          Approve
                        </button>
                      )}
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      )}

      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-lg font-bold">
                {showModal === 'project' && 'New Project'}
                {showModal === 'task' && 'New Task'}
                {showModal === 'milestone' && 'New Milestone'}
              </h2>
              <button onClick={() => setShowModal(null)}>
                <X className="w-5 h-5" />
              </button>
            </div>

            {showModal === 'project' && (
              <form onSubmit={handleCreateProject} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Name *</label>
                  <input name="name" required className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Description</label>
                  <textarea name="description" className="input-field" rows={3} />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Start Date *</label>
                  <input name="start_date" type="date" required className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">End Date</label>
                  <input name="end_date" type="date" className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Budget</label>
                  <input name="budget" type="number" className="input-field" />
                </div>
                <button type="submit" className="btn-primary w-full">Create Project</button>
              </form>
            )}

            {showModal === 'task' && (
              <form onSubmit={handleCreateTask} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Task Name *</label>
                  <input name="name" required className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Start Date *</label>
                  <input name="start_date" type="date" required className="input-field" />
                </div>
                <button type="submit" className="btn-primary w-full">Create Task</button>
              </form>
            )}

            {showModal === 'milestone' && (
              <form onSubmit={handleCreateMilestone} className="space-y-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Milestone Name *</label>
                  <input name="name" required className="input-field" />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Planned Date *</label>
                  <input name="planned_date" type="date" required className="input-field" />
                </div>
                <button type="submit" className="btn-primary w-full">Create Milestone</button>
              </form>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

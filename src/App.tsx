import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'
import { SearchPage } from './pages/SearchPage'
import { KnowledgePage } from './pages/KnowledgePage'
import { AgentsPage } from './pages/AgentsPage'
import { FilesPage } from './pages/FilesPage'
import { TasksPage } from './pages/TasksPage'

interface Project {
  id: string
  name: string
  description: string | null
  created_at: number
  updated_at: number
}

interface AppInfo {
  name: string
  version: string
  description: string
}

function App() {
  const [appInfo, setAppInfo] = useState<AppInfo | null>(null)
  const [projects, setProjects] = useState<Project[]>([])
  const [newProjectName, setNewProjectName] = useState('')
  const [newProjectDesc, setNewProjectDesc] = useState('')
  const [loading, setLoading] = useState(false)
  const [activeTab, setActiveTab] = useState('dashboard')

  useEffect(() => {
    // Initialize database and load data
    initializeApp()
  }, [])

  const initializeApp = async () => {
    setLoading(true)
    try {
      // Get app info
      const info = await invoke<AppInfo>('get_app_info')
      setAppInfo(info)

      // Initialize database
      await invoke('init_database')

      // Load projects
      await loadProjects()
    } catch (error) {
      console.error('Failed to initialize app:', error)
    } finally {
      setLoading(false)
    }
  }

  const loadProjects = async () => {
    try {
      const projectList = await invoke<Project[]>('get_projects')
      setProjects(projectList)
    } catch (error) {
      console.error('Failed to load projects:', error)
    }
  }

  const createProject = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!newProjectName.trim()) return

    try {
      await invoke('create_project', {
        name: newProjectName,
        description: newProjectDesc || null
      })
      setNewProjectName('')
      setNewProjectDesc('')
      await loadProjects()
    } catch (error) {
      console.error('Failed to create project:', error)
    }
  }

  return (
    <div className="app">
      <nav className="sidebar">
        <div className="logo">
          🧠 CortexMind
        </div>
        <div className="nav-items">
          <button
            className={`nav-item ${activeTab === 'dashboard' ? 'active' : ''}`}
            onClick={() => setActiveTab('dashboard')}
          >
            📊 Dashboard
          </button>
          <button
            className={`nav-item ${activeTab === 'projects' ? 'active' : ''}`}
            onClick={() => setActiveTab('projects')}
          >
            📁 Projects
          </button>
          <button
            className={`nav-item ${activeTab === 'knowledge' ? 'active' : ''}`}
            onClick={() => setActiveTab('knowledge')}
          >
            🧠 Knowledge
          </button>
          <button
            className={`nav-item ${activeTab === 'search' ? 'active' : ''}`}
            onClick={() => setActiveTab('search')}
          >
            🔍 Search
          </button>
          <button
            className={`nav-item ${activeTab === 'agents' ? 'active' : ''}`}
            onClick={() => setActiveTab('agents')}
          >
            🔌 Agents
          </button>
          <button
            className={`nav-item ${activeTab === 'files' ? 'active' : ''}`}
            onClick={() => setActiveTab('files')}
          >
            📁 Files
          </button>
          <button
            className={`nav-item ${activeTab === 'tasks' ? 'active' : ''}`}
            onClick={() => setActiveTab('tasks')}
          >
            ✅ Tasks
          </button>
          <button
            className={`nav-item ${activeTab === 'settings' ? 'active' : ''}`}
            onClick={() => setActiveTab('settings')}
          >
            ⚙️ Settings
          </button>
        </div>
        {appInfo && (
          <div className="app-version">
            v{appInfo.version}
          </div>
        )}
      </nav>

      <main className="main-content">
        {loading ? (
          <div className="loading">
            <div className="spinner"></div>
            <p>Loading CortexMind...</p>
          </div>
        ) : (
          <>
            {activeTab === 'dashboard' && (
              <div className="page">
                <header>
                  <h1>📊 Dashboard</h1>
                  <p>Welcome to CortexMind - Your AI Experience, Perfected</p>
                </header>
                <div className="stats">
                  <div className="stat-card">
                    <div className="stat-value">{projects.length}</div>
                    <div className="stat-label">Projects</div>
                  </div>
                  <div className="stat-card">
                    <div className="stat-value">0</div>
                    <div className="stat-label">Knowledge Items</div>
                  </div>
                  <div className="stat-card">
                    <div className="stat-value">0</div>
                    <div className="stat-label">Active Agents</div>
                  </div>
                  <div className="stat-card">
                    <div className="stat-value">0</div>
                    <div className="stat-label">Sessions</div>
                  </div>
                </div>
              </div>
            )}

            {activeTab === 'projects' && (
              <div className="page">
                <header>
                  <h1>📁 Projects</h1>
                  <p>Manage your development projects</p>
                </header>

                <div className="create-project-form">
                  <h2>Create New Project</h2>
                  <form onSubmit={createProject}>
                    <input
                      type="text"
                      placeholder="Project name"
                      value={newProjectName}
                      onChange={(e) => setNewProjectName(e.target.value)}
                      required
                    />
                    <input
                      type="text"
                      placeholder="Description (optional)"
                      value={newProjectDesc}
                      onChange={(e) => setNewProjectDesc(e.target.value)}
                    />
                    <button type="submit">Create Project</button>
                  </form>
                </div>

                <div className="projects-list">
                  <h2>Your Projects</h2>
                  {projects.length === 0 ? (
                    <p className="empty-state">No projects yet. Create your first project above!</p>
                  ) : (
                    <div className="project-grid">
                      {projects.map(project => (
                        <div key={project.id} className="project-card">
                          <h3>{project.name}</h3>
                          {project.description && (
                            <p>{project.description}</p>
                          )}
                          <div className="project-meta">
                            <span>Created: {new Date(project.created_at * 1000).toLocaleDateString()}</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            )}

            {activeTab === 'knowledge' && <KnowledgePage />}

            {activeTab === 'search' && <SearchPage />}

            {activeTab === 'agents' && <AgentsPage />}

            {activeTab === 'files' && <FilesPage />}

            {activeTab === 'tasks' && <TasksPage />}

            {activeTab === 'settings' && (
              <div className="page">
                <header>
                  <h1>⚙️ Settings</h1>
                  <p>Configure CortexMind</p>
                </header>
                <div className="empty-state">
                  <p>Settings coming soon!</p>
                  <p>Configure AI models, vector database, and sync options.</p>
                </div>
              </div>
            )}
          </>
        )}
      </main>
    </div>
  )
}

export default App

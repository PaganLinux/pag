import { useEffect, useState, FormEvent } from 'react';
import { Plus, ExternalLink } from 'lucide-react';
import * as api from '../api/client';
import toast from 'react-hot-toast';

export default function Repos() {
  const [repos, setRepos] = useState<api.Repo[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');

  const load = () => {
    setLoading(true);
    api.repos.list().then(setRepos).finally(() => setLoading(false));
  };

  useEffect(() => { load(); }, []);

  const handleCreate = async (e: FormEvent) => {
    e.preventDefault();
    try {
      await api.repos.create({ name, description });
      toast.success('Repository created in Gitea');
      setShowForm(false);
      setName(''); setDescription('');
      load();
    } catch {
      toast.error('Failed to create repository');
    }
  };

  if (loading) {
    return <div className="flex items-center justify-center h-64 text-gray-500">Loading...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between flex-wrap gap-3">
        <div>
          <h2 className="text-2xl font-bold">Repositories</h2>
          <p className="text-gray-500 mt-1">Gitea integration · git.paganlinux.eu</p>
        </div>
        <button onClick={() => setShowForm(true)} className="flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-all text-sm">
          <Plus size={16} /> New Repository
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {repos.map(r => (
          <div key={r.id} className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-5 hover:border-purple-700/30 transition-all">
            <div className="flex items-start justify-between mb-2">
              <h3 className="font-semibold text-lg">{r.full_name}</h3>
              {r.active ? (
                <span className="text-xs px-2 py-0.5 rounded-full bg-green-500/10 text-green-400 border border-green-500/30">Active</span>
              ) : (
                <span className="text-xs px-2 py-0.5 rounded-full bg-gray-500/10 text-gray-400 border border-gray-500/30">Inactive</span>
              )}
            </div>
            {r.description && <p className="text-sm text-gray-400 mb-3">{r.description}</p>}
            <div className="flex items-center gap-3 text-sm">
              {r.clone_url && (
                <a href={r.clone_url} target="_blank" rel="noopener" className="flex items-center gap-1 text-purple-400 hover:text-purple-300 transition-colors">
                  <ExternalLink size={14} /> Clone
                </a>
              )}
              <span className="text-gray-500 text-xs">{new Date(r.created_at).toLocaleDateString()}</span>
            </div>
          </div>
        ))}
        {repos.length === 0 && (
          <div className="col-span-full text-center py-12 text-gray-500">
            No repositories yet. Create one to link with Gitea.
          </div>
        )}
      </div>

      {/* Create modal */}
      {showForm && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4" onClick={() => setShowForm(false)}>
          <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-6 w-full max-w-md" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-semibold mb-4">Create Repository in Gitea</h3>
            <form onSubmit={handleCreate} className="space-y-3">
              <input value={name} onChange={e => setName(e.target.value)} placeholder="Repository name" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" required />
              <input value={description} onChange={e => setDescription(e.target.value)} placeholder="Description (optional)" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" />
              <div className="flex gap-2 pt-2">
                <button type="button" onClick={() => setShowForm(false)} className="flex-1 px-4 py-2 border border-[#2a2a3e] rounded-lg text-gray-400">Cancel</button>
                <button type="submit" className="flex-1 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700">Create in Gitea</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

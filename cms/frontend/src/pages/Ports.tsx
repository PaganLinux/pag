import { useEffect, useState, FormEvent } from 'react';
import { Plus, Trash2 } from 'lucide-react';
import StatusBadge from '../components/StatusBadge';
import * as api from '../api/client';
import toast from 'react-hot-toast';

export default function Ports() {
  const [ports, setPorts] = useState<api.Port[]>([]);
  const [loading, setLoading] = useState(true);
  const [showForm, setShowForm] = useState(false);

  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [version, setVersion] = useState('');
  const [pagbuildPath, setPagbuildPath] = useState('');

  const load = () => {
    setLoading(true);
    api.ports.list().then(setPorts).finally(() => setLoading(false));
  };

  useEffect(() => { load(); }, []);

  const handleCreate = async (e: FormEvent) => {
    e.preventDefault();
    try {
      await api.ports.create({ name, description, version, pagbuild_path: pagbuildPath });
      toast.success('Port created');
      setShowForm(false);
      setName(''); setDescription(''); setVersion(''); setPagbuildPath('');
      load();
    } catch {
      toast.error('Failed to create port');
    }
  };

  const handleDelete = async (id: number) => {
    if (!confirm('Delete this port?')) return;
    try {
      await api.ports.delete(id);
      toast.success('Port deleted');
      load();
    } catch {
      toast.error('Failed to delete');
    }
  };

  if (loading) {
    return <div className="flex items-center justify-center h-64 text-gray-500">Loading...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between flex-wrap gap-3">
        <div>
          <h2 className="text-2xl font-bold">PagPorts</h2>
          <p className="text-gray-500 mt-1">{ports.length} ports</p>
        </div>
        <button onClick={() => setShowForm(true)} className="flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-all text-sm">
          <Plus size={16} /> New Port
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {ports.map(p => (
          <div key={p.id} className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-4 hover:border-purple-700/30 transition-all">
            <div className="flex items-start justify-between mb-3">
              <div>
                <h3 className="font-semibold">{p.name}</h3>
                {p.category && <p className="text-xs text-purple-400 mt-0.5">{p.category}</p>}
              </div>
              <StatusBadge status={p.status} />
            </div>
            {p.description && <p className="text-sm text-gray-400 mb-3">{p.description}</p>}
            {p.version && <p className="text-xs text-gray-500 mb-2">v{p.version}</p>}
            <code className="block text-xs bg-[#0a0a0f] border border-[#2a2a3e] rounded p-2 font-mono text-gray-400 truncate">{p.pagbuild_path}</code>
            <button onClick={() => handleDelete(p.id)} className="mt-3 flex items-center gap-1 text-xs text-red-400 hover:text-red-300 transition-colors">
              <Trash2 size={12} /> Delete
            </button>
          </div>
        ))}
        {ports.length === 0 && (
          <div className="col-span-full text-center py-12 text-gray-500">No ports created yet</div>
        )}
      </div>

      {/* Create modal */}
      {showForm && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4" onClick={() => setShowForm(false)}>
          <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-6 w-full max-w-md" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-semibold mb-4">New Port</h3>
            <form onSubmit={handleCreate} className="space-y-3">
              <input value={name} onChange={e => setName(e.target.value)} placeholder="Port name" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" required />
              <input value={description} onChange={e => setDescription(e.target.value)} placeholder="Description" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" />
              <input value={version} onChange={e => setVersion(e.target.value)} placeholder="Version" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" />
              <input value={pagbuildPath} onChange={e => setPagbuildPath(e.target.value)} placeholder="Path to PAGBUILD file" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" required />
              <div className="flex gap-2 pt-2">
                <button type="button" onClick={() => setShowForm(false)} className="flex-1 px-4 py-2 border border-[#2a2a3e] rounded-lg text-gray-400">Cancel</button>
                <button type="submit" className="flex-1 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700">Create</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

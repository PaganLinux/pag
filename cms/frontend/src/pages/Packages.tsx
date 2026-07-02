import { useEffect, useState, FormEvent } from 'react';
import { Plus, RefreshCw, Search, Upload } from 'lucide-react';
import StatusBadge from '../components/StatusBadge';
import * as api from '../api/client';
import toast from 'react-hot-toast';

export default function Packages() {
  const [pkgs, setPkgs] = useState<api.Package[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [showForm, setShowForm] = useState(false);
  const [showUpload, setShowUpload] = useState(false);

  // Form state
  const [name, setName] = useState('');
  const [version, setVersion] = useState('');
  const [description, setDescription] = useState('');
  const [arch, setArch] = useState('x86_64');
  const [pagbuild, setPagbuild] = useState('');

  const load = () => {
    setLoading(true);
    api.packages.list({ search: search || undefined })
      .then(res => setPkgs(res.packages))
      .finally(() => setLoading(false));
  };

  useEffect(() => { load(); }, []);

  const handleCreate = async (e: FormEvent) => {
    e.preventDefault();
    try {
      await api.packages.create({ name, version, description, arch });
      toast.success('Package created');
      setShowForm(false);
      setName(''); setVersion(''); setDescription(''); setArch('x86_64');
      load();
    } catch {
      toast.error('Failed to create package');
    }
  };

  const handleUpload = async (e: FormEvent) => {
    e.preventDefault();
    try {
      await api.packages.upload({ name, version, description, arch, pagbuild });
      toast.success('Package uploaded with PAGBUILD');
      setShowUpload(false);
      setPagbuild('');
      load();
    } catch {
      toast.error('Failed to upload');
    }
  };

  const handleBuild = async (pkgId: number) => {
    try {
      await api.builds.create(pkgId);
      toast.success('Build queued');
      load();
    } catch {
      toast.error('Failed to queue build');
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between flex-wrap gap-3">
        <div>
          <h2 className="text-2xl font-bold">Packages</h2>
          <p className="text-gray-500 mt-1">{pkgs.length} packages</p>
        </div>
        <div className="flex gap-2">
          <button onClick={() => setShowUpload(true)} className="flex items-center gap-2 px-4 py-2 bg-purple-600/20 text-purple-400 border border-purple-600/30 rounded-lg hover:bg-purple-600/30 transition-all text-sm">
            <Upload size={16} /> Upload PAGBUILD
          </button>
          <button onClick={() => setShowForm(true)} className="flex items-center gap-2 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-all text-sm">
            <Plus size={16} /> New Package
          </button>
        </div>
      </div>

      {/* Search */}
      <div className="relative">
        <Search size={18} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
        <input
          type="text"
          value={search}
          onChange={e => setSearch(e.target.value)}
          onKeyDown={e => e.key === 'Enter' && load()}
          placeholder="Search packages..."
          className="w-full bg-[#1a1a2e] border border-[#2a2a3e] rounded-lg pl-10 pr-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600"
        />
      </div>

      {/* Table */}
      <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="text-left text-gray-500 border-b border-[#2a2a3e] bg-[#12121a]">
                <th className="p-4 font-medium">Name</th>
                <th className="p-4 font-medium">Version</th>
                <th className="p-4 font-medium">Arch</th>
                <th className="p-4 font-medium">Status</th>
                <th className="p-4 font-medium">Actions</th>
              </tr>
            </thead>
            <tbody>
              {pkgs.map(p => (
                <tr key={p.id} className="border-b border-white/5 hover:bg-white/5">
                  <td className="p-4">
                    <span className="font-medium">{p.name}</span>
                    {p.description && <p className="text-xs text-gray-500 mt-0.5">{p.description}</p>}
                  </td>
                  <td className="p-4 text-gray-400">{p.version}-{p.release}</td>
                  <td className="p-4"><code className="text-xs bg-white/5 px-1.5 py-0.5 rounded">{p.arch}</code></td>
                  <td className="p-4"><StatusBadge status={p.build_status} /></td>
                  <td className="p-4">
                    <button onClick={() => handleBuild(p.id)} className="flex items-center gap-1 text-xs px-2.5 py-1.5 bg-purple-600/10 text-purple-400 border border-purple-600/20 rounded-md hover:bg-purple-600/20 transition-all">
                      <RefreshCw size={12} /> Rebuild
                    </button>
                  </td>
                </tr>
              ))}
              {pkgs.length === 0 && (
                <tr><td colSpan={5} className="p-8 text-center text-gray-500">No packages found</td></tr>
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Create modal */}
      {showForm && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4" onClick={() => setShowForm(false)}>
          <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-6 w-full max-w-md" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-semibold mb-4">New Package</h3>
            <form onSubmit={handleCreate} className="space-y-3">
              <input value={name} onChange={e => setName(e.target.value)} placeholder="Package name" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" required />
              <input value={version} onChange={e => setVersion(e.target.value)} placeholder="Version (e.g. 1.0.0)" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" required />
              <input value={description} onChange={e => setDescription(e.target.value)} placeholder="Description" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" />
              <select value={arch} onChange={e => setArch(e.target.value)} className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600">
                <option value="x86_64">x86_64</option>
                <option value="aarch64">aarch64</option>
              </select>
              <div className="flex gap-2 pt-2">
                <button type="button" onClick={() => setShowForm(false)} className="flex-1 px-4 py-2 border border-[#2a2a3e] rounded-lg text-gray-400 hover:text-gray-200 transition-colors">Cancel</button>
                <button type="submit" className="flex-1 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition-colors">Create</button>
              </div>
            </form>
          </div>
        </div>
      )}

      {/* Upload modal */}
      {showUpload && (
        <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50 p-4" onClick={() => setShowUpload(false)}>
          <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-6 w-full max-w-lg" onClick={e => e.stopPropagation()}>
            <h3 className="text-lg font-semibold mb-4">Upload PAGBUILD</h3>
            <form onSubmit={handleUpload} className="space-y-3">
              <input value={name} onChange={e => setName(e.target.value)} placeholder="Package name" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" required />
              <input value={version} onChange={e => setVersion(e.target.value)} placeholder="Version" className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 focus:outline-none focus:border-purple-600" required />
              <textarea value={pagbuild} onChange={e => setPagbuild(e.target.value)} placeholder="Paste PAGBUILD content..." rows={10} className="w-full bg-[#0a0a0f] border border-[#2a2a3e] rounded-lg px-4 py-2.5 text-gray-100 font-mono text-sm focus:outline-none focus:border-purple-600" />
              <div className="flex gap-2 pt-2">
                <button type="button" onClick={() => setShowUpload(false)} className="flex-1 px-4 py-2 border border-[#2a2a3e] rounded-lg text-gray-400">Cancel</button>
                <button type="submit" className="flex-1 px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700">Upload & Parse</button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

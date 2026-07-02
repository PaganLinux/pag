import { useEffect, useState } from 'react';
import { Hammer } from 'lucide-react';
import StatusBadge from '../components/StatusBadge';
import * as api from '../api/client';

export default function Builds() {
  const [builds, setBuilds] = useState<api.Build[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.builds.list()
      .then(res => setBuilds(res.builds))
      .finally(() => setLoading(false));

    // Auto-refresh co 10 sekund
    const interval = setInterval(() => {
      api.builds.list().then(res => setBuilds(res.builds));
    }, 10000);
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return <div className="flex items-center justify-center h-64 text-gray-500">Loading...</div>;
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold">Build Queue</h2>
        <p className="text-gray-500 mt-1">{builds.length} builds · auto-refresh every 10s</p>
      </div>

      <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="text-left text-gray-500 border-b border-[#2a2a3e] bg-[#12121a]">
                <th className="p-4 font-medium">Job ID</th>
                <th className="p-4 font-medium">Package</th>
                <th className="p-4 font-medium">Arch</th>
                <th className="p-4 font-medium">Status</th>
                <th className="p-4 font-medium">Started</th>
                <th className="p-4 font-medium">Finished</th>
              </tr>
            </thead>
            <tbody>
              {builds.map(b => (
                <tr key={b.id} className="border-b border-white/5 hover:bg-white/5">
                  <td className="p-4"><code className="text-xs bg-white/5 px-1.5 py-0.5 rounded font-mono">{b.job_id.slice(0, 12)}...</code></td>
                  <td className="p-4">
                    <span className="font-medium">{b.package_name || `#${b.package_id}`}</span>
                    {b.package_version && <span className="text-gray-500 ml-1.5">{b.package_version}</span>}
                  </td>
                  <td className="p-4"><code className="text-xs bg-white/5 px-1.5 py-0.5 rounded">{b.arch}</code></td>
                  <td className="p-4"><StatusBadge status={b.status} /></td>
                  <td className="p-4 text-gray-400">{b.started_at ? new Date(b.started_at).toLocaleString() : '—'}</td>
                  <td className="p-4 text-gray-400">{b.finished_at ? new Date(b.finished_at).toLocaleString() : '—'}</td>
                </tr>
              ))}
              {builds.length === 0 && (
                <tr><td colSpan={6} className="p-8 text-center text-gray-500">No builds yet</td></tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

import { useEffect, useState } from 'react';
import { Package, Hammer, Box, GitBranch, Activity } from 'lucide-react';
import StatCard from '../components/StatCard';
import StatusBadge from '../components/StatusBadge';
import * as api from '../api/client';

export default function Dashboard() {
  const [stats, setStats] = useState<api.Stats | null>(null);
  const [recentBuilds, setRecentBuilds] = useState<api.Build[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([api.stats.get(), api.builds.list()])
      .then(([s, b]) => {
        setStats(s);
        setRecentBuilds(b.builds.slice(0, 5));
      })
      .finally(() => setLoading(false));
  }, []);

  if (loading) {
    return <div className="flex items-center justify-center h-64 text-gray-500">Loading...</div>;
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold">Dashboard</h2>
        <p className="text-gray-500 mt-1">Overview of PaganLinux CMS</p>
      </div>

      {/* Stats grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard title="Total Packages" value={stats?.total_packages ?? 0} icon={<Package size={22} className="text-purple-400" />} color="text-purple-400" />
        <StatCard title="Total Ports" value={stats?.total_ports ?? 0} icon={<Box size={22} className="text-blue-400" />} color="text-blue-400" />
        <StatCard title="Total Builds" value={stats?.total_builds ?? 0} subtitle={`${stats?.builds_today ?? 0} today`} icon={<Hammer size={22} className="text-yellow-400" />} color="text-yellow-400" />
        <StatCard title="Success Rate" value={stats ? `${stats.total_builds > 0 ? Math.round((stats.successful_builds / stats.total_builds) * 100) : 0}%` : '0%'} icon={<Activity size={22} className="text-green-400" />} color="text-green-400" />
      </div>

      {/* Recent builds */}
      <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-5">
        <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
          <Hammer size={18} className="text-yellow-400" />
          Recent Builds
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="text-left text-gray-500 border-b border-[#2a2a3e]">
                <th className="pb-3 font-medium">Package</th>
                <th className="pb-3 font-medium">Arch</th>
                <th className="pb-3 font-medium">Status</th>
                <th className="pb-3 font-medium">Created</th>
              </tr>
            </thead>
            <tbody>
              {recentBuilds.map(b => (
                <tr key={b.id} className="border-b border-white/5">
                  <td className="py-2.5 pr-4">
                    <span className="font-medium">{b.package_name || `#${b.package_id}`}</span>
                    {b.package_version && <span className="text-gray-500 ml-1">{b.package_version}</span>}
                  </td>
                  <td className="py-2.5 pr-4 text-gray-400">{b.arch}</td>
                  <td className="py-2.5 pr-4"><StatusBadge status={b.status} /></td>
                  <td className="py-2.5 text-gray-500">{new Date(b.created_at).toLocaleDateString()}</td>
                </tr>
              ))}
              {recentBuilds.length === 0 && (
                <tr><td colSpan={4} className="py-6 text-center text-gray-500">No builds yet</td></tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

interface StatCardProps {
  title: string;
  value: number | string;
  subtitle?: string;
  icon: React.ReactNode;
  color: string;
}

export default function StatCard({ title, value, subtitle, icon, color }: StatCardProps) {
  return (
    <div className="bg-[#1a1a2e] border border-[#2a2a3e] rounded-xl p-5 hover:border-purple-700/50 transition-all">
      <div className="flex items-start justify-between">
        <div>
          <p className="text-sm text-gray-400 mb-1">{title}</p>
          <p className={`text-3xl font-bold ${color}`}>{value}</p>
          {subtitle && <p className="text-xs text-gray-500 mt-1">{subtitle}</p>}
        </div>
        <div className={`p-2 rounded-lg bg-white/5`}>{icon}</div>
      </div>
    </div>
  );
}

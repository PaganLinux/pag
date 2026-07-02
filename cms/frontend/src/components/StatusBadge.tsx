interface StatusBadgeProps {
  status: string;
}

const colors: Record<string, string> = {
  success: 'bg-green-500/10 text-green-400 border-green-500/30',
  failed: 'bg-red-500/10 text-red-400 border-red-500/30',
  building: 'bg-yellow-500/10 text-yellow-400 border-yellow-500/30',
  running: 'bg-blue-500/10 text-blue-400 border-blue-500/30',
  queued: 'bg-gray-500/10 text-gray-400 border-gray-500/30',
  pending: 'bg-gray-500/10 text-gray-400 border-gray-500/30',
  active: 'bg-green-500/10 text-green-400 border-green-500/30',
  cancelled: 'bg-red-500/10 text-red-400 border-red-500/30',
  outdated: 'bg-yellow-500/10 text-yellow-400 border-yellow-500/30',
  broken: 'bg-red-500/10 text-red-400 border-red-500/30',
  archived: 'bg-gray-500/10 text-gray-400 border-gray-500/30',
};

export default function StatusBadge({ status }: StatusBadgeProps) {
  const color = colors[status] || colors.pending;
  return (
    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border ${color}`}>
      {status}
    </span>
  );
}

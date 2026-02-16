import { Play, Pause, Square, SkipForward, Zap, Users, Clock, Wifi, WifiOff } from 'lucide-react';
import { useSimulationStore } from '../stores/simulation';
import type { SimulationSpeed } from '../types';

const SPEEDS: { label: string; value: SimulationSpeed }[] = [
  { label: '1x', value: 'Normal' },
  { label: '2x', value: 'Fast' },
  { label: '5x', value: 'VeryFast' },
  { label: '10x', value: 'Maximum' },
];

function formatTime(time: { semester: number; week: number; day_of_week: number; hour: number } | null): string {
  if (!time) return '--';
  const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
  const day = days[time.day_of_week % 7] || '?';
  return `S${time.semester} W${time.week} ${day} ${time.hour}:00`;
}

export function TopBar() {
  const {
    running, speed, time, tick, agents, connected,
    startSimulation, stopSimulation, stepSimulation, setSpeed,
  } = useSimulationStore();

  return (
    <header className="h-12 panel border-b border-border flex items-center px-4 gap-3 relative z-20">
      {/* Logo */}
      <div className="flex items-center gap-2 mr-2">
        <div className="w-6 h-6 rounded bg-gradient-to-br from-accent-cyan to-accent-violet flex items-center justify-center">
          <Zap size={14} className="text-void" />
        </div>
        <span className="font-display font-bold text-sm tracking-wide text-text-primary">
          AI SCHOOL
        </span>
        <span className="text-[10px] font-mono text-text-muted px-1.5 py-0.5 bg-surface-overlay rounded">
          v0.1
        </span>
      </div>

      {/* Separator */}
      <div className="w-px h-6 bg-border" />

      {/* Simulation Controls */}
      <div className="flex items-center gap-1">
        {!running ? (
          <button
            onClick={startSimulation}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-medium bg-accent-emerald/15 text-accent-emerald hover:bg-accent-emerald/25 transition-smooth"
          >
            <Play size={12} /> Start
          </button>
        ) : (
          <button
            onClick={stopSimulation}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-medium bg-accent-rose/15 text-accent-rose hover:bg-accent-rose/25 transition-smooth"
          >
            <Square size={12} /> Stop
          </button>
        )}

        <button
          onClick={stepSimulation}
          className="flex items-center gap-1.5 px-2.5 py-1.5 rounded text-xs font-medium text-text-secondary hover:bg-surface-overlay transition-smooth"
          title="Step"
        >
          <SkipForward size={12} />
        </button>

        {running && (
          <button
            onClick={() => setSpeed(speed === 'Paused' ? 'Normal' : 'Paused')}
            className="flex items-center gap-1.5 px-2.5 py-1.5 rounded text-xs font-medium text-text-secondary hover:bg-surface-overlay transition-smooth"
          >
            <Pause size={12} />
          </button>
        )}
      </div>

      {/* Separator */}
      <div className="w-px h-6 bg-border" />

      {/* Speed Control */}
      <div className="flex items-center gap-0.5 bg-surface-overlay rounded p-0.5">
        {SPEEDS.map(({ label, value }) => (
          <button
            key={value}
            onClick={() => setSpeed(value)}
            className={`px-2 py-1 rounded text-[11px] font-mono font-medium transition-smooth ${
              speed === value
                ? 'bg-accent-cyan/20 text-accent-cyan'
                : 'text-text-muted hover:text-text-secondary'
            }`}
          >
            {label}
          </button>
        ))}
      </div>

      {/* Spacer */}
      <div className="flex-1" />

      {/* Time Display */}
      <div className="flex items-center gap-2 text-xs font-mono">
        <Clock size={12} className="text-text-muted" />
        <span className="text-accent-cyan">{formatTime(time)}</span>
        <span className="text-text-muted">T{tick}</span>
      </div>

      {/* Separator */}
      <div className="w-px h-6 bg-border" />

      {/* Agent Count */}
      <div className="flex items-center gap-1.5 text-xs">
        <Users size={12} className="text-text-muted" />
        <span className="font-mono text-text-secondary">{agents.length}</span>
      </div>

      {/* Connection */}
      <div className="flex items-center gap-1.5">
        {connected ? (
          <Wifi size={12} className="text-accent-emerald" />
        ) : (
          <WifiOff size={12} className="text-accent-rose" />
        )}
      </div>
    </header>
  );
}

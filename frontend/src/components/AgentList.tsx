import { UserPlus, Search } from 'lucide-react';
import { useState } from 'react';
import { useSimulationStore } from '../stores/simulation';

const MBTI_COLORS: Record<string, string> = {
  E: '#00d4ff', I: '#8b5cf6',
  S: '#fb923c', N: '#34d399',
  T: '#3b82f6', F: '#fb7185',
  J: '#fbbf24', P: '#6366f1',
};

function getMbtiColor(mbti: string): string {
  if (!mbti || mbti.length < 1) return '#5a5c70';
  return MBTI_COLORS[mbti[0]] || '#5a5c70';
}

const ACTIVITY_LABELS: Record<string, { label: string; color: string }> = {
  Studying: { label: '学习中', color: '#3b82f6' },
  Socializing: { label: '社交中', color: '#34d399' },
  Resting: { label: '休息中', color: '#94a3b8' },
  Reflecting: { label: '反思中', color: '#8b5cf6' },
  Activity: { label: '活动中', color: '#ec4899' },
  Moving: { label: '移动中', color: '#fbbf24' },
  Troubled: { label: '困扰中', color: '#fb7185' },
};

export function AgentList() {
  const { agents, selectedAgentId, selectAgent, generateAgents } = useSimulationStore();
  const [search, setSearch] = useState('');

  const filtered = agents.filter(a =>
    a.name.toLowerCase().includes(search.toLowerCase()) ||
    a.mbti.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <aside className="w-56 panel border-r border-border flex flex-col overflow-hidden">
      {/* Header */}
      <div className="p-3 border-b border-border-subtle">
        <div className="flex items-center justify-between mb-2">
          <h2 className="text-xs font-semibold text-text-secondary tracking-wider uppercase">Agents</h2>
          <button
            onClick={() => generateAgents(5)}
            className="p-1 rounded hover:bg-surface-overlay text-text-muted hover:text-accent-cyan transition-smooth"
            title="Generate 5 random agents"
          >
            <UserPlus size={14} />
          </button>
        </div>
        <div className="relative">
          <Search size={12} className="absolute left-2 top-1/2 -translate-y-1/2 text-text-muted" />
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search..."
            className="w-full bg-surface-overlay border border-border-subtle rounded pl-7 pr-2 py-1.5 text-xs text-text-primary placeholder:text-text-muted focus:outline-none focus:border-accent-cyan/40"
          />
        </div>
      </div>

      {/* Agent List */}
      <div className="flex-1 overflow-y-auto p-1.5 space-y-0.5">
        {filtered.length === 0 && agents.length === 0 && (
          <div className="flex flex-col items-center justify-center h-full text-text-muted text-xs gap-2 px-4 text-center">
            <p>No agents yet</p>
            <button
              onClick={() => generateAgents(5)}
              className="px-3 py-1.5 rounded bg-accent-cyan/10 text-accent-cyan hover:bg-accent-cyan/20 transition-smooth text-xs"
            >
              Generate 5 Agents
            </button>
          </div>
        )}
        {filtered.map((agent) => {
          const isSelected = agent.id === selectedAgentId;
          const activityInfo = ACTIVITY_LABELS[agent.activity] || { label: agent.activity, color: '#5a5c70' };

          return (
            <button
              key={agent.id}
              onClick={() => selectAgent(isSelected ? null : agent.id)}
              className={`w-full text-left p-2 rounded-lg transition-smooth group ${
                isSelected
                  ? 'bg-accent-cyan/10 border border-accent-cyan/30'
                  : 'hover:bg-surface-overlay border border-transparent'
              }`}
            >
              <div className="flex items-center gap-2">
                {/* Avatar */}
                <div className="relative flex-shrink-0">
                  <div
                    className="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold"
                    style={{ background: `${getMbtiColor(agent.mbti)}20`, color: getMbtiColor(agent.mbti) }}
                  >
                    {agent.name.charAt(0)}
                  </div>
                  {/* Activity indicator dot */}
                  <div
                    className="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-surface"
                    style={{ background: activityInfo.color }}
                  />
                </div>

                {/* Info */}
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-1">
                    <span className="text-xs font-medium text-text-primary truncate">{agent.name}</span>
                  </div>
                  <div className="flex items-center gap-1.5 mt-0.5">
                    <span
                      className="text-[10px] font-mono font-medium px-1 rounded"
                      style={{ background: `${getMbtiColor(agent.mbti)}15`, color: getMbtiColor(agent.mbti) }}
                    >
                      {agent.mbti}
                    </span>
                    <span className="text-[10px] text-text-muted truncate">{activityInfo.label}</span>
                  </div>
                </div>
              </div>
            </button>
          );
        })}
      </div>
    </aside>
  );
}

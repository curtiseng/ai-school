import { X, MessageCircle, Brain, Heart, Target, BookOpen, TrendingUp, Send } from 'lucide-react';
import { useState } from 'react';
import { useSimulationStore } from '../stores/simulation';
import { api } from '../api/client';

function MbtiBar({ label, value, leftLabel, rightLabel }: {
  label: string; value: number; leftLabel: string; rightLabel: string;
}) {
  const pct = ((value + 1) / 2) * 100;
  const isLeft = value < 0;

  return (
    <div className="space-y-1">
      <div className="flex justify-between text-[10px] font-mono">
        <span className={isLeft ? 'text-accent-cyan' : 'text-text-muted'}>{leftLabel}</span>
        <span className="text-text-muted">{label}</span>
        <span className={!isLeft ? 'text-accent-violet' : 'text-text-muted'}>{rightLabel}</span>
      </div>
      <div className="h-1.5 bg-surface-overlay rounded-full overflow-hidden relative">
        <div className="absolute top-0 left-1/2 w-px h-full bg-border" />
        <div
          className="h-full rounded-full absolute top-0 transition-all duration-500"
          style={{
            background: isLeft ? '#00d4ff' : '#8b5cf6',
            left: isLeft ? `${pct}%` : '50%',
            width: `${Math.abs(pct - 50)}%`,
          }}
        />
      </div>
    </div>
  );
}

function EmotionMeter({ label, value, color }: { label: string; value: number; color: string }) {
  return (
    <div className="flex items-center gap-2">
      <span className="text-[10px] text-text-muted w-8">{label}</span>
      <div className="flex-1 h-1.5 bg-surface-overlay rounded-full overflow-hidden">
        <div
          className="h-full rounded-full transition-all duration-500"
          style={{ width: `${Math.max(0, Math.min(100, (value + 1) * 50))}%`, background: color }}
        />
      </div>
      <span className="text-[10px] font-mono text-text-muted w-8 text-right">
        {value.toFixed(1)}
      </span>
    </div>
  );
}

function AbilityBar({ label, value, color }: { label: string; value: number; color: string }) {
  return (
    <div className="flex items-center gap-2">
      <span className="text-[10px] text-text-muted w-10">{label}</span>
      <div className="flex-1 h-1.5 bg-surface-overlay rounded-full overflow-hidden">
        <div
          className="h-full rounded-full transition-all duration-500"
          style={{ width: `${value * 100}%`, background: color }}
        />
      </div>
      <span className="text-[10px] font-mono text-text-muted w-6 text-right">
        {(value * 100).toFixed(0)}
      </span>
    </div>
  );
}

function DetailView() {
  const { selectedAgentDetail: agent, agents, selectedAgentId } = useSimulationStore();

  const liveAgent = agents.find(a => a.id === selectedAgentId);
  if (!liveAgent) return null;

  const personality = agent?.personality || liveAgent.personality;
  const emotion = liveAgent.emotion;
  const abilities = agent?.abilities || liveAgent.abilities;

  return (
    <div className="flex-1 overflow-y-auto p-3 space-y-4">
      {/* Header */}
      <div className="flex items-center gap-3">
        <div className="w-10 h-10 rounded-full bg-accent-cyan/10 flex items-center justify-center text-accent-cyan font-bold">
          {liveAgent.name.charAt(0)}
        </div>
        <div>
          <h3 className="font-semibold text-sm text-text-primary">{liveAgent.name}</h3>
          <div className="flex items-center gap-2 mt-0.5">
            <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-accent-cyan/10 text-accent-cyan">
              {liveAgent.mbti}
            </span>
            <span className="text-[10px] text-text-muted">{liveAgent.career}</span>
          </div>
        </div>
      </div>

      {/* Current state */}
      <div className="space-y-1.5">
        <h4 className="text-[10px] font-semibold text-text-secondary tracking-wider uppercase flex items-center gap-1.5">
          <Brain size={10} /> Current State
        </h4>
        <div className="grid grid-cols-2 gap-1.5 text-[10px]">
          <div className="bg-surface-overlay rounded p-1.5">
            <span className="text-text-muted">Location</span>
            <p className="text-text-primary font-medium mt-0.5">{liveAgent.location}</p>
          </div>
          <div className="bg-surface-overlay rounded p-1.5">
            <span className="text-text-muted">Activity</span>
            <p className="text-text-primary font-medium mt-0.5">{liveAgent.activity}</p>
          </div>
        </div>
        {liveAgent.current_thought && (
          <div className="bg-surface-overlay rounded p-2 text-[10px]">
            <span className="text-text-muted">Thought</span>
            <p className="text-text-secondary mt-0.5 italic">"{liveAgent.current_thought}"</p>
          </div>
        )}
      </div>

      {/* MBTI */}
      {personality && (
        <div className="space-y-2">
          <h4 className="text-[10px] font-semibold text-text-secondary tracking-wider uppercase flex items-center gap-1.5">
            <Target size={10} /> Personality
          </h4>
          <MbtiBar label="E/I" value={personality.e_i} leftLabel="E" rightLabel="I" />
          <MbtiBar label="S/N" value={personality.s_n} leftLabel="S" rightLabel="N" />
          <MbtiBar label="T/F" value={personality.t_f} leftLabel="T" rightLabel="F" />
          <MbtiBar label="J/P" value={personality.j_p} leftLabel="J" rightLabel="P" />
        </div>
      )}

      {/* Emotion */}
      <div className="space-y-2">
        <h4 className="text-[10px] font-semibold text-text-secondary tracking-wider uppercase flex items-center gap-1.5">
          <Heart size={10} /> Emotion
        </h4>
        <EmotionMeter label="Val" value={emotion.valence} color="#34d399" />
        <EmotionMeter label="Aro" value={emotion.arousal} color="#fbbf24" />
        <EmotionMeter label="Str" value={emotion.stress} color="#fb7185" />
      </div>

      {/* Abilities */}
      {abilities && (
        <div className="space-y-2">
          <h4 className="text-[10px] font-semibold text-text-secondary tracking-wider uppercase flex items-center gap-1.5">
            <TrendingUp size={10} /> Abilities
          </h4>
          <AbilityBar label="Academic" value={abilities.academic} color="#3b82f6" />
          <AbilityBar label="Social" value={abilities.social} color="#34d399" />
          <AbilityBar label="Resilience" value={abilities.resilience} color="#fbbf24" />
          <AbilityBar label="Creative" value={abilities.creativity} color="#ec4899" />
        </div>
      )}
    </div>
  );
}

function ChatView() {
  const { selectedAgentId, agents } = useSimulationStore();
  const [role, setRole] = useState('teacher');
  const [message, setMessage] = useState('');
  const [messages, setMessages] = useState<{ role: 'user' | 'agent'; text: string }[]>([]);
  const [loading, setLoading] = useState(false);

  const agent = agents.find(a => a.id === selectedAgentId);

  const sendMessage = async () => {
    if (!message.trim() || !selectedAgentId || loading) return;

    const userMsg = message;
    setMessages(prev => [...prev, { role: 'user', text: userMsg }]);
    setMessage('');
    setLoading(true);

    try {
      const res = await api.chat(selectedAgentId, role, userMsg);
      setMessages(prev => [...prev, { role: 'agent', text: res.reply }]);
    } catch {
      setMessages(prev => [...prev, { role: 'agent', text: '(Chat API not available yet)' }]);
    }
    setLoading(false);
  };

  const ROLES = [
    { id: 'teacher', label: '老师' },
    { id: 'principal', label: '校长' },
    { id: 'counselor', label: '辅导员' },
  ];

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {/* Role selector */}
      <div className="p-2 border-b border-border-subtle flex gap-1">
        {ROLES.map(r => (
          <button
            key={r.id}
            onClick={() => setRole(r.id)}
            className={`px-2 py-1 rounded text-[10px] font-medium transition-smooth ${
              role === r.id
                ? 'bg-accent-violet/15 text-accent-violet'
                : 'text-text-muted hover:text-text-secondary'
            }`}
          >
            {r.label}
          </button>
        ))}
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-2 space-y-2">
        {messages.length === 0 && (
          <div className="flex items-center justify-center h-full text-text-muted text-xs text-center px-4">
            <p>Start a conversation with {agent?.name || 'the agent'} as {ROLES.find(r => r.id === role)?.label}</p>
          </div>
        )}
        {messages.map((msg, i) => (
          <div
            key={i}
            className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
          >
            <div
              className={`max-w-[85%] rounded-lg px-2.5 py-1.5 text-xs ${
                msg.role === 'user'
                  ? 'bg-accent-violet/15 text-text-primary'
                  : 'bg-surface-overlay text-text-secondary'
              }`}
            >
              {msg.text}
            </div>
          </div>
        ))}
        {loading && (
          <div className="flex justify-start">
            <div className="bg-surface-overlay rounded-lg px-3 py-2 text-xs text-text-muted">
              Thinking...
            </div>
          </div>
        )}
      </div>

      {/* Input */}
      <div className="p-2 border-t border-border-subtle">
        <div className="flex gap-1.5">
          <input
            type="text"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && sendMessage()}
            placeholder="Send a message..."
            className="flex-1 bg-surface-overlay border border-border-subtle rounded px-2.5 py-1.5 text-xs text-text-primary placeholder:text-text-muted focus:outline-none focus:border-accent-violet/40"
          />
          <button
            onClick={sendMessage}
            disabled={loading || !message.trim()}
            className="p-1.5 rounded bg-accent-violet/15 text-accent-violet hover:bg-accent-violet/25 disabled:opacity-30 transition-smooth"
          >
            <Send size={12} />
          </button>
        </div>
      </div>
    </div>
  );
}

export function DetailPanel() {
  const { selectedAgentId, selectAgent, rightPanel, setRightPanel } = useSimulationStore();

  if (!selectedAgentId) {
    return (
      <aside className="w-64 panel border-l border-border flex items-center justify-center">
        <div className="text-center text-text-muted text-xs px-4">
          <BookOpen size={20} className="mx-auto mb-2 opacity-50" />
          <p>Select an agent to view details</p>
        </div>
      </aside>
    );
  }

  return (
    <aside className="w-64 panel border-l border-border flex flex-col overflow-hidden">
      {/* Tabs */}
      <div className="flex items-center border-b border-border-subtle">
        <button
          onClick={() => setRightPanel('detail')}
          className={`flex-1 py-2 text-[10px] font-semibold tracking-wider uppercase transition-smooth ${
            rightPanel === 'detail'
              ? 'text-accent-cyan border-b-2 border-accent-cyan'
              : 'text-text-muted hover:text-text-secondary'
          }`}
        >
          Detail
        </button>
        <button
          onClick={() => setRightPanel('chat')}
          className={`flex-1 py-2 text-[10px] font-semibold tracking-wider uppercase transition-smooth flex items-center justify-center gap-1 ${
            rightPanel === 'chat'
              ? 'text-accent-violet border-b-2 border-accent-violet'
              : 'text-text-muted hover:text-text-secondary'
          }`}
        >
          <MessageCircle size={10} /> Chat
        </button>
        <button
          onClick={() => selectAgent(null)}
          className="p-2 text-text-muted hover:text-text-secondary transition-smooth"
        >
          <X size={12} />
        </button>
      </div>

      {rightPanel === 'detail' ? <DetailView /> : <ChatView />}
    </aside>
  );
}

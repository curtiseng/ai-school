import { useEffect, useRef, useState } from 'react';
import { X, Zap } from 'lucide-react';
import { useSimulationStore } from '../stores/simulation';
import type { SimulationEvent } from '../types';

const EVENT_TYPE_STYLES: Record<string, { bg: string; border: string; icon: string }> = {
  Routine: { bg: 'bg-slate-500/10', border: 'border-slate-500/30', icon: '📋' },
  SocialInteraction: { bg: 'bg-emerald-500/10', border: 'border-emerald-500/30', icon: '💬' },
  Academic: { bg: 'bg-blue-500/10', border: 'border-blue-500/30', icon: '📚' },
  Conflict: { bg: 'bg-red-500/10', border: 'border-red-500/30', icon: '⚡' },
  Cooperation: { bg: 'bg-amber-500/10', border: 'border-amber-500/30', icon: '🤝' },
  SpecialEvent: { bg: 'bg-purple-500/10', border: 'border-purple-500/30', icon: '🌟' },
  System: { bg: 'bg-cyan-500/10', border: 'border-cyan-500/30', icon: '⚙️' },
  Intervention: { bg: 'bg-orange-500/10', border: 'border-orange-500/30', icon: '🎯' },
};

const EVENT_TYPE_LABELS: Record<string, string> = {
  Routine: '日常',
  SocialInteraction: '社交',
  Academic: '学业',
  Conflict: '冲突',
  Cooperation: '合作',
  SpecialEvent: '特殊事件',
  System: '系统',
  Intervention: '干预',
};

interface ToastItem {
  id: string;
  event: SimulationEvent;
  entering: boolean;
  exiting: boolean;
}

export function EventToast() {
  const { eventLog, running } = useSimulationStore();
  const [toasts, setToasts] = useState<ToastItem[]>([]);
  const prevLengthRef = useRef(0);

  useEffect(() => {
    if (!running) return;

    const prevLen = prevLengthRef.current;
    const newLen = eventLog.length;

    if (newLen > prevLen && prevLen > 0) {
      const newEvents = eventLog.slice(prevLen);
      const newToasts: ToastItem[] = newEvents.map((evt) => ({
        id: evt.id || `${evt.timestamp.tick}-${Math.random()}`,
        event: evt,
        entering: true,
        exiting: false,
      }));

      setToasts((prev) => [...prev, ...newToasts].slice(-5));

      // Remove entering state after animation
      setTimeout(() => {
        setToasts((prev) =>
          prev.map((t) =>
            newToasts.some((nt) => nt.id === t.id) ? { ...t, entering: false } : t
          )
        );
      }, 50);

      // Auto-dismiss after 6 seconds
      newToasts.forEach((toast) => {
        setTimeout(() => {
          setToasts((prev) =>
            prev.map((t) => (t.id === toast.id ? { ...t, exiting: true } : t))
          );
          setTimeout(() => {
            setToasts((prev) => prev.filter((t) => t.id !== toast.id));
          }, 300);
        }, 6000);
      });
    }

    prevLengthRef.current = newLen;
  }, [eventLog, running]);

  const dismiss = (id: string) => {
    setToasts((prev) =>
      prev.map((t) => (t.id === id ? { ...t, exiting: true } : t))
    );
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 300);
  };

  if (toasts.length === 0) return null;

  return (
    <div className="fixed top-14 right-4 z-50 flex flex-col gap-2 pointer-events-none" style={{ maxWidth: '400px' }}>
      {toasts.map((toast) => {
        const style = EVENT_TYPE_STYLES[toast.event.event_type] || EVENT_TYPE_STYLES.Routine;
        const label = EVENT_TYPE_LABELS[toast.event.event_type] || toast.event.event_type;

        return (
          <div
            key={toast.id}
            className={`pointer-events-auto rounded-lg border shadow-lg backdrop-blur-sm
              ${style.bg} ${style.border}
              transition-all duration-300 ease-out
              ${toast.entering ? 'translate-x-full opacity-0' : toast.exiting ? 'translate-x-full opacity-0' : 'translate-x-0 opacity-100'}
            `}
          >
            <div className="px-4 py-3">
              {/* Header */}
              <div className="flex items-center justify-between mb-1.5">
                <div className="flex items-center gap-2">
                  <span className="text-base">{style.icon}</span>
                  <span className="text-xs font-semibold text-text-primary">{label}</span>
                  <span className="text-[10px] font-mono text-accent-cyan bg-accent-cyan/10 px-1.5 py-0.5 rounded">
                    T{toast.event.timestamp.tick}
                  </span>
                </div>
                <button
                  onClick={() => dismiss(toast.id)}
                  className="p-0.5 rounded hover:bg-white/10 text-text-muted hover:text-text-primary transition-smooth"
                >
                  <X size={12} />
                </button>
              </div>
              {/* Narrative */}
              <p className="text-sm text-text-secondary leading-relaxed">
                {toast.event.narrative}
              </p>
              {/* Involved agents */}
              {toast.event.involved_agents && toast.event.involved_agents.length > 0 && (
                <div className="mt-1.5 flex items-center gap-1">
                  <Zap size={10} className="text-text-muted" />
                  <span className="text-[10px] text-text-muted">
                    {toast.event.involved_agents.length} 位学生参与
                  </span>
                </div>
              )}
            </div>
          </div>
        );
      })}
    </div>
  );
}

import { useState } from 'react';
import { Sliders, Zap, FileDown, ChevronUp, ChevronDown } from 'lucide-react';
import { useSimulationStore } from '../stores/simulation';
import { api } from '../api/client';

const PRESET_EVENTS = [
  { label: '期中考试', value: 'MidtermExam' },
  { label: '社团招新', value: 'ClubRecruitment' },
  { label: '运动会', value: 'SportsMeet' },
];

function ParamSlider({ label, value, onChange }: {
  label: string; value: number; onChange: (v: number) => void;
}) {
  return (
    <div className="flex items-center gap-2">
      <span className="text-[10px] text-text-muted w-16 shrink-0">{label}</span>
      <input
        type="range"
        min={0}
        max={100}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
        className="flex-1 h-1 appearance-none bg-surface-overlay rounded-full cursor-pointer
          [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2.5 [&::-webkit-slider-thumb]:h-2.5
          [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-accent-cyan [&::-webkit-slider-thumb]:cursor-pointer"
      />
      <span className="text-[10px] font-mono text-text-muted w-6 text-right">{value}</span>
    </div>
  );
}

export function BottomBar() {
  const { eventLog } = useSimulationStore();
  const [expanded, setExpanded] = useState(false);
  const [params, setParams] = useState({
    difficulty: 50,
    socialDensity: 50,
    competitivePressure: 50,
    randomEvents: 30,
  });

  const handleTriggerEvent = async (eventType: string) => {
    try {
      await api.triggerEvent(eventType as never);
    } catch (e) {
      console.error('Failed to trigger event:', e);
    }
  };

  const handleExport = async () => {
    try {
      const data = await api.exportData();
      const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `ai-school-export-${Date.now()}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      console.error('Export failed:', e);
    }
  };

  return (
    <footer className={`panel border-t border-border transition-all duration-300 ${expanded ? 'h-44' : 'h-10'}`}>
      {/* Toggle bar */}
      <div
        className="h-10 flex items-center px-4 gap-4 cursor-pointer"
        onClick={() => setExpanded(!expanded)}
      >
        <div className="flex items-center gap-1.5 text-xs text-text-secondary">
          <Sliders size={12} />
          <span className="font-medium">Intervention</span>
        </div>

        {/* Quick event buttons (visible when collapsed) */}
        {!expanded && (
          <div className="flex items-center gap-1">
            {PRESET_EVENTS.map((evt) => (
              <button
                key={evt.value}
                onClick={(e) => { e.stopPropagation(); handleTriggerEvent(evt.value); }}
                className="px-2 py-0.5 rounded text-[10px] bg-surface-overlay text-text-muted hover:text-accent-amber hover:bg-accent-amber/10 transition-smooth"
              >
                <Zap size={8} className="inline mr-0.5" />
                {evt.label}
              </button>
            ))}
          </div>
        )}

        {/* Recent event */}
        {!expanded && eventLog.length > 0 && (
          <div className="flex-1 text-[10px] text-text-muted truncate font-mono">
            T{eventLog[eventLog.length - 1].timestamp.tick}: {eventLog[eventLog.length - 1].narrative.slice(0, 60)}
          </div>
        )}

        <div className="flex items-center gap-2 ml-auto">
          <button
            onClick={(e) => { e.stopPropagation(); handleExport(); }}
            className="p-1 rounded text-text-muted hover:text-accent-cyan transition-smooth"
            title="Export data"
          >
            <FileDown size={12} />
          </button>
          {expanded ? <ChevronDown size={12} className="text-text-muted" /> : <ChevronUp size={12} className="text-text-muted" />}
        </div>
      </div>

      {/* Expanded content */}
      {expanded && (
        <div className="px-4 pb-3 flex gap-6">
          {/* Parameters */}
          <div className="flex-1 space-y-2">
            <h4 className="text-[10px] font-semibold text-text-secondary tracking-wider uppercase">Parameters</h4>
            <ParamSlider label="Difficulty" value={params.difficulty} onChange={(v) => setParams(p => ({ ...p, difficulty: v }))} />
            <ParamSlider label="Social" value={params.socialDensity} onChange={(v) => setParams(p => ({ ...p, socialDensity: v }))} />
            <ParamSlider label="Competition" value={params.competitivePressure} onChange={(v) => setParams(p => ({ ...p, competitivePressure: v }))} />
            <ParamSlider label="Random" value={params.randomEvents} onChange={(v) => setParams(p => ({ ...p, randomEvents: v }))} />
          </div>

          {/* Events */}
          <div className="w-40 space-y-2">
            <h4 className="text-[10px] font-semibold text-text-secondary tracking-wider uppercase">Trigger Event</h4>
            <div className="space-y-1">
              {PRESET_EVENTS.map((evt) => (
                <button
                  key={evt.value}
                  onClick={() => handleTriggerEvent(evt.value)}
                  className="w-full text-left px-2 py-1.5 rounded text-[10px] bg-surface-overlay text-text-secondary hover:text-accent-amber hover:bg-accent-amber/10 transition-smooth"
                >
                  <Zap size={10} className="inline mr-1" />
                  {evt.label}
                </button>
              ))}
            </div>
          </div>

          {/* Event log */}
          <div className="w-56 space-y-2">
            <h4 className="text-[10px] font-semibold text-text-secondary tracking-wider uppercase">Event Log</h4>
            <div className="space-y-0.5 max-h-24 overflow-y-auto">
              {eventLog.slice(-10).reverse().map((evt, i) => (
                <div key={i} className="text-[9px] font-mono text-text-muted py-0.5 border-b border-border-subtle">
                  <span className="text-accent-cyan">T{evt.timestamp.tick}</span>{' '}
                  <span className="text-text-secondary">{evt.narrative.slice(0, 50)}</span>
                </div>
              ))}
              {eventLog.length === 0 && (
                <p className="text-[10px] text-text-muted">No events yet</p>
              )}
            </div>
          </div>
        </div>
      )}
    </footer>
  );
}

import { create } from 'zustand';
import type {
  Agent, AgentDetail, SimulationTime, SimulationSpeed,
  SimulationEvent, SimulationUpdate, WorldSnapshot,
} from '../types';
import { api } from '../api/client';

interface SimulationStore {
  // Connection
  connected: boolean;
  ws: WebSocket | null;

  // Simulation state
  running: boolean;
  speed: SimulationSpeed;
  time: SimulationTime | null;
  tick: number;

  // World
  agents: Agent[];
  snapshot: WorldSnapshot | null;
  events: SimulationEvent[];
  eventLog: SimulationEvent[];

  // UI
  selectedAgentId: string | null;
  selectedAgentDetail: AgentDetail | null;
  rightPanel: 'detail' | 'chat';

  // Actions
  connect: () => void;
  disconnect: () => void;
  fetchAgents: () => Promise<void>;
  fetchStatus: () => Promise<void>;
  selectAgent: (id: string | null) => void;
  setRightPanel: (panel: 'detail' | 'chat') => void;

  // Simulation control
  startSimulation: () => Promise<void>;
  stopSimulation: () => Promise<void>;
  stepSimulation: () => Promise<void>;
  setSpeed: (speed: SimulationSpeed) => Promise<void>;

  // Agent
  generateAgents: (count: number) => Promise<void>;
}

export const useSimulationStore = create<SimulationStore>((set, get) => ({
  connected: false,
  ws: null,
  running: false,
  speed: 'Paused',
  time: null,
  tick: 0,
  agents: [],
  snapshot: null,
  events: [],
  eventLog: [],
  selectedAgentId: null,
  selectedAgentDetail: null,
  rightPanel: 'detail',

  connect: () => {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/ws/simulation`;
    const ws = new WebSocket(wsUrl);

    ws.onopen = () => {
      set({ connected: true, ws });
    };

    ws.onclose = () => {
      set({ connected: false, ws: null });
      // Reconnect after delay
      setTimeout(() => get().connect(), 2000);
    };

    ws.onmessage = (event) => {
      try {
        const update: SimulationUpdate = JSON.parse(event.data);
        const state = get();

        switch (update.type) {
          case 'Tick': {
            // Convert snapshot agents to array
            const agentEntries = update.snapshot.agents
              ? Object.values(update.snapshot.agents)
              : [];

            const agents: Agent[] = agentEntries.map((a) => ({
              id: typeof a.id === 'string' ? a.id : (a.id as { Uuid?: string })?.Uuid || String(a.id),
              name: a.config.name,
              mbti: a.config.personality.mbti || '',
              location: typeof a.location === 'string' ? a.location : String(a.location),
              activity: a.activity as Agent['activity'],
              emotion: a.emotion,
              career: a.config.career_aspiration.ideal_career,
              personality: a.config.personality,
              abilities: a.abilities,
              current_thought: a.current_thought,
            }));

            set({
              time: update.time,
              tick: update.time.tick,
              snapshot: update.snapshot,
              agents,
              events: update.events,
              eventLog: [...state.eventLog, ...update.events].slice(-100),
            });
            break;
          }
          case 'SpeedChanged':
            set({ speed: update.speed });
            break;
          case 'Started':
            set({ running: true });
            break;
          case 'Stopped':
            set({ running: false });
            break;
        }
      } catch (e) {
        console.warn('Failed to parse WS message:', e);
      }
    };

    set({ ws });
  },

  disconnect: () => {
    const { ws } = get();
    if (ws) ws.close();
    set({ ws: null, connected: false });
  },

  fetchAgents: async () => {
    try {
      const { agents } = await api.listAgents();
      set({ agents });
    } catch (e) {
      console.error('Failed to fetch agents:', e);
    }
  },

  fetchStatus: async () => {
    try {
      const status = await api.getStatus();
      set({
        running: status.running,
        tick: status.tick,
        speed: status.speed,
      });
    } catch (e) {
      console.error('Failed to fetch status:', e);
    }
  },

  selectAgent: async (id) => {
    set({ selectedAgentId: id, selectedAgentDetail: null });
    if (id) {
      try {
        const detail = await api.getAgent(id);
        set({ selectedAgentDetail: detail });
      } catch (e) {
        console.error('Failed to fetch agent detail:', e);
      }
    }
  },

  setRightPanel: (panel) => set({ rightPanel: panel }),

  startSimulation: async () => {
    await api.start();
    set({ running: true });
  },

  stopSimulation: async () => {
    await api.stop();
    set({ running: false });
  },

  stepSimulation: async () => {
    const result = await api.step();
    if (result.success) {
      set({ tick: result.tick });
      await get().fetchAgents();
    }
  },

  setSpeed: async (speed) => {
    await api.setSpeed(speed);
    set({ speed });
  },

  generateAgents: async (count) => {
    await api.generateAgents(count);
    await get().fetchAgents();
  },
}));

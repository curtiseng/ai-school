import type { Agent, AgentDetail, SimulationStatus, PresetEvent } from '../types';

const BASE = '';

async function request<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${url}`, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  });
  if (!res.ok) throw new Error(`API error: ${res.status}`);
  return res.json();
}

export const api = {
  // Simulation
  getStatus: () => request<SimulationStatus>('/api/simulation/status'),
  start: () => request<{ success: boolean }>('/api/simulation/start', { method: 'POST' }),
  stop: () => request<{ success: boolean }>('/api/simulation/stop', { method: 'POST' }),
  step: () => request<{ success: boolean; tick: number; events: number; warnings: string[] }>(
    '/api/simulation/step', { method: 'POST' }
  ),
  setSpeed: (speed: string) => request<{ success: boolean }>(
    '/api/simulation/speed', { method: 'PUT', body: JSON.stringify({ speed }) }
  ),

  // Agents
  listAgents: () => request<{ agents: Agent[] }>('/api/agents'),
  getAgent: (id: string) => request<AgentDetail>(`/api/agents/${id}`),
  createAgent: (data: {
    name: string; e_i: number; s_n: number; t_f: number; j_p: number;
    ideal_career?: string; age?: number;
  }) => request<{ success: boolean }>('/api/agents', { method: 'POST', body: JSON.stringify(data) }),
  generateAgents: (count: number) => request<{ success: boolean }>(
    '/api/agents/generate', { method: 'POST', body: JSON.stringify({ count }) }
  ),

  // Intervention
  triggerEvent: (event: PresetEvent) => request<{ success: boolean }>(
    '/api/interventions/event', { method: 'POST', body: JSON.stringify({ event }) }
  ),

  // Chat
  chat: (agentId: string, role: string, message: string) => request<{ reply: string; impact: string }>(
    `/api/agents/${agentId}/chat`, { method: 'POST', body: JSON.stringify({ role, message }) }
  ),

  // Analysis
  getSnapshot: () => request<Record<string, unknown>>('/api/analysis/snapshot'),
  getEvents: () => request<{ events: unknown[] }>('/api/analysis/events'),
  exportData: () => request<Record<string, unknown>>('/api/analysis/export'),
};

// Core IDs
export type AgentId = string;
export type LocationId = string;
export type EventId = string;

// Simulation Time
export interface SimulationTime {
  semester: number;
  week: number;
  day_of_week: number;
  hour: number;
  tick: number;
}

// Personality
export interface PersonalityParams {
  e_i: number;
  s_n: number;
  t_f: number;
  j_p: number;
  stability: number;
  mbti?: string;
}

// Emotion
export interface EmotionalState {
  valence: number;
  arousal: number;
  stress: number;
}

// Abilities
export interface AbilityMetrics {
  academic: number;
  social: number;
  resilience: number;
  creativity: number;
}

// Agent
export type AgentActivity =
  | 'Studying'
  | 'Socializing'
  | 'Resting'
  | 'Reflecting'
  | 'Activity'
  | 'Moving'
  | 'Troubled';

export interface Agent {
  id: AgentId;
  name: string;
  mbti: string;
  location: LocationId;
  activity: AgentActivity;
  emotion: EmotionalState;
  career: string;
  personality?: PersonalityParams;
  abilities?: AbilityMetrics;
  current_thought?: string;
}

export interface AgentDetail extends Agent {
  personality: PersonalityParams;
  abilities: AbilityMetrics;
  current_thought: string;
}

// Location
export type LocationType =
  | 'Classroom'
  | 'Library'
  | 'Playground'
  | 'Cafeteria'
  | 'Dormitory'
  | 'ClubRoom'
  | 'Auditorium'
  | 'StudyRoom'
  | 'RestArea';

export interface Location {
  id: LocationId;
  name: string;
  location_type: LocationType | { Classroom: { subject: string | null } } | { ClubRoom: { club_name: string | null } };
  capacity: number;
  position: [number, number];
  adjacent: LocationId[];
}

// Events
export type EventType =
  | 'Routine'
  | 'SocialInteraction'
  | 'Academic'
  | 'Conflict'
  | 'Cooperation'
  | 'SpecialEvent'
  | 'System'
  | 'Intervention';

export interface SimulationEvent {
  id: EventId;
  event_type: EventType;
  timestamp: SimulationTime;
  involved_agents: AgentId[];
  narrative: string;
  intensity: number;
}

// Simulation Speed
export type SimulationSpeed =
  | 'Paused'
  | 'Normal'
  | 'Fast'
  | 'VeryFast'
  | 'Maximum'
  | 'Unlimited';

// Relationship
export interface Relationship {
  agent_a: AgentId;
  agent_b: AgentId;
  closeness: number;
  trust: number;
}

// World Snapshot
export interface WorldSnapshot {
  time: SimulationTime;
  agents: Record<string, {
    id: AgentId;
    config: { name: string; personality: PersonalityParams; career_aspiration: { ideal_career: string } };
    location: LocationId;
    activity: AgentActivity;
    emotion: EmotionalState;
    abilities: AbilityMetrics;
    current_thought: string;
  }>;
  relationships: Relationship[];
  active_events: string[];
}

// WebSocket updates
export type SimulationUpdate =
  | { type: 'Tick'; time: SimulationTime; snapshot: WorldSnapshot; events: SimulationEvent[] }
  | { type: 'SpeedChanged'; speed: SimulationSpeed }
  | { type: 'Started' }
  | { type: 'Stopped' };

// Simulation status
export interface SimulationStatus {
  running: boolean;
  tick: number;
  time_display: string;
  agent_count: number;
  speed: SimulationSpeed;
}

// Preset events
export type PresetEvent =
  | 'MidtermExam'
  | 'ClubRecruitment'
  | 'SportsMeet'
  | { FriendshipConflict: { agent_a: AgentId; agent_b: AgentId } }
  | { TeacherPraise: { target: AgentId } }
  | { TeacherCriticism: { target: AgentId } }
  | { NewStudent: { name: string } }
  | { Custom: { description: string } };

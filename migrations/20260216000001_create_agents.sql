-- Agent 表
CREATE TABLE IF NOT EXISTS agents (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    personality JSONB NOT NULL,
    career_aspiration JSONB NOT NULL,
    background TEXT,
    age SMALLINT NOT NULL DEFAULT 16,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    simulation_id UUID NOT NULL
);

-- Agent 状态表（运行时状态）
CREATE TABLE IF NOT EXISTS agent_states (
    agent_id UUID PRIMARY KEY REFERENCES agents(id),
    location VARCHAR(255) NOT NULL DEFAULT 'dormitory',
    activity JSONB NOT NULL DEFAULT '{"type": "Resting"}',
    emotion JSONB NOT NULL DEFAULT '{"valence": 0.3, "arousal": 0.3, "stress": 0.2}',
    abilities JSONB NOT NULL DEFAULT '{"academic": 0.5, "social": 0.5, "resilience": 0.5, "creativity": 0.5}',
    current_thought TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_agents_simulation ON agents(simulation_id);

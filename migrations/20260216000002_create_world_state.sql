-- 仿真会话表
CREATE TABLE IF NOT EXISTS simulations (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    config JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'created',
    current_tick BIGINT NOT NULL DEFAULT 0,
    current_time JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 关系矩阵表
CREATE TABLE IF NOT EXISTS relationships (
    agent_a UUID NOT NULL REFERENCES agents(id),
    agent_b UUID NOT NULL REFERENCES agents(id),
    closeness REAL NOT NULL DEFAULT 0.0,
    trust REAL NOT NULL DEFAULT 0.5,
    tags TEXT[] DEFAULT '{}',
    last_interaction TIMESTAMPTZ,
    simulation_id UUID NOT NULL,
    PRIMARY KEY (agent_a, agent_b, simulation_id)
);

-- 事件日志表
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY,
    simulation_id UUID NOT NULL REFERENCES simulations(id),
    event_type VARCHAR(50) NOT NULL,
    trigger_type VARCHAR(50) NOT NULL,
    tick BIGINT NOT NULL,
    simulation_time JSONB NOT NULL,
    involved_agents UUID[] NOT NULL DEFAULT '{}',
    narrative TEXT NOT NULL,
    state_changes JSONB NOT NULL DEFAULT '[]',
    intensity REAL NOT NULL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_events_simulation ON events(simulation_id, tick);
CREATE INDEX idx_events_type ON events(event_type);

-- 快照表
CREATE TABLE IF NOT EXISTS snapshots (
    id UUID PRIMARY KEY,
    simulation_id UUID NOT NULL REFERENCES simulations(id),
    tick BIGINT NOT NULL,
    world_state JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_snapshots_simulation ON snapshots(simulation_id, tick);

-- 干预日志表
CREATE TABLE IF NOT EXISTS intervention_logs (
    id UUID PRIMARY KEY,
    simulation_id UUID NOT NULL REFERENCES simulations(id),
    tick BIGINT NOT NULL,
    intervention_type VARCHAR(50) NOT NULL,
    details JSONB NOT NULL,
    affected_agents UUID[] NOT NULL DEFAULT '{}',
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_interventions_simulation ON intervention_logs(simulation_id, tick);

import { Stage, Layer, Rect, Circle, Text, Group, Line } from 'react-konva';
import { useEffect, useRef, useState, useCallback } from 'react';
import { useSimulationStore } from '../stores/simulation';
import type { Agent } from '../types';

// Campus locations with coordinates from backend (scaled to canvas)
const LOCATIONS = [
  { id: 'classroom_math', name: '数学教室', type: 'Classroom', x: 200, y: 100, w: 100, h: 70, color: '#3b82f6' },
  { id: 'classroom_chinese', name: '语文教室', type: 'Classroom', x: 350, y: 100, w: 100, h: 70, color: '#3b82f6' },
  { id: 'classroom_english', name: '英语教室', type: 'Classroom', x: 500, y: 100, w: 100, h: 70, color: '#3b82f6' },
  { id: 'classroom_science', name: '实验室', type: 'Classroom', x: 200, y: 200, w: 100, h: 70, color: '#3b82f6' },
  { id: 'library', name: '图书馆', type: 'Library', x: 650, y: 130, w: 110, h: 80, color: '#8b5cf6' },
  { id: 'study_room', name: '自习室', type: 'StudyRoom', x: 650, y: 240, w: 90, h: 60, color: '#6366f1' },
  { id: 'playground', name: '操场', type: 'Playground', x: 350, y: 370, w: 160, h: 110, color: '#34d399' },
  { id: 'cafeteria', name: '食堂', type: 'Cafeteria', x: 170, y: 370, w: 120, h: 90, color: '#fb923c' },
  { id: 'dormitory', name: '宿舍', type: 'Dormitory', x: 60, y: 470, w: 130, h: 80, color: '#64748b' },
  { id: 'club_room', name: '社团活动室', type: 'ClubRoom', x: 500, y: 280, w: 100, h: 65, color: '#ec4899' },
  { id: 'auditorium', name: '礼堂', type: 'Auditorium', x: 640, y: 340, w: 120, h: 80, color: '#f59e0b' },
  { id: 'rest_area', name: '休息区', type: 'RestArea', x: 350, y: 275, w: 90, h: 60, color: '#94a3b8' },
  { id: 'hallway', name: '走廊', type: 'RestArea', x: 300, y: 190, w: 140, h: 50, color: '#2a2b3a' },
];

const MBTI_COLORS: Record<string, string> = {
  E: '#00d4ff', I: '#8b5cf6', S: '#fb923c', N: '#34d399',
  T: '#3b82f6', F: '#fb7185', J: '#fbbf24', P: '#6366f1',
};

function getAgentColor(mbti: string): string {
  return MBTI_COLORS[mbti?.[0]] || '#5a5c70';
}

// Calculate agent position within a location (distribute evenly)
function getAgentPosition(
  locationId: string,
  agentIndex: number,
  totalInLocation: number,
): { x: number; y: number } {
  const loc = LOCATIONS.find(l => l.id === locationId);
  if (!loc) return { x: 400, y: 300 };

  const cx = loc.x + loc.w / 2;
  const cy = loc.y + loc.h / 2;

  if (totalInLocation <= 1) return { x: cx, y: cy };

  const cols = Math.ceil(Math.sqrt(totalInLocation));
  const row = Math.floor(agentIndex / cols);
  const col = agentIndex % cols;
  const spacing = 22;
  const startX = cx - ((cols - 1) * spacing) / 2;
  const startY = cy - ((Math.ceil(totalInLocation / cols) - 1) * spacing) / 2;

  return {
    x: startX + col * spacing,
    y: startY + row * spacing,
  };
}

interface AgentNodeProps {
  agent: Agent;
  x: number;
  y: number;
  isSelected: boolean;
  onSelect: () => void;
}

function AgentNode({ agent, x, y, isSelected, onSelect }: AgentNodeProps) {
  const color = getAgentColor(agent.mbti);
  const [hover, setHover] = useState(false);

  return (
    <Group
      x={x}
      y={y}
      onClick={onSelect}
      onTap={onSelect}
      onMouseEnter={() => setHover(true)}
      onMouseLeave={() => setHover(false)}
    >
      {/* Selection ring */}
      {isSelected && (
        <Circle radius={14} fill="transparent" stroke={color} strokeWidth={2} opacity={0.6} />
      )}
      {/* Agent circle */}
      <Circle
        radius={hover ? 11 : 10}
        fill={`${color}30`}
        stroke={color}
        strokeWidth={1.5}
      />
      {/* Initial */}
      <Text
        text={agent.name.charAt(0)}
        fontSize={10}
        fontFamily="Outfit"
        fontStyle="bold"
        fill={color}
        align="center"
        verticalAlign="middle"
        offsetX={4}
        offsetY={5}
      />
      {/* Name label */}
      {(hover || isSelected) && (
        <>
          <Rect
            x={-24}
            y={14}
            width={48}
            height={16}
            fill="rgba(10, 11, 15, 0.85)"
            cornerRadius={3}
          />
          <Text
            text={agent.name}
            fontSize={9}
            fontFamily="Outfit"
            fill="#e8e9f0"
            x={-24}
            y={16}
            width={48}
            align="center"
          />
        </>
      )}
      {/* Thought bubble */}
      {(hover || isSelected) && agent.current_thought && (
        <>
          <Rect
            x={-60}
            y={-35}
            width={120}
            height={18}
            fill="rgba(10, 11, 15, 0.9)"
            stroke={`${color}40`}
            strokeWidth={0.5}
            cornerRadius={4}
          />
          <Text
            text={agent.current_thought.slice(0, 18) + (agent.current_thought.length > 18 ? '...' : '')}
            fontSize={8}
            fontFamily="JetBrains Mono"
            fill="#8b8da0"
            x={-56}
            y={-32}
            width={112}
            align="center"
          />
        </>
      )}
    </Group>
  );
}

export function CampusMap() {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });
  const { agents, selectedAgentId, selectAgent } = useSimulationStore();

  const handleResize = useCallback(() => {
    if (containerRef.current) {
      setDimensions({
        width: containerRef.current.offsetWidth,
        height: containerRef.current.offsetHeight,
      });
    }
  }, []);

  useEffect(() => {
    handleResize();
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, [handleResize]);

  // Group agents by location
  const agentsByLocation: Record<string, Agent[]> = {};
  for (const agent of agents) {
    const locId = agent.location;
    if (!agentsByLocation[locId]) agentsByLocation[locId] = [];
    agentsByLocation[locId].push(agent);
  }

  // Calculate scale
  const mapWidth = 820;
  const mapHeight = 600;
  const scaleX = dimensions.width / mapWidth;
  const scaleY = dimensions.height / mapHeight;
  const scale = Math.min(scaleX, scaleY, 1.2);
  const offsetX = (dimensions.width - mapWidth * scale) / 2;
  const offsetY = (dimensions.height - mapHeight * scale) / 2;

  // Find socializing agent pairs for connection lines
  const socializingPairs: { a: Agent; b: Agent; ax: number; ay: number; bx: number; by: number }[] = [];
  for (const [locId, locAgents] of Object.entries(agentsByLocation)) {
    const socializing = locAgents.filter(a => a.activity === 'Socializing');
    for (let i = 0; i < socializing.length - 1; i += 2) {
      const a = socializing[i];
      const b = socializing[i + 1];
      const aIdx = locAgents.indexOf(a);
      const bIdx = locAgents.indexOf(b);
      const aPos = getAgentPosition(locId, aIdx, locAgents.length);
      const bPos = getAgentPosition(locId, bIdx, locAgents.length);
      socializingPairs.push({ a, b, ax: aPos.x, ay: aPos.y, bx: bPos.x, by: bPos.y });
    }
  }

  return (
    <div ref={containerRef} className="flex-1 relative grid-bg noise-overlay overflow-hidden">
      <Stage
        width={dimensions.width}
        height={dimensions.height}
        offsetX={-offsetX}
        offsetY={-offsetY}
        scaleX={scale}
        scaleY={scale}
      >
        <Layer>
          {/* Location areas */}
          {LOCATIONS.map((loc) => (
            <Group key={loc.id}>
              <Rect
                x={loc.x}
                y={loc.y}
                width={loc.w}
                height={loc.h}
                fill={`${loc.color}08`}
                stroke={`${loc.color}30`}
                strokeWidth={1}
                cornerRadius={6}
              />
              <Text
                text={loc.name}
                fontSize={10}
                fontFamily="Outfit"
                fill={`${loc.color}80`}
                x={loc.x}
                y={loc.y + 4}
                width={loc.w}
                align="center"
              />
              {/* Agent count badge */}
              {(agentsByLocation[loc.id]?.length ?? 0) > 0 && (
                <>
                  <Circle
                    x={loc.x + loc.w - 4}
                    y={loc.y + 4}
                    radius={7}
                    fill={loc.color}
                    opacity={0.8}
                  />
                  <Text
                    text={String(agentsByLocation[loc.id]?.length ?? 0)}
                    fontSize={8}
                    fontFamily="JetBrains Mono"
                    fontStyle="bold"
                    fill="#0a0b0f"
                    x={loc.x + loc.w - 8}
                    y={loc.y}
                    width={8}
                    align="center"
                  />
                </>
              )}
            </Group>
          ))}

          {/* Social connection lines */}
          {socializingPairs.map(({ a, b, ax, ay, bx, by }, i) => (
            <Line
              key={`line-${i}`}
              points={[ax, ay, bx, by]}
              stroke="#34d39940"
              strokeWidth={1}
              dash={[3, 3]}
            />
          ))}

          {/* Agent nodes */}
          {Object.entries(agentsByLocation).flatMap(([locId, locAgents]) =>
            locAgents.map((agent, idx) => {
              const pos = getAgentPosition(locId, idx, locAgents.length);
              return (
                <AgentNode
                  key={agent.id}
                  agent={agent}
                  x={pos.x}
                  y={pos.y}
                  isSelected={agent.id === selectedAgentId}
                  onSelect={() => selectAgent(agent.id === selectedAgentId ? null : agent.id)}
                />
              );
            })
          )}
        </Layer>
      </Stage>

      {/* Empty state */}
      {agents.length === 0 && (
        <div className="absolute inset-0 flex items-center justify-center z-10 pointer-events-none">
          <div className="text-center">
            <p className="text-text-muted text-sm mb-1">Campus is empty</p>
            <p className="text-text-muted text-xs">Generate agents to begin simulation</p>
          </div>
        </div>
      )}
    </div>
  );
}

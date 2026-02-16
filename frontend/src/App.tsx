import { useEffect } from 'react';
import { TopBar } from './components/TopBar';
import { AgentList } from './components/AgentList';
import { CampusMap } from './components/CampusMap';
import { DetailPanel } from './components/DetailPanel';
import { BottomBar } from './components/BottomBar';
import { useSimulationStore } from './stores/simulation';

export default function App() {
  const { connect, disconnect, fetchStatus, fetchAgents } = useSimulationStore();

  useEffect(() => {
    connect();
    fetchStatus();
    fetchAgents();

    return () => disconnect();
  }, [connect, disconnect, fetchStatus, fetchAgents]);

  return (
    <div className="h-screen w-screen flex flex-col bg-void overflow-hidden">
      <TopBar />
      <div className="flex-1 flex overflow-hidden">
        <AgentList />
        <CampusMap />
        <DetailPanel />
      </div>
      <BottomBar />
    </div>
  );
}

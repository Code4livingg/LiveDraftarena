import { useState } from 'react';
import WalletConnect from './components/WalletConnect';
import LobbyPage from './components/LobbyPage';
import DraftRoomPage from './components/DraftRoomPage';

// Simple page state management
type Page = 'wallet' | 'lobby' | 'draftroom';

interface AppState {
  currentPage: Page;
  selectedRoomChainId?: string;
  walletConnected: boolean;
  currentChainId?: string;
  userAddress?: string;
}

function App() {
  const [appState, setAppState] = useState<AppState>({
    currentPage: 'wallet',
    walletConnected: false,
  });

  const handleWalletConnected = (chainId: string, userAddress: string) => {
    setAppState(prev => ({
      ...prev,
      walletConnected: true,
      currentChainId: chainId,
      userAddress,
      currentPage: 'lobby',
    }));
  };

  const handleEnterRoom = (chainId: string) => {
    setAppState(prev => ({
      ...prev,
      selectedRoomChainId: chainId,
      currentPage: 'draftroom',
    }));
  };

  const handleBackToLobby = () => {
    setAppState(prev => ({
      ...prev,
      currentPage: 'lobby',
      selectedRoomChainId: undefined,
    }));
  };

  return (
    <div style={{ padding: '20px', fontFamily: 'Arial, sans-serif' }}>
      <h1>LiveDraft Arena</h1>
      <p style={{ color: '#666', marginBottom: '20px' }}>
        Real Linera blockchain integration • Conway testnet • Backend GraphQL
      </p>
      
      {/* Simple conditional rendering based on current page */}
      {appState.currentPage === 'wallet' && (
        <WalletConnect onConnected={handleWalletConnected} />
      )}
      
      {appState.currentPage === 'lobby' && (
        <LobbyPage 
          chainId={appState.currentChainId!}
          userAddress={appState.userAddress!}
          onEnterRoom={handleEnterRoom}
        />
      )}
      
      {appState.currentPage === 'draftroom' && appState.selectedRoomChainId && (
        <DraftRoomPage 
          roomChainId={appState.selectedRoomChainId}
          userAddress={appState.userAddress!}
          onBackToLobby={handleBackToLobby}
        />
      )}
    </div>
  );
}

export default App;
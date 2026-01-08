import React, { useState, useEffect } from 'react';
import { checkBackendHealth, getPlayerInfo, getCurrentPlayerId } from '../graphql';

interface WalletConnectProps {
  onConnected: (chainId: string, userAddress: string) => void;
}

const WalletConnect: React.FC<WalletConnectProps> = ({ onConnected }) => {
  const [connecting, setConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [backendHealthy, setBackendHealthy] = useState<boolean | null>(null);
  const [playerInfo, setPlayerInfo] = useState<string>('');

  // Check backend health and get player info on component mount
  useEffect(() => {
    const checkHealth = async () => {
      const healthy = await checkBackendHealth();
      setBackendHealthy(healthy);
      
      if (healthy) {
        // Get player info from backend
        const info = await getPlayerInfo();
        setPlayerInfo(info);
      }
    };
    checkHealth();
  }, []);

  const handleConnect = async () => {
    setConnecting(true);
    setError(null);

    try {
      // Check backend health first
      const healthy = await checkBackendHealth();
      if (!healthy) {
        throw new Error('Backend service is not available');
      }

      // Get player info from backend (includes player ID and Owner address)
      const info = await getPlayerInfo();
      setPlayerInfo(info);
      
      // Simulate connection delay
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Use player ID as both chain ID and user address for demo
      // In production, these would come from the backend
      const playerId = getCurrentPlayerId();
      const mockChainId = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65";
      const mockUserAddress = playerId; // Use player ID as user identifier
      
      onConnected(mockChainId, mockUserAddress);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to connect');
    } finally {
      setConnecting(false);
    }
  };

  return (
    <div>
      <h2>Connect to Linera</h2>
      <p>Connect to the Linera Conway testnet via backend service</p>
      
      {/* Backend health status */}
      <div style={{ marginBottom: '20px', padding: '10px', backgroundColor: '#f5f5f5', borderRadius: '4px' }}>
        <strong>Backend Status:</strong>{' '}
        {backendHealthy === null ? (
          <span style={{ color: '#666' }}>Checking...</span>
        ) : backendHealthy ? (
          <span style={{ color: '#28a745' }}>✅ Connected to http://localhost:8080/graphql</span>
        ) : (
          <span style={{ color: '#dc3545' }}>❌ Backend service unavailable</span>
        )}
      </div>
      
      {/* Player identity info */}
      {playerInfo && (
        <div style={{ marginBottom: '20px', padding: '10px', backgroundColor: '#e7f3ff', borderRadius: '4px' }}>
          <strong>Your Identity:</strong> {playerInfo}
          <br />
          <small style={{ color: '#666' }}>
            Player ID: {getCurrentPlayerId()} (persisted in browser)
          </small>
        </div>
      )}
      
      <button 
        onClick={handleConnect} 
        disabled={connecting || !backendHealthy}
        style={{
          padding: '10px 20px',
          fontSize: '16px',
          backgroundColor: connecting || !backendHealthy ? '#ccc' : '#007bff',
          color: 'white',
          border: 'none',
          borderRadius: '4px',
          cursor: connecting || !backendHealthy ? 'not-allowed' : 'pointer',
        }}
      >
        {connecting ? 'Connecting...' : 'Connect Wallet'}
      </button>
      
      {error && (
        <div style={{ color: 'red', marginTop: '10px' }}>
          Error: {error}
        </div>
      )}
      
      <div style={{ marginTop: '20px', fontSize: '14px', color: '#666' }}>
        <h3>Multi-User Identity System:</h3>
        <ul>
          <li>✅ Deterministic player ID per browser session</li>
          <li>✅ Player ID persisted across page refreshes</li>
          <li>✅ Different browsers get different player IDs</li>
          <li>✅ Backend maps player ID to Linera Owner</li>
          <li>✅ Multiple users can play simultaneously</li>
        </ul>
      </div>
    </div>
  );
};

export default WalletConnect;
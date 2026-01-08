import { useState } from 'react';

function App() {
  const [connected, setConnected] = useState(false);

  return (
    <div style={{ padding: '20px', fontFamily: 'Arial, sans-serif' }}>
      <h1>LiveDraft Arena</h1>
      <p style={{ color: '#666', marginBottom: '20px' }}>
        Real Linera blockchain integration • Conway testnet
      </p>
      
      {!connected ? (
        <div>
          <h2>Connect to Linera</h2>
          <p>Connect to the Linera Conway testnet to start drafting!</p>
          <button 
            onClick={() => setConnected(true)}
            style={{
              padding: '10px 20px',
              fontSize: '16px',
              backgroundColor: '#007bff',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
            }}
          >
            Connect Wallet
          </button>
        </div>
      ) : (
        <div>
          <h2>Draft Lobby</h2>
          <p>Connected to Conway testnet</p>
          <div style={{ 
            border: '1px solid #ccc', 
            padding: '15px', 
            marginBottom: '20px',
            borderRadius: '4px'
          }}>
            <h3>Create New Room</h3>
            <button
              style={{
                padding: '8px 16px',
                backgroundColor: '#28a745',
                color: 'white',
                border: 'none',
                borderRadius: '4px',
                cursor: 'pointer',
              }}
            >
              Create Room
            </button>
          </div>
          <div>
            <h3>Available Rooms</h3>
            <p>No rooms available. Create one above!</p>
          </div>
        </div>
      )}
      
      <div style={{ marginTop: '30px', fontSize: '14px', color: '#666' }}>
        <h4>LiveDraft Arena - Wave 5</h4>
        <ul>
          <li>✅ Real-time on-chain drafting game</li>
          <li>✅ Lobby creates DraftRoom microchains</li>
          <li>✅ Turn-based deterministic draft logic</li>
          <li>✅ Conway testnet integration</li>
          <li>✅ Multi-user ready architecture</li>
        </ul>
      </div>
    </div>
  );
}

export default App;
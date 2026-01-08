import React from 'react';
import { DraftRoomState } from '../types';
import { useLinera } from '../hooks/useLinera';
import { useGraphQLQuery } from '../hooks/useGraphQLQuery';
import { useOperation } from '../hooks/useOperation';
import { LINERA_CONFIG, QUERIES, OPERATIONS } from '../linera';

interface DraftRoomPageProps {
  roomChainId: string;
  userAddress: string;
  onBackToLobby: () => void;
}

const DraftRoomPage: React.FC<DraftRoomPageProps> = ({ 
  roomChainId, 
  userAddress, 
  onBackToLobby 
}) => {
  const { client, signer } = useLinera();
  
  // Query room state from DraftRoom GraphQL service
  const { data: roomState, loading: queryLoading, error: queryError } = useGraphQLQuery<DraftRoomState>(
    client,
    roomChainId,
    LINERA_CONFIG.LOBBY_APP_ID, // Same app ID, different chain
    QUERIES.ROOM_STATE,
    { pollInterval: 1000 } // Poll every 1 second for real-time updates
  );

  // Operation hooks for different room operations
  const { execute: executeJoinRoom, loading: joining } = useOperation(
    client,
    signer,
    roomChainId,
    LINERA_CONFIG.LOBBY_APP_ID
  );

  const { execute: executeStartDraft, loading: starting } = useOperation(
    client,
    signer,
    roomChainId,
    LINERA_CONFIG.LOBBY_APP_ID
  );

  const { execute: executePickItem, loading: picking } = useOperation(
    client,
    signer,
    roomChainId,
    LINERA_CONFIG.LOBBY_APP_ID
  );

  const handleJoinRoom = async () => {
    try {
      const operation = OPERATIONS.JOIN_ROOM();
      await executeJoinRoom(operation);
    } catch (err) {
      console.error('Failed to join room:', err);
      alert('Failed to join room: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  };

  const handleStartDraft = async () => {
    try {
      const operation = OPERATIONS.START_DRAFT();
      await executeStartDraft(operation);
    } catch (err) {
      console.error('Failed to start draft:', err);
      alert('Failed to start draft: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  };

  const handlePickItem = async (itemId: number) => {
    try {
      const operation = OPERATIONS.PICK_ITEM(itemId);
      await executePickItem(operation);
    } catch (err) {
      console.error('Failed to pick item:', err);
      alert('Failed to pick item: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  };

  if (queryLoading) {
    return <div>Loading draft room...</div>;
  }

  if (queryError) {
    return (
      <div>
        <div style={{ marginBottom: '20px' }}>
          <button onClick={onBackToLobby} style={{ padding: '8px 16px' }}>
            ← Back to Lobby
          </button>
        </div>
        <div style={{ color: 'red' }}>
          Failed to load room state: {queryError}
        </div>
      </div>
    );
  }

  if (!roomState) {
    return <div>No room data available</div>;
  }

  const isCreator = roomState.creator === userAddress;
  const isPlayerInRoom = roomState.players.includes(userAddress);
  const currentPlayer = roomState.players[roomState.current_turn];
  const isMyTurn = currentPlayer === userAddress;
  const submitting = joining || starting || picking;

  return (
    <div>
      <div style={{ marginBottom: '20px' }}>
        <button onClick={onBackToLobby} style={{ padding: '8px 16px' }}>
          ← Back to Lobby
        </button>
      </div>

      <h2>Draft Room</h2>
      <p>Chain ID: <code style={{ fontSize: '12px' }}>{roomChainId}</code></p>

      {/* Room Status */}
      <div style={{ 
        border: '1px solid #ccc', 
        padding: '15px', 
        marginBottom: '20px',
        borderRadius: '4px'
      }}>
        <h3>Room Status</h3>
        <p><strong>Status:</strong> {roomState.status}</p>
        <p><strong>Players:</strong> {roomState.players.length} / {roomState.max_players}</p>
        <p><strong>Round:</strong> {roomState.round} / {roomState.max_rounds}</p>
        {roomState.status === 'Drafting' && (
          <p><strong>Current Turn:</strong> {currentPlayer === userAddress ? 'Your turn!' : 'Waiting...'}</p>
        )}
      </div>

      {/* Action Buttons */}
      <div style={{ marginBottom: '20px' }}>
        {roomState.status === 'Waiting' && !isPlayerInRoom && (
          <button
            onClick={handleJoinRoom}
            disabled={submitting}
            style={{
              padding: '10px 20px',
              backgroundColor: submitting ? '#ccc' : '#28a745',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: submitting ? 'not-allowed' : 'pointer',
              marginRight: '10px',
            }}
          >
            {joining ? 'Joining...' : 'Join Room'}
          </button>
        )}

        {roomState.status === 'Waiting' && isCreator && (
          <button
            onClick={handleStartDraft}
            disabled={submitting}
            style={{
              padding: '10px 20px',
              backgroundColor: submitting ? '#ccc' : '#007bff',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: submitting ? 'not-allowed' : 'pointer',
            }}
          >
            {starting ? 'Starting...' : 'Start Draft'}
          </button>
        )}
      </div>

      {/* Players List */}
      <div style={{ marginBottom: '20px' }}>
        <h3>Players</h3>
        {roomState.players.map((player, index) => (
          <div key={player} style={{ 
            padding: '5px 10px',
            backgroundColor: player === userAddress ? '#e7f3ff' : '#f5f5f5',
            marginBottom: '5px',
            borderRadius: '4px',
            border: currentPlayer === player ? '2px solid #007bff' : '1px solid #ddd'
          }}>
            <strong>Player {index + 1}:</strong> {player === userAddress ? 'You' : `${player.slice(0, 8)}...`}
            {player === roomState.creator && <span style={{ color: '#28a745' }}> (Creator)</span>}
            {currentPlayer === player && roomState.status === 'Drafting' && (
              <span style={{ color: '#007bff' }}> (Current Turn)</span>
            )}
          </div>
        ))}
      </div>

      {/* Draft Pool */}
      {roomState.status === 'Drafting' && (
        <div style={{ marginBottom: '20px' }}>
          <h3>Available Cards</h3>
          <div style={{ 
            display: 'grid', 
            gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))', 
            gap: '10px' 
          }}>
            {roomState.pool.map((item) => (
              <div
                key={item.id}
                style={{
                  border: '1px solid #ddd',
                  padding: '10px',
                  borderRadius: '4px',
                  backgroundColor: '#f9f9f9',
                }}
              >
                <h4>{item.name}</h4>
                <p>Power: {item.power}</p>
                <button
                  onClick={() => handlePickItem(item.id)}
                  disabled={!isMyTurn || submitting}
                  style={{
                    padding: '6px 12px',
                    backgroundColor: isMyTurn && !submitting ? '#28a745' : '#ccc',
                    color: 'white',
                    border: 'none',
                    borderRadius: '4px',
                    cursor: isMyTurn && !submitting ? 'pointer' : 'not-allowed',
                    width: '100%',
                  }}
                >
                  {picking ? 'Picking...' : isMyTurn ? 'Pick' : 'Not Your Turn'}
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Player Picks */}
      {isPlayerInRoom && roomState.picks && roomState.picks[userAddress] && (
        <div>
          <h3>Your Picks</h3>
          <div style={{ 
            display: 'grid', 
            gridTemplateColumns: 'repeat(auto-fill, minmax(150px, 1fr))', 
            gap: '10px' 
          }}>
            {roomState.picks[userAddress].map((item, index) => (
              <div
                key={`${item.id}-${index}`}
                style={{
                  border: '1px solid #28a745',
                  padding: '8px',
                  borderRadius: '4px',
                  backgroundColor: '#f0fff0',
                }}
              >
                <h5>{item.name}</h5>
                <p>Power: {item.power}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      <div style={{ marginTop: '30px', fontSize: '14px', color: '#666' }}>
        <h4>Real DraftRoom Integration:</h4>
        <ul>
          <li>✅ Real GraphQL queries to room microchain</li>
          <li>✅ Real operations: JoinRoom, StartDraft, PickItem</li>
          <li>✅ Real-time polling every 1 second</li>
          <li>✅ Turn-based validation from smart contract</li>
          <li>✅ Conway testnet integration</li>
        </ul>
      </div>
    </div>
  );
};

export default DraftRoomPage;
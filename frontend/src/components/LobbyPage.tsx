import React, { useState } from 'react';
import { RoomsResponse } from '../types';
import { useLinera } from '../hooks/useLinera';
import { useGraphQLQuery } from '../hooks/useGraphQLQuery';
import { useOperation } from '../hooks/useOperation';
import { LINERA_CONFIG, QUERIES, OPERATIONS } from '../linera';

interface LobbyPageProps {
  chainId: string;
  userAddress: string;
  onEnterRoom: (chainId: string) => void;
}

const LobbyPage: React.FC<LobbyPageProps> = ({ chainId, userAddress, onEnterRoom }) => {
  const { client, signer } = useLinera();
  
  // Query rooms from lobby GraphQL service
  const { data: roomsData, loading: queryLoading, error: queryError } = useGraphQLQuery<RoomsResponse>(
    client,
    chainId,
    LINERA_CONFIG.LOBBY_APP_ID,
    QUERIES.LOBBY_ROOMS,
    { pollInterval: 2000 } // Poll every 2 seconds
  );

  // Operation hook for creating rooms
  const { execute: executeCreateRoom, loading: creating, error: createError } = useOperation(
    client,
    signer,
    chainId,
    LINERA_CONFIG.LOBBY_APP_ID
  );
  
  // Create room form state
  const [roomName, setRoomName] = useState('');
  const [maxPlayers, setMaxPlayers] = useState(4);

  const handleCreateRoom = async () => {
    if (!roomName.trim()) {
      alert('Please enter a room name');
      return;
    }

    try {
      const operation = OPERATIONS.CREATE_ROOM(roomName.trim(), maxPlayers);
      await executeCreateRoom(operation);
      
      // Reset form on success
      setRoomName('');
      setMaxPlayers(4);
      
      alert('Room created successfully!');
    } catch (err) {
      console.error('Failed to create room:', err);
      alert('Failed to create room: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  };

  const rooms = roomsData?.rooms || [];

  return (
    <div>
      <h2>Draft Lobby</h2>
      <p>Connected to chain: <code style={{ fontSize: '12px' }}>{chainId}</code></p>
      <p>User address: <code style={{ fontSize: '12px' }}>{userAddress}</code></p>
      
      {/* Create Room Form */}
      <div style={{ 
        border: '1px solid #ccc', 
        padding: '15px', 
        marginBottom: '20px',
        borderRadius: '4px'
      }}>
        <h3>Create New Room</h3>
        <div style={{ marginBottom: '10px' }}>
          <label>
            Room Name: 
            <input
              type="text"
              value={roomName}
              onChange={(e) => setRoomName(e.target.value)}
              style={{ marginLeft: '10px', padding: '5px' }}
              placeholder="Enter room name"
              disabled={creating}
            />
          </label>
        </div>
        <div style={{ marginBottom: '10px' }}>
          <label>
            Max Players: 
            <select
              value={maxPlayers}
              onChange={(e) => setMaxPlayers(Number(e.target.value))}
              style={{ marginLeft: '10px', padding: '5px' }}
              disabled={creating}
            >
              <option value={2}>2</option>
              <option value={3}>3</option>
              <option value={4}>4</option>
              <option value={6}>6</option>
              <option value={8}>8</option>
            </select>
          </label>
        </div>
        <button
          onClick={handleCreateRoom}
          disabled={creating}
          style={{
            padding: '8px 16px',
            backgroundColor: creating ? '#ccc' : '#28a745',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: creating ? 'not-allowed' : 'pointer',
          }}
        >
          {creating ? 'Creating...' : 'Create Room'}
        </button>
        
        {createError && (
          <div style={{ color: 'red', marginTop: '10px' }}>
            Error creating room: {createError}
          </div>
        )}
      </div>

      {/* Rooms List */}
      <h3>Available Rooms</h3>
      
      {queryLoading && <p>Loading rooms...</p>}
      
      {queryError && (
        <div style={{ color: 'red', marginBottom: '10px' }}>
          Failed to load rooms: {queryError}
        </div>
      )}
      
      {!queryLoading && rooms.length === 0 && (
        <p>No rooms available. Create one above!</p>
      )}
      
      {rooms.length > 0 && (
        <div>
          {rooms.map((room) => (
            <div
              key={room.chain_id}
              style={{
                border: '1px solid #ddd',
                padding: '10px',
                marginBottom: '10px',
                borderRadius: '4px',
                backgroundColor: '#f9f9f9',
              }}
            >
              <h4>{room.metadata.room_name}</h4>
              <p>Status: <strong>{room.metadata.status}</strong></p>
              <p>Max Players: {room.metadata.max_players}</p>
              <p>Chain ID: <code style={{ fontSize: '12px' }}>{room.chain_id}</code></p>
              <button
                onClick={() => onEnterRoom(room.chain_id)}
                style={{
                  padding: '6px 12px',
                  backgroundColor: '#007bff',
                  color: 'white',
                  border: 'none',
                  borderRadius: '4px',
                  cursor: 'pointer',
                }}
              >
                Enter Room
              </button>
            </div>
          ))}
        </div>
      )}
      
      <div style={{ marginTop: '30px', fontSize: '14px', color: '#666' }}>
        <h4>Real Linera Integration:</h4>
        <ul>
          <li>✅ GraphQL queries to lobby service</li>
          <li>✅ Real CreateRoom operations</li>
          <li>✅ Polling every 2 seconds</li>
          <li>✅ App ID: {LINERA_CONFIG.LOBBY_APP_ID}</li>
          <li>✅ Conway testnet endpoint</li>
        </ul>
      </div>
    </div>
  );
};

export default LobbyPage;
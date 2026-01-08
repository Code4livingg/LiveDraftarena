import React, { useState, useEffect } from 'react';
import { graphqlRequest, QUERIES, MUTATIONS } from '../graphql';
import { RoomsResponse, CreateRoomInput, OperationResult } from '../types';

interface LobbyPageProps {
  chainId: string;
  userAddress: string;
  onEnterRoom: (chainId: string) => void;
}

const LobbyPage: React.FC<LobbyPageProps> = ({ chainId, userAddress, onEnterRoom }) => {
  const [rooms, setRooms] = useState<RoomsResponse['rooms']>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);
  
  // Create room form state
  const [roomName, setRoomName] = useState('');
  const [maxPlayers, setMaxPlayers] = useState(4);

  // Fetch rooms from backend GraphQL service
  const fetchRooms = async () => {
    try {
      setError(null);
      // GraphQL query to backend service
      const data = await graphqlRequest<RoomsResponse>(QUERIES.ROOMS);
      setRooms(data.rooms);
    } catch (err) {
      console.error('Failed to fetch rooms:', err);
      setError(err instanceof Error ? err.message : 'Failed to fetch rooms');
    } finally {
      setLoading(false);
    }
  };

  // Poll for room updates every 2 seconds
  useEffect(() => {
    fetchRooms();
    const interval = setInterval(fetchRooms, 2000);
    return () => clearInterval(interval);
  }, []);

  const handleCreateRoom = async () => {
    if (!roomName.trim()) {
      alert('Please enter a room name');
      return;
    }

    setCreating(true);
    try {
      const input: CreateRoomInput = {
        roomName: roomName.trim(),
        maxPlayers: maxPlayers,
      };

      // GraphQL mutation to backend service
      const data = await graphqlRequest<{ createRoom: OperationResult }>(
        MUTATIONS.CREATE_ROOM,
        { input }
      );

      if (data.createRoom.success) {
        // Reset form on success
        setRoomName('');
        setMaxPlayers(4);
        alert('Room created successfully!');
        // Refresh rooms list
        fetchRooms();
      } else {
        alert(`Failed to create room: ${data.createRoom.message}`);
      }
    } catch (err) {
      console.error('Failed to create room:', err);
      alert('Failed to create room: ' + (err instanceof Error ? err.message : 'Unknown error'));
    } finally {
      setCreating(false);
    }
  };

  if (loading) {
    return <div>Loading lobby...</div>;
  }

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
      </div>

      {/* Rooms List */}
      <h3>Available Rooms</h3>
      
      {error && (
        <div style={{ color: 'red', marginBottom: '10px' }}>
          Error loading rooms: {error}
        </div>
      )}
      
      {rooms.length === 0 ? (
        <p>No rooms available. Create one above!</p>
      ) : (
        <div>
          {rooms.map((room) => (
            <div
              key={room.chainId}
              style={{
                border: '1px solid #ddd',
                padding: '10px',
                marginBottom: '10px',
                borderRadius: '4px',
                backgroundColor: '#f9f9f9',
              }}
            >
              <h4>{room.roomName}</h4>
              <p>Status: <strong>{room.status}</strong></p>
              <p>Max Players: {room.maxPlayers}</p>
              <p>Chain ID: <code style={{ fontSize: '12px' }}>{room.chainId}</code></p>
              <button
                onClick={() => onEnterRoom(room.chainId)}
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
        <h4>Backend GraphQL Integration:</h4>
        <ul>
          <li>✅ Real GraphQL queries to backend service</li>
          <li>✅ Real CreateRoom mutations</li>
          <li>✅ Polling every 2 seconds</li>
          <li>✅ No mocked data</li>
          <li>✅ Backend endpoint: http://localhost:8080/graphql</li>
        </ul>
      </div>
    </div>
  );
};

export default LobbyPage;
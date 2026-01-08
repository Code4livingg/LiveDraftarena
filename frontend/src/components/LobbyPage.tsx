import React, { useState, useEffect } from 'react';
import { graphqlRequest, QUERIES, MUTATIONS } from '../graphql';
import { RoomsResponse, CreateRoomInput, OperationResult } from '../types';

interface LobbyPageProps {
  chainId: string;
  userAddress: string;
  onEnterRoom: (chainId: string) => void;
}

const LobbyPage: React.FC<LobbyPageProps> = ({
  chainId,
  userAddress,
  onEnterRoom,
}) => {
  const [rooms, setRooms] = useState<RoomsResponse['rooms']>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);

  const [roomName, setRoomName] = useState('');
  const [maxPlayers, setMaxPlayers] = useState(4);

  /* -------------------- Fetch Rooms -------------------- */
  const fetchRooms = async () => {
    try {
      console.log('🔄 Fetching rooms...');
      setError(null);

      const data = await graphqlRequest<RoomsResponse>(QUERIES.ROOMS);

      console.log('✅ Rooms fetched:', data.rooms);
      setRooms(data.rooms);
    } catch (err) {
      console.error('❌ Failed to fetch rooms:', err);
      setError(err instanceof Error ? err.message : 'Failed to fetch rooms');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchRooms();
    const interval = setInterval(fetchRooms, 2000);
    return () => clearInterval(interval);
  }, []);

  /* -------------------- Create Room -------------------- */
  const handleCreateRoom = async () => {
    console.log('🔥 Create Room button clicked');
    console.log(
      '🌍 GraphQL Endpoint:',
      import.meta.env.VITE_BACKEND_GRAPHQL_URL
    );

    if (!roomName.trim()) {
      alert('Please enter a room name');
      return;
    }

    setCreating(true);

    try {
      const input: CreateRoomInput = {
        roomName: roomName.trim(),
        maxPlayers,
      };

      console.log('📦 CreateRoom input:', input);

      const data = await graphqlRequest<{ createRoom: OperationResult }>(
        MUTATIONS.CREATE_ROOM,
        { input }
      );

      console.log('✅ CreateRoom response:', data);

      if (data.createRoom.success) {
        setRoomName('');
        setMaxPlayers(4);
        alert('Room created successfully!');
        fetchRooms();
      } else {
        alert(`Create failed: ${data.createRoom.message}`);
      }
    } catch (err) {
      console.error('❌ CreateRoom error:', err);
      alert(
        'Failed to create room: ' +
          (err instanceof Error ? err.message : 'Unknown error')
      );
    } finally {
      setCreating(false);
    }
  };

  /* -------------------- UI -------------------- */
  if (loading) {
    return <div>Loading lobby...</div>;
  }

  return (
    <div>
      <h2>Draft Lobby</h2>

      <p>
        Connected to chain:{' '}
        <code style={{ fontSize: '12px' }}>{chainId}</code>
      </p>
      <p>
        User address:{' '}
        <code style={{ fontSize: '12px' }}>{userAddress}</code>
      </p>

      {/* -------- Create Room -------- */}
      <div
        style={{
          border: '1px solid #ccc',
          padding: '15px',
          marginBottom: '20px',
          borderRadius: '4px',
        }}
      >
        <h3>Create New Room</h3>

        <div style={{ marginBottom: '10px' }}>
          <label>
            Room Name:
            <input
              type="text"
              value={roomName}
              onChange={(e) => setRoomName(e.target.value)}
              disabled={creating}
              style={{ marginLeft: '10px', padding: '5px' }}
            />
          </label>
        </div>

        <div style={{ marginBottom: '10px' }}>
          <label>
            Max Players:
            <select
              value={maxPlayers}
              onChange={(e) => setMaxPlayers(Number(e.target.value))}
              disabled={creating}
              style={{ marginLeft: '10px', padding: '5px' }}
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
          {creating ? 'Creating…' : 'Create Room'}
        </button>
      </div>

      {/* -------- Rooms List -------- */}
      <h3>Available Rooms</h3>

      {error && (
        <div style={{ color: 'red', marginBottom: '10px' }}>
          Error: {error}
        </div>
      )}

      {rooms.length === 0 ? (
        <p>No rooms available. Create one above.</p>
      ) : (
        rooms.map((room) => (
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
            <p>
              Chain ID:{' '}
              <code style={{ fontSize: '12px' }}>{room.chainId}</code>
            </p>

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
        ))
      )}
    </div>
  );
};

export default LobbyPage;

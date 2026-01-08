import { useState, useCallback } from 'react';
import { Signer, LineraClient, createLineraClient } from '../linera';
import {
  getStoredChainId,
  storeChainId,
  isValidChainId,
} from '../linera';

interface LineraState {
  client: LineraClient | null;
  signer: Signer | null;
  chainId: string | null;
  userAddress: string | null;
  connected: boolean;
  loading: boolean;
  error: string | null;
}

export const useLinera = () => {
  const [state, setState] = useState<LineraState>({
    client: null,
    signer: null,
    chainId: null,
    userAddress: null,
    connected: false,
    loading: false,
    error: null,
  });

  // Initialize client and signer
  const connect = useCallback(async () => {
    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      // For now, create a mock signer since we don't have the real implementation
      const mockSigner: Signer = { address: "mock_address" };
      
      // Create client
      const client = createLineraClient(mockSigner);
      
      // Get user address
      const userAddress = mockSigner.address;
      
      // Try to get stored chain ID
      const storedChainId = getStoredChainId();
      
      setState(prev => ({
        ...prev,
        client,
        signer: mockSigner,
        userAddress,
        chainId: storedChainId,
        connected: true,
        loading: false,
      }));

      return { client, signer: mockSigner, userAddress, chainId: storedChainId };
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Failed to connect';
      setState(prev => ({
        ...prev,
        loading: false,
        error: errorMessage,
      }));
      throw error;
    }
  }, []);

  // Set active chain ID
  const setActiveChain = useCallback((chainId: string) => {
    if (!isValidChainId(chainId)) {
      throw new Error('Invalid chain ID format');
    }
    
    storeChainId(chainId);
    setState(prev => ({ ...prev, chainId }));
  }, []);

  // Disconnect and clear state
  const disconnect = useCallback(() => {
    setState({
      client: null,
      signer: null,
      chainId: null,
      userAddress: null,
      connected: false,
      loading: false,
      error: null,
    });
  }, []);

  return {
    ...state,
    connect,
    setActiveChain,
    disconnect,
  };
};
import { useState, useEffect, useCallback } from 'react';
import { LineraClient, Signer } from '@linera/client';
import {
  createLineraClient,
  createOrLoadSigner,
  getStoredChainId,
  storeChainId,
  getUserAddress,
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
      // Create client
      const client = createLineraClient();
      
      // Create or load signer
      const signer = await createOrLoadSigner();
      
      // Get user address
      const userAddress = getUserAddress(signer);
      
      // Try to get stored chain ID
      const storedChainId = getStoredChainId();
      
      setState(prev => ({
        ...prev,
        client,
        signer,
        userAddress,
        chainId: storedChainId,
        connected: true,
        loading: false,
      }));

      return { client, signer, userAddress, chainId: storedChainId };
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
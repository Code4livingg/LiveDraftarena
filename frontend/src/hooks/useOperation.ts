import { useState, useCallback } from 'react';
import { LineraClient, Signer } from '@linera/client';
import { executeOperation } from '../linera';

interface OperationState {
  loading: boolean;
  error: string | null;
}

export const useOperation = (
  client: LineraClient | null,
  signer: Signer | null,
  chainId: string | null,
  applicationId: string
) => {
  const [state, setState] = useState<OperationState>({
    loading: false,
    error: null,
  });

  const execute = useCallback(async (operation: any) => {
    if (!client || !signer || !chainId) {
      throw new Error('Client, signer, or chain ID not available');
    }

    setState({ loading: true, error: null });

    try {
      const result = await executeOperation(
        client,
        signer,
        chainId,
        applicationId,
        operation
      );
      
      setState({ loading: false, error: null });
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Operation failed';
      setState({ loading: false, error: errorMessage });
      throw error;
    }
  }, [client, signer, chainId, applicationId]);

  return {
    ...state,
    execute,
  };
};
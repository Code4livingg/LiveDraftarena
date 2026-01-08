import { useState, useEffect, useCallback } from 'react';
import { LineraClient } from '@linera/client';
import { executeQuery } from '../linera';

interface QueryState<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
}

interface UseGraphQLQueryOptions {
  pollInterval?: number; // in milliseconds
  enabled?: boolean;
}

export const useGraphQLQuery = <T>(
  client: LineraClient | null,
  chainId: string | null,
  applicationId: string,
  query: string,
  options: UseGraphQLQueryOptions = {}
) => {
  const { pollInterval = 2000, enabled = true } = options;
  
  const [state, setState] = useState<QueryState<T>>({
    data: null,
    loading: false,
    error: null,
  });

  const executeQueryCallback = useCallback(async () => {
    if (!client || !chainId || !enabled) {
      return;
    }

    setState(prev => ({ ...prev, loading: true, error: null }));

    try {
      const result = await executeQuery(client, chainId, applicationId, query);
      setState(prev => ({
        ...prev,
        data: result,
        loading: false,
      }));
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Query failed';
      setState(prev => ({
        ...prev,
        loading: false,
        error: errorMessage,
      }));
    }
  }, [client, chainId, applicationId, query, enabled]);

  // Initial query and polling
  useEffect(() => {
    if (!enabled || !client || !chainId) {
      return;
    }

    // Execute immediately
    executeQueryCallback();

    // Set up polling if interval is provided
    if (pollInterval > 0) {
      const interval = setInterval(executeQueryCallback, pollInterval);
      return () => clearInterval(interval);
    }
  }, [executeQueryCallback, pollInterval, enabled, client, chainId]);

  // Manual refetch function
  const refetch = useCallback(() => {
    executeQueryCallback();
  }, [executeQueryCallback]);

  return {
    ...state,
    refetch,
  };
};
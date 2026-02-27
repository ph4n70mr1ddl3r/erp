interface ApiErrorResponse {
  error?: string;
  message?: string;
}

export function getErrorMessage(error: unknown, fallback = 'An unexpected error occurred'): string {
  if (!error) return fallback;
  
  if (typeof error === 'string') return error;
  
  if (error instanceof Error) {
    return error.message || fallback;
  }
  
  if (typeof error === 'object' && error !== null) {
    const apiError = error as ApiErrorResponse;
    if (apiError.error) return apiError.error;
    if (apiError.message) return apiError.message;
    
    if ('response' in error) {
      const axiosError = error as { response?: { data?: ApiErrorResponse } };
      if (axiosError.response?.data?.error) return axiosError.response.data.error;
      if (axiosError.response?.data?.message) return axiosError.response.data.message;
    }
  }
  
  return fallback;
}

export function isAxiosError(error: unknown): error is { response: { status: number; data: unknown } } {
  return (
    typeof error === 'object' &&
    error !== null &&
    'response' in error &&
    typeof (error as { response: unknown }).response === 'object'
  );
}

export function isNetworkError(error: unknown): boolean {
  if (!isAxiosError(error)) return false;
  return !error.response;
}

export function isUnauthorizedError(error: unknown): boolean {
  if (!isAxiosError(error)) return false;
  return error.response.status === 401;
}

export function isForbiddenError(error: unknown): boolean {
  if (!isAxiosError(error)) return false;
  return error.response.status === 403;
}

export function isNotFoundError(error: unknown): boolean {
  if (!isAxiosError(error)) return false;
  return error.response.status === 404;
}

export function isValidationError(error: unknown): boolean {
  if (!isAxiosError(error)) return false;
  return error.response.status === 400 || error.response.status === 422;
}

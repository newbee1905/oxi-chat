const handleErrors = (res: Response) => {
  if (!res.ok) {
    throw Error(`${res.status}`);
  }
  return res;
};

export const fetchWithTimeout = async (url: string, options) => {
  // default timeout = 8 seconds
  const { timeout = 8000 } = options;

  const controller = new AbortController();
  const id = setTimeout(() => controller.abort(), timeout);

  const response = await fetch(url, {
    ...options,
    signal: controller.signal,
  });
  clearTimeout(id);

  return handleErrors(response);
};

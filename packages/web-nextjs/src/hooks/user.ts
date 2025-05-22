import useSWR from "swr";
import React from "react";

export type User = {
  id: number;
  name: string;
};

const getMe = (): Promise<User | undefined> =>
  fetch("/api/users/me").then((res) => {
    if (res.ok) {
      return res.json();
    }
    return undefined;
  });

export const useMe = () => useSWR("/api/users/me", getMe);

export const useLogin = (callback: () => void) => {
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState();
  const run = React.useCallback(
    (username: string) => {
      setLoading(true);
      return fetch("/api/users/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ username }),
      })
        .then((res) => {
          if (res.ok) {
            return callback();
          }
          throw res.json();
        })
        .catch(setError)
        .finally(() => setLoading(false));
    },
    [setLoading, setError],
  );
  return {
    error,
    loading,
    run,
  };
};

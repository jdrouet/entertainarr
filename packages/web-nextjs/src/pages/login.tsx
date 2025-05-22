import { useLogin } from "@entertainarr/hooks/user";
import { useRouter } from "next/navigation";
import React from "react";

export default function Login() {
  const router = useRouter();

  const login = useLogin(() => {
    router.replace("/");
  });

  const handleSubmit = React.useCallback(
    (event: React.FormEvent) => {
      event.preventDefault();
      login.run("Jeremie");
    },
    [login.run],
  );

  return (
    <div className="flex items-center justify-center min-h-screen bg-gray-100">
      <div className="bg-white p-8 rounded-lg shadow-lg w-full max-w-sm">
        <h1 className="text-2xl font-semibold text-gray-800 mb-6 text-center">
          Login
        </h1>
        <form noValidate onSubmit={handleSubmit}>
          <div className="mb-4">
            <input
              id="username"
              type="text"
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-400"
              placeholder="Enter your username"
              required
            />
          </div>
          <button
            type="submit"
            className="w-full bg-indigo-600 text-white py-2 rounded-md hover:bg-indigo-700 transition duration-200"
          >
            Sign In
          </button>
        </form>
      </div>
    </div>
  );
}

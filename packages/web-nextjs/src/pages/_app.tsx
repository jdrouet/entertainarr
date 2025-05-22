import { Loading } from "@entertainarr/components/loading";
import { Redirect } from "@entertainarr/components/redirect";
import { useMe } from "@entertainarr/hooks/user";
import "@entertainarr/styles/globals.css";
import type { AppProps } from "next/app";
import React from "react";

export default function App({ Component, pageProps, router }: AppProps) {
  const user = useMe();

  if (!user.data && user.isLoading) {
    return <Loading />;
  }

  if (!user.data && router.pathname !== "/login") {
    return <Redirect to="/login" />;
  }

  return <Component {...pageProps} />;
}

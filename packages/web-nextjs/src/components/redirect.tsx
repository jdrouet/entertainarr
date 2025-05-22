import { useRouter } from "next/router";
import React from "react";

export type Props = {
  to: string;
};

export const Redirect = ({ to }: Props) => {
  const router = useRouter();

  React.useEffect(() => {
    router.push(to);
  }, [router, to]);

  return null;
};

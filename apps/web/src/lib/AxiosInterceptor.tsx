import { useEffect, type ReactNode } from "react";
import { useNotification } from "@/hooks/use-notification";
import { api } from "./api-client";

export function AxiosInterceptor({ children }: { children: ReactNode }) {
  const { notify } = useNotification();

  useEffect(() => {
    const responseInterceptor = api.interceptors.response.use(
      (response) => {
        return response.data;
      },
      (error) => {
        const message = error.response?.data?.message || error.message;

        notify.error({
          title: "Error",
          description: message,
        });

        return Promise.reject(error);
      },
    );

    return () => {
      api.interceptors.response.eject(responseInterceptor);
    };
  }, [notify]);

  return <>{children}</>;
}

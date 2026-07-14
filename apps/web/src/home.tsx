import { useEffect } from "react";
import { useNotification } from "./hooks/use-notification";

import { api } from "@/lib/api-client";

export const Home = () => {
  const { notify } = useNotification();

  useEffect(() => {
    api.post("/media/import");
  }, []);

  const showpopu = () => {
    notify.success({
      title: "Import Complete",
      description: "Import was completed sucessfully",
      duration: 90000,
    });
    notify.error({
      title: "Import Complete",
      description: "Import was completed sucessfully",
      action: {
        label: "View",
        onClick: () => {
          console.log("test");
        },
      },
    });
  };

  return (
    <div className="">
      <button onClick={showpopu}> click for notificiation</button>
    </div>
  );
};

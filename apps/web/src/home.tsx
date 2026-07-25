import { useNotification } from "./hooks/use-notification";

export const Home = () => {
  const { notify } = useNotification();

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

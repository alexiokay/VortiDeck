// import { useConnectionStore } from "@/stores/Connection";

export const useSendCommand = async (command: String) => {
  const { $socket } = useNuxtApp();

  const commandMessage = JSON.stringify({
    action: "command",
    command: command,
  });
  $socket.value?.send(commandMessage);
};

import { invoke } from "@tauri-apps/api/core";

export default defineNuxtPlugin(() => {
  // Define the return type of discoverService
  interface ServiceInfo {
    name: string;
    websocket_url: string;
  }

  async function discoverService(): Promise<ServiceInfo[] | null> {
    try {
      // Safely invoke the Tauri command
      const result = await invoke<ServiceInfo[]>("discover_websocket");
      console.log("Discovered services:", result); // Log the result
      if (result && result.length > 0) {
        // alert(`Found ${result.length} services: ${JSON.stringify(result)}`);
      } else {
        // alert("No services found");
      }
      return result; // Return the list of services
    } catch (error) {
      console.error("Error discovering services:", error);
      // alert("Error discovering services");
      return null; // Return null in case of error
    }
  }

  const config = useRuntimeConfig();
  const socket = ref<WebSocket | null>(null); // Reactive WebSocket instance
  const isWebSocketConnected = ref(false); // Reactive connection status

  let reconnectInterval: number | null = null;

  // Function to establish WebSocket connection
  const connectWebSocket = async () => {
    // alert("connecting");
    // Prevent redundant connections
    if (socket.value && socket.value.readyState === WebSocket.OPEN) {
      console.log("Already connected.");
      return;
    }

    // Fetch all the WebSocket URLs from the discovered services
    const serviceInfos = await discoverService();
    if (!serviceInfos || serviceInfos.length === 0) {
      console.error("No WebSocket services found.");

      return; // Stop further execution if discovery fails
    }

    // For now, connect to the first service (you can modify this to connect to any)
    const serviceInfo = serviceInfos[0];
    if (!serviceInfo.websocket_url) {
      console.error("Invalid WebSocket URL.");
      return;
    }

    // Create a new WebSocket instance
    socket.value = new WebSocket(serviceInfo.websocket_url);
    //socket.value = new WebSocket("ws://192.168.178.129:9001/");

    // Generate or retrieve a client ID
    let clientId = localStorage.getItem("clientId");
    if (!clientId) {
      clientId = crypto.randomUUID(); // Generate a unique ID
      localStorage.setItem("clientId", clientId);
    }

    // Attach event listeners to the WebSocket
    socket.value.onopen = () => {
      console.log("WebSocket connection established");
      isWebSocketConnected.value = true;

      // Send initial device information
      socket.value?.send(
        JSON.stringify({
          device: navigator.userAgent,
          clientId,
          action: "connect",
        })
      );

      // Stop reconnection attempts on successful connection
      if (reconnectInterval !== null) {
        clearInterval(reconnectInterval);
        reconnectInterval = null;
      }
    };

    socket.value.onerror = (error) => {
      console.error("WebSocket error:", error);
    };

    socket.value.onclose = (event) => {
      console.log("WebSocket connection closed:", event);
      isWebSocketConnected.value = false;
      if (!event.wasClean) {
        attemptReconnect();
      }
    };

    socket.value.onmessage = (event) => {
      const message = JSON.parse(event.data);
      console.log("Received message:", message);
      // alert("Message received");
    };
  };

  // Function to handle reconnection logic
  const attemptReconnect = () => {
    if (reconnectInterval === null) {
      reconnectInterval = setInterval(() => {
        console.log("Attempting to reconnect...");
        connectWebSocket();
      }, 5000); // Attempt reconnection every 5 seconds
    }
  };

  // Initially connect WebSocket
  connectWebSocket();

  return {
    provide: {
      socket,
      isWebSocketConnected,
    },
  };
});

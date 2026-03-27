import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Button } from "@/components/ui/button";

function App() {
  const [msg, setMsg] = useState("");
  return (
    <div className="flex h-screen flex-col items-center justify-center gap-4">
      <h1 className="font-bold text-4xl">Home</h1>
      {msg && <p>{msg}</p>}
      <Button
        variant="outline"
        onClick={async () => {
          setMsg(await invoke("greet", { name: "namewa" }));
        }}
      >
        Wow
      </Button>
    </div>
  );
}

export default App;

import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Link, useNavigate } from "react-router";
import { useAppSettings } from "@/components/AppSettingsProvider";
import { Button } from "@/components/ui/button";

function App() {
  const appSettings = useAppSettings();
  const navigate = useNavigate();
  const [msg, setMsg] = useState("");
  if (appSettings.project_folder !== null){
    navigate("project");
  }
  return (
    <div className="flex h-screen flex-col items-center justify-center gap-4">
      <h1 className="font-bold text-4xl">Home</h1>
      {msg && <p>{msg}</p>}
      <Button
        variant="outline"
        onClick={async () => {
          await invoke("test_change_settings");
          setMsg(await invoke("greet", { name: "namewa" }));
        }}
      >
        Wow
      </Button>
      <Button
        variant="outline"
        onClick={async () => {
          await invoke("pick_project_folder");
        }}
      >
        Test Folder Picker
      </Button>
      <Link to="project">Project</Link>
    </div>
  );
}

export default App;

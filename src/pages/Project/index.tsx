import { useEffect } from "react";
import { useNavigate } from "react-router";
import { useAppSettings } from "@/components/AppSettingsProvider";

function ProjectHomePage() {
  const appSettings = useAppSettings();
  const navigate = useNavigate();

  useEffect(() => {
    if (appSettings.project_folder === null) {
      navigate("/");
    }
  }, [appSettings, navigate]);

  return (
    <main className="container">
      <div className="flex h-full flex-col items-center justify-center gap-4">
        <h1>You have a project: {appSettings.project_folder}</h1>
        <h1>where is this?</h1>
      </div>
    </main>
  );
}

export default ProjectHomePage;

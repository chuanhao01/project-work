import { useAppSettings } from "@/components/AppSettingsProvider";

function ProjectHomePage() {
  const appSettings = useAppSettings();
  return (
    <main className="container">
      <div className="flex h-screen flex-col items-center justify-center gap-4">
        <h1>You have a project: {appSettings.project_folder}</h1>
      </div>
    </main>
  );
}

export default ProjectHomePage;

import type { PropsWithChildren } from "react";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarProvider,
  SidebarTrigger,
  SidebarGroupAction,
  SidebarGroupContent,
} from "@/components/ui/sidebar";
import { ChevronDown, Plus } from "lucide-react";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { NavItem } from "./NavItem";
import { Outlet } from "react-router";

const navItems = [
  {
    title: "Views",
    url: "#",
    isActive: true,
    items: [
      {
        title: "Calendar",
        url: "#",
      },
      {
        title: "Kanban",
        url: "#",
      },
    ],
  },
  {
    title: "Issues",
    url: "#",
  },
];

function AppSidebar() {
  return (
    <Sidebar>
      <SidebarHeader />
      <SidebarContent>
        <NavItem title={"Main?"} items={navItems} />
        <SidebarGroup />
      </SidebarContent>
      <SidebarFooter />
    </Sidebar>
  );
}

function ProjectLayout() {
  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="h-screen flex-1 overflow-auto">
        <header className="flex h-16 items-center border-b px-4">
          <SidebarTrigger />
        </header>

        <div className="p-6">
          <Outlet />
        </div>
      </main>

    </SidebarProvider>
  );
}

export default ProjectLayout;

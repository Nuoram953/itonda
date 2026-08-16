import * as React from "react";
import { Link } from "@tanstack/react-router";
import {
  ChevronDown,
  Download,
  Film,
  Gamepad2,
  Home,
  Library,
  Settings,
  Trophy,
  Tv,
} from "lucide-react";
import {
  Sidebar as ShadcnSidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  SidebarTrigger,
  useSidebar,
} from "@/components/ui/sidebar";
import { ServerAgentStatus } from "./ServerAgentStatus";
import { NowPlaying } from "./NowPlaying";
import { cn } from "@/lib/utils";

export const Sidebar = ({
  className,
  ...props
}: React.ComponentProps<typeof ShadcnSidebar>) => {
  const { state } = useSidebar();
  const [libraryOpen, setLibraryOpen] = React.useState(true);

  return (
    <ShadcnSidebar
      collapsible="icon"
      className={cn("select-none", className)}
      {...props}
    >
      <SidebarHeader className="h-16 justify-center px-4 group-data-[collapsible=icon]:px-0">
        <div className="flex items-center justify-between group-data-[collapsible=icon]:justify-center w-full">
          <div className="flex items-center gap-2 px-1 group-data-[collapsible=icon]:hidden">
            <span className="text-xs font-semibold uppercase tracking-wider text-text-muted">
              Itonda
            </span>

            <span className="rounded-md bg-white/5 border border-white/10 px-1.5 py-0.5 text-xs font-mono text-text-muted">
              v0.1.0
            </span>
          </div>
          <SidebarTrigger
            className="hover:bg-surface-hover hover:text-foreground text-text-muted cursor-pointer"
            title={
              state === "collapsed" ? "Expand Sidebar" : "Collapse Sidebar"
            }
            aria-label={
              state === "collapsed" ? "Expand Sidebar" : "Collapse Sidebar"
            }
          />
        </div>
      </SidebarHeader>

      <SidebarContent className="space-y-1">
        <SidebarGroup>
          <SidebarGroupLabel className="text-xs font-bold tracking-wider text-text-muted/60 uppercase">
            Discover
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  render={
                    <Link
                      to="/"
                      activeOptions={{ exact: true }}
                      className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                    />
                  }
                  tooltip="Home"
                >
                  <Home className="w-4 h-4" />
                  <span>Home</span>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton
                  render={
                    <Link
                      to="/rankings"
                      activeOptions={{ exact: true }}
                      className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                    />
                  }
                  tooltip="Rankings"
                >
                  <Trophy className="w-4 h-4" />
                  <span>Rankings</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel className="text-xs font-bold tracking-wider text-text-muted/60 uppercase">
            Library
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  render={
                    <Link
                      to="/media"
                      activeOptions={{ exact: true }}
                      className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                    />
                  }
                  tooltip="Library"
                >
                  <Library className="w-4 h-4" />
                  <span>Library</span>
                </SidebarMenuButton>

                <SidebarMenuAction
                  aria-label="toggle"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    setLibraryOpen((prev) => !prev);
                  }}
                  className="cursor-pointer text-text-muted hover:text-foreground"
                >
                  <ChevronDown
                    className={cn(
                      "w-3.5 h-3.5 transition-transform duration-200",
                      !libraryOpen && "-rotate-90",
                    )}
                  />
                </SidebarMenuAction>

                {libraryOpen && (
                  <SidebarMenuSub className="border-l border-white/10 ml-3.5 pl-2">
                    <SidebarMenuSubItem>
                      <SidebarMenuSubButton
                        render={
                          <Link
                            to="/media"
                            search={{ type: "game" }}
                            activeOptions={{ exact: false }}
                            className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                          />
                        }
                      >
                        <Gamepad2 className="w-3.5 h-3.5" />
                        <span>Games</span>
                      </SidebarMenuSubButton>
                    </SidebarMenuSubItem>

                    <SidebarMenuSubItem>
                      <SidebarMenuSubButton
                        render={
                          <Link
                            to="/media"
                            search={{ type: "movie" }}
                            activeOptions={{ exact: false }}
                            className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                          />
                        }
                      >
                        <Film className="w-3.5 h-3.5" />
                        <span>Movies</span>
                      </SidebarMenuSubButton>
                    </SidebarMenuSubItem>

                    <SidebarMenuSubItem>
                      <SidebarMenuSubButton
                        render={
                          <Link
                            to="/media"
                            search={{ type: "tv_show" }}
                            activeOptions={{ exact: false }}
                            className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                          />
                        }
                      >
                        <Tv className="w-3.5 h-3.5" />
                        <span>TV Series</span>
                      </SidebarMenuSubButton>
                    </SidebarMenuSubItem>
                  </SidebarMenuSub>
                )}
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarGroup>
          <SidebarGroupLabel className="text-xs font-bold tracking-wider text-text-muted/60 uppercase">
            System
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  render={
                    <Link
                      to="/downloads"
                      activeOptions={{ exact: true }}
                      className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                    />
                  }
                  tooltip="Downloads"
                >
                  <Download className="w-4 h-4" />
                  <span>Downloads</span>
                </SidebarMenuButton>
              </SidebarMenuItem>

              <SidebarMenuItem>
                <SidebarMenuButton
                  render={
                    <Link
                      to="/settings"
                      activeOptions={{ exact: true }}
                      className="[&.active]:bg-primary/10 [&.active]:text-primary [&.active]:font-semibold"
                    />
                  }
                  tooltip="Settings"
                >
                  <Settings className="w-4 h-4" />
                  <span>Settings</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter className="bg-surface-raised border-t border-white/10 p-2 space-y-2">
        <NowPlaying />
        <ServerAgentStatus />
      </SidebarFooter>
    </ShadcnSidebar>
  );
};

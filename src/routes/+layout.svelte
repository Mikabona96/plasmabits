<script lang="ts">
  import { page } from "$app/state";
  import Download from "$lib/shared/icons/Download.svelte";
  import Folder from "$lib/shared/icons/Folder.svelte";
  import Gear from "$lib/shared/icons/Gear.svelte";
  import Menu from "$lib/shared/icons/Menu.svelte";
  import Submenu from "$lib/shared/icons/Submenu.svelte";
  import { cn } from "$lib/shared/utils/cn";
  import "../style.css";

  let { children } = $props();
  const pathname = $derived(page.url.pathname);

  const navItems = [
    {
      name: "Menu items",
      href: "/",
      icon: Menu,
    },
    {
      name: "Submenus",
      href: "/submenu",
      icon: Submenu,
    },
    {
      name: "Install app",
      href: "/install",
      icon: Download,
    },
    {
      name: "Icon creator",
      href: "/icons",
      icon: Folder,
    },
  ];
</script>

<div
  class="flex text-white border-t border-t-silver h-screen w-full overflow-hidden"
>
  <nav
    class="flex flex-col gap-1.25 w-55 h-full bg-linear-120 border-r border-r-silver from-gray-200/99 to-blue-50/95 pt-6 pb-4.25 px-3.25"
  >
    {#each navItems as item (item.name)}
      <a
        href={item.href}
        class={cn(
          "text-black flex items-center gap-2 px-2.5 py-1.5 rounded-md transition-colors duration-200 hover:bg-veil/99",
          {
            "bg-shadow-veil/99 hover:bg-shadow-veil/99":
              page.url.pathname === item.href,
          },
        )}
      >
        <item.icon />
        <span>{item.name}</span>
      </a>
    {/each}

    <button
      class="mt-auto text-black flex items-center gap-2 px-2.5 py-1.5 rounded-md"
    >
      <Gear />
      <span>Settings/Help</span>
    </button>
  </nav>
  <main
    class="flex flex-col min-h-0 text-black bg-porcelain/20 p-6 py-5.5 flex-1 font-Inter overflow-hidden"
  >
    {@render children()}
  </main>
</div>

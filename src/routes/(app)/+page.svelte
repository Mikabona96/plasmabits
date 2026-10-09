<script lang="ts">
  import Button from "$lib/shared/components/Button.svelte";
  import RadioButtonsBox from "$lib/shared/components/RadioButtonsBox.svelte";

  let menuMode = $state<"item" | "submenu">("item");
  const link = {
    item: "/dolphin-context-menu-item",
    submenu: "#",
  };
</script>

<div class="flex flex-col flex-1 items-center justify-center">
  <h1 class="text-white text-[clamp(2.25rem,10vw,3rem)] px-10 mb-16 flex-none">
    Choose an option
  </h1>
  <div class="h-fit w-fit flex gap-4 justify-center items-center">
    <RadioButtonsBox
      bind:option={menuMode}
      value="item"
      groupName="menuMode"
      title="Menu item"
      description="Add a single action in Dolphin context menu"
    />
    <RadioButtonsBox
      bind:option={menuMode}
      value={"submenu"}
      groupName={"menuMode"}
      title="Submenu"
      description="Group multiple actions together"
    />
  </div>
  <a class="mt-16 w-fit" href={link[menuMode]}>
    <Button class="w-fit px-16">Next Step</Button>
  </a>
</div>

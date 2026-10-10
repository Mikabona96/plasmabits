<script lang="ts">
  interface MimeType {
    name: string;
    mimeType: string;
  }

  interface MimeTypeCategory {
    name: string;
    types: MimeType[];
  }

  interface Props {
    categories: MimeTypeCategory[];
    value?: string[];
    placeholder?: string;
  }

  let {
    categories,
    value = $bindable([]),
    placeholder = "Show this action for",
  }: Props = $props();

  let isOpen = $state(false);
  let openCategories = $state<string[]>([]);
  let selectedTypes = $state<string[]>([]);

  const selectedCount = $derived(value.length);

  function openModal() {
    selectedTypes = [...value];

    openCategories = categories.length ? [categories[0].name] : [];

    isOpen = true;
  }

  function closeModal() {
    isOpen = false;
  }

  function apply() {
    value = [...selectedTypes];
    isOpen = false;
  }

  function toggleCategory(categoryName: string) {
    if (openCategories.includes(categoryName)) {
      openCategories = openCategories.filter((name) => name !== categoryName);
    } else {
      openCategories = [...openCategories, categoryName];
    }
  }

  function isSelected(mimeType: string) {
    return selectedTypes.includes(mimeType);
  }

  function toggleType(mimeType: string) {
    if (isSelected(mimeType)) {
      selectedTypes = selectedTypes.filter((type) => type !== mimeType);
    } else {
      selectedTypes = [...selectedTypes, mimeType];
    }
  }

  function isCategorySelected(category: MimeTypeCategory) {
    return (
      category.types.length > 0 &&
      category.types.every((type) => selectedTypes.includes(type.mimeType))
    );
  }

  function isCategoryPartiallySelected(category: MimeTypeCategory) {
    const selected = category.types.filter((type) =>
      selectedTypes.includes(type.mimeType),
    ).length;

    return selected > 0 && selected < category.types.length;
  }

  function toggleCategoryTypes(category: MimeTypeCategory) {
    if (isCategorySelected(category)) {
      const categoryMimeTypes = new Set(
        category.types.map((type) => type.mimeType),
      );

      selectedTypes = selectedTypes.filter(
        (mimeType) => !categoryMimeTypes.has(mimeType),
      );
    } else {
      const categoryMimeTypes = category.types.map((type) => type.mimeType);

      selectedTypes = [
        ...selectedTypes,
        ...categoryMimeTypes.filter(
          (mimeType) => !selectedTypes.includes(mimeType),
        ),
      ];
    }
  }
</script>

<div class="w-full">
  <button
    type="button"
    class="flex w-full items-center justify-between rounded-lg border border-graphite-700 bg-graphite-800 px-3 py-1 text-left text-white cursor-pointer"
    onclick={openModal}
  >
    <span class={selectedCount ? "text-white" : "text-graphite-500"}>
      {selectedCount
        ? `${selectedCount} type${selectedCount === 1 ? "" : "s"} selected`
        : placeholder}
    </span>

    <svg class="h-4 w-4" viewBox="0 0 20 20" fill="none">
      <path
        d="M5 7.5L10 12.5L15 7.5"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    </svg>
  </button>
</div>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 px-4"
    role="presentation"
    onclick={(event) => {
      if (event.target === event.currentTarget) {
        closeModal();
      }
    }}
  >
    <div
      class="flex max-h-[80vh] w-full max-w-xl flex-col rounded-lg border border-graphite-700 bg-graphite-800"
      role="dialog"
      aria-modal="true"
      aria-label={placeholder}
    >
      <div
        class="flex items-center justify-between border-b border-graphite-700 px-4 py-3"
      >
        <span class="text-lg text-white">
          {placeholder}
        </span>

        <button
          type="button"
          class="flex h-7 w-7 items-center justify-center rounded-lg text-graphite-400 hover:bg-graphite-700 hover:text-white cursor-pointer"
          onclick={closeModal}
          aria-label="Close"
        >
          <svg class="h-5 w-5" viewBox="0 0 20 20" fill="none">
            <path
              d="M5 5L15 15M15 5L5 15"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linecap="round"
            />
          </svg>
        </button>
      </div>

      <div class="overflow-y-auto px-2 py-2">
        {#each categories as category}
          <div>
            <div
              class="flex items-center gap-2 rounded-lg px-2 py-2 hover:bg-graphite-700"
            >
              <button
                type="button"
                class="flex h-4 w-4 shrink-0 items-center justify-center rounded border-2 border-graphite-500 cursor-pointer"
                onclick={() => toggleCategoryTypes(category)}
                aria-label={`Select ${category.name}`}
              >
                {#if isCategorySelected(category)}
                  <span class="h-2 w-2 rounded-sm bg-white"></span>
                {:else if isCategoryPartiallySelected(category)}
                  <span class="h-2 w-2 rounded-sm bg-graphite-400"></span>
                {/if}
              </button>

              <button
                type="button"
                class="flex flex-1 items-center justify-between text-left text-white cursor-pointer"
                onclick={() => toggleCategory(category.name)}
              >
                <span>{category.name}</span>

                <svg
                  class={[
                    "h-4 w-4 transition-transform",
                    openCategories.includes(category.name) ? "rotate-180" : "",
                  ]}
                  viewBox="0 0 20 20"
                  fill="none"
                >
                  <path
                    d="M5 7.5L10 12.5L15 7.5"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                </svg>
              </button>
            </div>

            {#if openCategories.includes(category.name)}
              <div class="pb-1">
                {#each category.types as type}
                  <label
                    class="flex cursor-pointer items-center gap-2 rounded-lg px-2 py-1.5 pl-8 hover:bg-graphite-700"
                  >
                    <input
                      type="checkbox"
                      class="peer sr-only"
                      checked={isSelected(type.mimeType)}
                      onchange={() => toggleType(type.mimeType)}
                    />

                    <div
                      class="flex h-4 w-4 shrink-0 items-center justify-center rounded border-2 border-graphite-500 peer-checked:border-white peer-checked:bg-white"
                    >
                      {#if isSelected(type.mimeType)}
                        <svg
                          class="h-3 w-3 text-graphite-950"
                          viewBox="0 0 20 20"
                          fill="none"
                        >
                          <path
                            d="M4 10L8 14L16 6"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                          />
                        </svg>
                      {/if}
                    </div>

                    <span class="text-sm text-white">
                      {type.name}
                    </span>
                  </label>
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      </div>

      <div
        class="flex items-center justify-between border-t border-graphite-700 px-4 py-3"
      >
        <span class="text-sm text-graphite-400">
          {selectedTypes.length}
          {selectedTypes.length === 1 ? "type" : "types"}
          selected
        </span>

        <div class="flex items-center gap-2">
          <button
            type="button"
            class="rounded-lg border border-graphite-700 px-4 py-1 text-white cursor-pointer hover:bg-graphite-700"
            onclick={closeModal}
          >
            Cancel
          </button>

          <button
            type="button"
            class="rounded-lg bg-white px-4 py-1 text-graphite-950 cursor-pointer"
            onclick={apply}
          >
            Apply
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

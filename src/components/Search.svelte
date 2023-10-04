<script lang="ts">
  import { shortcut } from "@svelte-put/shortcut";
  import { invoke } from "@tauri-apps/api/tauri";

  let search = "";
  let results: {
    name: string;
    path: string;
    type: "file" | "folder";
  }[] = [];

  $: {
    if (search === "") {
      results = [];
    }
  }

  function handleShortcuts(action: "focus" | "blur") {
    if (action === "focus") {
      document.getElementById("search")?.focus();
    } else {
      document.getElementById("search")?.blur();
    }
  }

  function handleSubmit(e: Event) {
    e.preventDefault();
    invoke("search_indexes", { query: search }).then((res: unknown) => {
      const data = JSON.parse(res as string) as {
        name: string;
        path: string;
        type: "file" | "folder";
      }[];

      if (!res || data.length === 0) {
        results = [
          {
            name: "No results found",
            path: "Try changing your search query",
            type: "file",
          },
        ];
      } else {
        results = data;
      }
    });
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text);
  }
</script>

<svelte:window
  use:shortcut={{
    trigger: {
      key: "l",
      modifier: ["ctrl", "meta"],
      callback: () => handleShortcuts("focus"),
    },
  }}
  use:shortcut={{
    trigger: {
      key: "Escape",
      modifier: [],
      callback: () => handleShortcuts("blur"),
    },
  }}
/>
<div class="flex flex-col w-full">
  <form
    on:submit={handleSubmit}
    class="w-full flex items-center justify-center py-4 my-4"
  >
    <input
      id="search"
      type="text"
      placeholder="[ctrl+l] Search for a file..."
      bind:value={search}
    />
  </form>
  <div class="flex flex-col gap-2">
    {#each results as result}
      <div
        class="p-4 flex justify-between items-center rounded-md border border-neutral-800 hover:bg-neutral-900"
      >
        <div
          class="flex flex-col gap-2 overflow-hidden overflow-ellipsis select-text"
        >
          <span>{result.name}</span>
          <span class="text-xs text-neutral-400 pr-4">{result.path}</span>
        </div>
        <div
          class="text-xs cursor-pointer"
          on:click={(e) => {
            // set inner html to copy
            e.preventDefault();

            const btn = e.currentTarget;
            btn.innerHTML = "copied";

            copyToClipboard(result.path);

            // switch back to copy after 1 second
            setTimeout(() => {
              btn.innerHTML = "copy";
            }, 1000);
          }}
          aria-hidden
        >
          copy
        </div>
      </div>
    {/each}
  </div>
</div>

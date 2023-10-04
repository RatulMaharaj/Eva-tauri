<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let folderList = [];
  let folder = "";
  let isUpdating = false;

  function getFolderList() {
    invoke("get_folders_to_index").then((res) => {
      folderList = JSON.parse(res);
    });
  }

  $: console.log({ folderList });

  function addFolder() {
    invoke("add_folders_to_index", {
      folder,
    }).then(() => {
      getFolderList();
    });
  }

  function removeFolder(f) {
    invoke("remove_folders_from_index", {
      folder: f,
    }).then(() => {
      getFolderList();
    });
  }

  function updateAll() {
    isUpdating = true;
    invoke("update_indexes").then(() => {
      isUpdating = false;
    });
  }

  $: console.log({ isUpdating });

  onMount(() => {
    getFolderList();
  });
</script>

<div class="my-4 w-full">
  <div class="flex flex-row gap-4 mb-4">
    <input
      type="text"
      placeholder="Enter a folder path to add"
      bind:value={folder}
    />
    <button on:click={addFolder}>Add Folder</button>
    <button on:click={updateAll} disabled={isUpdating}
      >{isUpdating ? `Updating..` : `Update All`}</button
    >
  </div>
  <ul class="w-full flex flex-col gap-2 text-center">
    {#if folderList.length === 0}
      <li class="p-4 rounded-md border border-neutral-800 hover:bg-neutral-900">
        No folders added yet
      </li>
    {/if}
    {#each folderList as item}
      <li
        class="p-4 rounded-md flex border items-center justify-between border-neutral-800 hover:bg-neutral-900"
      >
        <span>{item}</span>
        <svg
          on:click={() => removeFolder(item)}
          aria-hidden="true"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="w-5 h-5 text-white hover:text-red-500"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0"
          />
        </svg>
      </li>
    {/each}
  </ul>
</div>

<script lang="ts">
    import { writable } from "svelte/store";
    import { invoke } from '@tauri-apps/api/tauri';

    let title;
    let savedMessage;
    let executablePath;

    function chooseExecutable() {
        invoke('file_dialog').then((message) => executablePath = message)
    }

    function saveData() {
        invoke('save_to_db', { title: title, exePath: executablePath })
        savedMessage = "Data saved, you can now close the modal."
    }
</script>

<div>
    <div class="newgame">
        <input type="text" name="Title" placeholder="Title" bind:value={title}>
        <div class="show-path">
            <button on:click={chooseExecutable} class="button">Add Executable</button>
            <p class="path" contenteditable=true bind:innerHTML={executablePath}>None</p>
        </div>
    </div>

    <p contenteditable="true" bind:innerHTML={savedMessage}>
    </p>

    <button on:click={saveData} class="button">
        Done
    </button>
</div>

<style>

    .path {
        padding: 4px 4px 4px 4px;
    }

    .show-path {
        display: flex;
    }

    .button {
        margin: 6px 6px 0px 0px;
    }
    .newgame {
        display: flex;
        flex-grow: 1;
        flex-direction: column;

        padding-right: 5%;
    }

    .newgame > input::placeholder {
        color: rgb(75, 75, 75)
    }

    .newgame > input {
        font-size: 17px;
    }

    button {
        font-size: 13px;
    }

    p {
        line-height: 0px;
        padding-left: 3px;
    }
</style>
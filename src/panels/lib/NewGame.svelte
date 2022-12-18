<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { getContext } from 'svelte';

    export let close: any;
    let title: string;
    let savedMessage: string;
    let executablePath: any;

    // TS Function -> Rust Function
    // - Opens a File selector dialog
    function chooseExecutable() {
        invoke('file_dialog').then((message) => executablePath = message)
    }

    // TS Function -> Rust Function
    // - Saves the data on the input fields to a SQLite DB
    function saveData() {
        invoke('save_to_db', { title: title, exePath: executablePath })
    }
</script>

<div class="modal-main">
    <div class="newgame">
        <input type="text" name="Title" placeholder="Title" bind:value={title}>
        <div class="show-path">
            <!-- When the button is clicked, run chooseExecutable -->
            <button on:click={chooseExecutable} class="button">Add Executable</button>

            <!-- Binds the inner html to executablePath -->
            <p class="path" contenteditable=true bind:innerHTML={executablePath}>None</p>
        </div>
    </div>

    <!-- I think you get it by now -->
    <button on:click={() => {saveData(); close()}} class="button">
        Done
    </button>
</div>

<style>
    .path {
        padding-top: 6px;
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
        font-size: 13px;
    }

    button {
        font-size: 13px;
    }

    p {
        line-height: 0px;
        padding-left: 3px;
    }

    .show-path > p {
        font-size: 13px;
    }

    .show-path {
        font-size: 13px;
    }
</style>
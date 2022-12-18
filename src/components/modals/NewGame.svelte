<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';
    import { getContext } from 'svelte';
    import "./../../styles/NewGame.css"

    const { close }: any = getContext("simple-modal")
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
            <button on:click={chooseExecutable} class="ng-button">Add Executable</button>

            <!-- Binds the inner html to executablePath -->
            <p class="path" contenteditable=true bind:innerHTML={executablePath}>None</p>
        </div>
    </div>

    <!-- I think you get it by now -->
    <button on:click={() => {saveData(); close()}} class="ng-button done-btn">
        Done
    </button>
</div>
<script lang="ts">
    import { writable } from "svelte/store";
    import Modal from "svelte-simple-modal";
    import NewGame from "./lib/NewGame.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    const modal = writable(null);
    const showModal = () => modal.set(NewGame);
    const games = invoke("get_from_db");
</script>

<main class="container">
    <div class="main">
        <div class="top">
            <h1 style="text-align:left; display:inline-block;">Library</h1>

            <!-- Creates a modal when the button is clicked -->
            <Modal show={$modal}>
                <button on:click={showModal}>
                    <i class="fa-solid fa-plus" />
                </button>
            </Modal>
        </div>

        {#await games}
            <p>...loading</p>
        {:then data}
            {#each data as game}
                <div>name : {game.name}</div>
                <div>path : {game.exe_path}</div>
                <div>playtime : {game.playtime}</div>
            {/each}
        {:catch error}
            <p style="color: red">{error.message}</p>
        {/await}
    </div>
</main>

<style>
    .top {
        display: inline;

        flex-grow: 1;
    }

    .top > button {
        background-color: #3f3f3f;
        border-color: #060606;
        border-style: solid;
        border-width: 1px;
        border-radius: 3px;

        margin-top: 1.2rem;

        color: #fff;
        font-size: 20px;
        float: right;

        cursor: pointer;
    }
</style>

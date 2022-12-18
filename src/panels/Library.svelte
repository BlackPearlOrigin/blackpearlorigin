<script lang="ts">
    import { getGames, deleteGame, runGame, editGame } from "./Library";
    import NewGame from "./lib/NewGame.svelte";

    import { getContext } from "svelte";
    const { open }: any = getContext('simple-modal')
    const showModal = () => open(NewGame)

    let games: any = getGames()

</script>

<main class="container">
    <div class="main">
        <div class="top">
            <h1 style="text-align:left; display:inline-block;">Library</h1>
            <!-- Creates a modal when the button is clicked -->
            <button on:click={showModal}>Add</button>
        </div>

        <!-- svelte-ignore empty-block -->
        {#await games}
        {:then data}
            {#each data as game}
                <div class="game-panel">
                    <div class="game-text">
                        <p class="game-title">{game.name}</p>
                        <p class="small-info">{game.playtime} hours played</p>
                    </div>
                    <div class="buttons">
                        <button class="game-button-run" on:click={() => runGame(game.exe_path)}>Run</button>
                        <button class="game-button-run" on:click={() => editGame(game.name)}>Edit</button>
                        <button class="game-button-delete" on:click={() => { games = deleteGame(game.name) }}>Delete</button>
                    </div>
                </div>
            {/each}
        {:catch error}
            <p style="color: red">{error.message}</p>
        {/await}
    </div>
</main>

<style>

    .buttons {
        padding: 8px 8px 8px 8px;
    }

    .game-button-run {
        background-color: #040404;
        border-style: dashed;
        border-width: 1px;
        border-color: #00ff66;
        color: #00ff66;
        padding: 4px 16px 4px 16px;
        margin-right: 8px;
        cursor: pointer;
    }

    .game-button-delete {
        background-color: #040404;
        border-style: dashed;
        border-width: 1px;
        border-color: #ff0000;
        color: #ff0000;
        padding: 4px 16px 4px 16px;
        margin-right: 8px;
        cursor: pointer;
    }

    .game-text {
        display: flex;
    }

    .game-panel {
        padding: 8px 8px 8px 8px;
        margin: 8px 0px 8px 0px;
        border-style: dashed;
        border-width: 1px;
        border-color: #00ff66;
    }

    .game-panel {
        background-color: #040404;
    }

    .game-panel p {
        margin: 0;
    }

    .game-title {
        padding: 0px 8px 0px 8px;
        font-weight: 550;
        font-size: 24px;
    }

    .small-info {
        padding: 2px 0px 0px 8px;
        font-size: 16px;
        color: #cccccc;
    }

    .top {
        display: inline;
        flex-grow: 1;
    }

    .top > button {
        background-color: #040404;
        border-color: #00ff66;
        border-style: dashed;
        border-width: 1px;
        margin-top: 1.2rem;
        padding: 4px 16px 4px 16px;
        color: #fff;
        font-size: 16px;
        float: right;
        cursor: pointer;
        transition: all 0.25s;
    }

    .top > button:hover {
        text-shadow: 0px 0px 12px hsla(0, 0%, 100%, 0.8);
    }
</style>

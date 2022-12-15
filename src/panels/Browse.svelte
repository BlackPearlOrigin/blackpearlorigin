<script lang="ts">
    import { browse } from "./Browse"

    let inputText: string;
    let selectedScraper: string;

    const data = browse()
</script>

<main class="container">
    <div class="main">
        <h1 style="text-align:left">Browse</h1>

        <div class="search">
            <input placeholder="Search" type="text" bind:value={inputText}>
            <select bind:value={selectedScraper} name="Plugins">
                <option value="Select">Select a plugin</option>
                {#await data}
                    {void(0)}
                {:then d}
                    {#each d.scrapers as Scrapers }
                        <option value="{Scrapers.location}">{Scrapers.name}</option>
                    {/each}
                {/await}
            </select>
            <button type="submit" on:click={void(0)}>
                <i class="fa-solid fa-magnifying-glass"></i>
            </button>
        </div>
    </div>
</main>

<style>
    .search {
        display: flex;
    }

    .search > input {
        font-size: 1rem;
        color: white;
        background-color: rgb(41, 41, 41);
        border-color: #393939;
        border-style: solid;
        border-width: 2px;
        border-radius: 3px;
        padding: 5px 5px 5px 5px;

        flex-grow: 1;
    }

    .search > input[type=text]:focus {
        outline-style: none;
    }

    .search > button {
        margin-left: 10px;
        padding: 5px 10px 5px 10px;
        background-color: rgb(41, 41, 41);
        border-color: #393939;
        border-style: solid;
        border-width: 2px;
        border-radius: 3px;

        cursor: pointer;
    }

    .search > select {
        margin-left: 8px;
        border-radius: 5px;

        border-color: #393939;
        border-width: 4px;
    }

    .search > button > i {
        color: white;
    }
</style>
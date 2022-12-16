<script lang="ts">
    import { browse, search } from "./Browse"

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
                    {d}
                    {#each d.scrapers as Scrapers }
                        <option value="{Scrapers.location}">{Scrapers.name.replace(/(\.exe)|(\.py)/g, "")}</option>
                    {/each}
                {/await}
            </select>
            <button type="submit" on:click={() => search("", selectedScraper)}>
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
        background-color: #0a0a0a;
        border-color: #060606;
        border-style: solid;
        border-width: 1px;
        border-radius: 3px;
        padding: 8px 8px 8px 8px;

        flex-grow: 1;
    }

    .search > input[type=text]:focus {
        outline-style: none;
    }

    .search > button {
        margin-left: 10px;
        padding: 8px 16px 8px 16px;
        background-color: #0a0a0a;
        border-color: #060606;
        border-style: solid;
        border-width: 1px;
        border-radius: 3px;

        cursor: pointer;
    }

    .search > select {
        margin-left: 8px;
    }

    .search > button > i {
        color: white;
    }
</style>
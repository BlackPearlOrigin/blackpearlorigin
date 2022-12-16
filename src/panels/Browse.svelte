<script lang="ts">
    import { browse, search, displayResults } from "./Browse"

    let inputText: string;
    let selectedScraper: string;
    let searchData: any = {
        response: []
    };
    let buttonClicked: number;

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
                    {#each d.scrapers as Scrapers}
                        <option value="{Scrapers.location}">{Scrapers.name.replace(/(\.exe)|(\.lua)/g, "")}</option>
                    {/each}
                {/await}
            </select>
            <button type="submit" on:click={() => search(selectedScraper, inputText).then(() => {
                searchData = displayResults()
            })}>
                <i class="fa-solid fa-magnifying-glass"></i>
            </button>
        </div>
        {#await searchData}
            {void(0)}
        {:then sd}
            {#each sd.response as Response}
                {#if sd.response.length == 0}
                    <h1>No games found</h1>
                {/if}
                <div class="game">
                    <p>{Response.Title}</p>
                    <a href={Response.URL1} target="_blank" rel="noreferrer">
                        Download
                    </a>
                </div>
            {/each}
        {/await}
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
        font-size: 16px;
    }

    .game {
        display: block;

        border-style: solid;
        border-color: #555555;
        background-color: #555555;
        border-radius: 5px;

        margin: 10px 0 10px 0;
        padding: 0px 6px 18px 6px;

        line-height: 10px;
    }

    .game p {
        padding-bottom: 5px;
        font-size: 18px;

        font-weight: 700;
    }

    .game a {
        text-decoration: none;
        
        border-style: solid;
        border-color: #3a3a3a;
        background-color: #3a3a3a;

        color: white;
        margin-bottom: 100px;
        border-width: 0px;
        border-radius: 5px;

        padding: 4px 4px 4px 4px;
    }
</style>
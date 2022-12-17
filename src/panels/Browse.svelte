<script lang="ts">
    import { browse, search, displayResults } from "./Browse"
    
    // Defines variables for the:
    // - Search text
    // - Scraper selected
    // - And the search data
    let inputText: string;
    let selectedScraper: string;
    let searchData: any = {
        response: []
    };

    const data = browse()
</script>

<main class="container">
    <div class="main">
        <h1 style="text-align:left">Browse</h1>

        <div class="search">
            <input placeholder="Search" type="text" bind:value={inputText}>
            <select bind:value={selectedScraper} name="Plugins">
                <option value="Select">Select a plugin</option>

                <!-- Awaits the data to be resolved -->
                <!-- After that adds an option for each scraper -->
                <!-- svelte-ignore empty-block -->
                {#await data}
                {:then d}
                    {#each d.scrapers as Scrapers}
                        <option value="{Scrapers.location}">{Scrapers.name.replace(/(\.exe)|(\.lua)/g, "")}</option>
                    {/each}
                {/await}
            </select>

            <!-- When the button is clicked, refefine the var searchData -->
            <!-- With the function displayResults -->
            <button type="submit" on:click={() => search(selectedScraper, inputText).then(() => {
                searchData = displayResults()
            })}>
                <i class="fa-solid fa-magnifying-glass"></i>
            </button>
        </div>

        <!-- Awaits for search data to be resolved-->
        <!-- After that add an div with the game title -->
        <!-- And url for each object -->
        <!-- svelte-ignore empty-block -->
        {#await searchData}
        {:then sd}
            {#each sd.response as Response}
                {#if sd.response.length == 0}
                    <h1>No games found</h1>
                {/if}
                <div class="game">
                    <p>{Response.title}</p>
                    {#each Response.urls as urls }
                        <a href={urls} target="_blank" rel="noreferrer">
                            <i class="fa-solid fa-download"></i> Download
                        </a>
                    {/each}
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

        padding: 4px 10px 4px 10px;
    }
</style>
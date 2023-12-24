<!--
    TODO: Add a basic scraper to the backend so i can implement 
          on the frontend
-->

<script lang="ts">
    import { searchGame } from '../scripts/Browse';
    import { t } from '../locale/i18n';
    import type { SearchedGame } from '../scripts/Interfaces';

    // Defines variables for the:
    // - Search text
    // - Selected plugin
    // - And the search data
    let inputText: string;
    let searchData: SearchedGame[] = [];
</script>

<!--
	Checks if the enter key has been pressed
 	If it has been pressed, then re-define the variable searchData
 	Otherwise, don't do nothing
-->
<svelte:window
    on:keydown="{({ key }) => {
        if (key === 'Enter') {
            // search for game
        }
    }}"
/>

<main class="container">
    <div class="main">
        <div class="search">
            <button
                type="submit"
                on:click="{() =>
                    searchGame(inputText).then((data) => {
                        searchData = JSON.parse(data);
                    })}"
            >
                <i class="fa-solid fa-magnifying-glass"></i>
            </button>
            <input
                placeholder="{$t('library.searchGame')}"
                type="text"
                bind:value="{inputText}"
            />
        </div>

        <!--
			Awaits for search data to be resolved
			After that add an div with the game title
			And url for each object
		-->
        {#if searchData.length === 0}
            <h1 class="noresults">No results found</h1>
        {/if}
        {#each searchData as Response}
            <div class="game">
                <p>{Response.name}</p>
                {#each Response.links as url}
                    <a href="{url.link}" target="_blank" rel="noreferrer">
                        <i class="fa-solid fa-download"></i>
                        {url.link.toString().startsWith('magnet:')
                            ? $t('browse.downloadTextMagnet')
                            : $t('browse.downloadText')}
                    </a>
                {/each}
            </div>
        {/each}
    </div>
</main>

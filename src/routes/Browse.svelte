<script lang="ts">
    import { getPlugins, searchGame, handleKeypress } from '../scripts/Browse';
    import '../styles/Browse.scss';
    import { t } from '../locale/i18n';
    import type { SearchedGame } from '../scripts/Interfaces';
    import { log } from '../scripts/Main';

    // Defines variables for the:
    // - Search text
    // - Selected plugin
    // - And the search data
    let inputText: string;
    let selectedPlugin: string = '';
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
            handleKeypress(key, selectedPlugin, inputText)
                .then((data) => {
                    searchData = data;
                })
                .catch((error) => {
                    log(1, `No games searched. msg: ${error}`);
                    searchData = [];
                });
        }
    }}"
/>

<main class="container">
    <div class="main">
        <!-- <h1>{$t('browseText')}</h1> -->

        <div class="search">
            <button
                type="submit"
                on:click="{() =>
                    searchGame(selectedPlugin, inputText).then((data) => {
                        searchData = data;
                    })}"
            >
                <i class="fa-solid fa-magnifying-glass"></i>
            </button>
            <input
                placeholder="{$t('library.searchGame')}"
                type="text"
                bind:value="{inputText}"
            />
            <select bind:value="{selectedPlugin}" name="Plugins">
                <option selected>{$t('browse.selectPlugin')}</option>

                <!--
					Awaits the data to be resolved
					After that adds an option for each plugin
				-->
                {#await getPlugins() then plugins}
                    {#each plugins as plugin}
                        <option value="{plugin.path}">{plugin.name}</option>
                    {/each}
                {/await}
            </select>
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
                <p>{Response.title}</p>
                {#each Response.links as url}
                    <a href="{url.url}" target="_blank" rel="noreferrer">
                        <i class="fa-solid fa-download"></i>
                        {url.label}
                    </a>
                {/each}
            </div>
        {/each}
    </div>
</main>

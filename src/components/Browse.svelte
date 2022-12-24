<script lang="ts">
	import { browse, search, displayResults } from '../scripts/Browse';
	import '../styles/Browse.css';
	import { t } from '../locale/i18n';

  let key: string;

  const handleKeypress = (event: any) => {
    key = event.key;
    
    if (key.toString() == "Enter") {
      search(selectedScraper, inputText).then(() => {
        searchData = displayResults()
      })
    }
  }
  

	// Defines variables for the:
	// - Search text
	// - Scraper selected
	// - And the search data
	let inputText: string;
	let selectedScraper: string;
	let searchData: any = {
		response: [],
	};

	const data = browse();
</script>

<svelte:window on:keydown="{handleKeypress}" />


<main class="container">
	<div class="main">
		<h1>{$t('browseText')}</h1>

		<div class="search">
			<input
				placeholder="{$t('browse.search')}"
				type="text"
				bind:value="{inputText}"
			/>
			<select bind:value="{selectedScraper}" name="Plugins">
				<option value="Select">{$t('browse.selectPlugin')}</option>

				<!-- Awaits the data to be resolved -->
				<!-- After that adds an option for each scraper -->
				<!-- svelte-ignore empty-block -->
				{#await data then d}
					{#each d.scrapers as Scrapers}
						<option value="{Scrapers.location}"
							>{Scrapers.name.replace(
								/(\.exe)|(\.lua)/g,
								''
							)}</option
						>
					{/each}
				{/await}
			</select>

			<!-- When the button is clicked, refefine the var searchData -->
			<!-- With the function displayResults -->
			<button
				type="submit"
				on:click="{() =>
					search(selectedScraper, inputText).then(() => {
						searchData = displayResults();
					})}"
			>
				<i class="fa-solid fa-magnifying-glass"></i>
			</button>
		</div>

		<!-- Awaits for search data to be resolved-->
		<!-- After that add an div with the game title -->
		<!-- And url for each object -->
		{#await searchData then sd}
			{#each sd.response as Response}
				{#if sd.response.length == 0}
					<h1>{$t('browse.nothingFound')}</h1>
				{/if}
				<div class="game">
					<p>{Response.title}</p>
					{#each Response.urls as urls}
						<a href="{urls}" target="_blank" rel="noreferrer">
							<i class="fa-solid fa-download"></i>
							{$t('browse.downloadText')}
						</a>
					{/each}
				</div>
			{/each}
		{/await}
	</div>
</main>

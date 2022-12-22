<script lang="ts">
	// Imports all pages and modules
	import translations from './locale/locales';
	import Browse from './components/Browse.svelte';
	import Downloads from './components/Downloads.svelte';
	import Home from './components/Home.svelte';
	import Library from './components/Library.svelte';
	import Preferences from './components/Preferences.svelte';
	import { Modal } from 'svelte-simple-modal';
	import { dict, t } from './locale/i18n';
	import { loadLocale } from './scripts/Main';

	$: dict.set(translations);

	// Loads the current locale
	loadLocale();

	var currentPage: number;

	enum Pages {
		Home = 0,
		Browse = 2,
		Library = 1,
		Downloads = 3,
		Prefs = 4,
	}

	// TS Function
	// - Changes the currentPage variable to the number on args
	function SwitchPage(Option: number) {
		currentPage = Option;
	}
</script>

<svelte:head>
	<script
		src="https://kit.fontawesome.com/dacbc752b2.js"
		crossorigin="anonymous"
	></script>
</svelte:head>

<!-- Only touch this file if adding a new page -->
<!-- Or styling a Modal -->
<!-- Otherwise, ignore it -->
<main class="container">
	<div class="sidenav">
		<img src="icon.png" width="100" class="branding" alt="branding" />
		<button class="menu-button" on:click="{() => SwitchPage(Pages.Home)}">
			{$t('homeText')}
		</button>
		<button class="menu-button" on:click="{() => SwitchPage(Pages.Browse)}">
			{$t('browseText')}
		</button>
		<button
			class="menu-button"
			on:click="{() => SwitchPage(Pages.Library)}"
		>
			{$t('libraryText')}
		</button>
		<button
			class="menu-button"
			on:click="{() => SwitchPage(Pages.Downloads)}"
		>
			{$t('downloadsText')}
		</button>
		<button class="menu-button" on:click="{() => SwitchPage(Pages.Prefs)}">
			{$t('prefsText')}
		</button>
	</div>

	{#if currentPage == 0}
		<Home />
	{:else if currentPage == 1}
		<Modal
			styleBg="{{ backgroundColor: 'rgba(0, 0, 0, 0.5)' }}"
			styleWindow="{{
				backgroundColor: '#000',
				border: '1px solid #00ff00',
				borderRadius: '0px',
				float: 'center',
			}}"
			closeButton="{false}"
		>
			<Library />
		</Modal>
	{:else if currentPage == 2}
		<Browse />
	{:else if currentPage == 3}
		<Downloads />
	{:else if currentPage == 4}
		<Preferences />
	{:else}
		<Home />
	{/if}
</main>

<script lang="ts">
	// Imports all pages and modules
	import translations from './locale/locales';
	import Browse from './routes/Browse.svelte';
	import Library from './routes/Library.svelte';
	import Preferences from './routes/Preferences.svelte';
	import { Modal } from 'svelte-simple-modal';
	import { dict, t } from './locale/i18n';
	import { loadLocale } from './scripts/Main';
	import { Router, Link, Route } from 'svelte-navigator';
	$: dict.set(translations);

	// Loads the current locale
	loadLocale();
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
<Router>
	<main
		class="container"
		style="
		--item-bg-color: #00000065;
		--bg: #121719;
		--bg-darker: #0b0e10;
		--accent-color: #733380;
		--darker-accent: #632c6e;
		--text-color: #fff;
		--black-text: #000;
		--border-search-color: #ffffff68;
		--white-bg-color: #ffffff;
		--white-comp-color: #d2d5d6;
		--white-ph-color: #696969;
		--edit-game-color: #0087fe;
		--delete-game-color: #ff0000;
		--game-modal-border-color: #fff;
		--game-modal-bg: #0b0e10bd;
		--game-modal-run: #fff;
		--settings-button: #24282a;
	"
	>
		<div class="sidenav">
			<img src="Logo.svg" width="100" class="branding" alt="branding" />
			<Link class="menu-button" to="browse">{$t('browseText')}</Link>
			<Link class="menu-button" to="/">{$t('libraryText')}</Link>
			<Link class="menu-button" to="prefs">{$t('prefsText')}</Link>
		</div>
		<Route path="browse">
			<Browse />
		</Route>

		<Route path="/" primary="{false}">
			<Modal
				styleBg="{{ backgroundColor: 'rgba(0, 0, 0, 0.2)' }}"
				styleWindow="{{
					backgroundColor: 'var(--bg)',
					border: '1px solid var(--bg)',
					borderRadius: '30px',
					float: 'center',
				}}"
				transitionBgProps="{{ duration: 0 }}"
				closeButton="{false}"
			>
				<Library />
			</Modal>
		</Route>

		<Route path="prefs">
			<Preferences />
		</Route>
	</main>
</Router>

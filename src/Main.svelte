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
	<main class="container">
		<div class="sidenav">
			<img src="icon.png" width="100" class="branding" alt="branding" />
			<Link class="menu-button" to="browse">{$t('browseText')}</Link>
			<Link class="menu-button" to="/">{$t('libraryText')}</Link>
			<Link class="menu-button" to="prefs">{$t('prefsText')}</Link>
		</div>
		<Route path="browse">
			<Browse />
		</Route>

		<Route path="/" primary="{false}">
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
		</Route>

		<Route path="prefs">
			<Preferences />
		</Route>
	</main>
</Router>

<script lang="ts">
	// Imports all pages and modules
	import translations from './locale/locales.js';
	import Browse from './routes/Browse.svelte';
	import Library from './routes/Library.svelte';
	import Preferences from './routes/Preferences.svelte';
	import { Modal } from 'svelte-simple-modal';
	import { dict, t } from './locale/i18n.js';
	import { loadLocale } from './scripts/Main.js';
	import { Router, Link, Route } from 'svelte-navigator';
	import { Grid, AppsOutline, SettingsOutline } from 'svelte-ionicons';
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
		--item-bg-color: #0B0E10;
		--bg: #121719;
		--bg-darker: #0b0e10;
		--accent-color: #0B0D0E;
		--darker-accent: #1B2427;
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
		--input-blur-size: 2px;
		--game-modal-blur-size: 6px;
		--border-radius: 10px;
	"
	>
		<div class="sidenav">
			<div class="branding">
				<img src="Logo.svg" width="100" alt="branding" />
			</div>

			<div class="menu-item">
				<div class="menu-button">
					<AppsOutline size="20px" />
					<Link class="link" to="browse">{$t('browseText')}</Link>
				</div>
			</div>
			<div class="menu-item">
				<div class="menu-button">
					<Grid size="20px" />
					<Link class="link" to="/">{$t('libraryText')}</Link>
				</div>
			</div>
			<div class="menu-item">
				<div class="menu-button">
					<SettingsOutline size="20px" />
					<Link class="link" to="prefs">{$t('prefsText')}</Link>
				</div>
			</div>
		</div>
		<Route path="browse">
			<Browse />
		</Route>

		<Route path="/" primary="{false}">
			<Modal
				styleBg="{{ backgroundColor: 'rgba(0, 0, 0, 0.2)' }}"
				styleWindow="{{
					backgroundColor: '#0b0d0e',
					border: '1px solid #0b0d0e',
					borderRadius: '15px',
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

<script lang="ts">
	import translations from '../locale/locales.js';
	import { dict, locale, t } from '../locale/i18n.js';
	import '../styles/Preferences.scss';
	import {
		installScraper,
		saveLangData,
		wipeLibrary,
	} from '../scripts/Preferences.js';
	import languageNames from '../locale/languages.json';
	import { getScrapers } from '../scripts/Browse.js';
	import pkgJSON from '../../package.json';
	import { Cube, TrashBin, Albums } from 'svelte-ionicons';

	const scrapersList = getScrapers();

	$: languages = Object.keys(translations);
	$: dict.set(translations);
</script>

<main class="container">
	<div class="main">
		<div class="section">
			<div class="plugin-card">
				<div class="header">
					<span>{$t('pluginText')}</span>
					<Cube size="18px" />
				</div>
				<div class="buttons">
					<button id="install" on:click="{installScraper}">
						<Cube class="cube" size="18px" />
						{$t('preferences.installPlugin')}</button
					>
					<button id="wipe" on:click="{wipeLibrary}"
						><TrashBin class="bin" size="18px" />
						{$t('preferences.wipeLibrary')}</button
					>
				</div>
			</div>

			<div class="plugin-card">
				<div class="header">
					<span>{$t('themeText')}</span>
					<Albums size="18px" />
				</div>
				<p class="buttons">{$t('comingSoonText')}</p>
			</div>

			<div class="plugin-card">
				<div class="header">
					<span>{$t('languageText')}</span>
					<Cube size="18px" />
				</div>
				<div class="buttons">
					<select bind:value="{$locale}">
						{#each languages as languageCode, i}
							<option value="{languageCode}"
								>{languageNames[i]}</option
							>
						{/each}
					</select>

					<button
						class="save-button"
						on:click="{() => saveLangData($locale)}"
						>{$t('preferences.saveText')}</button
					>
				</div>
			</div>
		</div>

		<div class="available-plugins">
			<div class="header">
				<span>{$t('preferences.availablePlugins')}</span>
				<Cube size="18px" />
			</div>
			<div class="buttons">
				{#await scrapersList then scrapersList}
					{#each scrapersList.scrapers as scraper}
						<ul>- {scraper.name}</ul>
					{/each}
				{/await}
			</div>
		</div>
		<span class="ver"> Project Black Pearl v{pkgJSON.version} </span>
	</div>
</main>

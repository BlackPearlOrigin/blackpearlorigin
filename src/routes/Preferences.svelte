<script lang="ts">
	import translations from '../locale/locales';
	import { dict, locale, t } from '../locale/i18n';
	import '../styles/Preferences.scss';
	import {
		installScraper,
		saveLangData,
		wipeLibrary,
	} from '../scripts/Preferences';
	import languageNames from '../locale/languages.json';
	import { getScrapers } from '../scripts/Browse';

	const scrapersList = getScrapers();

	$: languages = Object.keys(translations);
	$: dict.set(translations);
</script>

<main class="container">
	<div class="main">
		<h1>{$t('prefsText')}</h1>
		<div class="section">
			<div class="button-group">
				<button id="install" on:click="{installScraper}"
					><i class="fa-solid fa-download"></i>
					{$t('preferences.installPlugin')}</button
				>
				<button id="wipe" on:click="{wipeLibrary}"
					><i class="fa-solid fa-trash-can"></i>
					{$t('preferences.wipeLibrary')}</button
				>
			</div>
			<div class="availb-plugins">
				<label for="available-plugins">Available plugins:</label>
				<div class="available-plugins">
					{#await scrapersList then scrapersList}
						{#each scrapersList.scrapers as scraper}
							<span>{scraper.name}</span>
						{/each}
					{/await}
				</div>
			</div>
			<div class="lang-settings">
				<label for="select">{$t('languageText')}:</label>
				<div class="locale-settings">
					<select bind:value="{$locale}">
						{#each languages as languageCode, i}
							<option value="{languageCode}"
								>{languageNames[i]}</option
							>
						{/each}
					</select>
				</div>
			</div>
			<button class="save-button" on:click="{() => saveLangData($locale)}"
				>{$t('preferences.saveText')}</button
			>
		</div>
	</div>
</main>

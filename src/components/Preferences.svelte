<script lang="ts">
	import translations from '../locale/locales';
	import { dict, locale, t } from '../locale/i18n';
	import '../styles/Preferences.css';
	import {
		installScraper,
		saveData,
		wipeLibrary,
	} from '../scripts/Preferences';

	$: languages = Object.keys(translations);
	$: dict.set(translations);
</script>

<main class="container">
	<div class="main">
		<h1>{$t('prefsText')}</h1>
		<div class="section">
			<div class="button-group">
				<button id="install" on:click="{installScraper}"
					>{$t('preferences.installPlugin')}</button
				>
				<button id="wipe" on:click="{wipeLibrary}"
					>{$t('preferences.wipeLibrary')}</button
				>
			</div>
			<label for="select">{$t('languageText')}</label>
			<div class="locale-settings">
				<select bind:value="{$locale}">
					{#each languages as lang}
						<option value="{lang}">{lang}</option>
					{/each}
				</select>
			</div>
			<button class="save-button" on:click="{() => saveData($locale)}"
				>{$t('preferences.saveText')}</button
			>
		</div>
	</div>
</main>

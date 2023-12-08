<script lang="ts">
    import translations from '../locale/locales.js';
    import { dict, locale, t } from '../locale/i18n.js';
    import {
        saveData,
        resetTheme,
        wipeLibrary,
    } from '../scripts/Preferences.js';
    import languageNames from '../locale/languages.json';
    import {
        Cube,
        TrashBin,
        Albums,
        OpenOutline,
        Close,
        CloudDownload,
        CloudUpload,
    } from 'svelte-ionicons';
    import { convertFileSrc } from '@tauri-apps/api/tauri';

    $: languages = Object.keys(translations);
    $: dict.set(translations);

    let stylesheetUrl: string;
    let updaterStatus: boolean;
</script>

<main class="container">
    <div class="main">
        <div class="section">
            <div class="plugin-card">
                <div class="header">
                    <span>{$t('themeText')}</span>
                    <Albums size="18px" />
                </div>

                <div class="buttons">
                    <input
                        type="text"
                        class="input"
                        placeholder="{$t('preferences.insertThemeURL')}"
                        bind:value="{stylesheetUrl}"
                    />

                    <button
                        on:click="{() =>
                            resetTheme().then(() => {
                                stylesheetUrl = '';
                            })}">{$t('preferences.resetToDefault')}</button
                    >
                </div>
            </div>

            <div class="plugin-card">
                <div class="header">
                    <span>Updater</span>
                    <CloudDownload size="18px" />
                </div>

                <div class="checkbox">
                    <input type="checkbox" bind:checked="{updaterStatus}" />
                    <p>{$t('preferences.suppressUpdater')}</p>
                </div>
            </div>

            <div class="plugin-card">
                <div class="header">
                    <span>{$t('languageText')}</span>
                    <Cube size="18px" />
                </div>
                <div class="buttons">
                    <select bind:value="{$locale}">
                        {#each languages as languageCode, i}
                            <option value="{languageCode}">
                                {languageNames[i]}
                            </option>
                        {/each}
                    </select>
                </div>
            </div>

            <div class="plugin-card">
                <div class="header">
                    <span>Save</span>
                    <CloudUpload size="18px" />
                </div>

                <div class="buttons">
                    <button
                        class="save-button"
                        on:click="{() =>
                            saveData(
                                $locale,
                                updaterStatus,
                                stylesheetUrl.startsWith(
                                    'https://raw.githubusercontent',
                                ) || stylesheetUrl === undefined
                                    ? stylesheetUrl
                                    : convertFileSrc(stylesheetUrl),
                            )}"
                    >
                        {$t('preferences.saveText')}
                    </button>
                </div>
            </div>
        </div>
    </div>
</main>

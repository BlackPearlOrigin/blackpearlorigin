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
        Albums,
        CloudDownload,
        CloudUpload,
        ExtensionPuzzle,
    } from 'svelte-ionicons';
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import { getConfig } from '../scripts/Main.js';

    $: languages = Object.keys(translations);
    $: dict.set(translations);

    let stylesheetUrl: string;
    let updaterStatus: boolean;
    let enabledScrapers: { [key: string]: boolean } = {
        rezi: true,
        fitgirl: true,
    };

    const loadData = async () => {
        const config = await getConfig();

        stylesheetUrl = config.cssUrl;
        updaterStatus = config.updater;
        enabledScrapers = config.enabledScrapers;
    };

    loadData();
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
                        placeholder="Insert a theme URL"
                        bind:value="{stylesheetUrl}"
                    />

                    <button
                        on:click="{() =>
                            resetTheme().then(() => {
                                stylesheetUrl = '';
                            })}">Reset to default</button
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
                    <p>Suppress updater</p>
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
                                stylesheetUrl === undefined
                                    ? undefined
                                    : convertFileSrc(stylesheetUrl),
                                enabledScrapers,
                            )}"
                    >
                        {$t('preferences.saveText')}
                    </button>
                </div>
            </div>

            <div class="plugin-card">
                <div class="header">
                    <span>Scrapers</span>
                    <ExtensionPuzzle size="18px"></ExtensionPuzzle>
                </div>
                <div class="selector" id="first">
                    <input
                        type="checkbox"
                        bind:checked="{enabledScrapers.rezi}"
                    />
                    <p>Rezi</p>
                </div>
                <div class="selector">
                    <input
                        type="checkbox"
                        bind:checked="{enabledScrapers.fitgirl}"
                    />
                    <p>FitGirl</p>
                </div>
            </div>
        </div>
    </div>
</main>

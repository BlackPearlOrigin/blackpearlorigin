<script lang="ts">
    import translations from '../locale/locales.js';
    import { dict, locale, t } from '../locale/i18n.js';
    import '../styles/Preferences.scss';
    import {
        installPlugin,
        saveLangData,
        wipeLibrary,
    } from '../scripts/Preferences.js';
    import languageNames from '../locale/languages.json';
    import { getPlugins } from '../scripts/Browse.js';
    import pkgJSON from '../../package.json';
    import {
        Cube,
        TrashBin,
        Albums,
        OpenOutline,
        Close,
    } from 'svelte-ionicons';

    const plugins = getPlugins();

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
                    <button id="install" on:click="{installPlugin}">
                        <Cube class="cube" size="18px" />
                        {$t('preferences.installPlugin')}
                    </button>
                    <button id="wipe" on:click="{wipeLibrary}">
                        <TrashBin class="bin" size="18px" />
                        {$t('preferences.wipeLibrary')}
                    </button>
                </div>
            </div>

            <div class="plugin-card">
                <div class="header">
                    <span>{$t('themeText')}</span>
                    <Albums size="18px" />
                </div>
                <p class="buttons">{$t('preferences.comingSoon')}</p>
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

                    <button
                        class="save-button"
                        on:click="{() => saveLangData($locale)}"
                    >
                        {$t('preferences.saveText')}
                    </button>
                </div>
            </div>
        </div>

        <div class="available-plugins">
            <div class="header">
                <span>{$t('preferences.availablePlugins')}</span>
                <Cube size="18px" />
            </div>
            <ul class="cards">
                {#await plugins then plugins}
                    {#each plugins as plugin}
                        <div class="card">
                            <div class="card-left">
                                <p class="card-header">
                                    {plugin.name}
                                    <a
                                        href="{plugin.source}"
                                        target="_blank"
                                        rel="noreferrer"
                                        title="Open plugin source"
                                    >
                                        <OpenOutline size="20px" />
                                    </a>
                                </p>
                                <div class="card-footer">
                                    <span class="version">
                                        <b>
                                            {$t(
                                                'preferences.pluginCard.version'
                                            )}
                                        </b>
                                        {plugin.version}
                                    </span>
                                    <span class="author" title="Plugin author">
                                        <b>
                                            {$t(
                                                'preferences.pluginCard.author'
                                            )}
                                        </b>
                                        {plugin.author}
                                    </span>
                                    <span class="desc">
                                        <b>
                                            {$t('preferences.pluginCard.desc')}
                                        </b>
                                        {plugin.description}
                                    </span>
                                </div>
                            </div>

                            <div class="buttons">
                                <!-- TODO: Actually make the button work -->
                                <button class="remove">
                                    <Close size="22px" />
                                </button>
                            </div>
                        </div>
                    {/each}
                {/await}
            </ul>
        </div>
        <span class="ver"> Project Black Pearl v{pkgJSON.version} </span>
    </div>
</main>

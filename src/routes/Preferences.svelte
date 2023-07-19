<script lang="ts">
    import translations from '../locale/locales.js';
    import { dict, locale, t } from '../locale/i18n.js';
    import '../styles/Preferences.scss';
    import {
        installPlugin,
        saveData,
        steamLogin,
        steamUnlink,
        uninstallPlugin,
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
        CloudDownload,
        CloudUpload,
        Link,
    } from 'svelte-ionicons';
    import { getConfig } from '../scripts/Main.js';

    const plugins = getPlugins();

    $: languages = Object.keys(translations);
    $: dict.set(translations);

    let updaterStatus: boolean;
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
                    <span>Updater</span>
                    <CloudDownload size="18px" />
                </div>

                <div class="checkbox">
                    <input type="checkbox" bind:checked="{updaterStatus}" />
                    <p>Turn updater off</p>
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
                    <span>{$t('steamLoginText')}</span>
                    <Cube size="18px" />
                </div>
                <div class="buttons">
                    {#if localStorage.getItem('steamData') !== null && localStorage
                            .getItem('steamData')
                            .includes('steam')}
                        <div class="steam">
                            <div class="steamUser">
                                <img
                                    class="steamUserPicture"
                                    src="{JSON.parse(
                                        localStorage.getItem('steamData')
                                    ).avatarfull}"
                                    alt="{JSON.parse(
                                        localStorage.getItem('steamData')
                                    ).personaname}"
                                />
                                {JSON.parse(localStorage.getItem('steamData'))
                                    .personaname}
                            </div>
                            <button on:click="{steamUnlink}">
                                {$t('preferences.steamUnlink')}</button
                            >
                        </div>
                    {:else}
                        <button on:click="{steamLogin}">
                            <Link class="bin" size="18px" />
                            {$t('preferences.steamLogin')}
                        </button>
                    {/if}
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
                        on:click="{() => saveData($locale, updaterStatus)}"
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
                                <button
                                    class="remove"
                                    on:click="{() => uninstallPlugin(plugin)}"
                                >
                                    <Close size="22px" />
                                </button>
                            </div>
                        </div>
                    {/each}
                    {#if plugins.length === 0}
                        <h3 class="noresults">None yet...</h3>
                    {/if}
                {/await}
            </ul>
        </div>
        <span class="ver"> Black Pearl Origin v{pkgJSON.version} </span>
    </div>
</main>

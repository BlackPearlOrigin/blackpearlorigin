<script lang="ts">
    // Imports all pages and modules
    import translations from './locale/locales.js';
    import Browse from './routes/Browse.svelte';
    import Library from './routes/Library.svelte';
    import Preferences from './routes/Preferences.svelte';
    import Toast from './routes/modals/Toast.svelte';
    import { Modal } from 'svelte-simple-modal';
    import { dict, t } from './locale/i18n.js';
    import { getConfig, loadLocale } from './scripts/Main.js';
    import { Router, Link, Route } from 'svelte-navigator';
    import { Grid, AppsOutline, SettingsOutline } from 'svelte-ionicons';
    import { checkUpdate } from '@tauri-apps/api/updater';
    import { SvelteToast, toast } from '@zerodevx/svelte-toast';

    $: dict.set(translations);

    // Loads the current locale
    loadLocale();

    (async () => {
        const config = await getConfig();

        const { shouldUpdate } = await checkUpdate();

        if (!config.updater) return;
        if (shouldUpdate) {
            toast.push('New update available', {
                component: {
                    src: Toast,
                },
                theme: {
                    '--toastBackground': '#171717',
                },
                duration: 6000,
                pausable: true,
            });
        } else {
        }
    })();
</script>

<svelte:head>
    <script
        src="https://kit.fontawesome.com/dacbc752b2.js"
        crossorigin="anonymous"
    ></script>
</svelte:head>

<SvelteToast />

<!-- Only touch this file if adding a new page -->
<!-- Or styling a Modal -->
<!-- Otherwise, ignore it -->
<body>
    <Router>
        <main class="container">
            <div class="sidenav">
                <div class="branding">
                    <img src="bpo.png" width="100" alt="branding" />
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
</body>

<script lang="ts">
    // Imports all pages and modules
    import translations from './locale/locales.js';
    import Browse from './routes/Browse.svelte';
    import Library from './routes/Library.svelte';
    import Preferences from './routes/Preferences.svelte';
    import Toast from './routes/modals/Toast.svelte';
    import { Modal } from 'svelte-simple-modal';
    import { dict, t } from './locale/i18n.js';
    import { loadLocale } from './scripts/Main.js';
    import { Router, Link, Route } from 'svelte-navigator';
    import { Grid, AppsOutline, SettingsOutline } from 'svelte-ionicons';
    import { checkUpdate } from '@tauri-apps/api/updater';
    import { SvelteToast, toast } from '@zerodevx/svelte-toast';

    $: dict.set(translations);

    // Loads the current locale
    loadLocale();

    (async () => {
        const { shouldUpdate } = await checkUpdate();

        if (shouldUpdate) {
            toast.push('New update available', {
                component: {
                    src: Toast,
                },
                theme: {
                    '--toastBackground': '#171717',
                },
                duration: 10000,
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
<body
    style="
		--shadow-color: rgba(0, 0, 0, 0.25);
		--radius-round: 4px;
		--radius-normal: 4px;
		--font-color: #dddddd;
		--global-background-color: #171717;
		--sidebar-background-color: #101010;
		--sidebar-font-weight: 300;
		--sidebar-logo-color: #fff;
		--settings-container-color: #101010;
		--settings-container-button-color: #121212;
		--settings-container-button-border-color: #323232;
		--library-border-color: #323232;
		--library-element-background: #121212;
        --library-element-background-hovered: #181818;
		--library-search-placeholder-color: #dddddd;
		--library-search-font-color: #dddddd;
		--library-name-font-weight: 400;
		--library-game-modal-title-font-weight: 800;
		--library-game-modal-desc-font-weight: 400;
		--library-game-modal-run-button-background: #121212;
		--library-game-modal-run-button-font-color: #dddddd;
		--library-game-modal-run-button-font-weight: 900;
		--library-game-modal-edit-button-color: #fff;
		--library-game-modal-delete-button-color: #fff;
		--library-game-modal-edit-delete-size: 2rem;
		--library-game-modal-background-color: #101010;
		--browse-border-color: #323232;
		--browse-element-background: #121212;
        --browse-element-background-hovered: #181818;
		--browse-input-placeholder-color: #dddddd;
		--browse-element-font-weight: 500;
		--browse-game-border: #323232;
		--browse-game-background-color: #0b0e10;
		--browse-game-dl-border: #323232;
		--browse-game-name-font-weight: 700;
		--browse-game-dl-font-weight: 500;
		--modal-background-color: #0b0d0e;
		--modal-element-border-color: #323232;
		--modal-element-background-color: #121212;
		--modal-img-background-color: #0b0d0e;
		--modal-img-border-color: #323232;
		--modal-fetch-background-color: #0b0d0e;
		--modal-fetch-font-weight: 500;
		--modal-fetch-hover-color: #2d3739;
		--modal-border-color: #323232;
		--modal-placeholder-color: #fff;
	"
>
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

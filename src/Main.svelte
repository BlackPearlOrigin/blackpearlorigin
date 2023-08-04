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
    import NewGame from './routes/modals/NewGame.svelte';
    import { getContext } from 'svelte';

    import { getGames } from './scripts/Library.js';

    $: dict.set(translations);

    // Gets the open function from simple-modal context

    // TODO: Move this.
    // When the modal is closed re-run the function getGames
    const showNewModal = () =>
        open(
            // 1° Arg: Component
            // 2° Arg: Props
            // 3° Arg: Options
            // 4° Arg: Callbacks
            NewGame,
            {},
            {},
            {
                onClose: () => {
                    games = getGames();
                },
            }
        );

    // Loads the current locale
    loadLocale();

    let games: any = getGames();

    (async () => {
        const config = await getConfig();

        const { shouldUpdate } = await checkUpdate();

        if (!config.updater) return;
        if (shouldUpdate) {
            toast.push('New update available', {
                component: {
                    src: Toast,
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
            <div class="top">
                <input
                    type="text"
                    placeholder="{$t('library.searchGame')}"
                    class="search-bar"
                    on:keydown="{(event) =>
                        // TODO: Dropdown with entries once something is typed & new page at "Add New" instead of modal.
                        event.key === 'Enter' ? showNewModal() : undefined}"
                />
            </div>
            <div class="sidenav">
                <div class="branding">
                    <img src="bpo.png" width="100" alt="branding" />
                </div>

                <!-- TODO: When searching and Enter is pressed, open Browse - otherwise this page is hidden (nothing is searched) -->

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

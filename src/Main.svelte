<script lang="ts">
    // Imports all pages and modules
    import translations from './locale/locales.js';
    import Browse from './routes/Browse.svelte';
    import Library from './routes/Library.svelte';
    import Preferences from './routes/Preferences.svelte';
    import { Modal } from 'svelte-simple-modal';
    import { dict, t } from './locale/i18n.js';
    import { loadLocale } from './scripts/Main.js';
    import { Router, Link, Route } from 'svelte-navigator';
    import { Grid, AppsOutline, SettingsOutline } from 'svelte-ionicons';
    $: dict.set(translations);

    // Loads the current locale
    loadLocale();
</script>

<svelte:head>
    <script
        src="https://kit.fontawesome.com/dacbc752b2.js"
        crossorigin="anonymous"
    ></script>
</svelte:head>

<!-- Only touch this file if adding a new page -->
<!-- Or styling a Modal -->
<!-- Otherwise, ignore it -->
<body
    style="
		--shadow-color: rgba(0, 0, 0, 0.25);
		--radius-round: 100px;
		--radius-normal: 15px;
		--font-color: #fff;
		--global-background-color: #2d3739;
		--sidebar-background-color: #0b0d0e;
		--sidebar-font-weight: 300;
		--sidebar-logo-color: #fff;
		--sidebar-button-hover: #2d3739;
		--settings-container-color: #1e1e1e;
		--settings-container-button-color: #0b0e10;
		--settings-container-button-border-color: #0b0e10;
		--library-border-color: #fff;
		--library-element-background: #0b0e10;
		--library-search-placeholder-color: #fff;
		--library-search-font-color: #fff;
		--library-name-font-weight: 400;
		--library-game-modal-title-font-weight: 800;
		--library-game-modal-desc-font-weight: 400;
		--library-game-modal-run-button-background: #fff;
		--library-game-modal-run-button-font-color: #000;
		--library-game-modal-run-button-font-weight: 900;
		--library-game-modal-edit-button-color: #0087fe;
		--library-game-modal-delete-button-color: #ff0000;
		--library-game-modal-edit-delete-size: 2rem;
		--library-game-modal-background-color: #0b0d0e;
		--browse-border-color: #fff;
		--browse-element-background: #0b0e10;
		--browse-input-placeholder-color: #fff;
		--browse-element-font-weight: 500;
		--browse-game-border: #fff;
		--browse-game-background-color: #0b0e10;
		--browse-game-dl-border: #fff;
		--browse-game-name-font-weight: 700;
		--browse-game-dl-font-weight: 500;
		--modal-background-color: #0b0d0e;
		--modal-element-border-color: #fff;
		--modal-element-background-color: #0b0d0e;
		--modal-img-background-color: #0b0d0e;
		--modal-img-border-color: #0b0d0e;
		--modal-fetch-background-color: #0b0d0e;
		--modal-fetch-font-weight: 500;
		--modal-fetch-hover-color: #2d3739;
		--modal-border-color: #fff;
		--modal-placeholder-color: #fff;
	"
>
    <Router>
        <main class="container">
            <div class="sidenav">
                <div class="branding">
                    <img src="Logo.svg" width="100" alt="branding" />
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

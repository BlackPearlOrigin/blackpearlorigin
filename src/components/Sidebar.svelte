<script lang="ts">
    import { t } from '../locale/i18n.js';
    import { Link } from 'svelte-navigator';
    import { Grid, AppsOutline, SettingsOutline } from 'svelte-ionicons';
</script>

<nav class="container">
    <div class="sidenav">
        <div class="branding">
            <img src="bpo.png" width="100" alt="branding" />
        </div>

        <div class="menu-item">
            <div class="menu-button">
                <AppsOutline size="20px" />
                <Link class="link" to="/browse">{$t('browseText')}</Link>
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
                <Link class="link" to="/prefs">{$t('prefsText')}</Link>
            </div>
        </div>
    </div>
</nav>

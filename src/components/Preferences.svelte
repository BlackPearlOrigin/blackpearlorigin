<script lang="ts">
    import "../styles/Preferences.css";
    import { installScraper, wipeLibrary } from "../scripts/Preferences";
</script>

<main class="container">
    <div class="main">
        <h1>Preferences</h1>
        <div class="section">
            <button on:click={installScraper}>Install a scraper</button>
            <button class="wipe" on:click={wipeLibrary}>Wipe library</button>
        </div>
    </div>
</main>

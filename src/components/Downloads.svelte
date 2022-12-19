<script lang="ts">
    import "../styles/Downloads.css";
    import { downloadTest } from "../scripts/Downloads";
</script>

<main class="container">
    <div class="main">
        <h1>Downloads</h1>
        <button on:click={downloadTest}>Download test</button>
    </div>
</main>

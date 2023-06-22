<script lang="ts">
    import { log } from '../../scripts/Main';
    import '../../styles/Modal.scss';

    import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
    import { relaunch } from '@tauri-apps/api/process';

    const update = async () => {
        const { manifest } = await checkUpdate();

        log(
            0,
            `Installing update: ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
        );

        await installUpdate();
        await relaunch();
    };
</script>

<div class="toast">
    <p>New update available.</p>
    <button on:click="{update}">Download</button>
</div>

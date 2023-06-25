<script lang="ts">
    import { log } from '../../scripts/Main';
    import '../../styles/Modal.scss';
    import { t } from '../../locale/i18n';
    import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
    import { relaunch } from '@tauri-apps/api/process';
    import { toast } from '@zerodevx/svelte-toast';

    const update = async (toastText) => {
        // const { manifest } = await checkUpdate();

        // log(
        //     0,
        //     `Installing update: ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
        // );

        // await installUpdate();
        toast.push(toastText, {
            theme: {
                '--toastBackground': '#171717',
            },
            duration: 5000,
        });

        setTimeout(async () => {
            // await relaunch();
        }, 5000);
    };
</script>

<div class="toast">
    <p>{$t('modals.updater.updateAvailable')}</p>
    <button on:click="{() => update($t('modals.updater.reboot'))}"
        >Download</button
    >
</div>

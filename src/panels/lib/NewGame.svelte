<script lang="ts">
    import { writable } from "svelte/store";

    let title: string;
    let exeLocation: string;
    let imgLocation: string;
    let inn;

    function returnData() {
        let resObj: object = {
            title: title,
            exe: exeLocation,
            img: imgLocation
        }

        inn = "Data saved, you can now close the modal"
    }

    // Implement game adding here
    // Zuni
</script>

<div>
    <div class="newgame">
        <input type="text" name="Title" placeholder="Title" bind:value={title}>
        <input type="text" name="Location" placeholder="Location" bind:value={exeLocation}>
        <input type="text" name="Image" placeholder="Image" bind:value={imgLocation}>
    </div>

    <p contenteditable="true" bind:innerHTML={inn}>
    </p>

    <button on:click={returnData}>
        Done
    </button>
</div>

<style>
    .newgame {
        display: flex;
        flex-grow: 1;
        flex-direction: column;

        padding-right: 5%;
    }

    .newgame > input::placeholder {
        color: rgb(75, 75, 75)
    }

    .newgame > input {
        font-size: 17px;
    }

    button {
        font-size: 13px;
    }

    p {
        line-height: 0px;
        padding-left: 3px;
    }
</style>
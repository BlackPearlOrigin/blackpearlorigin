<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { getContext } from 'svelte';
	import './../../styles/Modal.scss';
	import { t } from '../../locale/i18n';
	import { saveData, editData } from '../../scripts/Library';

	const { close }: any = getContext('simple-modal');
	export let title: string;
	export let id: number;
	export let executablePath: any;
	export let description: string;
	export let imagePath: any;

	// Defines a function that checks if the same string is empty
	function isEmpty(string: string) {
		return string === undefined || string.length === 0 || !string.trim();
	}

	// TS Function -> Rust Function
	// - Opens a File selector dialog
	function chooseExecutable() {
		invoke('file_dialog').then((message) => (executablePath = message));
	}
	function chooseImage() {
		invoke('image_dialog').then((message) => (imagePath = message));
	}

	export let operationToPerform: string = 'Save';
	async function operation_handler(operation: string) {
		if (operation === 'Save') {
			// Checks if there is no title, description, executable or image
			// -----------------------------------------------------------
			// If the user doesn't select an title, set it as "No title"
			// If the user doesn't select an description set it blank
			// If the user doesn't select an image, set it to "None"
			// And if the user doesn't select an exec, don't allow them to confirm

			if (isEmpty(title)) title = 'No title';
			if (isEmpty(description)) description = '';
			if (isEmpty(executablePath)) return;
			if (isEmpty(imagePath)) imagePath = 'None';

			saveData(title, executablePath, description, imagePath);
			close();
		} else if (operation === 'Edit') {
			// Checks if there are no title, description, exec or img
			// If one of them isn't selected, do not let the user exit
			// Except for the description
			if (isEmpty(title)) return;
			if (isEmpty(description)) description = '';
			if (isEmpty(executablePath)) return;
			if (isEmpty(imagePath)) return;

			editData(id, title, executablePath, description, imagePath);
			close();
		}
	}
</script>

<div class="modal-main">
	<div class="newgame">
		<input
			type="text"
			name="Title"
			placeholder="{$t('modals.newGame.gameTitle')}"
			bind:value="{title}"
		/>
		<div class="show-path">
			<!-- When the button is clicked, run chooseExecutable -->
			<button on:click="{chooseExecutable}" class="ng-button"
				>{$t('modals.newGame.addExec')}</button
			>

			<!-- Binds the inner html to executablePath -->
			<p
				class="path"
				contenteditable="true"
				bind:innerHTML="{executablePath}"
			></p>
		</div>
		<div class="show-path">
			<!-- When the button is clicked, run chooseImage -->
			<button on:click="{chooseImage}" class="ng-button"
				>{$t('modals.newGame.addImg')}</button
			>

			<!-- Binds the inner html to imagePath -->
			<p
				class="path"
				contenteditable="true"
				bind:innerHTML="{imagePath}"
			></p>
		</div>
		<textarea
			name="Description"
			placeholder="{$t('modals.newGame.desc')}"
			bind:value="{description}"></textarea>
	</div>

	<!-- I think you get it by now -->
	<button
		on:click="{() => {
			operation_handler(operationToPerform);
		}}"
		class="ng-button done-btn"
	>
		{$t('modals.newGame.done')}
	</button>
</div>

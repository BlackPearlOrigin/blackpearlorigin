<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { getContext } from 'svelte';
	import './../../styles/Modal.css';
	import { t } from '../../locale/i18n';
	import { saveData, editData } from '../../scripts/Library';

	const { close }: any = getContext('simple-modal');
	export let title: string;
	export let id: number;
	export let executablePath: any;
	export let description: string;
	export let imagePath: any;

	// TS Function -> Rust Function
	// - Opens a File selector dialog
	function chooseExecutable() {
		invoke('file_dialog').then((message) => (executablePath = message));
	}
	function chooseImage() {
		invoke('image_dialog').then((message) => (imagePath = message));
	}

	export let operationToPerform: string = 'Save';
	function operation_handler(operation: string) {
		if (operation === 'Save') {
      console.log(`${title}, ${executablePath}, ${description}, ${imagePath}`)
  
      if (title === undefined) title = 'No title'
      if (description === undefined) description = 'No description'
      if (executablePath === 'None') return;
      if (imagePath === 'None') return;
			saveData(title, executablePath, description, imagePath);
		} else if (operation === 'Edit') {
			editData(id, title, executablePath, description, imagePath);
		}
		close();
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
			>
				{$t('modals.newGame.none')}
			</p>
		</div>
		<div class="show-path">
			<!-- When the button is clicked, run chooseImage -->
			<button on:click="{chooseImage}" class="ng-button"
				>{$t('modals.newGame.addImg')}</button
			>

			<!-- Binds the inner html to imagePath -->
			<p class="path" contenteditable="true" bind:innerHTML="{imagePath}">
				{$t('modals.newGame.none')}
			</p>
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
			close();
		}}"
		class="ng-button done-btn"
	>
		{$t('modals.newGame.done')}
	</button>
</div>

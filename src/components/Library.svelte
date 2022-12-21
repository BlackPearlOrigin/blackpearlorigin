<script lang="ts">
	import {
		getGames,
		deleteGame,
		runGame,
		editGame,
	} from '../scripts/Library';
	import { getContext } from 'svelte';
	import NewGame from './modals/NewGame.svelte';
	import '../styles/Library.css';
	import { t } from '../locale/i18n';

	// Gets the open function from simple-modal context
	const { open }: any = getContext('simple-modal');

	// When the modal is closed re-run the function getGames
	const showModal = () =>
		open(
			NewGame,
			{},
			{},
			{
				onClose: () => {
					games = getGames();
				},
			}
		);

	let games: any = getGames();
	function operation_handler(operation: () => void) {
		operation();
		games = getGames();
	}
</script>

<main class="container">
	<div class="main">
		<div class="top">
			<h1 style="display:inline-block;">{$t('libraryText')}</h1>
			<!-- Creates a modal when the button is clicked -->
			<button on:click="{() => showModal()}">{$t('library.add')}</button>
		</div>

		<!-- Awaits for games to be resolved -->
		<!-- After that, loop over every object in that array -->
		<!-- And add those results to a div -->
		<!-- svelte-ignore empty-block -->
		{#await games then data}
			{#each data as game}
				<div class="game-panel">
					<div class="game-text">
						<img
							class="game-image"
							src="{game.image}"
							alt="{game.name}"
							height="100"
							width="100"
						/>
						<p class="game-title">{game.name}</p>
						<p class="small-info">
							{game.playtime}
							{$t('library.playtime')}
						</p>
					</div>
					<div class="buttons">
						<button
							class="game-button-run"
							on:click="{() =>
								operation_handler(() =>
									runGame(game.exe_path)
								)}">{$t('library.run')}</button
						>
						<button
							class="game-button-run"
							on:click="{() =>
								operation_handler(() => editGame(game.name))}"
							>{$t('library.editGame')}</button
						>
						<button
							class="game-button-delete"
							on:click="{() => {
								operation_handler(() => deleteGame(game.name));
							}}">{$t('library.deleteGame')}</button
						>
					</div>
				</div>
			{/each}
		{:catch error}
			<p style="color: red">{error.message}</p>
		{/await}
	</div>
</main>

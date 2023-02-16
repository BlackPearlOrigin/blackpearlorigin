<script lang="ts">
	import {
		getGames,
		deleteGame,
		runGame,
		getFilteredGames,
		operationHandler,
	} from '../scripts/Library';
	import { getContext } from 'svelte';
	import NewGame from './modals/NewGame.svelte';
	import '../styles/Library.scss';
	import { t } from '../locale/i18n';
	import { convertFileSrc } from '@tauri-apps/api/tauri';

	// Gets the open function from simple-modal context
	const { open }: any = getContext('simple-modal');

	// When the modal is closed re-run the function getGames
	const showNewModal = () =>
		open(
			// 1° Arg: Component
			// 2° Arg: Props
			// 3° Arg: Options
			// 4° Arg: Callbacks
			NewGame,
			{},
			{},
			{
				onClose: () => {
					games = getGames();
				},
			}
		);

	// Show modal to edit games using prepolulated values
	const showEditModal = (game: any) =>
		open(
			// 1° Arg: Component
			// 2° Arg: Props
			// 3° Arg: Options
			// 4° Arg: Callbacks
			NewGame,
			{
				id: game.id,
				title: game.name,
				description: game.description,
				imagePath: game.image,
				executablePath: game.exe_path,
				operationToPerform: 'Edit',
			},
			{},
			{
				onClose: () => {
					games = getGames();
				},
			}
		);

	let query: string;
	let games: any = getGames();
</script>

<main class="container">
	<div class="main">
		<div class="top">
			<!-- Creates a modal when the button is clicked -->
			<button on:click="{() => showNewModal()}"
				>{$t('library.add')}</button
			>
			<input
				type="text"
				placeholder="Search"
				class="search-bar"
				bind:value="{query}"
			/>
		</div>

		<!-- Awaits for games to be resolved -->
		<!-- After that, loop over every object in that array -->
		<!-- And add those results to a div -->
		<!-- svelte-ignore empty-block -->
		<div class="game-panel">
			{#await games then data}
				{#each getFilteredGames(data, query) as game}
					<div class="game-panel">
						<div class="game-text">
							<button>
								<img
									class="game-image"
									src="{game.image == 'None'
										? 'Default.png'
										: convertFileSrc(game.image)}"
									alt="{game.name}"
									height="270"
									width="200"
								/>
								<p class="game-title">{game.name}</p>
							</button>
							<!-- <div class="game-info">
							<p class="small-info">
								{game.playtime}
								{$t('library.playtime')}
							</p>
							<p class="game-desc">
								{game.description}
							</p>
						</div> -->
						</div>
						<!-- <div class="buttons">
						<button
							class="game-button-run"
							on:click="{() =>
								operationHandler(() => runGame(game.exe_path))}"
							>{$t('library.run')}</button
						>
						<button
							class="game-button-run"
							on:click="{() =>
								operationHandler(() =>
									showEditModal(game)
								).then(() => {
									games = getGames();
								})}">{$t('library.editGame')}</button
						>
						<button
							class="game-button-delete"
							on:click="{() => {
								{
									console.log(game.id);
								}
								operationHandler(() =>
									deleteGame(game.id)
								).then(() => {
									games = getGames();
								});
							}}">{$t('library.deleteGame')}</button
						>
					</div> -->
					</div>
				{/each}
			{:catch error}
				<p style="color: red">{error.message}</p>
			{/await}
		</div>
	</div>
</main>

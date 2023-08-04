<script lang="ts">
    import {
        getGames,
        deleteGame,
        runGame,
        getFilteredGames,
        operationHandler,
    } from '../scripts/Library.js';
    import { getContext } from 'svelte';
    import NewGame from './modals/NewGame.svelte';
    import { t } from '../locale/i18n.js';
    import { convertFileSrc } from '@tauri-apps/api/tauri';
    import type { Game } from '../scripts/Interfaces.js';
    import { AddOutline } from 'svelte-ionicons';

    let gameModal: HTMLDialogElement;
    let gameOnModal: Game;
    let gameModalOpened: boolean;

    // Gets the open function from simple-modal context
    const { open }: any = getContext('simple-modal');

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

    const openGameModal = () => gameModal.showModal();

    let query: string;
    let games: any = getGames();
</script>

<main class="container">
    <div class="main">
        <!-- Awaits for games to be resolved -->
        <!-- After that, loop over every object in that array -->
        <!-- And add those results to a div -->
        <!-- svelte-ignore empty-block -->
        <div class="game-panel">
            {#await games then data}
                {#each getFilteredGames(data, query) as game}
                    <div class="game-panel">
                        <div class="game-text">
                            <button
                                on:click="{() => {
                                    gameModalOpened = true;

                                    gameOnModal = game;
                                    openGameModal();
                                }}"
                            >
                                <img
                                    class="game-image"
                                    src="{game.image == 'None'
                                        ? 'Default.png'
                                        : convertFileSrc(game.image)}"
                                    alt="{game.name}"
                                    height="210"
                                    width="150"
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
                    </div>
                {/each}

                <div class="gm-flex">
                    <dialog
                        class="game-modal"
                        bind:this="{gameModal}"
                        onclick="event.target==this && this.close()"
                    >
                        {#if gameModalOpened}
                            <span id="game-name">
                                {gameOnModal.name}
                            </span>

                            <img
                                id="game-image"
                                src="{gameOnModal.image == 'None'
                                    ? 'Default.png'
                                    : convertFileSrc(gameOnModal.image)}"
                                alt="{gameOnModal.name}"
                                height="210"
                                width="150"
                            />

                            <button
                                class="game-button-run"
                                id="execute"
                                on:click="{() =>
                                    operationHandler(() =>
                                        runGame(gameOnModal.exe_path)
                                    )}">{$t('library.run')}</button
                            >
                            <p id="game-desc" class="game-desc">
                                {gameOnModal.description}
                            </p>

                            <div class="buttons">
                                <button
                                    class="game-button-run"
                                    on:click="{() =>
                                        operationHandler(() =>
                                            showEditModal(gameOnModal)
                                        ).then(() => {
                                            games = getGames();
                                        })}"
                                    ><i class="fa-solid fa-pen"></i></button
                                >
                                <button
                                    class="game-button-delete"
                                    on:click="{() => {
                                        {
                                            console.log(gameOnModal.id);
                                        }
                                        operationHandler(() =>
                                            deleteGame(gameOnModal.id)
                                        ).then(() => {
                                            games = getGames();
                                        });
                                    }}"
                                    ><i class="fa-solid fa-trash"></i></button
                                >
                            </div>
                        {/if}
                    </dialog>
                </div>
            {:catch error}
                <p style="color: red">{error.message}</p>
            {/await}
        </div>
    </div>
</main>

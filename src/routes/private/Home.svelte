<script lang="ts">
	import { pbpNews } from '../../scripts/private/Home';
	import '../styles/Home.scss';
	import { t } from '../../locale/i18n';
</script>

<main class="container">
	<div style="text-align:center;" class="main">
		<h1 class="welcome-text">
			{$t('home.welcomeText')} <b class="underscore">|</b>
		</h1>
	</div>

	<div class="main">
		{#await pbpNews()}
			<p class="news">{$t('loadingText')}</p>
		{:then newsResolved}
			{#each newsResolved as newsObj}
				<div class="news">
					<p class="version">{newsObj.version}</p>
					<div class="additions">
						{#each newsObj.additions as additions}
							<li>{additions}</li>
						{/each}
					</div>
				</div>
			{/each}
		{/await}
	</div>
</main>

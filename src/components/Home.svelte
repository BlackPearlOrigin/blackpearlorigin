<script lang="ts">
	import { pbpNews } from '../scripts/Home';
	import '../styles/Home.css';
	import { t } from '../locale/i18n';
</script>

<main class="container">
	<div style="text-align:center;" class="main">
		<h1>{$t('home.welcomeText')} <b class="underscore">|</b></h1>
	</div>

	<div class="main">
		{#await pbpNews()}
			<p class="title">Loading</p>
		{:then newsResolved}
			<div class="title">
				<span id="title">Changelog:</span>
				{newsResolved.title}
			</div>
			<div class="body">
				{newsResolved.body}
			</div>
		{/await}
	</div>
</main>

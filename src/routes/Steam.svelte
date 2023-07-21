<script lang="ts">
    import translations from '../locale/locales.js';
    import { dict, locale, t } from '../locale/i18n.js';
    import languageNames from '../locale/languages.json';
    import '../styles/Steam.scss';
    const urlParams = new URLSearchParams(window.location.search);
    const steamid = urlParams.get('steamid');
    if (steamid !== null) {
        const steamData = {
            steamid,
        };
        localStorage.setItem('steamData', JSON.stringify(steamData));
    }
</script>

<main class="wrapper">
    <div class="icon-container">
        <img
            class="icon"
            src="https://media.discordapp.net/attachments/1117412589689909339/1117412636586430536/bpo_square.png?width=468&height=468"
            alt=""
        />
        <svg
            xmlns="http://www.w3.org/2000/svg"
            style="height: 1.5rem; width: 1.5rem;"
            viewBox="0 0 512 512"
        >
            <path
                style="height: 3.5rem; width: 3.5rem;"
                d="M208 352h-64a96 96 0 010-192h64M304 160h64a96 96 0 010 192h-64M163.29 256h187.42"
                fill="none"
                stroke="white"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="36"></path>
        </svg>
        <img
            class="icon"
            src="https://i.pinimg.com/originals/c8/4e/eb/c84eeb9ddc71a1895c5f7b1d94280458.jpg"
            alt="Steam Logo"
        />
    </div>
    <h1 class="text">You can now close this window.</h1>
</main>

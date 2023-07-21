//! DO NOT FUCKING EDIT THIS FILE
import './styles/Global.scss';
import Main from './Main.svelte';
import axios from 'axios';

const app = new Main({
    target: document.getElementById('app'),
});

function updateSteamData() {
    if (
        localStorage.getItem('steamData') !== null &&
        localStorage.getItem('steamData').includes('steam')
    ) {
        let config = {
            method: 'get',
            maxBodyLength: Infinity,
            url: `https://bpo-steam-dev.vercel.app/api/user?steamId=${
                JSON.parse(localStorage.getItem('steamData')).steamid
            }`,
            Headers: {},
        };
        axios
            .request(config)
            .then((res) => {
                const steamData = res.data.response.players[0];
                localStorage.setItem('steamData', JSON.stringify(steamData));
            })
            .catch((error) => {
                console.log(error);
            });
    }
}
updateSteamData();

export default app;

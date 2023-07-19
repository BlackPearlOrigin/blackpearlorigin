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
        const currentData = JSON.parse(localStorage.getItem('steamData'));
        let config = {
            method: 'get',
            maxBodyLength: Infinity,
            url: `http://localhost:5274/user?steamId=${currentData.steamid}`,
        };
        axios
            .request(config)
            .then((res) => {
                localStorage.setItem(
                    'steamData',
                    JSON.stringify(res.data.response.players[0])
                );
            })
            .catch((error) => {
                console.log(error);
            });
    }
}
updateSteamData();

export default app;

// ==UserScript==
// @name         Genius sidebar chooser
// @namespace    http://tampermonkey.net/
// @version      2024-03-16
// @description  try to take over the world!
// @author       ckie
// @match        https://genius.com/*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=genius.com
// @grant        none
// ==/UserScript==

(async function () {
    "use strict";
    const $ = async (...args) => {
        let res;
        while (true) {
            res = document.querySelector(...args);
            if (res) {
                // it might disappear, presumably as the DOM is rearranged
                await new Promise((r) => setTimeout(r, 100));
                if (res.innerHTML) return res;
            }
            await new Promise((r) => setTimeout(r, 100));
        }
    };
    const el = await $(".AlbumTracklist__Container-sc-123giuo-0");
    const sb = await $(".RightSidebar__Container-pajcl2-0");
    el.parentNode.style.position = "relative";
    el.style.position = "absolute";
    sb.insertBefore(el, sb.firstChild);
    (await $(".LyricsSidebarAd__Recommendations-sc-1duvwla-1")).style.display = "none";

    //return; // so extra
    let innrv = 0;
    setInterval(async () => {
        if (!(document.hasFocus() || (innrv++)%5==0)) return;
        const resp = await fetch("http://localhost:14726/mpc");
        let [album, song,stamp] = (await resp.text()).split("\n");
        const geniusAlb = await $("a.PrimaryAlbum__Title-cuci8p-4");
        if (!geniusAlb.innerText.startsWith(album)) return;

        song = song.replace(/^.+? - /,"");
        const songs = [...el.children].map(p=> p.children[0].children[1] || p.children[0]);

        const sel = songs.filter(e=>e).filter(e => e.innerText.includes(song));
        if (!sel.length) return;
        if (sel[0] instanceof HTMLDivElement) {
            // NOTE: play stat, broken & its a silly concept
            //sel[0].innerHTML+=`<code style="font-size:0.6em;font-family:monospace;">${stamp}</code>`;
        }
        else sel[0].click(); // switch to song
    }, 500);



})();

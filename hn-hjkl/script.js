// ==UserScript==
// @name         HN hJKl scroll
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  Scroll Hacker News comments with the J and K keys
// @author       ckie (https://ckie.dev)
// @match        https://news.ycombinator.com/item?id=*
// @icon         https://www.google.com/s2/favicons?domain=ycombinator.com
// @grant        none
// ==/UserScript==

(function() {
    'use strict';
    const doc = document;
    const styleEle = doc.createElement("style");
    styleEle.id = "hn-hjkl-scroll";
    const scrollFocusClass = "hjkl-scroll-focus";
    styleEle.innerHTML = `
    .${scrollFocusClass} {
        background: rgba(255,0,0,0.2);
    }
    `;
    doc.head.appendChild(styleEle);
    function setFocusOn(node, enable) {
        node.classList.toggle(scrollFocusClass, enable);
        node.scrollIntoView();
        window.scrollByLines(-10);

    }
    const nodes = [...doc.querySelector(".comment-tree tbody").children];
    const storageKey = `hjkl-item-${new URLSearchParams(location.search).get("id")}`;
    let currentIdx = parseInt(localStorage[storageKey]);
    setFocusOn(nodes[currentIdx], true);
    doc.addEventListener("keydown", e => {
        const keymap = {KeyJ: x=>x+1, KeyK: x=>x-1, ArrowUp: x=>x-1, ArrowDown: x=>x+1, Home: x=>0, End: x=>nodes.length-1};
        if (!Object.keys(keymap).includes(e.code)) return;
        e.preventDefault();
        setFocusOn(nodes[currentIdx], false);
        currentIdx = keymap[e.code](currentIdx);
        if (currentIdx < 0) currentIdx = nodes.length - 1;
        if (currentIdx >= nodes.length) currentIdx = 0;
        setFocusOn(nodes[currentIdx], true);
        localStorage[storageKey] = currentIdx;
    });
})();

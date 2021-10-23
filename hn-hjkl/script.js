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
    }
    const nodes = [...doc.querySelector(".comment-tree tbody").children];
    setFocusOn(nodes[0], true);
    let currentIdx = 0;
    doc.addEventListener("keydown", e => {
        setFocusOn(nodes[currentIdx], false);
        const incrmt = (e.key == "j" ? 1 : e.key == "k" ? -1 : 0);
        if (incrmt == 0) return;
        currentIdx += incrmt;
        if (currentIdx == 0) currentIdx = nodes.length - 1;
        if (currentIdx >= nodes.length) currentIdx = 0;
        setFocusOn(nodes[currentIdx], true);
        nodes[currentIdx].scrollIntoView();
        window.scrollByLines(-10);
    });
})();

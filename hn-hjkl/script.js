// ==UserScript==
// @name         HN hJKl scroll
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  Scroll Hacker News comments with the J and K keys
// @author       ckie (https://ckie.dev)
// @match        https://news.ycombinator.com/*
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
        // if we're focused on the first item, we probably also want to see the top of the page too (:
        if (node.parentNode.children[0] == node) {
            window.scrollTo(0, 0);
        } else {
            window.scrollByLines(-10);
        }
    }
    // "item" is a thread with comments, "news" is the front page
    const pageType = location.pathname.replace(/^\//, "");
    const nodes = [...doc.querySelector(pageType == "item" ? ".comment-tree tbody" : ".itemlist tbody").children];
    const storageKey = `hjkl-url-${location.path + location.search}`;
    let currentIdx = parseInt(localStorage[storageKey] || "0");
    setFocusOn(nodes[currentIdx], true);
    doc.addEventListener("keydown", e => {
        const keymap = {KeyJ: x=>x+1,
                        KeyK: x=>x-1,
                        ArrowUp: x=>x-1,
                        ArrowDown: x=>x+1,
                        Home: x=>0,
                        End: x=>nodes.length-1,
                        Enter: x => {
                            const node = nodes[currentIdx];
                            node.querySelector(".titlelink, .subtext a:last-child").click();
                            return x;
                        }
                       };
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

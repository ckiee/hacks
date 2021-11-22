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
    const qs = (...args) => document.querySelector(...args);
    const qsa = (...args) => document.querySelectorAll(...args);

    //
    const styleEle = doc.createElement("style");
    styleEle.id = "hn-hjkl-scroll";
    const scrollFocusClass = "hjkl-scroll-focus";
    styleEle.innerHTML = `
        .${scrollFocusClass} {
            background: rgba(255,0,0,0.2);
        }
        `;
    doc.head.appendChild(styleEle);
    //

    function setFocusOn(node, enable, scroll = true) {
        node.classList.toggle(scrollFocusClass, enable);
        sessionStorage[storageKey] = currentIdx;
        if (scroll) {
            node.scrollIntoView({ block: "center", behavior: "smooth" });
        }
    }

    const isItemPage = location.pathname == "/item";
    const nodes = isItemPage
          ? [qs(".fatitem"), ...qs(".comment-tree tbody").children]
          : [...qsa(".itemlist .athing")];

    const storageKey = `hjkl-url-${location.path + location.search}`;
    let currentIdx = parseInt(sessionStorage[storageKey] || "0");

    setFocusOn(nodes[currentIdx], true);

    doc.addEventListener("keydown", e => {
        const keymap = {
            KeyJ: x => x + 1,
            KeyK: x => x - 1,
            ArrowUp: x => x - 1,
            ArrowDown: x => x + 1,
            Home: x => 0,
            End: x => nodes.length - 1,
            Enter: x => {
                const node = nodes[currentIdx];
                node.querySelector(".titlelink, .subtext a:last-child").click();
                return x;
            },
            KeyC: x => {
                const node = nodes[currentIdx];
                const parentChildren = [...node.parentElement.children];
                const nextNode = parentChildren[parentChildren.indexOf(node) + 1];
                nextNode.children[nextNode.children.length - 1].click();
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
    });

    nodes.forEach((node, idx) => {
        node.addEventListener("click", e => {
            //no scroll since it's already visible
            setFocusOn(nodes[currentIdx], false, false);
            currentIdx = idx;
            setFocusOn(nodes[idx], true, false);
        });
    });

})();

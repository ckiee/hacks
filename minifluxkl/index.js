// minifluxkl by ckie https://pupc.at/mf
// https://github.com/ckiee/hacks/tree/master/minifluxkl/index.js
document.addEventListener("DOMContentLoaded", () => {
    const $ = document.querySelector.bind(document);
    const $$ = (...r) => [...document.querySelectorAll(r)];

    if (
        location.pathname.match(
            /^\/(history|unread|feed\/\d+\/entries(?:\/\w+)?)$/
        )
    ) {
        const ZOOM = 1.25;
        const USE_EXT = true;

        const stylesheet = new CSSStyleSheet();
        stylesheet.replaceSync(`
@keyframes spin {
to { transform: rotate(360deg); }
}
.viewer {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4px;
    & .preview {
        width: 100%;
        margin: 0px 2px;
        max-height: calc(100vh - 4px);

        /* Force us to be a scroll container, then disallow overscroll caught bubbling from the iframe */
        /*padding-bottom: 1px;
        overflow-y: scroll;
        overscroll-behavior: contain;
        scrollbar-width: none;*/

        & .spinner {
            display: flex;
            width: 100%;
            height: 100%;
            justify-content: center;
            align-items: center;

            color: #396;
            font-size: 5em;
            animation: spin linear 2.5s infinite;
        }
        &:has(iframe[data-loaded]:not(.xkl-bg)) .spinner {
            display: none;
        }
        & iframe {
            width: ${ZOOM * 100}%;
            height: ${ZOOM * 100}%;
            transform-origin: left top;
            transform: scale(${1 / ZOOM});

            border: 1px solid #eee;

            display: none;
            &[data-loaded]:not(.xkl-bg) { display: block; }
        }
    }
}

body {
    padding: 0px 64px;
    max-width: unset !important;
}
`);
        document.adoptedStyleSheets.push(stylesheet);
        /** @type {HTMLElement} */
        const main = $("main");
        /** @type {HTMLDivElement} */
        const items = $(".items");
        const viewer = document.createElement("div");
        viewer.classList.add("viewer");

        const preview = document.createElement("div");
        preview.innerHTML = `<div class="spinner">🐈</div>`;

        let iframe = document.createElement("iframe");
        let bgIframe = document.createElement("iframe");
        bgIframe.classList.add("xkl-bg");
        [iframe, bgIframe].map((fr) =>
            fr.addEventListener("load", () =>
                fr.setAttribute("data-loaded", true)
            )
        );

        preview.appendChild(iframe);
        preview.appendChild(bgIframe);
        preview.classList.add("preview");

        // Reparent .items to be under new .viewer under main
        main.replaceChild(viewer, items);
        viewer.appendChild(items);
        viewer.appendChild(preview);

        const articles = $$(".items > article");

        let focused;
        const viewUrlOf = (a) => {
            const ext = a
                .querySelector(".item-meta-icons-external-url a")
                .getAttribute("href");
            const int = a.querySelector(".item-title a").getAttribute("href");
            let useExt = USE_EXT;
            if (new URL(ext).hostname == "cloudwithlightning.net")
                useExt = false;
            if (new URL(ext).hostname == "www.tumblr.com") useExt = false;
            if (
                ["mei.puppycat.house", "pupcat-dev.tailnet.ckie.dev"].includes(
                    new URL(ext).hostname
                ) &&
                new URL(ext).pathname == "/visual"
            )
                useExt = false;
            if (new URL(ext).hostname.match(/(\.?|^)substack\.com$/))
                useExt = false;
            // if (new URL(ext).hostname.match(/(?:^|\.)dreamwidth\.org$/)) useExt = false;

            return useExt ? ext : int;
        };

        articles.map(
            (article) =>
                (article.onfocus = (e) => {
                    if (focused !== article) {
                        articles.map((e) => e.classList.remove("current-item"));
                        article.classList.add("current-item");
                        const idx = articles.indexOf(article);

                        let ofs =
                            idx == 0 ? 0 : -articles[idx - 1].offsetHeight;
                        if (article !== articles[0]) {
                            article.scrollIntoView(true);
                            window.scrollBy({ top: ofs });
                        }
                        preview.style.transform = `translateY(${
                            article.offsetTop - preview.offsetTop + ofs
                        }px)`;
                        if (viewUrlOf(article) == bgIframe.src) {
                            const previouslyPrimary = iframe;
                            iframe = bgIframe;
                            bgIframe = previouslyPrimary;

                            previouslyPrimary.classList.add("xkl-bg");
                            iframe.classList.remove("xkl-bg");
                            console.log("SWAP!");
                        } else {
                            iframe.src = "about:blank";
                            iframe.removeAttribute("data-loaded");
                        }
                        iframe.src = viewUrlOf(article);
                        bgIframe.removeAttribute("data-loaded");
                        bgIframe.src = viewUrlOf(
                            articles[articles.indexOf(article) + 1]
                        );
                    }
                    focused = article;
                })
        );

        articles[0].focus(); // only works if the _page_ is focused (FF on X11)
        articles[0].onfocus();
    }
});

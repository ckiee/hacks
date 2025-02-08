// llm used -- morally compromised but overall good
// TODO: performance opts lacking for large webpages.
(async () => {
  const overlay = document.createElement("div");
  overlay.style.position = "absolute";
  overlay.style.fontSize = "16px";
  overlay.style.background = "black";
  overlay.style.color = "white";

  overlay.style.padding = "4px 8px";
  overlay.style.borderRadius = "4px";
  overlay.style.pointerEvents = "none";
  overlay.style.zIndex = "10000";
  overlay.style.display = "none";
  document.body.appendChild(overlay);

  let translations = {};

  try {
    const response = await fetch("https://api.linku.la/v1/words");
    if (!response.ok) throw new Error("linku.la API error");
    translations = await response.json();
  } catch (error) {
    console.error("Error fetching translations", error);
  }

  const containers = document.querySelectorAll("*");
  if (!containers.length) return;

  containers.forEach((container) => {
    container.addEventListener("mousemove", (event) => {
      let range;
      if (document.caretRangeFromPoint) {
        range = document.caretRangeFromPoint(event.clientX, event.clientY);
      } else if (document.caretPositionFromPoint) {
        const caretPos = document.caretPositionFromPoint(
          event.clientX,
          event.clientY
        );
        if (caretPos) {
          range = document.createRange();
          range.setStart(caretPos.offsetNode, caretPos.offset);
        }
      }
      if (!range) return;

      const node = range.startContainer;
      if (!node || node.nodeType !== Node.TEXT_NODE) return;

      const text = node.textContent;
      const offset = range.startOffset;

      let start = offset;
      while (start > 0 && /\S/.test(text[start - 1])) start--;

      let end = offset;
      while (end < text.length && /\S/.test(text[end])) end++;

      const word = text.slice(start, end).replace(/\W/g, "");
      if (!word) {
        overlay.style.display = "none";
        return;
      }

      const definition =
        translations[word]?.translations?.en?.definition;

      overlay.style.display = definition ? "block" : "none";
      overlay.textContent = `${word}: ${definition}`;
      overlay.style.left = `${event.pageX + 10}px`;
      overlay.style.top = `${event.pageY + 10}px`;
    });

    container.addEventListener("mouseleave", () => {
      overlay.style.display = "none";
    });
  });
})();

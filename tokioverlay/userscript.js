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

  let cachedDefs =
{"a":"(interjection) ah, oh, ha, eh, um, oy; (particle) [placed after something for emphasis or emotion]","akesi":"reptile, amphibian, scaly creature, crawling creature","ala":"not, nothing, no; (particle) [negates a word or phrase]; (particle) [forms a yes-no question]; (number) zero","alasa":"hunt, forage, search, attempt; (preverb) try to","ale":"all, every, everything, entirety, universe; (number) one hundred","ali":"[pronunciation variant of ale]","anpa":"bottom, underside; below, beneath; defeated, humble, lowly","ante":"different, altered; modify, change; other; difference","anu":"(particle) [separates multiple possibilities, replacing another particle], or","apeja":"guilt, shame, shun, stigma, disgrace; to accuse, to single out, to expose, to dishonor, to embarrass","awen":"stay, remain, wait, pause; protect, keep safe; continue; (preverb) continue to","e":"(particle) [marks the start of a direct object]","en":"(particle) [separates multiple subjects]","epiku":"epic, cool, awesome, amazing","esun":"trade, barter, exchange, swap, buy, sell; market, shop, fair, bazaar, place of business","ijo":"thing, object, entity, being, matter, phenomenon","ike":"negative quality, e.g. bad, unpleasant, harmful, unneeded","ilo":"tool, implement, machine, device","insa":"inside, center, between, middle, midpoint, internal","isipin":"to think, brainstorm, rationalize, conclude, ponder","jaki":"disgusting, unclean, unsanitary, toxic, repulsive, rotten","jami":"yummy; eliciting or stimulating a positive sensory experience","jan":"human being, person, somebody","jasima":"reflect, echo; mirror, duplicate","jelo":"yellow, amber, golden, lime yellow, yellowish orange","jo":"hold, carry, possess, contain, own","jonke":"hjönk, goose sound, goose","kala":"fish, marine animal, sea creature, swimming creature","kalama":"to produce sound; sound, e.g. sing, thunder, drum, clap, laugh, beep","kama":"arriving, coming, future, summoned; (preverb) to become, manage to, succeed in","kamalawala":"anarchy, uprising, revolt, rebellion","kapesi":"brown, gray, beige","kasi":"plant, vegetation; herb, leaf","ken":"can, may, ability, permission; possibility, maybe; allow, enable; (preverb) to be able to","kepeken":"(preposition) using, by means of","kijetesantakalu":"raccoon, kinkajou; any procyonid; any musteloid","kiki":"spiky, sharp; angle, point; triangular","kili":"fruit, vegetable, mushroom","kin":"(particle) [after phrase or at sentence start] too, also, as well, additionally","kipisi":"split, cut, slice; piece, part; sharp, pointy","kiwen":"hard object e.g. metal, stone, wood","ko":"semi-solid, e.g. paste, powder, goo, sand, soil, clay; squishy, moldable; sticky","kokosila":"to speak a non-Toki Pona language in an environment where Toki Pona is more appropriate","kon":"air, breath, wind; essence, spirit, soul, ghost; unseen agent","konwe":"animacy, life, autonomy; autonomous being, living thing, organism; alive, animate, dynamic; to animate, to bring to life","ku":"interacting with the book Toki Pona Dictionary (2021) by Sonja Lang","kule":"color, pigment; category, genre, flavor; colorful, diverse","kulijo":"(interjection) [casually express appreciation or acknowledgement]; cool, fine, okay","kulupu":"group, community, society, company, nation, collection, team, crowd","kute":"ear, hearing organ; hear, listen, pay attention to, obey","la":"(particle) [mark the previous statement as context to the following statement]","lanpan":"take, seize, steal","lape":"sleep, rest, break from an activity or work","laso":"turquoise, blue, green, cyan, indigo, lime green","lawa":"head, mind, brain; control, lead, guide; government, leader; rule, law","leko":"square, cube, block, blocky object e.g. bricks, stairs","len":"cloth, clothing, fabric, textile; covered, hidden, secret, private","lete":"cold, cool, frozen; freeze, chill; raw, uncooked","li":"(particle) [marks the start of an indicative verb (statement)]","lili":"small, short, young; few; piece, part","linja":"long, flexible thing, e.g. rope, yarn, hair, fur, line, strand","linluwi":"bonded things which are stronger through their bonds e.g. network, internet, connection; weave, braid, interlace","lipu":"flat and bendable object, e.g. paper, card, leaf; written text or document, e.g. book, website, clay tablet","loje":"red, magenta, scarlet, pink, rust-colored, reddish orange","lon":"present, existing, real, true; (preposition) located at, in, during, in the context of","luka":"hand, arm, tactile limb, grasping limb; to grasp, interact with, feel using touch; (number) five","lukin":"see, look, view, examine, read, watch; visual; eye, seeing organ; (preverb) try to","lupa":"hole, pit, cave, doorway, window, portal","ma":"earth, land, soil; country, territory, world; outdoors","majuna":"old, aged, ancient","mama":"parent, ancestor; creator, originator; caretaker, sustainer, guardian","mani":"money, currency; thing of value e.g. gold, investment, livestock","meli":"woman, female, wife, girlfriend","melome":"lesbian, sapphic, WLW","meso":"midpoint, medium, mediocre; neither one not the other, neither fully is nor isn't","mi":"(pronoun) I, me, we, us","mije":"man, male, husband, boyfriend","mijomi":"gay, achillean, MLM","misa":"Glires or Eulipotyphla; rat, mouse, squirrel, rabbit, rodent; {~ suli} capybara","misikeke":"medical item or practice e.g. prescriptions, meditation, exercise, bandages, therapy","moku":"eat, drink, consume, swallow, ingest; food, edible thing","moli":"death, dead, die, dying; kill, murder","monsi":"back, behind, rear","monsuta":"fear, nervousness, dread; scary, frightening; scary thing e.g. predator, threat, danger","mu":"(animal noise or communication, onomatopoeia)","mulapisu":"pizza","mun":"moon, night sky object, star, celestial body","musi":"fun, game, entertainment, art, play, amusing, interesting, comical, silly","mute":"many, several, very; quantity; (number) twenty","n":"(interjection) hm, uh, mm, er, umm, [indicate thinking or pause]","namako":"spice, ornament, adornment; extra, additional","nanpa":"number; (particle) [ordinal number], -th","nasa":"strange, unusual, silly, abnormal, unexpected; drunk, intoxicated","nasin":"method, doctrine, tradition; path, road, way","nena":"protuberances e.g. bump, button, hill, nose","ni":"this, that, these, those","nimi":"word, name","nimisin":"any non-pu word; any new word; any joke word","nja":"meow, cat sound","noka":"foot, leg, organ of locomotion, roots","o":"(particle) [marks the end of a vocative (who is being spoken to)], [marks the start of an imperative (command, wish, instruction)], should","ojuta":"adaptation of the English meme \"ligma\", (literally) \"o uta\" (lick)","oke":"(acknowledgement or acceptance)","oko":"see, look, view, examine, read, watch; visual; eye, seeing organ; (preverb) try to","olin":"to have a strong emotional bond with, e.g. affection, appreciation, compassion, respect; platonic, romantic, or familial relationships","omekapo":"goodbye, farewell, see you later, (literally) \"o moku e kala pona\" (eat a good fish)","ona":"(third-person pronoun) he, she, it, they","open":"begin, start, open, turn on; beginning","owe":"totalitarian, Orwellian; of or relating to Big Brother from the book Ninety Eighty-Four","pakala":"damaged, broken, botched, harmed, messed up; mistake","pake":"stop, cease, halt; to block the way, to interrupt; to prevent","pakola":"[misspelling of pakala]","pali":"work, activity; create, build, design; put effort toward, take action on","palisa":"long and hard thing e.g. branch, pole, rod, stick, spine, mast","pan":"grains, starchy foods, baked goods e.g. rice, sorghum, bread, noodles, masa, porridge, injera","pana":"give, send, emit, provide, put, release","penpo":"(equivalent to \"(pi) toki pona taso\")","pi":"(particle) [group the following words into one modifier for the previous word]","pika":"electric, electronic, conductive, mechanical, online; electricity, lightning, thunder, network","pilin":"experience e.g. emotion, feeling, touch; heart (physical or emotional)","pimeja":"dark, unlit; dark color, e.g. black, purple, brown","pini":"finish, stop, prevent; close, disable, turn off; ended, past; edge, end, conclusion","pipi":"insect, bug, spider, tiny crawling creature","po":"(number) four","poka":"hip, side; next to, nearby, vicinity","poki":"container e.g. bag, bowl, box, cup, cupboard, drawer, folder","pona":"positive quality, e.g. good, pleasant, helpful, friendly, useful, peaceful","powe":"unreal, false, untrue; pretend; deceive, trick","pu":"interacting with the book Toki Pona: The Language of Good by Sonja Lang","puwa":"fluffy, soft, squishy (something that can be compressed and will try to go back to its original shape)","sama":"same, similar, alike; (adjective) peer, fellow, each other; (preposition) similar to, same as","san":"(number) three","seli":"hot, warm; heat, fire, flame; burn","selo":"outer layer, e.g. skin, peel, shell, bark; outer shape, outer form, boundary","seme":"(particle) [indicate a question by marking missing info in a sentence]; what, which, who","sewi":"up, top, above, highest part; divine, sacred, supernatural; awesome, inspiring, excelling","sijelo":"body, shape, physical state, torso, substance, form","sike":"circle, sphere, spiral, round thing e.g. ball, wheel; repeating thing e.g. cycle, orbit, loop","sin":"new, fresh, update; repeat, do again","sina":"(pronoun) you, y'all","sinpin":"vertical surface e.g. wall, board; front of something e.g. face","sitelen":"image, picture, representation, symbol, mark, writing","soko":"mushroom, fungus, lichen","sona":"knowledge, information, data; know, be skilled in, be wise about; (preverb) know how to","soto":"left, left side, port side","soweli":"fuzzy creature, land animal, beast","su":"interacting with a book from the illustrated story book series that began with The Wonderful Wizard of Oz, produced by Sonja Lang","suli":"big, heavy, large, long, tall, wide; important, relevant","suno":"light, shine, glow, radiance; sun, light source; brightness","supa":"flat horizontal surface, especially to put or rest things on e.g. bed, floor, desk, plate, table, platform, stage","sutopatikuna":"platypus","suwi":"sweet, fragrant; cute, adorable","taki":"sticky, magnetic; bond, attract, attach, clip","tan":"(preposition) from, because of; cause, origin","taso":"only, exclusively; (particle) [marks a sentence as qualifying or contradictory], but, however","tawa":"motion, e.g. walking, shaking, flight, travel; (preposition) to, for, going to, from the perspective of","te":"(particle) [opens a quote]","teje":"right, right side, starboard","telo":"liquids e.g. water, gasoline, soda, lava, soup, oil, ink","tenpo":"time, event, situation, moment, period, duration","to":"(particle) [closes a quote]","toki":"communicate, say, think; conversation, story; language","tomo":"indoor space or shelter e.g. room, building, home, tent, shack","tonsi":"nonbinary, gender nonconforming, genderqueer, transgender*","tu":"(number) two; separate, divide, split","unpa":"sex, to have sex with","unu":"purple, violet","usawi":"magic, sorcery; enchant; magical, supernatural, occult, incomprehensible","uta":"mouth, lips, throat, consuming orifice","utala":"fight, compete, battle; competition, challenge; struggle, strive","wa":"(interjection) [indicating awe or amazement]","walo":"light-colored, white, pale, light gray, cream","wan":"(number) one; singular; combine, join, mix, fuse","waso":"bird, flying creature, winged animal","wasoweli":"animal with qualities of both waso and soweli","wawa":"power, energy, strength; confident, intense, forceful; amazing, impressive","weka":"absent, away, distant; remove, get rid of","wekama":"leave and come back, be temporarily absent with the expectation of returning","wile":"want, desire, wish, require; (preverb) want to","wuwojiti":"to use one or more of the \"banned\" syllables wu, wo, ji, or ti; to break toki pona phonotactics","yupekosi":"to behave like George Lucas and revise your old creative works and actually make them worse"};
  let translations;

  // some websites have csp policies that block this fetch;
  // try {
  //   const response = await fetch("https://api.linku.la/v1/words");
  //   if (!response.ok) throw new Error("linku.la API error");
  //   translations = await response.json();
  // } catch (error) {
  //   console.error("Error fetching translations", error);
  // }

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
            (translations
                ? translations[word]?.translations?.en?.definition
                : cachedDefs[word]
            )?.toLowerCase();

      overlay.style.display = definition ? "block" : "none";
      overlay.textContent = `${word}: ${definition}`;
      overlay.style.left = `${event.pageX + 12}px`;
      overlay.style.top = `${event.pageY + 12}px`;
    });

    container.addEventListener("mouseleave", () => {
      overlay.style.display = "none";
    });
  });
})();

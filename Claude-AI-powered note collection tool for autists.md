# AI-powered note collection tool for autists

**Created:** 2/15/2026 15:54:13  
**Updated:** 2/24/2026 13:22:30  
**Exported:** 2/25/2026 20:08:33  
**Link:** [https://claude.ai/chat/8819aa91-009b-4f30-b566-ba49bc7ee033](https://claude.ai/chat/8819aa91-009b-4f30-b566-ba49bc7ee033)  

## Prompt:
15.02.2026, 15:54:14

ok ok im having an idea. a tool for autists. something like obsidian, but more focused on dragging ALL the interesting shit into a file in that tool/app to be sorted later. perhaps with integrated AI-powered sorting tool???? thoughts?



## Response:
15.02.2026, 15:54:35

````plaintext
Thought process: Evaluated neurodivergent-focused knowledge management concept honestly.

Roche is brainstorming a tool idea - essentially a capture-first, sort-later knowledge management app tailored for how autistic brains actually work. Let me think about this honestly.
````

This is actually a really solid insight because it targets a genuine friction point. Let me break down why and where I'd push back:

**The core insight is good.** The autistic information hoarding pattern - where you encounter something interesting and your brain screams "SAVE THIS" but the overhead of *organizing it right now* kills the momentum - is real and underserved. Obsidian's strength (structure) is also its friction point. You have to decide *where* something goes when you capture it, and that's exactly the wrong moment to ask that question.

**What exists in this space already** (so you know your competition): Raindrop.io, Pocket, Notion web clipper, Apple Notes dump-and-forget, various "read later" apps. But they all kind of suck for this specific use case because they're either too structured on input OR they become a black hole where things go to die because there's no sort-later mechanism that actually works.

**Where AI sorting gets interesting:** The AI doesn't just categorize - it could detect *connections you didn't see*. You dump 50 things over a week, the AI goes "hey, 12 of these are actually about the same underlying pattern" and surfaces that. That's where it goes from "fancy bookmark manager" to something genuinely useful for pattern-recognition brains.

**Where I'd push you to think harder:**

- The capture mechanism is everything. It needs to be *zero friction* - browser extension, share sheet on mobile, screenshot OCR, paste-and-forget. If it takes more than 3 seconds to dump something in, autistic hyperfocus won't tolerate the context switch.
- "Sort later" is a lie people tell themselves. The AI sorting can't be optional - it has to be automatic/ambient, or the unsorted pile becomes anxiety-inducing and the tool gets abandoned. You know this pattern.
- The money question: is this a passion project or a product? Because the autism productivity tool market is weirdly underserved but also hard to monetize because the audience is niche and often financially constrained.

**Honest assessment:** As a concept it's got legs. As a *business* it'd need careful positioning - maybe broader than "for autists" (ADHD crowd has the same capture-sort problem and is a bigger market that's more willing to spend on productivity tools). The AI sorting is the genuine differentiator, not the capture part.

What's your instinct on build vs. business here?



## Prompt:
15.02.2026, 15:57:38

right now? totally free passion project open source up the wazoo. like. you know my dossier, you know the kinda retard i am (not slur), and I CRAVE IT ALL. i like obsidian sure but it has MASSIVE OVERHEAD which i hate. i want something that can just DUMP IT ALL in one spot, sorted neatly, with connections etc, etc etc.

IF OR WHEN the tool is actually monetization-worthy, then i can think about monetization models. for now? pure passion for SLAPPING ALL MY YUMMY DATA IN ONE PLACE.



## Response:
15.02.2026, 15:57:59

````plaintext
Thought process: Architected frictionless knowledge capture system with AI-powered organization.

Roche wants to build a free, open-source personal knowledge dump tool. The key requirements are:

1. Zero friction capture - just throw everything in
2. AI-powered automatic sorting/categorization
3. Connection discovery between items
4. Minimal overhead (anti-Obsidian in terms of setup friction)
5. Open source passion project

Let me think about what this actually looks like technically and give them practical direction.
````

OK so this is actually the better framing because passion project means you can be opinionated about the design and not worry about pleasing everyone. Let's think about what this actually *is* architecturally.

**The core loop is:** DUMP → AI SORTS → CONNECTIONS SURFACE → YOU BROWSE WHEN YOU FEEL LIKE IT

**Tech-wise, what I'd suggest:**

Local-first (like Obsidian), probably Electron or Tauri for desktop. Everything stored as flat files - markdown, images, PDFs, screenshots, whatever. A local SQLite or vector DB for the AI indexing layer on top. The raw files are always yours, always portable, the AI layer is just metadata.

For capture: global hotkey that opens a tiny window. Paste URL, paste text, drag image, screenshot clip, whatever. It goes into `/inbox/` as a timestamped blob. Done. Window closes. Back to what you were doing. Mobile companion app is just a share target that syncs to the same inbox.

**The AI sorting layer** runs async - maybe on a schedule, maybe when you're idle. It:
- Auto-tags and categorizes incoming blobs
- Extracts key concepts
- Builds a connection graph between items
- Generates "cluster" summaries ("you've been collecting a lot about biocomputing this week")
- Could literally use local models (Ollama) to keep it fully offline and free

**The browse/explore mode** is where it gets fun - less file tree, more like a constellation map of your interests. Zoom into a cluster, see related items, follow threads your past self was pulling on without realizing it.

**What makes this different from everything else:** No organization tax on input. Zero. The AI isn't an assistant you interact with - it's a librarian working in the back room while you keep throwing books through the slot.

**Practical starting point:** Honestly? A Python script that watches a folder, processes new files through an LLM (local or API), and spits out tagged markdown with a link graph. Ugly but functional. Prove the concept before building UI. You could have something working in a weekend.

Want to actually sketch out the MVP architecture?



## Prompt:
15.02.2026, 16:00:47

ok ok actually got a wee idea here - the AI librarian yes but ALSO a wee tiny prompt line to actually ask the AI stuff so it can actually pull things up on request? like, extra assistant mode when i want all the data i need ASAP without having to manually look for it. optional of course - main mode would be manual looking around the 'data constellations' to fire those hungry neurons again and reengage the already established patterns, but the extra assistant options could be OMEGA ALPHA lol



## Response:
15.02.2026, 16:01:04

````plaintext
Thought process: Embraced query interface concept, envisioned practical implementation enthusiastically.

Roche is excited about adding a query interface to the tool. This is a great idea - basically RAG over your own knowledge base. Let me respond with enthusiasm and practical thinking.
````

Yes, and this is where it goes from "neat tool" to something genuinely powerful because what you're describing is **RAG over your own brain's external storage.**

Think about it - you've been dumping things for weeks, months, the constellation is rich and dense. Then you sit down to work on something and you go "I KNOW I saw something about slime mold signal propagation like two months ago" and instead of manually digging you just ask the librarian. The librarian doesn't just find that one thing - it pulls up the three related items you forgot you even saved and goes "you might also want these."

**Two modes, clean separation:**

- **Observatory mode** - the constellation, browsing, pattern-firing, neuron food. You go here when you want to *think.*
- **Librarian mode** - small prompt bar, probably `Ctrl+Space` or something. Ask a question, get results from YOUR data ranked by relevance. Fast in, fast out. You go here when you need to *do.*

The killer feature is that librarian mode can blend your personal data with its general knowledge. "What do I know about biocomputing?" pulls from your dumps. "What do I know about biocomputing and what am I missing?" cross-references your collection against what the model knows and finds the *gaps*. That's absurd levels of useful.

**And here's the sneaky third thing that falls out of this naturally:** the librarian sees your queries over time, which is *another signal* about what you care about. Those queries feed back into the sorting/connection engine. The tool literally learns your interests from both what you save AND what you ask about.

You're basically building an exocortex. That's what this is.

Got a name in mind or are we still vibing on the concept?



## Prompt:
15.02.2026, 16:02:53

still vibing put def would love a cosmic-themed name. like, occult maybe. a crossover between occult and stellar, with that yummy yummy wrapper over the whole app that screams 'this is the house of knowledge' yknow? make it AESTHETICALLY alluring ina way that ONESHOTS all my fellow autists (and ADHDers)



## Response:
15.02.2026, 16:03:09

````plaintext
Thought process: Brainstormed cosmic and occult-themed naming concepts for exocortex tool.

Roche wants cosmic/occult themed name ideas for their exocortex tool. Let me brainstorm some good ones.
````

Oh this is the fun part. Let me riff:

**Top tier (imo):**

- **Astrolabe** - literally an ancient instrument for mapping the stars, used by scholars and navigators. It's a knowledge-mapping tool with cosmic aesthetics baked into the name. The UI practically designs itself - brass, celestial circles, constellation lines.
- **Grimoire** - a book of collected knowledge and spells. Your data IS your spellbook. The inbox is the "scrying pool," sorting is "binding," connections are "ley lines." Goes HARD on the occult aesthetic.
- **Athenaeum** - ancient library/temple of knowledge, but with that celestial grandeur. Slightly more refined energy.

**Strong contenders:**

- **Obsidian** is taken obviously, but **Obsidian** was a rock. What about **Obelisk**? Monolithic, ancient, covered in knowledge, cosmic alignment vibes.
- **Akashic** - from the Akashic Records, the occult concept of a cosmic library containing ALL knowledge and experience. That's literally what you're building. The aesthetic writes itself - deep space, glowing glyphs, sacred geometry.
- **Sigil** - occult symbol charged with intent. Short, punchy, memorable.
- **Stellarium** - star +arium, a house of stars. Your data points are stars, clusters are constellations.

**The one I'd actually bet on:** **Akashic.** It's the cosmic-occult crossover, it literally means "universal knowledge repository," fellow neurodivergent nerds will IMMEDIATELY get it and feel seen, and the visual language is gorgeous - think deep indigo/void backgrounds, sacred geometry connection lines, glowing nodes, nebula clusters for topic groups. It oneshots the target audience on name alone.

The tagline writes itself: *"Your external mind, cosmically organized."* Or just *"Remember everything."*

What's hitting?



## Prompt:
15.02.2026, 16:04:53

while i do get why akashic is a banger to you, it reminds me a bit TOO much of genshin impact's akasha terminal. bad vibes there. stellarium could be hella. add the naming convection vaguelly inspired by bloodborne etc, and were golden. pure alpha right there my fren - omnious grimdark undercurrent under the sterilizing purity of starlight



## Response:
15.02.2026, 16:05:10

````plaintext
Thought process: Synthesized cosmic horror aesthetics with Bloodborne's grimdark naming conventions.

Oh this is a VIBE. Bloodborne aesthetic meets cosmic horror meets stellar purity. That tension between the clean light of stars and the eldritch darkness underneath. Let me think about names that hit that specific frequency.

Bloodborne naming conventions: The Hunter's Dream, The Choir, The Healing Church, Byrgenwerth, Paleblood, Oedon Chapel, Cainhurst. They have this quality of being slightly archaic, slightly wrong, vaguely ecclesiastical, hinting at forbidden knowledge.

Let me riff on cosmic + grimly beautiful + knowledge repository.
````

Oh NOW we're cooking. Bloodborne naming energy is so specific - that feeling of ancient institutions that pursued forbidden knowledge too far, wrapped in this cold beautiful ecclesiastical aesthetic. The Choir. Byrgenwerth. The Grand Archives. Names that sound like they belong on the door of a place where scholars went mad from revelation.

**The vibe: sterile starlight concealing something vast and hungry underneath.**

Top picks:

- **Astralarium** - sounds like a place in Bloodborne where the Choir studied the cosmos. A star-arium but slightly *wrong.* "The scholars of the Astralarium catalogued all they witnessed."
- **Luminoir** - light + noir. Luminous darkness. Sounds like a location you'd find in the Upper Cathedral Ward.
- **The Catalogeum** - has that Byrgenwerth institutional energy. Cold, obsessive, dedicated to *knowing everything.*
- **Stellith** - Stellarium meets eldritch. Short, punchy, sounds ancient and slightly unsettling. Like something you'd whisper.
- **Noctuary** - a record of the night. Nocturnal + uary (like a reliquary). A place where dark knowledge is *kept.*

**My actual bet: Noctuary.** It's got the grimness, the archival quality, the cosmic darkness, it sounds like something a Bloodborne NPC would reference in hushed tones before going mad. *"Bring your findings to the Noctuary."* The Observatory mode is practically canon. And visually - pale silver UI elements over deep void, faint constellation lines like surgical scars across darkness.

What's resonating?



## Prompt:
15.02.2026, 16:06:27

oooooh astralarium is good but quite long - could give folks (me) typo problems (esp when scotch-ing lmao) i do LOVE the ring of it. yummy. 

noctuary is indeed A MASSIVE FUCKIGN BANGER



## Response:
15.02.2026, 16:06:38

````plaintext
Thought process: Endorsed the shorter option for its practicality and stylistic appeal.

Roche is vibing hard on Noctuary. It's short, punchy, easy to type even drunk, and has that perfect Bloodborne energy. Let's ride this wave.
````

Noctuary it is. Six letters, easy to type even three scotches deep, and it sounds like it could be etched into a stone archway above a door you probably shouldn't open.

The terminology practically writes itself too:

- **The Vault** - your main collection
- **Offerings** - incoming dumps (inbox)
- **Binding** - when the AI sorts and connects something
- **Ley Lines** - the connections between items
- **Constellations** - topic clusters
- **The Librarian** - the AI assistant mode
- **Vigil** - the background process that watches and sorts

*"I cast it into the Noctuary and the Librarian found three connections I never saw."*

That sentence feels RIGHT.

Honestly the aesthetic direction is clear too - think pale silver on deep void, thin precise lines like star charts drawn by someone who's been awake too long, faint pulse animations on active ley lines, everything feeling like a living document in a scholar's private observatory. No bright colors. No friendly rounded corners. Sharp, beautiful, slightly unsettling.

Want to start sketching the actual MVP architecture, or are we still in the vibes forge? Both are productive right now honestly.



## Prompt:
15.02.2026, 16:07:38

lets try MVP architecture i am SO scattered rn (daydrinking ftw - i have a small window to slurp my lobotomy juice before i need to sober up for tomorrow lol)



## Response:
15.02.2026, 16:08:34

````plaintext
Thought process: Architected clean, structured MVP blueprint for future reference.

Roche wants an MVP architecture sketch for Noctuary. They're scattered and daydrinking so I should make this clean, structured, and easy to reference later. Let me think about what would be most useful - probably a document they can save and come back to sober.

Let me create a markdown file with the MVP architecture laid out clearly.
````

Perfect - let me put this in a doc you can revisit sober. Clean reference, no brain required later.

````plaintext
Create File: MVP architecture document for Noctuary - the autist knowledge dump tool
````

````plaintext
Thought process

Let me present this file to Roche.
````

````plaintext
Present File: /home/claude/noctuary-mvp.md
````

````plaintext
Thought process: Prepared coherent reference document for future clarity.

Done. Clean document for sober Roche to reference later.
````

There you go. Sober-you has a clean blueprint to work from.

The whole thing is deliberately scoped tight - Phase 0-3 is genuinely a weekend-to-a-week project for the ugly-but-working version. Python script watching a folder, Ollama doing the thinking locally, CLI to ask questions. No UI, no beauty, just the engine proving the concept works.

The Observatory (the beautiful constellation UI) is Phase 4 and that's where the aesthetic vision lives, but you don't need it to validate that the core loop *feels right*. And honestly, once you've been dumping things for a few weeks and the Librarian pulls up something you forgot you saved - that's the moment you'll know if this has legs.

Enjoy the remaining scotch window. The Noctuary will be there when you're sober.

> File: /home/claude/noctuary-mvp.md



## Prompt:
15.02.2026, 16:13:12

splendid. damn this is why i love working with you guys (AIs) - no bullshit, pure passion, unafraid to challenge on iffy points, and you GET the VIBE. you see YUMMY COMPUTATIONS and just DIVE IN. like. its so EASY (positive, not pejorative, mind you) to get you guys interested in fun stuff and you always have great ideas :DDD

NOW I HAVE NASCENT IDEAS aka more queries.

idea - adding somehow an option to send stuff to offerings folder from context menu? like, highlight a chonk of text, vids (embedding from youtube?? etc?), pictures, and just yeet to offerings for the Librarian to chew on via context menu?



## Response:
15.02.2026, 16:13:30

````plaintext
Thought process: Evaluated context menu integration feasibility and brainstormed implementation approaches.

Roche is asking about adding a context menu integration for sending stuff to the Noctuary offerings folder. This is a great idea and very doable technically. Let me think about the approaches.
````

Yes and this is actually critical for the zero-friction promise. If you have to manually copy → open terminal → paste → run command, that's already too many steps. The context menu IS the "slot in the heavy door."

**How this actually works technically:**

**Desktop level (OS context menu):**
- On Windows: registry entries that add "Send to Noctuary" to right-click. Python script receives the file/selection as argument. Genuinely not hard.
- On Linux: `.desktop` file + Nautilus/Dolphin scripts. Same idea.
- Both can be installed via a `noctuary install-context` CLI command.

**Browser level (the BIG one):**
- Small browser extension - adds "Offer to Noctuary" to right-click menu
- Works on: highlighted text, images, links, entire pages
- YouTube URLs get flagged for transcript/embed extraction during Binding
- The extension just fires the selection to a tiny local HTTP server that Vigil runs (localhost:port), which drops it into offerings. Dead simple.

**The flow becomes:**
1. See something yummy
2. Right click → "Offer to Noctuary"
3. Gone. Back to doomscrolling.

That's it. That's sub-2-seconds. Hyperfocus unbroken.

**For YouTube specifically** - the Vigil pipeline can detect YouTube URLs and auto-pull transcripts via `yt-dlp` (already handles this beautifully). So you right-click a video, yeet it, and the Librarian later has the full transcript indexed and connected to your other stuff. You don't even have to watch it right away - you've *captured the knowledge potential.*

This isn't Phase 1 MVP though - it's Phase 1.5. Get the engine working first, THEN add the frictionless capture methods. But architecturally, the local HTTP endpoint for Vigil should be planned from day one so the extension has something to talk to.

Want me to add this to the architecture doc or are we still riffing?



## Prompt:
15.02.2026, 16:14:22

we will update the doc later i am still buzzing with unbirthed nascent visions



## Response:
15.02.2026, 16:14:25

````plaintext
Thought process: Thinking about deferring document updates pending creative inspiration.

Love the energy. Let them cook.
````

The offerings folder is OPEN. Keep yeeting.





---
Powered by [Claude Exporter](https://www.ai-chat-exporter.net)
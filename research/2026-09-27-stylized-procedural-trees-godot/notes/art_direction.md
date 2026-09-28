# Art Direction for Painterly, Ghibli-Style Stylized Trees (toon-shaded 3D, ~45° third-person camera, 5–40 m)

Scope note: Primary sources for *specific* shipped-game foliage internals (Genshin, BotW/TotK, Animal Crossing, Harvest Moon: AWL, Ni no Kuni) are thin to nonexistent publicly. Most concrete, verifiable technique documentation comes from artist breakdowns (80.lv, Kids With Sticks, simonschreibt.de, Harry Alisavakis, Blender tutorials) and a few GDC talks. Several key pages (Polycount threads, ArtStation) returned HTTP 403 and could only be read via search snippets; this is flagged per item. Everything outside "Cited Findings" is inference and labelled so. All techniques are engine-agnostic; Blender/Unreal/Unity names are given where the source used them.

## 1. Silhouette and canopy design (clumps, hierarchy, negative space, trunk, roots)

### Takeaway
Ghibli-style painters and stylized-game artists treat a canopy as a small number of clustered masses ("bunches"), not leaves: large soft masses first, a few detailed accents last. Game artists build this literally as sculpted spheres/blobs on a simple branch skeleton, then fringe them with cards. Sourced guidance on exact clump counts, size ratios and sky-hole design is essentially absent. Those numbers below are inference.

### Cited Findings
- Kazuo Oga (Totoro, Only Yesterday, Princess Mononoke, Spirited Away) paints wet-into-wet, dropping paint onto soaked paper to get a "soft atmospheric base into which he can place smaller details of branches and leaves." He does the large soft passages first and the details last. — [Gurney Journey: Demo by Kazuo Oga](http://gurneyjourney.blogspot.com/2017/03/demo-by-kazuo-oga.html) (via search snippet); [Open Culture](https://www.openculture.com/2021/01/a-look-inside-the-painting-process-of-the-studio-ghibli-artist-kazuo-oga.html)
- Oga uses only two brushes, a flat hira-fude and a pointed sakuyo-fude. Everything "rough" (sky, clouds, distant mountains, rocks, plants) is done with the large flat brush. He "paints leaves roughly with hira-fude and adds a few detailed leaves on it." — [Gurney Journey: Kazuo Oga](http://gurneyjourney.blogspot.com/2011/02/kazuo-oga.html) (search snippet)
- Oga works impressionistically. His goal is "to get the essential point across effectively," painting something that *feels* like the subject rather than rendering it comprehensively. — [Animation Obsessive: What Kazuo Oga Thinks About When He Thinks About Backgrounds](https://animationobsessive.substack.com/p/what-kazuo-oga-thinks-about-when)
- Ghibli-style painting guidance says to cluster leaves into bunches that convey overall form instead of rendering individual leaves, and to manage where cloud edges meet tree edges so foreground and background stay distinct. — [GVAAT's Workshop: How to Paint Ghibli Backgrounds](https://gvaat.com/blog/how-to-paint-ghibli-backgrounds/)
- David Holland ("Meadows", UE4) built "very simple trees" with Blender's Sapling Tree Generator, placed spheres on the branches and sculpted them into canopy shapes, then scattered cards over those spheres with the hair particle system. — [80.lv: Meadows: Creating Stylized Nature in UE4](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)
- Kids With Sticks studied the vegetation in Totoro, Mononoke, Spirited Away and Nausicaä for their UE4 Ghibli project and used intersecting planes over spheres as particle leaves. — [Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/); [80.lv search snippet](https://80.lv/articles/working-on-an-environment-in-ghibli-style)
- Sea of Thieves environment rules (Rare, GDC 2018 "Visual Adventures on Sea of Thieves", Ryan Stevenson) were reported as follows. Individual assets should have an interesting, differentiated silhouette, both within a species and against other assets. Trees should be tested alongside bushes and rocks for "sympathetic relationships." Structures are simplified, and objects are "realistically wonky." — [GDC Vault](https://gdcvault.com/play/1025015/Visual-Adventures-on-Sea-of) (talk page); rules via [search summary / Lee Piper ArtStation](https://leepip.artstation.com/projects/bKP0Kg) and [Habrador blog](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html)
- Fortnite's stylization rule was to remove all parallel lines from real-world references, giving cartoon distortion rather than realistic imperfection. — [Habrador blog summarising the Fortnite GDC talk](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html)
- Oga adds place-specific accents, such as Dendrobium orchids growing on a Spirited Away tree, based on real observation around Mt. Takao. He emphasises on-site research because "photographs alone can't convey the atmosphere or the scale." — [Animation Obsessive](https://animationobsessive.substack.com/p/what-kazuo-oga-thinks-about-when)

### Inferences
- (Inference) **Clump hierarchy.** For a mid-size deciduous tree seen at 5–40 m, aim for 1 dominant mass, 2–4 secondary masses and 3–8 small "accent" puffs on the silhouette edge, roughly in a 3:2:1 size ratio (big/medium/small). This mirrors Oga's "large flat brush first, few detailed leaves last" and standard big-medium-small design practice. Avoid equal-size, evenly spaced puffs: they read as "broccoli" or clip-art.
- (Inference) **Canopy envelope.** Make the canopy's overall envelope asymmetric and leaning (not a sphere). Tilt masses toward the light/open side, and give the underside a flatter, darker plane. Ghibli trees typically read as stacked horizontal cloud-shelves with a flat-ish bottom edge.
- (Inference) **Sky holes.** Punch 1–3 deliberate sky holes through the canopy, sized so they survive at 40 m (on the order of 10–20% of canopy width). Use many tiny holes only in the card fringe. Holes should be irregular shapes, not circles.
- (Inference) **Trunk gesture.** Use one clear S- or C-curve gesture, strong taper (base roughly 2–3× the diameter at the first fork), and a visible root flare that spreads into the ground. Show 1–3 primary branches entering the canopy mass; branches should disappear *into* clumps rather than all being hidden. The trunk being visible through the lower canopy and sky holes is a key Ghibli signifier.
- (Inference) **Species differentiation.** Distinguish species by silhouette class (tall column, round, spreading umbrella, weeping, conifer tiers) before colour, per the Sea of Thieves "differentiated silhouette" rule.

### Gaps
- No primary source found giving explicit clump counts, size ratios or sky-hole rules from Ghibli or any studio. The Art of Totoro/Spirited Away books were not accessible online.
- No public primary breakdown found for tree silhouette design in BotW/TotK, Animal Crossing, Harvest Moon: AWL, Ni no Kuni, Tunic or Sable.

## 2. Value and colour design for foliage

### Takeaway
The core of the Ghibli read is a clear separation between the light family and the shadow family. Dark recedes, light advances, and colour temperature and saturation carry mood. The main production levers are a stepped light ramp and gradients from vertex colour, height or AO. Per-tree and per-clump hue/value variation keeps forests alive. Exact band counts are not documented in sources; one breakdown found an extra shadow band barely visible and removed it.

### Cited Findings
- "Pay attention to the separation of the shadow from the light as it is going to be key to creating depth in a Ghibli style painting." Darker areas recede and brighter areas advance, so shadowed portions suggest the canopy interior and lit portions read as closer. — [GVAAT](https://gvaat.com/blog/how-to-paint-ghibli-backgrounds/)
- Use colour temperature and saturation to convey time of day and emotional tone. — [GVAAT](https://gvaat.com/blog/how-to-paint-ghibli-backgrounds/)
- Oga worked with about 21 poster colours (e.g. bleu céleste, olive green). He said "even with a small number of colors, you can create a wide range of colors through subtle mixing." — [Animation Obsessive](https://animationobsessive.substack.com/p/what-kazuo-oga-thinks-about-when)
- BotW foliage uses vertex colours to darken the bottom vertices or lighten the tip vertex, producing cheap colour gradients. — search snippet from the [Polycount BotW smooth foliage thread](https://polycount.com/discussion/209623/smooth-foliage-like-in-breath-of-the-wild-europa-by-helder-pinto-mini-tutorial) (page 403; unverified secondary). BotW is deferred and uses light probes for ambient colour — [ResetEra technical analysis](https://www.resetera.com/threads/zelda-breath-of-the-wild-the-technical-analysis.8197/) (fan analysis, not official).
- Kids With Sticks used a "highlight height mask" in the material to change bush/tree colours. — [Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/)
- Anime foliage pipeline (Blender): Shader to RGB → Color Ramp with custom colour stops for the cel look, an Ambient Occlusion node multiplied into the colour, and an optional Gradient Texture for directional darkening. — [Trung Duy Nguyen: Anime Foliage Pipeline](https://trungduyng.substack.com/p/tutorial-blender-anime-foliage-pipeline)
- aVersion of Reality "Stylized Tree Shader" techniques:
  - fake Lambert shading and a "faked sun angle" in the material
  - AO used both to darken *and* to brighten
  - noise mixed into the normals for variety
  - per-island random values (via Geometry Nodes) so each leaf cluster varies
  - distance-from-camera masks
  - fake density from object coordinates
  
  — [aVersion of Reality](http://www.aversionofreality.com/blog/2022/8/7/stylized-tree-shader)
- Harry Alisavakis's stylized leaves shader uses a stepped N·L with a configurable cutoff. He removed extra shadow banding as "barely visible (or possible useful) in most cases." Albedo is multiplied by light colour, and he adds a rim term. — [Harry Alisavakis: Stylized tree leaves](https://halisavakis.com/my-take-on-shaders-stylized-tree-leaves/)
- Sea of Thieves avoided granular noise in textures, an approach attributed to Ghibli inspiration, giving a brushy look up close. — [Habrador blog](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html) (secondary)

### Inferences
- (Inference) **Value structure.** Use 2 main bands (lit/shadow) with a soft, slightly wide terminator, plus 1 accent band: either a small highlight cap on the top/sun-facing puffs or a rim/translucent glow. Three hard bands on leaf geometry tends to fragment. Harry Alisavakis's finding supports dropping extra bands.
- (Inference) **Hue shift.** Lit side: warm, yellow-green, higher saturation. Shadow side: cool blue-green or teal, *slightly lower value but not black*. Keep the shadow relatively saturated, which is typical of Ghibli poster colour. Drive the shadow tint from sky/ambient colour so time-of-day grading works.
- (Inference) **Gradients.** Two gradients stack:
  1. A vertical trunk-to-tip gradient (darker, cooler at the canopy base and interior; lighter, warmer at the tips), from vertex colour or object-space height as in BotW and Kids With Sticks.
  2. A radial "interior-darkening" AO gradient from canopy centre to shell.
- (Inference) **Variation.** Apply small per-clump hue/value jitter (±3–6% value, a few degrees of hue) and larger per-tree jitter (instance-random tint, e.g. from world position hash). Keep all variation inside a restricted palette so the forest reads as one colour family. This follows Oga's "few colours, subtle mixing".
- (Inference) **Rim/translucency accent.** Reserve it for backlit situations. It is the cheapest way to get the Ghibli "glowing edge" when the sun is behind trees.

### Gaps
- No sourced numeric palette or band count from Ghibli, Genshin, BotW, Ni no Kuni or Animal Crossing.
- Could not access BotW-specific shader documentation beyond fan and secondary analyses.

## 3. Solid blob canopies vs alpha-cutout leaf cards vs hybrid

### Takeaway
There are three families.
- **Solid sculpted blobs** are cheap, clean, and have no overdraw. They read very "toy/Animal Crossing".
- **Pure card clouds** are fluffy and painterly, but heavy on overdraw and noisy without normal editing.
- **Hybrid** (opaque blob core plus a card fringe) gets the painterly broken silhouette while the core occludes interior cards and hides gaps. It is the most commonly documented stylized approach. Genshin reportedly mixes static geometry with sprites for large trees.

### Cited Findings
- The Airborn trees (analysed by Simon Trümpler) use a central blob mesh plus surrounding leaf cards. The big blob mesh "culls most" transparent planes, so "you only need to care about the stuff of the front/sides," which reduces overdraw. Normals are projected from the base sphere onto the leaf planes. — [simonschreibt.de: Airborn – Trees](https://simonschreibt.de/gat/airborn-trees/)
- Genshin Impact's large trees reportedly combine static geometry with sprites. Bushes and canopies are about 1,300–3,500 triangles each, with LODs. — [parsers.vc](https://parsers.vc/news/250124-the-art-of-game-rendering--a-deep-dive-into/). **Reliability warning:** uncited third-party article, original frame-analysis source not given.
- Meadows (UE4) uses sculpted sphere canopy volumes with cards scattered on them by particle system. — [80.lv](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)
- Anime foliage pipeline: camera-facing planes (Geometry Nodes aligns rotation to the camera) are instanced over a subdivided-cube emitter with scale randomness 0.2. Normals come from the emitter, and a scale step "inflates the bush." Blend and shadow mode are both Alpha Clip. — [Trung Duy Nguyen](https://trungduyng.substack.com/p/tutorial-blender-anime-foliage-pipeline)
- Viktoriia Zavhorodnia (akbutea), "Stylized Fluffy Trees Tutorial": canopy made in Tree It or Maya/ZBrush, then quads converted to billboards in a shader. — [ArtStation](https://www.artstation.com/artwork/lR4JJO) (403; search snippet only)
- A Godot Shaders "Stylized Fluffy Tree Leaves" shader exists with billboarding, fresnel and wind. — [godotshaders.com](https://godotshaders.com/shader/stylized-fluffy-tree-leaves/) (not fetched)
- Tunic's early prototypes used "entirely flat colour on low poly geometry, relying mostly on lighting to add depth." Dappled light on the forest floor suggests canopy above. — [Wireframe: Tunic](https://wireframe.raspberrypi.com/articles/wireframe-cover-star-tunic-and-the-art-of-keeping-a-secret)

### Inferences
- (Inference) **Choice by look.**
  - Solid blobs (faceted or smooth) fit Animal Crossing, Tunic, Harvest Moon: AWL and toy-like reads. They are best when outlines and flat colour carry the style.
  - Hybrid fits Ghibli, Genshin, BotW and Kena.
  - Pure cards suit hero trees or bushes where overdraw is affordable.
- (Inference) **How the fringe breaks the silhouette.** Cards should extend past the blob surface by roughly 10–30% of the local clump radius, only on the outer shell. Use leaf textures whose alpha is a *cluster of 3–7 painted leaf shapes*, not single leaves or photo sprays, so the edge reads as brush dabs. Concentrate fringe on upper/sun-side edges and thin it on the underside, which should stay a cleaner, darker plane.
- (Inference) **Why the core matters.** The blob core should be tinted to match the shadow colour so gaps between cards read as depth rather than holes. Its normals must be the same smoothed proxy normals as the cards so shading is continuous.
- (Inference) **Camera-facing cards.** Full billboarding keeps the canopy puffy from every angle. At a fixed ~45° camera, partial (Y-locked or view-aligned-with-offset) billboarding may be enough and avoids "swimming."

### Gaps
- No official breakdown found for Genshin, BotW/TotK, Animal Crossing, Ni no Kuni or Kena canopy construction. Kena interviews discuss wind and restoration regrowth, not construction ([Unreal Engine interview](https://www.unrealengine.com/en-US/developer-interviews/the-magic-of-creating-kena-bridge-of-spirits)).

## 4. Shading tricks for soft toon foliage

### Takeaway
The canonical technique is **normal transfer / normal projection from a smooth proxy**: sphere, ellipsoid, the inflated blob core, or a per-clump sphere. It is also called "spherized normals," "radial normals," or "custom normals from sphere" (Blender: Data Transfer → Face Corner Data → Custom Normals; older: Normal Edit modifier, Radial mode). Combine it with:
- two-sided normal correction
- vertex-colour/AO gradients
- a view/light-dot "fake SSS" translucency term
- *no* normal maps (they add noise)

### Cited Findings
- Edited vertex normals on foliage smooth the shading of cards. "If this isn't done, the rotation of the cards will make ugly, sudden changes in the shading." — [simonschreibt Airborn – Trees](https://simonschreibt.de/gat/airborn-trees/) (search snippet paraphrase)
- Neox (polyphobia.de) explained that normals are projected from the base sphere onto the leaf planes. This prevents unlit faces from looking too dark and creates smooth shadow gradients. The SlideNormalThief script (Slide London) automates it. — [simonschreibt Airborn – Trees](https://simonschreibt.de/gat/airborn-trees/)
- Meadows: "modify the card normals using a sphere encompassing the entire tree and transferring the sphere's normal information to the cards" (Blender Data Transfer). "TwoSided Sign multiplied with a three constant" flips back-facing card normals so both sides light the same. — [80.lv Meadows](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)
- Kids With Sticks transferred normals from a half-sphere. They found custom normals weren't the only problem and needed a "two-sided material with custom vertex normals" material function from the Unreal forums. In pre-4.1 Blender, "Auto Smooth" must be on or custom normals don't work. They also note normal transfer "has existed long before modern game development" and "still works great for stylized assets." — [Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/); [80.lv snippet](https://80.lv/articles/working-on-an-environment-in-ghibli-style)
- Blender recipe: Data Transfer modifier, source = sphere, "Face Corner Data" → "Custom Normals". Alternatively, Normal Edit modifier targeting a sphere at the origin, which gives "The Witness"-like smooth, low-noise, cel-shaded foliage. — [Polycount: soft normal foliage](https://polycount.com/discussion/217928/how-do-i-get-soft-normal-foliage-like-this) (403; snippet); [Habrador: How to make stylized "The Witness" trees in Blender](https://blog.habrador.com/2018/05/how-to-make-stylized-witness-trees-in.html)
- aVersion of Reality combines radial/spherical normals with compensation for object transforms, mixes noise into normals for variety, and uses Surface Gradients to combine normal-map detail with custom normals. — [aVersion of Reality](http://www.aversionofreality.com/blog/2022/8/7/stylized-tree-shader); video [YouTube](https://www.youtube.com/watch?v=5itzrrhg8TE)
- Harry Alisavakis's leaves shader:
  - Cull Off
  - "normal maps were just adding noise," so he dropped them
  - stepped N·L with a cutoff
  - fake SSS = dot(-lightDir, viewDir) raised to a power, multiplied by attenuation/shadow
  - alpha clip
  - wind via UV displacement from a panning texture
  - rim term, with a warning that it produces "weird cutoff artifacts" on procedurally generated trees
  
  — [Harry Alisavakis](https://halisavakis.com/my-take-on-shaders-stylized-tree-leaves/)
- Anime foliage pipeline: normals transferred from the emitter shape, then an "inflate" scale, Shader-to-RGB → ColorRamp, AO multiply. — [Trung Duy Nguyen](https://trungduyng.substack.com/p/tutorial-blender-anime-foliage-pipeline)
- In Unreal, the directional light "shadow sharpen" setting helps the toon look on trees. — [YouTube: How to Create Stylized Toon-Shaded Trees in Blender](https://www.youtube.com/watch?v=eOknkqWs76g) (search snippet)

### Inferences
- (Inference) **Proxy choice.** One sphere per tree gives the most "painted" single-mass gradient. One proxy per clump (or transfer from the inflated blob core) gives multi-puff shading, where each puff has its own light/shadow, which is closer to Ghibli cloud-shelves. A practical blend is to lerp 50–80% toward the per-clump normal and the rest toward the whole-tree sphere.
- (Inference) **Terminator noise.** To avoid noisy toon terminators on leaf geometry:
  - use smoothed proxy normals only
  - no normal maps on leaves
  - widen the ramp's smoothstep slightly (soft edge)
  - receive self-shadows through a softened or low-res shadow and optionally ignore self-shadowing on cards, since shadow-map acne on cards is a major noise source
  - drive AO from vertex colour or radial distance, not SSAO, on leaves
- (Inference) **Procedural (Godot) equivalents.** Write NORMAL in the vertex shader as normalize(VERTEX − clump_center) (or from a baked custom-normal attribute). Flip for back faces with FRONT_FACING. Implement fake SSS in `light()`. Bake interior AO as COLOR = saturate(distance_from_center / radius).

### Gaps
- No official GDC/SIGGRAPH talk found that documents foliage normal-transfer in a named AAA stylized game. Evidence is from indie and artist breakdowns and Polycount. The Polycount BotW "smooth foliage" mini-tutorial by Helder Pinto (Europa) could not be read (403).

## 5. Outlines on foliage in toon games

### Takeaway
Character outlines in toon games are typically inverted-hull. Environment foliage generally gets no outline or a faded/secondary outline, because outlines on alpha-cut cards become noisy. Sable is the documented case for heavy environment outlines, and it fades them with distance for readability.

### Cited Findings
- Genshin's character outline is an inverted hull: a second pass with a slight normal offset produces the ink outline. Recreations use Solidify with a negative thickness, flipped normals and a separate backface-culled outline material. — [parsers.vc](https://parsers.vc/news/250124-the-art-of-game-rendering--a-deep-dive-into/) (unverified); [Ben Ayers: Recreating the Genshin Impact Shader](https://bjayers.com/blog/9oOD/blender-npr-recreating-the-genshin-impact-shader)
- Sable outlines everything with thin black lines and uses "fading opacity" so outlines fade with distance. This emphasises perspective and disguises pop-in. Fog was "really, really key" for mid-to-long-distance readability. — [Game Developer: How Shedworks refined the art of Sable in pursuit of readability](https://www.gamedeveloper.com/marketing/how-shedworks-refined-the-art-of-sable-in-pursuit-of-readability); GDC talk: [The Art of 'Sable'](https://gdcvault.com/play/1027721/The-Art-of-Sable-Imperfection)
- A Godot shader inspired by Sable exists. — [80.lv](https://80.lv/articles/have-a-look-at-this-bright-godot-shader-inspired-by-sable) (not fetched)

### Inferences
- (Inference) BotW/TotK, Genshin, Ni no Kuni and Kena environment trees appear (from gameplay observation, not a source) to have no ink outline on foliage. Separation comes from value, rim and fog instead. Treat this as unverified.
- (Inference) If outlines are wanted on trees:
  - Apply an inverted hull only to the *blob core* and trunk, not the cards. The hull of a smooth blob gives a clean cloud-edge contour.
  - Alternatively, use post-process depth/normal edge detection with a high depth threshold, so only silhouette edges against sky and background fire and not intra-canopy card edges.
  - Tint outlines toward a dark foliage colour instead of black, and fade them with distance as Sable does.
  - Card-based foliage plus normal-edge detection produces fizzing lines. Rely on depth only, or mask foliage out of the normal-edge term.

### Gaps
- No primary source found documenting outline handling specifically for trees in Genshin, BotW/TotK, Tunic or Animal Crossing.

## 6. LOD and readability at distance (keeping forests from looking noisy)

### Takeaway
Stylized games manage noise by simplifying toward masses with distance, removing high-frequency texture noise, and leaning on fog and atmospheric perspective. Level of detail decreases with distance in the paintings too: detailed foreground trunks, one or two colours for background trunks.

### Cited Findings
- Kids With Sticks followed Ghibli's hierarchy. Background trunks use "one or two colors and slight detail," while foreground trunks are "intricately detailed, stratified" with distinctive brush strokes. — [Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/)
- Sea of Thieves: no granular noise in textures (Ghibli-inspired). Fortnite rule: "Don't add anything smaller than a mailbox" to maintain player flow. — [Habrador blog](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html)
- Sable: distance-faded outlines plus per-biome fog for mid/long-distance readability. — [Game Developer](https://www.gamedeveloper.com/marketing/how-shedworks-refined-the-art-of-sable-in-pursuit-of-readability)
- Genshin: canopy LODs, 1.3k–3.5k tris per bush/canopy (unverified). — [parsers.vc](https://parsers.vc/news/250124-the-art-of-game-rendering--a-deep-dive-into/)
- The aVersion of Reality shader uses distance-from-camera masks. — [aVersion of Reality](http://www.aversionofreality.com/blog/2022/8/7/stylized-tree-shader)
- Oga paints big soft masses first and adds only a few detailed leaves. — [Gurney Journey](http://gurneyjourney.blogspot.com/2017/03/demo-by-kazuo-oga.html)

### Inferences
- (Inference) **Distance bands for a 5–40 m camera.**
  - 5–15 m: full hybrid (blob plus fringe cards), leaf-cluster alpha visible, trunk bark strokes visible.
  - 15–30 m: fewer, larger cards (merge clusters), fringe only on the silhouette, no bark detail.
  - 30 m+: blob-only or impostor. Colour comes from the ramp and gradient alone, and the value range compresses toward the fog/sky colour.
  
  Because smoothed proxy normals are shared by cards and blob, the shading stays identical across LOD swaps and hides pops.
- (Inference) **Forest-scale readability.**
  - Group trees into masses (overlapping canopies read as one shape) with clearings.
  - Vary silhouette class across species.
  - Keep per-tree tint variation low-frequency.
  - Push distant rows cooler and lighter (atmospheric perspective via fog or height fog).
  - Avoid high-contrast small sky holes in background trees.
  - Clamp alpha-to-coverage/TAA shimmer by reducing card count with distance.
- (Inference) **Texture frequency.** Keep leaf-card texture frequency low: 3–7 dabs per card, no photographic detail, no per-leaf normal maps, so mipmapping doesn't produce grey mush.

### Gaps
- No primary sources on LOD systems for BotW/TotK, Animal Crossing, Ni no Kuni or Harvest Moon: AWL trees.
- No quantitative "detail surviving per metre" guidance found; the distance bands above are inference.

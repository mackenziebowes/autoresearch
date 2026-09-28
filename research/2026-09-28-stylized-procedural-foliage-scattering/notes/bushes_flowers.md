# Stylized procedural bushes, hedges, flowering shrubs and small flowers (Godot 4.7, Blender 5.2 GN)

Scope: round shrubs, hedges, flowering bushes (hydrangea / hibiscus / beach rose), small flowers and wildflower clumps for a cozy Ghibli / HM:AWL / Ni no Kuni seaside village, fixed ~45° camera, 5–40 m.

Read first: `reports/Stylized procedural trees for Godot.md` + its Addendum (2026-09-28), and the actual generator `FarmGameGodot/tools/tree/gn_tree.py` and shader `FarmGameGodot/shaders/tree.gdshaderinc` (both inspected for this note; statements about them below are from reading the code, not from web sources).

Labelling convention: **[S]** = sourced claim (link inline). **[EJ]** = engineering judgement / inference. **[CODE]** = read directly from our repo.

Honest overall caveat: the public record on *how* ACNH, HM:AWL, Story of Seasons, Ooblets, Garden Paws, Ni no Kuni or Genshin build bushes and flowers is essentially empty (no breakdowns found). Most bush/flower "rules" come from indie/student 80.lv breakdowns, Sea of Thieves artist notes, and the same proxy-normal lineage as the tree report. Numbers are [EJ] unless marked.

---

## Q1. Bushes: does the blob + cards hybrid transfer directly? What changes at 0.5–1.5 m?

### Takeaway
Yes — a bush is, to first order, "the tree generator with the wood stage switched off, a low flattened envelope, and every length constant scaled ~0.2×". The blob core + shell cards + proxy normals + seam darkening is exactly how stylized bushes are built in the published breakdowns. But `gn_tree.py` has several **hard-coded metre-scale constants** and an always-on trunk, so it does not transfer "for free": it needs a `Scale`/`Wood On` input pair, a ground-contact step (sunk base + card skirt), and a much higher card density per m³.

### Cited Findings
- Stylized bushes and trees in breakdowns are built by copying a sphere's (or blob's) vertex normals onto the leaf cards; "to create fluffy, cute, Ghibli-styled trees and bushes, artists copy the sphere's vertex normals into the normals of the foliage leaves"; sphere-normal projection "makes foliage cards appear very smooth without dark shadows casting onto themselves", with the caveat that shading can feel "too uniform for large canopy meshes" (a caveat that matters *less* at bush scale) — [80.lv search summary of Meadows / stylized foliage articles](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4) [S]
- Geometry Nodes pattern for exactly this: capture the blob mesh's normals with Capture Attribute *before* instancing leaf planes on its surface, so each leaf inherits the normal of the face it sits on; debug by plugging the captured vector into the shader colour — [Blender Artists, "Transferring normals for a stylized bush"](https://blenderartists.org/t/transferring-normals-for-a-stylized-bush/1517824) [S]. (Our generator already does the equivalent with `Sample Nearest Surface` of the `proxy` attribute [CODE].)
- Kids With Sticks (Ghibli-inspired UE4): bushes built from simple mapped planes with hand-painted leaf textures, assembled into larger composite meshes; "some bushes included an inside mesh, which is better for optimization and distance fields"; custom normals via DataTransfer (Auto Smooth required in older Blender); a "highlight height mask to change colors of bushes and trees" — [Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/) [S]. The "inside mesh" is our blob core.
- A stylized UE environment built bushes purely from leaf cards (placed manually in Maya or with Tree It), leaf-card textures from Substance Designer; all vegetation except trunks shared one master material; wind via vertex paint at the roots — [80.lv, Setting Up Trees, Bushes and Flowers](https://80.lv/articles/setting-up-trees-bushes-and-flowers-for-a-stylized-3d-environment) [S]
- Sea of Thieves: deciduous trees were blocked in with solid sphere cores "to combat overdraw", then leaf cards added; assets are always tested in collections (trees with bushes and rocks) for "sympathetic relationships"; "mid height plants work with low level flowers to bulk out areas with their differing overall and internal silhouettes — broadly speaking, pointy vs rounded" — [Lee Piper, Sea of Thieves asset creation (ArtStation; page returned 403, quoted from search snippet)](https://leepip.artstation.com/projects/NxBrbq); [Habrador GDC summary](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html) [S, partially unverified]
- Tree report already assigned bush preset values: height 0.6–1.5 m, "3–7 short stems or none", "5–15 puffs in flattened ellipsoid (h≈0.6r), top-biased, base sunk", "SDF fuse, coarse", fringe density 0–0.3, 300–800 tris LOD0, minimal wind, optional flowers via normal.z filter — [tree report, per-type table](../../2026-09-27-stylized-procedural-trees-godot/report.md) (these are the report's own [EJ] numbers, not studio rules). The Addendum's "blob with stickers" lesson (sparse cards on a smooth core look plastic; project the dab atlas onto the core in world space at ~35% and use more/larger cards) — [tree report Addendum] — applies directly to bushes and argues **against** the report's fringe 0–0.3 figure.

### What is hard-coded in `gn_tree.py` / `tree.gdshaderinc` and must change for bushes [CODE]
| Item (code) | Tree value | Problem at bush scale | Fix [EJ] |
|---|---|---|---|
| Stage 3/4 wood always built; `MeshLine` trunk from z=−0.3 to canopy bottom + `Trunk Into Canopy` even when `Iterations = 0` | on | A trunk stick is emitted under every bush | Add `Wood` bool input → Switch the wood branch to empty; optional "stems" mode = Iterations 5–15, Step 0.06, Tip Radius 0.006 |
| `MeshToVolume(core, …, "Size", 0.15)` voxel for attractors/cards | 0.15 m | ~7 voxels across a 1 m bush; card shell volume becomes blocky/empty | Drive from `Core Voxel` or a `Scale` input |
| `Core Voxel` default 0.14, report rule "canopy diameter / 20" | 0.14 | 1 m bush → 0.05 | 0.04–0.06 (tri count then ~300–900 after adaptivity 0.6) |
| Card offset `rand(0.03, 0.22)` | metres | 22 cm float-off on a 1 m bush = floating cards | × Scale (≈0.01–0.05) |
| `Card Density` is **per m³** of shell volume (`DistributePointsInVolume`) | 160 | Volume shrinks ~100× → ~2–10 cards | 2,000–6,000/m³ to land 60–200 cards |
| `Shell` 0.3, `Clump Spacing` 1.6, `Clump Radius` 0.55–1.35, `Card Size` 1.0 | m | all tree-scale | Shell 0.06–0.1; spacing 0.25–0.45; radius 0.15–0.4; card size 0.25–0.45 m |
| Underside deletion `cnz < -0.55`, `Shelf Flatten` 0.72 | | Fine; bush wants *more* flatten at base | Also clip everything below z=0 and sink (see ground contact) |
| Whole-tree spheroid stretch `xyz(1,1,1.8)`, radial `xyz(1,1,0.74)` | | Tuned for tall canopies | Expose; bush ≈ (1,1,1.0–1.2) |
| Wood `Col.a = z / 7.4`; shader `tree_height = 7.4`, `wind_strength 0.10 m` | | Wind height mask saturates/under-drives | Per-material `tree_height` ≈ bush height; sway 0.01–0.03 m |
| UV2 = card corner offset (m) ×0.25+0.5, decoded `(UV2-0.5)*4.0`; `card_scale` uniform | | Works at any scale (range ±2 m), **but** if you "generate at 5× then Transform ×0.2" the UV2 offsets are not scaled | If using the scale-at-end shortcut, also multiply the stored UV2 offset (store after transform) or set `card_scale = 0.2` |

**Scale-at-end shortcut [EJ]:** the cheapest port is to feed the unchanged tree graph a ~5 m bush-shaped envelope with `Wood` off, then `Transform Geometry` ×0.2 before export. Proxy normals are directions and survive uniform scale; seam/grad are scale-free. Only UV2 (metre offsets) and the shader's `tree_height`/`wind_strength` need adjusting. Card *texel size* shrinks 5× too, so raise `Card Size` to ~1.5–2 at build scale so dabs stay readable (a 0.2 m card at 40 m is ~2–3 px at 1080p — [EJ] estimate).

### Inferences — bush-scale construction rules [EJ]
- **Card size vs bush:** ~25–40% of bush diameter at LOD0 (0.25–0.45 m on a 1 m bush). Each card still carries 3–7 painted dabs, so the *dab* ends up ~5–10 cm — the "few detailed leaves" of Oga's method. Smaller cards → noise at 40 m.
- **Card count:** 60–200 per bush (vs 300–500 per tree). With the Addendum's core-dab projection at ~35%, 80–120 is likely enough. Tris: core 300–900 + cards 120–400 → **~500–1,300 tris LOD0**; the report's 300–800 is achievable only with ≤60 cards or a coarser core.
- **Stems/branches:** default *hidden*. Show 2–5 stems only on the sparse/wild variant (beach rose, young hibiscus) and only where a gap reveals them near the base; at 45° down-view the base is mostly occluded. Stems as Curve to Mesh with 4–5 sided profile, ~20–60 tris each.
- **Ground contact (the #1 bush tell):** (a) sink the core 10–20% of its height below z=0 (clip below ground so no hidden tris); (b) flatten the bottom 30% hard (`Shelf Flatten` ≥0.8); (c) a **skirt** of 8–20 cards placed around the base ring, tilted outward 30–60° and pushed down to touch the ground, tinted with `col_under` so the bush "sits"; (d) optionally a ground-aligned decal/blob-shadow quad (dark, soft, ~1.2× footprint) — cheap and very Ghibli (painted contact shadow). The tree generator deletes underside cards (`prz < -0.3`, `grad < 0.33`) which is correct for trees but will leave a bald base on bushes; the skirt is a separate small point set.
- **Silhouette:** Sea of Thieves' rounded-vs-pointy pairing suggests: round shrubs (dome), hedges (box/loaf), flowering shrubs (lumpy with protruding flower heads), wildflowers (spiky/pointy). Fortnite's "no parallel lines" → no perfectly symmetric domes; lean the mass toward the light, 1 dominant + 2–4 secondary lumps, "realistically wonky" ([Habrador](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html) [S] for the principles; the lump counts are [EJ]).
- **Envelope is the art-direction lever** (Addendum finding) — for bushes, keep a small library of 4–6 hand-sculpted/low-poly envelopes (dome, loaf, kidney, mound-pair, lopsided) rather than randomizing ellipsoids.

### Gaps
- No studio source gives card counts or tri budgets for stylized bushes. ACNH/Story of Seasons bush models have no public technical breakdown.
- Whether the tree's 80/20 per-clump/whole-bush proxy blend is right for bushes is untested; at bush size a single whole-bush sphere (Kids With Sticks / Meadows style) may read better from 20–40 m.

---

## Q2. Hedges: extruded blob along a curve, topiary vs wild, tiling, modular vs spline

### Takeaway
Two workable constructions: (1) **spline-generated**: resample a polyline, place clump points along it, SDF-fuse → one continuous blob + shell cards (our GN stages 1/2/5 almost unchanged); (2) **modular**: 2–3 m straight segments + end caps + corners, each generated by the same graph with closed-form ends. Commercial tools use spline placers of modular pieces. For a village with player-shaped gardens, generate modules procedurally and place them along paths with a spline placer; bake long authored hedges as one mesh per ~8–16 m chunk.

### Cited Findings
- UE "Modular Hedges and Bushes" pack ships a **SplinePlacer** tool to build hedgerows along a spline from modular pieces — [Fab / UE Marketplace](https://www.unrealengine.com/marketplace/en-US/product/modular-hedges-and-brushes) [S]
- A free Blender "Hedge Generator" creates customisable hedges "with just a curve" (clean suburban, maze, overgrown) — [Blender Everything](https://www.blendereverything.com/download.php?id=43) [S] (licence not stated on the listing — check before reuse).
- Forest Pack (3ds Max) trimmed hedges/topiary: either pre-built components that "fit together like a jigsaw", or leaves scattered on a spline-based scatter surface for a fully parametric hedge — [CG Record tutorial](https://tutorials.cgrecord.net/2017/04/creating-trimmed-hedges-and-topiary.html) [S]
- Modular kits in general: "A modular kit is a 3D tileset made of modules … designed to snap together" — [Level Design Book](https://book.leveldesignbook.com/process/env-art) [S]

### Inferences [EJ]
- **Trimmed box/topiary hedge:** envelope = rounded box (bevel radius 15–25% of height) → `Points to SDF Grid` is the wrong tool; use `Mesh to SDF Grid` of the rounded box, then add small bumps (a light Poisson clump layer, radius 0.1–0.2 m, *inset* heavily) + `SDF Grid Mean` so the top reads "clipped but leafy". Proxy normals: mostly the box's own smoothed normals (large-radius blur) rather than per-clump — topiary is supposed to read as a planar face with a soft edge. Cards short (0.15–0.25 m), dense, very low overhang (≤3 cm), no sky holes. This is closest to HM:AWL / ACNH hedge read.
- **Wild hedge / hedgerow:** envelope = swept profile (Curve to Mesh with a lumpy ellipse profile + noise on radius) → Poisson clumps on it → the tree pipeline unchanged. Vary height ±20% along length with low-frequency noise; break the top line every 1.5–3 m with a secondary lump (anti-parallel-lines rule).
- **Seams:** for modules, (a) generate a segment 10–15% longer than its snap length and clip with a plane, so the SDF surface crosses the cut orthogonally; (b) use **world-space** leaf-dab projection and world-space brush noise (already the case for core dabs in our shader [CODE: `core_dab_scale` "atlas repeats per metre"]) so textures continue across joints; (c) cards within ~0.3 m of a cut face are allowed to spill over it — overlap hides the join; (d) proxy normals at the cut must be the *extruded* direction (no end-cap bulge) — easiest by making the whole-hedge proxy a swept cylinder/rounded-box normal, not a spheroid. End caps are a separate module with a rounded end.
- **Spline vs modular in Godot:** a GDScript `@tool` that walks a `Path3D` and places module MultiMesh instances (with random 180° flips and ±5% scale) is cheap and lets level design redraw hedges. A single GN-swept hedge per path is prettier (no repetition) but needs re-export when the path changes. Recommendation: modules for the village grid, GN-swept for 2–3 hero hedges.
- Tri budget: ~600–1,200 tris per metre of wild hedge at LOD0; ~300–600/m for topiary (fewer cards).

### Gaps
- No public breakdown of how any cozy game (ACNH hedges, Story of Seasons fences/hedges) builds hedges.
- Blender 5.2 `Mesh to SDF Grid` exact node identifier not verified here (the tree code uses `GeometryNodePointsToSDFGrid`, `GeometryNodeSDFGridMean`, `GeometryNodeGridToMesh` [CODE]; introspect `bpy.types` for the mesh variant).

---

## Q3. Flowering bushes (hydrangea / hibiscus / beach rose): flower layer, colour placement, how cozy games do it

### Takeaway
Put flowers on a **separate, switchable layer** (its own surface/material, or a separate MultiMesh) placed on the upper/sun-side core surface in **clusters**, not sprinkled. Hydrangea = a few big domed "mophead" blobs (geometry, a second small SDF blob per head with a petal-dab texture); hibiscus/beach rose = sparse, larger single-flower cards or 5-petal low-poly meshes facing up/out. Keep flowers separate because seasonality (ACNH-style budding/bloom/off-season) is a gameplay state.

### Cited Findings
- ACNH has 7 bush species (camellia, azalea, hydrangea, plumeria, hibiscus, tea olive, holly), 13 variants; bushes show **buds before their season, flowers during it, and revert to plain green after** — i.e. the flower layer is a seasonal state on a shared green base — [Animal Crossing World](https://animalcrossingworld.com/guides/new-horizons/all-bush-types-colors-list-seasons-how-to-plant/); [Nookipedia](https://nookipedia.com/wiki/Bush) [S]
- Flowers in a stylized UE scene were modelled high-poly, **baked down to planes** (512 textures, separate sheets for flowers and grasses), then cut out with extra edges following the shape — [80.lv](https://80.lv/articles/setting-up-trees-bushes-and-flowers-for-a-stylized-3d-environment) [S]
- "Flat flowers" method: flowers and leaves are not sculpted; base colour and alpha painted in Photoshop, final shape is simple plane geometry in Maya — [ArtStation, Viktoriia Zavhorodnia "Stylized Flowers Tutorial"](https://www.artstation.com/artwork/VynVDR) [S, from search summary]
- Tree report already lists "Optional flowers via normal.z filter" for the bush preset — [tree report table] ([EJ] in the report).
- Oga paints masses first and "adds a few detailed leaves on it" — [Gurney Journey](http://gurneyjourney.blogspot.com/2011/02/kazuo-oga.html) [S]; the same logic applies to flowers: flowers are the "few details".

### Inferences [EJ]
- **Three options, ranked for this project:**
  1. *Second card layer* (cheap, painterly; default for hibiscus/beach rose/azalea): flower cards 0.08–0.2 m, alpha-scissored, same proxy normals as the leaves so they shade with the bush, offset outward 1–3 cm more than leaf cards so they sit on top. 8–30 per bush.
  2. *Geometry heads* (hydrangea, and any flower that must read at 40 m): each mophead = small SDF/icosphere blob (r 0.08–0.15 m, 40–120 tris) with a petal-dab texture projected world-space, proxy normals = own sphere blended 50% with bush proxy. 5–15 heads. Reads as a coloured mass, which is exactly how painters render hydrangea.
  3. *Texture on the core* (farthest LOD only): a flower-colour mask baked into core vertex colour / a flower channel; flowers become colour patches. Use at 30 m+ or as the LOD2 of options 1–2 so bloom colour doesn't pop off.
- **Placement rule — cluster, don't sprinkle:** Poisson-pick 2–5 *cluster centres* on the upper/sun-side core (proxy·sun > 0.3, proxy.z > 0.2), then place flowers within a radius 0.1–0.25 m of each centre with density falling off (e.g. keep prob = 1 − (d/R)²). Leave ≥40% of the visible surface flower-free so green frames the colour. Pure uniform scattering produces the "sprinkles" look.
- **Colour:** flower hue from a per-species palette of 2–4 swatches (e.g. hydrangea blue→violet→pink by a per-bush hash — hydrangea colour genuinely varies by soil, a nice seaside-village story detail), with per-head value jitter ±5%. Flower shadow side should shift toward the bush's shadow family (cool) but keep saturation — never grey.
- **Seasonality:** export flower layer as its own mesh surface (material slot) or its own MultiMesh so Godot can hide/show/scale (bud = 30% scale + green-tinted) per season without regenerating.
- **Beach rose (Rosa rugosa) specifics:** low, sprawling, lumpy mound; wrinkled dark leaves; sparse single 5-petal magenta/white flowers; orange-red hips in late season → use option 1 flowers + small red sphere instances for hips (cheap, very readable at a seaside).
- **Hibiscus:** taller, more open, glossy leaves; large (10–15 cm) single flowers facing out → option 1 with bigger, fewer cards (6–15), or 5-petal meshes (~30–60 tris) for the hero near-camera bushes.

### Gaps
- No technical breakdown found for how ACNH, HM:AWL, Story of Seasons, Ooblets, Garden Paws, Ni no Kuni or Genshin build bush flowers (geometry vs cards vs texture). Screenshots suggest ACNH uses modelled low-poly flower heads on a solid bush body, but I found no citable source.

---

## Q4. Small flowers and wildflower clumps: construction, instancing, 45° read, tint

### Takeaway
At 5–40 m and 45° down-view, individual stems barely read; **flower heads are what read**, as coloured dots in clusters. Build: (a) one or two procedurally generated *clump* meshes per species (3–9 stems with heads, 60–300 tris) scattered with MultiMesh; (b) a far tier that is either an up-facing flower-head card or just a colour patch in the grass/terrain. Heads must face mostly **up** (toward the 45° camera), not sideways as in real-life side views.

### Cited Findings
- Vogel's sunflower model: floret n at r = c·√n, θ = n·α with α = golden angle ≈ 137.5° gives interlacing phyllotaxis spirals — [Wolfram MathWorld, Vogel Spiral](https://mathworld.wolfram.com/VogelSpiral.html) [S]. Use for daisy/aster/sunflower disc centres and petal rings.
- Procedural blooming flowers in GN (Alex Martinelli): phyllotaxis + instancing; "the petals use Hair nodes' attributes, like the Roll curve" — [80.lv](https://80.lv/articles/magical-procedural-blossoming-flowers-created-with-blender-s-geometry-nodes) [S]
- Several GN flower tutorials exist (flower from any curve; Blender 4.0 flower series), all following curve-stem → instance petals at the curve end — [YouTube: Generate flowers from ANY curve](https://www.youtube.com/watch?v=M58jpYXHSSU); [Create a Beautiful Flower in Blender 4.0](https://www.youtube.com/watch?v=JYJFaamXWCw) [S, titles only, not watched]
- EmacEArt Godot 4 stylized grass: clumps on MultiMesh, "a meadow … of 524,000 clumps across three MultiMesh fields"; wind in three layers (whole clump sway, single-blade tremble, gust crossing the field); light/dark flicker at distance fixed by making blades "face the sky more", so "the field keeps one steady tone at any distance" — [EmacEArt devlog](https://emaceart.itch.io/emaceart-stylized-grass-grass-shader-for-godot-4/devlog/1646260/emaceart-stylized-grass-120-godot-4-grass-shader-without-the-light-and-dark-flicker); [itch page](https://emaceart.itch.io/emaceart-stylized-grass-grass-shader-for-godot-4) [S] (licence not stated in the fetched text). The "face the sky" normal fix is directly applicable to flower heads.
- Sea of Thieves pairs mid-height plants with low-level flowers, contrasting pointy vs rounded silhouettes — [Lee Piper (search snippet)](https://leepip.artstation.com/projects/NxBrbq) [S, unverified]
- Tiny Glade auto-generates flower clusters, moss and ivy on structures; its plant rendering was reportedly "loosely inspired by Nanite Foliage" — [80.lv interview (search summary; the fetched text did not contain the plant discussion)](https://80.lv/articles/exclusive-tiny-glade-developers-discuss-bevy-proceduralism-publishers-cozy-games) [S, unverified]
- Godot MultiMesh / visibility ranges / instance uniforms constraints (visibility range is per node, not per instance; `instance uniform` not per MultiMesh instance) — see tree report sections citing [Godot docs, Visibility ranges](https://docs.godotengine.org/en/latest/tutorials/3d/visibility_ranges.html) and [Godot blog on uniforms](https://godotengine.org/article/godot-40-gets-global-and-instance-shader-uniforms/) [S]

### Inferences [EJ]
- **Construction in GN (per clump asset):**
  1. `Points` (3–9) in a disc r 0.05–0.2 m (Vogel with jitter, or Poisson).
  2. Per point, a `Curve Line` stem (height 0.15–0.6 m, random ±25%), `Resample` 3–4 points, bend by noise + outward lean 5–25°; `Curve to Mesh` with a 3-sided profile r 3–5 mm (or skip mesh stems entirely below LOD0 — they're sub-pixel at 20 m+).
  3. At `Endpoint Selection(end)` instance a head: petal ring = `Instance on Points` of a petal quad/low-poly petal on a `Mesh Circle`/`Points` ring of 5–8 with `Align Rotation to Vector` to the radial direction, pitch 20–60° up; centre disc = small UV sphere/cone or Vogel florets for daisy-types. **Tilt heads toward +Z** (60–90° from horizontal) so the 45° camera sees faces.
  4. 2–4 leaf cards/blades at the base (reuse the grass blade).
  5. Store `Col` = proxy normal (clump-sphere for leaves, **up-ish vector** for heads), `grad`, and a `part` id (stem / leaf / head) so one shader can treat heads differently. Store wind weights (see Q6).
  6. Realize, merge, export glTF. Budget: 60–300 tris per clump LOD0; heads as 5–8 petal quads = 10–16 tris each.
- **Single-flower meshes vs cards vs clumps:** single flowers only for hero planters / interactable crops. Everywhere else, **clumps** (fewer instances, built-in clustering). Flower *cards* (a crossed pair or an up-facing quad with 3–7 painted heads) are the LOD1/far tier and the right choice for large wildflower meadows on the dunes.
- **Instancing counts:** a wildflower patch of 10×10 m might hold 50–200 clumps; the whole village maybe 2–10k clumps — trivial for MultiMesh compared with EmacEArt's 524k grass clumps. Chunk per 16–32 m (flowers are smaller than trees) with visibility ranges: clump mesh 0–20 m, head cards 20–40 m, terrain colour splat beyond.
- **45° read from 5–40 m:** heads 4–10 cm across are 1–3 px at 40 m (1080p, ~60° FOV — rough estimate), so at distance colour must come from *clusters* (many heads together) or an oversized "painterly" head (stylized flowers can be 1.5–2× real size, a common stylization move). Put flowers in drifts of 5–30 clumps with a soft falloff, not a uniform scatter; place drifts along path edges, fences and house fronts (where a painter would).
- **Tint variation from a palette:** per species 2–4 swatches in a small palette texture (1×N or 4×4); MultiMesh `INSTANCE_CUSTOM.x` = palette index + jitter, or hash of instance world position → index. Keep jitter ±5% value, ±4° hue; never random RGB. Drift-level coherence: pick the swatch per *drift* (seeded by drift id) with 10–20% of clumps taking a neighbour swatch, which yields "patches of blue, a few white" like painted meadows.

### Gaps
- No verified source for how Ghibli films or AWL/SoS render wildflower fields technically.
- Exact Blender 5.2 GN node identifiers for curve/petal steps were not introspected in this session (Blender MCP not connected).

---

## Q5. Shading: proxy normals for bushes, flower heads (unlit vs toon), value separation from grass

### Takeaway
Reuse the tree shader for bushes unchanged (painterly `light()`, proxy normals in `COLOR.rgb`, `grad` in `COLOR.a`, seam darkening), with bush-specific colour/wind uniforms. Flower heads should be **lit but compressed** (high ambient floor, narrow shadow band, low shadow darkening, slight emission-like lift) so their hue survives shade without looking unlit/glowy. Separate bushes from grass by **value and temperature**, not only hue.

### Cited Findings
- Proxy normals are the most important foliage shading trick; without them card rotation makes "ugly, sudden changes in the shading" — [simonschreibt.de, Airborn trees](https://simonschreibt.de/gat/airborn-trees/) [S]
- Our pipeline carries proxy normals in `COLOR.rgb` (`*.5+.5`, model space) and a light gradient in `COLOR.a`, blends with `spherize`, and card normals get `card_normal_bend` — [CODE: `tree.gdshaderinc` lines 1–7, 108–123]; Addendum confirms this survives Blender 5.2 → glTF → Godot 4.7 within 0.013 — [tree report Addendum] [S-internal]
- Alpha scissor casts shadows and suits foliage; alpha blend cannot cast shadows — [Godot docs, StandardMaterial3D](https://docs.godotengine.org/en/latest/tutorials/3d/standard_material_3d.html) [S]
- Grass flicker at distance fixed by pointing normals more to the sky — [EmacEArt devlog](https://emaceart.itch.io/emaceart-stylized-grass-grass-shader-for-godot-4/devlog/1646260/emaceart-stylized-grass-120-godot-4-grass-shader-without-the-light-and-dark-flicker) [S]
- Ghibli painting separates a light family from a (saturated, cool) shadow family; darks recede — [GVAAT](https://gvaat.com/blog/how-to-paint-ghibli-backgrounds/) [S, via tree report]
- Kids With Sticks used a height mask to vary bush colour — [Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/) [S]
- Existing Godot shaders usable as reference: TheMIU "Stylized Fluffy Tree Shader" (billboard strength, wind, fresnel, normal flip; "best suited for stylized foliage like bushes, trees, and low-poly vegetation"; licence **not stated in the repo text I fetched**; the tree report records the godotshaders.com version as CC0 and J Hell's base as MIT — treat as unresolved until the repo LICENSE is checked) — [GitHub](https://github.com/TheMIU/Stylized-Fluffy-Tree-Shader); [godotshaders](https://godotshaders.com/shader/stylized-fluffy-tree-leaves/) [S]. FaRu85/Godot-Foliage: bush/tree foliage from quads with red-channel leaf alpha — [GitHub](https://github.com/FaRu85/Godot-Foliage) [S] (licence not checked). "Bush Shader Code" on godotshaders is a **2D sprite** shader, not relevant — [godotshaders](https://godotshaders.com/shader/bush-shader-code/) [S].

### Inferences [EJ]
- **Bush proxy blend:** start at 60% per-clump / 40% whole-bush (vs tree 80/20) — at 1 m, per-clump terminators become small and busy from 20 m+. Tune by eye.
- **Bush vs grass value separation:** grass is the ground plane; bushes must read as volumes on it. Rules: bush `col_top` ~10–15% darker *or* more yellow than grass tips; bush `col_under` notably cooler/darker than grass base (teal vs grass olive); the contact shadow/skirt provides the edge. Avoid identical hue families — push bush greens bluer (seaside, rugosa/hydrangea foliage is dark) and grass warmer/yellower. Check in greyscale: the bush silhouette must separate from grass at the game camera.
- **Flower heads:** use the same painterly `light()` with a `part == head` path: shadow darkening 30–50% of the leaf value (not full), shadow tint = flower hue shifted cool, a small constant lift (≈ +0.1 emission of albedo) so they read in bush shade; normals = up vector blended with head sphere (≈70/30) so every head has the same lit value across a field (the EmacEArt anti-flicker principle). Fully unlit/emissive heads look like stickers at dusk and break time-of-day; fully toon-lit tiny heads fizz. White flowers: cap albedo at ~0.85 so they don't clip under bloom/tonemap.
- **Ink:** no outline on bushes/flowers except interactable ones (e.g. a harvestable berry bush) — inverted hull on the SDF core only, per tree report.
- **Shadows:** bush cards `cast_shadow = off`; core casts. Flowers `cast_shadow = off` entirely (tiny shadow maps just add noise) — use AO in vertex colour at clump base instead.

### Gaps
- No source measured value contrast targets for stylized grass vs bushes; numbers above are judgement.

---

## Q6. Wind: bushes (minimal clump sway) and flowers (stem bob)

### Takeaway
Bushes: reuse the tree's three-layer wind with tiny amplitudes (sway 1–3 cm at top, card flutter only). Flowers: height-weighted bend per clump + per-stem phase bob of the head; drive both from the same `global uniform` wind vector as trees. All data baked into vertex colour / UV at generation time.

### Cited Findings
- Crysis scheme: main bending scaled by normalized height + detail bending with per-leaf phase/stiffness stored in vertex colour — [NVIDIA GPU Gems 3, ch.16](https://developer.nvidia.com/gpugems/gpugems3/part-iii-rendering/chapter-16-vegetation-procedural-animation-and-shading-crysis) [S]
- Wind via vertex paint on the roots for bushes/flowers in a stylized UE scene — [80.lv](https://80.lv/articles/setting-up-trees-bushes-and-flowers-for-a-stylized-3d-environment) [S]
- EmacEArt: three wind layers — clump sway, blade tremble, gust wave across the field — [EmacEArt itch](https://emaceart.itch.io/emaceart-stylized-grass-grass-shader-for-godot-4) [S]
- Our tree shader: `wind_dir`, `wind_strength` (m of sway at crown), `wind_speed`, `tree_height`, `flutter` (card roll) uniforms; displacement `wdir * wind_strength * pow(h, 1.8) * sway` — [CODE `tree.gdshaderinc` 53–116]
- Victor Karp's Godot 4 foliage wind uses panning noise textures and world-position desync — [victorkarp.com](https://victorkarp.com/godot-foliage-wind/) [S]

### Inferences [EJ]
- **Bush:** `tree_height` = bush height, `wind_strength` 0.01–0.03, `flutter` 0.05–0.1 rad on cards only; core moves only via the height term (base pinned at z≤0). Per-bush phase from `NODE_POSITION_WORLD` or MultiMesh `INSTANCE_CUSTOM`.
- **Flowers (per vertex):** bake `R = (z / clump_height)^2` (stem bend), `G` = per-stem random phase, `B` = 1 on head vertices / 0 elsewhere, `A` = AO. Vertex shader: `offset = wind_dir * (gust(world_xz) * strength * R) + perp(wind_dir) * sin(TIME*f + G*6.28) * bob * R`, plus head nod: rotate head vertices about the stem tip by `B * small angle`. Amplitude 2–6 cm at the head, frequency 1.5–3 Hz for bob. Since the proxy/normal channel already lives in `COLOR.rgb` in our pipeline, flower wind data needs **UV2 or CUSTOM0** instead (UV2 is proven to survive import [Addendum]; CUSTOM mapping unverified per tree report).
- **Gust wave:** sample the same world-space noise texture as grass so a gust visibly crosses grass → flowers → bushes in sequence — the single cheapest "alive" cue.
- Player interaction (brush-through bend) for flowers: a global uniform array of 4–8 actor positions, push away within 0.5 m — optional, cheap.

### Gaps
- No cozy-game-specific wind breakdown for bushes/flowers found.

---

## Q7. Procedural generation in Blender 5.2 GN vs GDScript; what to bake

### Takeaway
Keep everything in the existing headless Blender 5.2 GN pipeline (it already runs a tree in ~0.7 s): bushes and hedges are **presets/variants of `FarmTree`** (with 2–3 new inputs), flowering bushes add a "flower layer" stage, flowers get a new small `FarmFlowerClump` group. Bake meshes + vertex attributes to glTF; do placement (drifts, hedge paths, per-instance tint/phase) in Godot at edit time with `@tool` GDScript into chunked MultiMeshes.

### Cited Findings
- GN groups can be built from Python and evaluated headless; our generator does exactly this (`build_group()`, `interface.new_socket`, Repeat Zone space colonization, SDF core, card realize, glTF export with `export_vertex_color_name="Col"`) — [CODE `gn_tree.py`]; general pattern — [CGWire](https://blog.cg-wire.com/blender-scripting-geometry-nodes-2/) [S]
- Blender 5.x ships volume-grid/SDF nodes (SDF boolean, mean, offset, grid to mesh) — [Blender Developers Blog](https://code.blender.org/2025/10/volume-grids-in-geometry-nodes/); [Blender 5.2 manual](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/volume/index.html) [S]
- NodeToPython (GPL-3.0) converts node graphs to Python for 4.2–5.2 — [GitHub](https://github.com/BrendanParmer/NodeToPython) [S]
- Addendum traps that apply unchanged: `Merge by Distance` "All" is transitive; `Grid to Mesh` is flat-shaded (add `Set Shade Smooth`); float2 corner attributes export as UV maps; modifier inputs via `md.properties.inputs.<Socket_N>.value`; no `.blend` inside the Godot project — [tree report Addendum] [S-internal]

### Inferences [EJ] — concrete generator changes
- **`FarmTree` additions:** `Scale` (float, multiplies every metre constant incl. `MeshToVolume` voxel, card offset, UV2 decode), `Wood` (bool; Switch stage 3–4 off), `Skirt Cards` (int; ring of base cards), `Sink` (float; translate core down + delete below z=0), `Proxy Mode` (per-clump / spheroid / swept-extrusion for hedges), `Flower Layer` sub-inputs (`Flower Object`, `Cluster Count`, `Cluster Radius`, `Per Cluster`, `Sun Min`), and a `Envelope Type` for hedges (object vs curve → `Curve to Mesh`).
- **Flower layer stage (after stage 5):** `Distribute Points on Faces` on the core with selection `proxy·Sun > t AND proxy.z > 0.2` → pick N cluster centres (Poisson, large spacing) → per centre, `Points` in disc → `Geometry Proximity` project onto core → offset along proxy 1–3 cm → `Instance on Points` of flower card/mesh collection (`Pick Instance` random) → store `part=3`, own material slot. Store buds as the same instances at 0.3 scale in a separate attribute so Godot can toggle.
- **Flower clump group:** see Q4 steps; outputs clump LOD0 mesh + a head-card LOD1 (render heads to an atlas once, or paint).
- **What to bake:** meshes (LOD0/1), `Col` (proxy normal + grad), UV (atlas cell), UV2 (card corner offset / wind data), material slots per part (core, cards, flowers, stems). **Do not bake:** placement, tint, per-instance phase, season state — those live in Godot (MultiMesh colour/custom data) so they are free to change.
- **GDScript-only alternative:** feasible for flower clumps (ArrayMesh from curves is ~200 lines) but would duplicate the Blender pipeline's normal/attribute conventions; not recommended except for runtime-grown crops.

### Gaps
- Blender MCP was not connected (ENOENT on `cmd`), so none of the new node steps were executed against 5.2 in this research.

---

## Q8. References, projects and licences

### Takeaway
There is no drop-in open-source stylized bush/flower generator that beats extending our own; useful references are shaders (for ideas) and breakdowns (for rules).

### Cited Findings
| Resource | What | Licence |
|---|---|---|
| [TheMIU/Stylized-Fluffy-Tree-Shader](https://github.com/TheMIU/Stylized-Fluffy-Tree-Shader) | Godot 4 billboard foliage shader, bushes/trees, wind, fresnel | Not stated in fetched repo text; tree report lists godotshaders version as CC0 (base by J Hell, MIT) — **verify** |
| [godotshaders: Simple, cheap stylized tree shader](https://godotshaders.com/shader/simple-cheap-stylized-tree-shader/) | billboard + green-channel alpha | MIT (per tree report) |
| [FaRu85/Godot-Foliage](https://github.com/FaRu85/Godot-Foliage) | bush/tree quad foliage shader | not checked |
| [EmacEArt Stylized Grass](https://emaceart.itch.io/emaceart-stylized-grass-grass-shader-for-godot-4) | Godot 4 MultiMesh grass, 3-layer wind, anti-flicker normals | not stated in fetched text |
| [IcterusGames/SimpleGrassTextured](https://github.com/IcterusGames/SimpleGrassTextured) | MultiMesh ground cover (could host flower clumps) | MIT (per tree report) |
| [Blender Everything Hedge Generator](https://www.blendereverything.com/download.php?id=43) | curve → hedge | not stated |
| [UE Modular Hedges and Bushes](https://www.unrealengine.com/marketplace/en-US/product/modular-hedges-and-brushes) | modular hedges + SplinePlacer (reference only) | commercial, UE |
| [NodeToPython](https://github.com/BrendanParmer/NodeToPython) | GN graph → Python | GPL-3.0 (tooling only; doesn't encumber exported meshes, per tree report) |
| YouTube: [Ghibli Style Anime Trees and Bushes in Blender (GN)](https://www.youtube.com/watch?v=4PUvnIEUHSM), [LIVENODING Ghibli bushes](https://www.youtube.com/watch?v=ACExE9Dy2Io), [Ghibli bushes UE5+Blender](https://www.youtube.com/watch?v=YfCWSSqaZSs) | GN bush tutorials | video (not watched; titles from search) |

### Gaps
- Licences for TheMIU repo, FaRu85, EmacEArt and the Hedge Generator must be checked before copying code.

---

## Q9. Recommended recipe per type, and how much of the tree pipeline transfers

### Takeaway
Round shrubs and wild hedges are ~85–90% the tree pipeline (same stages 1, 2, 5, same shader and export; new: scale-parametrised constants, wood off, skirt/sink). Topiary ~70% (swap Points→SDF for Mesh→SDF, different proxy mode). Flowering bushes ~80% + a new flower stage. Small flowers ~30–40% (shares export conventions, `Col` encoding, shader `light()` and wind uniforms; construction is a new, small curve-based GN group). All [EJ].

### Recipes [EJ unless marked]

**A. Round shrub (0.5–1.5 m)** — *tree pipeline, ~90% transfer*
- Envelope: library of 4–6 low-poly mounds, h ≈ 0.6–0.8 × width, lopsided toward the sun.
- Clumps: spacing 0.25–0.45, radius 0.15–0.4 (rand^1.8 skew kept), dominant 0.35–0.45.
- Core: voxel 0.04–0.06, smooth 1–2, Shelf Flatten 0.8, sink 10–20%, delete below ground, island cull kept. 300–900 tris.
- Cards: 60–150, size 0.25–0.45 m, shell 0.06–0.1, offset 0.01–0.05; underside rule kept + **8–20 skirt cards** at the base ring; core dab projection ~35% (Addendum).
- Wood: off (or 2–5 stems for sparse variants).
- Shader: `tree_core`/`tree_card` as-is; `tree_height` = bush height, `wind_strength` 0.02, proxy blend ~60/40; greens cooler/darker than grass; optional contact-shadow decal.
- LOD: LOD0 ≤ 20 m, LOD1 core + 30% cards 20–40 m; no impostor needed. MultiMesh chunks 32 m.
- Tris: ~500–1,300.

**B. Hedge** — *wild ~85%, topiary ~70% transfer*
- Wild: envelope = `Curve to Mesh` of a polyline with lumpy profile + height noise → Poisson clumps → SDF → cards → skirt both sides. Proxy = 50% per-clump / 50% swept-cylinder normal. ~600–1,200 tris/m.
- Topiary: rounded-box → Mesh to SDF + faint inset bumps → SDF Mean; proxy = blurred box normals; short dense cards (0.15–0.25 m, ≤3 cm overhang); no holes. ~300–600 tris/m.
- Delivery: straight 2 m and 3 m modules + end cap + 90° corner (+ gate gap), generated 10–15% overlong and clipped; world-space dab/brush projection hides seams; Godot `@tool` Path3D placer with random flip. Hero hedges: one GN sweep per path, chunked at 8–16 m.

**C. Flowering bush (hydrangea / hibiscus / beach rose)** — *shrub recipe + flower stage, ~80% transfer*
- Base: recipe A (hydrangea: rounder, denser, darker leaves; hibiscus: taller/open 1.2–2 m with 2–4 visible stems; rugosa: low sprawling 0.5–1 m, wrinkled dark leaves).
- Flowers: separate material slot / MultiMesh, 2–5 clusters on sun/top side, ≥40% of surface left green.
  - Hydrangea: 5–15 mophead geometry blobs (40–120 tris), petal-dab texture, palette blue/violet/pink per bush.
  - Hibiscus: 6–15 large (0.12–0.18 m) single-flower cards or 5-petal meshes facing out/up; red/pink/yellow.
  - Beach rose: 8–20 small (0.07–0.1 m) magenta/white 5-petal cards + optional red hip spheres (season).
- Heads shaded with compressed toon (partial shadow, slight lift); buds = same instances at 0.3 scale, green-tinted; far LOD bakes flower colour into core vertex colour.

**D. Small flowers / wildflower clumps** — *new `FarmFlowerClump` GN group, ~30–40% transfer (conventions, shader, wind, export)*
- Clump: 3–9 curve stems (0.15–0.6 m), Vogel/Poisson disc base, heads = ring of 5–8 petal quads via Instance on Points + Align Rotation, tilted up 60–90°, Vogel florets for daisy centres; 2–4 base leaf blades. 60–300 tris.
- Species set for a seaside village: daisy/chamomile (white), sea thrift (pink pom), lavender/lupin (pointy spikes — the "pointy" partner to round bushes), buttercup (yellow), cornflower (blue). 5–6 species × 3 variants.
- Placement: drifts of 5–30 clumps along paths/fences/house fronts and on dunes; palette swatch per drift with 10–20% neighbour swatch; tint via `INSTANCE_CUSTOM`.
- Rendering: MultiMesh chunks 16–32 m; clump mesh 0–20 m, up-facing head cards 20–40 m, terrain colour splat beyond. Head normals up-biased; no shadows cast; wind = height bend + per-stem bob via UV2 data; shared world gust noise with grass.

### Cited Findings (supporting the transfer claim)
- The bush/tree construction in breakdowns is identical in kind (cards + sphere/blob normals + optional inside mesh) — [Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/); [Blender Artists](https://blenderartists.org/t/transferring-normals-for-a-stylized-bush/1517824); [80.lv](https://80.lv/articles/setting-up-trees-bushes-and-flowers-for-a-stylized-3d-environment) [S]
- The generator's stages and hard-coded constants listed in Q1 — [CODE `gn_tree.py` lines 156–427, `tree.gdshaderinc`]

### Inferences
- Biggest risk in "bush = tree with no trunk": bald/floating base (tree deletes underside cards) and scale constants (voxel/offset/density). Both are fixed by the 4–5 new inputs above; everything else (SDF core, island cull, seam darkening, proxy encoding, UV2 billboard, export flags, gates) is reused verbatim.
- Gates to add (mirroring the Addendum's cheap geometric gates): bush — ≥60 cards, ≤1.5k tris, base contact (min z of core ≤ 0), width/height 1.2–2.0; hedge module — end-face profile matches neighbour within tolerance; flower clump — ≤300 tris, head normal·up ≥ 0.5.

### Gaps
- None of the recipes has been built or rendered; all numbers need a lookdev pass at the fixed game camera (front-lit must-pass shot per Addendum).

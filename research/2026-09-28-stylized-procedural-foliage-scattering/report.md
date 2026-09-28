# Grow the understory from one painterly pipeline

Almost everything below the tree canopy can come from the pipeline you already have. That pipeline is a headless Blender 5.2 Geometry Nodes generator writing glTF with proxy normals and wind data in vertex attributes, drawn in Godot 4.7 by the painterly `light()` through chunked MultiMeshes. What changes by category is the **construction primitive**, not the pipeline:

| Category | Primitive | Tree-pipeline reuse |
|---|---|---|
| Grass and ground cover | Opaque blade tufts on up-facing normals, fading into painted terrain | Shared shader, wind and export only |
| Bushes, hedges, flowering shrubs | The tree generator with the wood off, run at roughly 0.2× scale | About 80–90% |
| Ferns and large-leaf plants | Rachis curves with instanced pinnae, or a parametric (u,v) blade grid | Shared contract only |
| Small flowers | Small curve-stem clumps whose up-facing heads carry the read | Shared contract only |

At a fixed 45° camera the ground plane dominates the frame. So the lawn is read mostly from **terrain colour**, not from grass geometry. That is how A Short Hike ships its grass, and Ghost of Tsushima hands its far grass off to terrain texture the same way. Grass geometry is for silhouettes, motion and edges.

Scattering should follow the one pattern every production system uses (Horizon Zero Dawn, Far Cry 5, Ghost Recon Wildlands, Unreal PCG):

- Place layers **big to small**, with each placed object claiming a footprint that suppresses or attracts later layers.
- Build each layer's density from masks.
- Seed candidates by **world-cell hash**, so a small edit changes results only locally.
- Generate everything except grass as **clusters**, never sprinkles.

No existing Godot addon handles your three-mesh tree scenes with per-mesh shadow settings, so the recommendation is to **build a small in-editor `BiomeArea` tool**, which the tooling research estimated at 600–1,000 lines of GDScript. Borrow algorithms from the MIT-licensed ProtonScatter, and bake compact placement lists rather than scene data.

One correction to the earlier tree report matters for all of this. A source read of Godot 4.7 shows that `NODE_POSITION_WORLD` read in `vertex()` already returns the **per-instance** position for MultiMesh instances. The existing wind phase code should therefore work under instancing without `INSTANCE_CUSTOM`, but test it once. The public record on how Ghibli, Harvest Moon: A Wonderful Life, Ni no Kuni or Animal Crossing build foliage is essentially empty. Nearly every density, tri count, colour and blend value below is engineering judgement, meant to be calibrated at the front-lit game-camera shot that the tree build established as the must-pass view.

## The 45° camera makes ground colour, not blades, carry the lawn

### What shipped stylized games actually do

Stylized games that need a strong blade read, such as Breath of the Wild and Ghost of Tsushima, use real instanced blade geometry near the camera. Billboard clusters are "the oldest and still a very common way" to do it, and shells suit only very short fur-like cover ([hexaquo Pt 1](https://hexaquo.at/pages/grass-rendering-series-part-1-theory/); [80.lv](https://80.lv/articles/classic-video-games-trick-for-rendering-grass-fur)).

Ghost of Tsushima generates each blade on the GPU as a cubic Bézier and drives height and facing from Voronoi clumps ([GDC Vault](https://gdcvault.com/play/1027033/Advanced-Graphics-Summit-Procedural-Grass)). Secondary summaries of that talk add three details: distant tiles drop 3 of 4 blades, LOD transitions blend vertex positions to avoid pops, and "very far" grass becomes an artist-authored texture on the terrain ([tigerabrodi.blog](https://tigerabrodi.blog/grass-in-ghost-of-tsushima)). A Short Hike, the closest cozy precedent, largely skips grass geometry. It picks the terrain texture by normal, maps it in world space, and reduces the splatmap to its highest channel for sharp painted edges ([Robinson-Yu thread](https://threadreaderapp.com/thread/1113100182655262721.html)). The Ghibli-inspired UE4 "Meadows" project used painted grass cards plus a **large-scale blurry colour-variation texture** that made the grass "look fluffier and painterly" ([80.lv](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)).

### What that means at a 45° camera (judgement)

From a 45° down-looking camera, thin blades show little area and alias into hair-like noise beyond about 15 m. Most of the top-down coverage comes from the ground. That argues for **fewer, chunkier, opaque tufts** near the camera over a terrain shader that already reads as painted lawn at 25–40 m. It does not argue for Tsushima-density blades. Opaque tufts also avoid the alpha-scissor shimmer the tree card fringe needed antialiasing to fix.

### Normals and colour: grass as part of the ground

On shading, the sources converge on one trick: **force grass normals to the ground or up normal**.

- Jess Hider's stylized grass, used by Meadows, uses world Z-up normals ([Jess's UE4 Tutorials](https://jesshiderue4.wordpress.com/materials/stylized-wind-blown-grass/)). Ghibli-inspired UE breakdowns call forcing card normals up the "biggest breakthrough" ([80.lv](https://80.lv/articles/creating-a-ghibli-inspired-island-in-blender-ue5-substance-3d)).
- A CC0 Godot MultiMesh grass shader does the same ([godotshaders](https://godotshaders.com/shader/stylized-multimesh-grass-shader/)).
- A commercial Godot 4 grass fixed its "light and dark flicker" at distance by making blades "face the sky more" so "the field keeps one steady tone at any distance" ([EmacEArt devlog](https://emaceart.itch.io/emaceart-stylized-grass-grass-shader-for-godot-4/devlog/1646260/emaceart-stylized-grass-120-godot-4-grass-shader-without-the-light-and-dark-flicker)).
- Ghost of Tsushima-style rounding tilts normals outward across the blade width, then **blends them back toward the terrain normal with distance** to reduce aliasing ([Unity-Grass README](https://github.com/cainrademan/Unity-Grass); [GodotGrass, MIT](https://github.com/2Retr0/GodotGrass)).

For your `light()`, the judgement call follows directly from this. With up or terrain normals, the brush-noise terminator and the posterized cast-shadow bands fall on grass exactly as they fall on the ground. Evaluate the brush noise at the **tuft root's world position**, with the same `brush_scale` and `shadow_threshold` as the terrain material, and grass and ground read as one painted stroke. Keep the rim term off. Weight the back-light translucency by tip height squared.

Colour comes from a root-to-tip gradient, as in the Breath of the Wild-style vertex gradient ([Daniel Ilett](https://danielilett.com/2021-08-24-tut5-17-stylised-grass/)), whose root blends into the terrain colour underneath ([godotshaders, MIT](https://godotshaders.com/shader/stylized-cartoon-grass/)). On top of that goes a low-frequency world-space colour noise of 2–8 m. The tree build's lesson applies twice over here: the beach sun sits behind the camera, so separation must come from albedo, not lighting.

### Terrain colour at the root clashes with the wind data

The grass notes propose writing the terrain colour into MultiMesh per-instance `COLOR`. The tooling notes flag, as judgement, that `COLOR` in the vertex stage is vertex colour × instance colour when `use_colors` is on. Grass wind and height data live in vertex `COLOR_0`, so the two would collide. The cleaner path is to **bake a top-down terrain colour texture once** and sample it in the grass shader at the instance root. That frees `COLOR` and `INSTANCE_CUSTOM` for other data and gives the far terrain paint a matching source. The terrain is currently vertex-coloured in raw sRGB with no colour texture, so this is new work, but it is small.

### Wind and interaction

For wind, a scrolling world-space noise, specifically **Simplex Smooth with Ridged fractal**, produces the Ghibli "waves as gusts move through" behaviour. Stalks bend hard where the gust is strongest and settle to light shaking behind it ([hexaquo Pt 3, CC-BY-SA](https://hexaquo.at/pages/grass-rendering-series-part-3-animating-and-interacting-with-grass-in-godot/)). Promote `wind_dir`, `wind_speed` and `wind_strength` to `global uniform`s so grass, flowers, bushes and trees gust together. Keep the noise texture as a per-material uniform, because global `sampler2D` uniforms have a history of blank or non-updating bugs ([#107470](https://github.com/godotengine/godot/issues/107470); [#93164](https://github.com/godotengine/godot/issues/93164)).

At a 45° camera, the **colour** wave is more visible than the bend. Brightening grass by about 8% where gust noise is high gives the "rippling glow" of the Ghibli-island breakdown (judgement; [80.lv](https://80.lv/articles/creating-a-ghibli-inspired-island-in-blender-ue5-substance-3d)).

For player interaction, a `uniform vec4 pushers[8]` array is enough. It must be set from a `PackedVector3Array` or `PackedVector4Array` or it silently fails ([Bugnet](https://bugnet.io/blog/fix-godot-shader-uniform-array-not-updating)), and global uniforms cannot be arrays ([proposal #9553](https://github.com/godotengine/godot-proposals/discussions/9553)). Reserve a top-down trail render texture for persistent meadow footpaths, if you ever want them.

### Performance

Performance headroom is large. hexaquo rendered **400 blades/m² at 9 tris each, about 10M tris over 50×50 m, in just under 2 ms** in Godot 4, noting that sub-pixel geometry causes flicker ([hexaquo Pt 4](https://hexaquo.at/pages/grass-rendering-series-part-4-level-of-detail-tricks-for-infinite-plains-of-grass-in-godot/)). Shadows are the real lever. GodotGrass warns that grass shadow mapping comes "at a *very* high performance cost" ([GitHub](https://github.com/2Retr0/GodotGrass)), and SimpleGrassTextured recommends shadows off for short grass ([README](https://github.com/IcterusGames/SimpleGrassTextured/blob/main/README.md)).

### Beach grass and reeds are silhouette plants, not carpets

Beach and dune grass and reeds are a different kind of plant from lawn. Marram grows in clumps on active sand on the windward foredune ([Wikipedia](https://en.wikipedia.org/wiki/Ammophila_arenaria)), and its clump-versus-patch organization shapes the dune itself ([Frontiers 2021](https://www.frontiersin.org/journals/ecology-and-evolution/articles/10.3389/fevo.2021.761336/full)). No game-art source covers stylized dune grass or reeds, so these recipes are judgement:

- **Dune grass** is a fountain of 12–30 arching blades with a baked leeward lean, set in drifts with bare sand between.
- **Reeds** are rigid vertical clumps that pivot at the root, arranged in shoreline bands with the wind phase travelling along the bank.
- Both are tall enough that pure up-normals flatten them at 5–15 m. Give each a **per-clump spherical proxy normal** blended 50–70% with up, the canopy trick at small scale, so each clump has a lit side and a teal shadow side.

## Bushes and hedges are the tree generator at one-fifth scale

### Why a bush is a small tree canopy

Every published stylized-bush breakdown uses the same construction as your trees: a sphere or blob "inside mesh", leaf cards on its surface, and the blob's normals copied onto the cards ([Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/); [Blender Artists](https://blenderartists.org/t/transferring-normals-for-a-stylized-bush/1517824); [80.lv](https://80.lv/articles/setting-up-trees-bushes-and-flowers-for-a-stylized-3d-environment)). One caveat is that sphere-normal projection can look "too uniform for large canopy meshes" ([80.lv Meadows](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)), which matters less at bush size. Sea of Thieves adds two placement-level rules: solid sphere cores "to combat overdraw", and testing assets in collections, where mid-height plants pair with low flowers through "pointy vs rounded" silhouettes ([Lee Piper, search snippet; page returned 403](https://leepip.artstation.com/projects/NxBrbq); [Habrador](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html)).

### What has to change in `gn_tree.py` before it can make bushes

Reading `gn_tree.py` shows the port is not free. The graph hard-codes metre-scale constants:

| Constant | Tree value | Bush value (judgement) |
|---|---|---|
| Card-volume voxel | 0.15 m | 0.04–0.06 |
| Card offset | 0.03–0.22 m | Scaled down; 22 cm of float-off is a floating card on a 1 m bush |
| Card density (per m³ of shell) | 160 | 2,000–6,000, to land 60–200 cards |
| Shell | 0.3 m | 0.06–0.1 |
| Clump spacing | 1.6 m | 0.25–0.45 |
| Clump radius | 0.55–1.35 m | 0.15–0.4 |
| Card size | 1.0 m | 0.25–0.45 |

The graph also **always builds a trunk**, even at zero iterations, so a bush would get a stick. The fix is a handful of new inputs: `Scale`, `Wood` on/off, `Sink`, `Skirt Cards`, `Proxy Mode` and a flower-layer group. The quicker alternative is to feed the unchanged graph a 5 m bush-shaped envelope with the wood off and scale it by 0.2 at the end. If you do that, rescale the UV2 billboard offsets too, and set the shader's `tree_height` to the bush height.

### Ground contact is the number-one bush tell

The tree graph deletes underside cards, which is correct for trees but leaves a bald, floating bush base. Fix it in four steps (judgement):

1. Sink the core 10–20% of its height below ground.
2. Flatten the bottom 30%.
3. Add a separate **skirt of 8–20 cards** around the base ring, tilted outward and tinted with the underside colour.
4. Optionally add a soft painted contact-shadow quad.

The tree Addendum's "blob with stickers" finding also overrides the original report's bush fringe density of 0–0.3. Sparse cards on a smooth core look plastic, so bushes need the world-space dab projection on the core at about 35%, plus 60–150 cards. As in the Addendum, the **envelope is the main art-direction lever**, so keep a small library of 4–6 sculpted mounds (dome, loaf, kidney, lopsided) rather than random ellipsoids. Start the proxy blend at 60% per-clump / 40% whole-bush instead of the tree's 80/20, because per-clump terminators on a 1 m bush get busy from 20 m.

### Separating bushes from grass

Separate bushes from grass by value and temperature, not hue alone. Make the bush's lit greens bluer and slightly darker than grass tips, and its undersides a cooler teal against the grass's olive base. Check the result in greyscale at the game camera. All of this is judgement.

### Hedges

For hedges, commercial tooling places modular pieces along splines. The UE Modular Hedges pack ships a SplinePlacer ([Fab](https://www.unrealengine.com/marketplace/en-US/product/modular-hedges-and-brushes)), and Forest Pack offers either jigsaw modules or spline-surface scattering ([CG Record](https://tutorials.cgrecord.net/2017/04/creating-trimmed-hedges-and-topiary.html)). Three judgement calls follow:

- A **wild hedgerow** is the tree pipeline on a swept, lumpy tube envelope, with its height broken every 1.5–3 m.
- A **topiary hedge** needs a rounded box fed through *Mesh*-to-SDF rather than Points-to-SDF. It gets proxy normals from the blurred box rather than per clump, short dense cards and no sky holes. This is the read closest to Harvest Moon: A Wonderful Life and Animal Crossing.
- **Delivery**: build 2 m and 3 m modules, end caps and corners, each generated 10–15% overlong and then clipped. Hide the joins with the shader's world-space dab and brush projection, and place the modules with a Godot `Path3D` placer. Use one-off GN sweeps for two or three hero hedges.

### Flowering bushes

Flowering bushes add a flower layer that should stay **separate and switchable**. In Animal Crossing: New Horizons, bushes show buds before their season, flowers during it, and plain green afterward, so bloom is a state layered on a shared green base ([Nookipedia](https://nookipedia.com/wiki/Bush); [Animal Crossing World](https://animalcrossingworld.com/guides/new-horizons/all-bush-types-colors-list-seasons-how-to-plant/)). Put flowers on their own material slot or MultiMesh so seasons can hide them, or scale them to buds, without regenerating the bush.

Placement is the key judgement: **cluster, don't sprinkle**. Pick 2–5 cluster centres on the sun-facing top of the core, fill each within a 0.1–0.25 m radius with falloff, and leave at least 40% of the surface green. Oga's "few detailed leaves" logic from the tree report applies ([Gurney Journey](http://gurneyjourney.blogspot.com/2011/02/kazuo-oga.html)): flowers are the details laid over the mass.

Build hydrangea heads as small geometric mophead blobs, which read as coloured masses at 40 m, and hibiscus and beach rose as flower cards or 5-petal meshes. Shade the heads with a **compressed** toon response: partial shadow darkening, a cool-shifted but saturated shadow, and about +0.1 lift. Unlit heads look like stickers at dusk, and fully toon-lit tiny heads fizz.

## Ferns and big leaves are curves and blades, not blobs

### Ferns

Ferns are not canopies. SpeedTree's frond generator "creates a mesh that lies along a branch spine" with Fold, Curl, Gravity and Roll controls. Its **Spread** lighting option points normals away from the spine, which is SpeedTree's built-in version of your proxy-normal trick ([SpeedTree docs](https://docs.unity3d.com/speedtree-modeler/manual/frond-generator-properties.html)). Electric Square's Green Chapel ferns layered large dead fronds at the bottom and young curling ones at the top, and added anchored individual leaves "to break up the flat look of the frond cards" ([80.lv](https://80.lv/articles/creating-a-mysterious-scene-with-lifelike-foliage-using-speedtree)).

The arrangement maths is classical. The golden-angle rosette is φ = n·137.5°, and deviations of just 0.2° give visibly wrong patterns ([ABOP ch.4](http://algorithmicbotany.org/papers/abop/abop-ch4.pdf)). ABOP's compound-leaf models give older basal pinnae greater length, and warn that a blade built from triangles along the midrib beats a traced polygon "if the blade bends" ([ABOP ch.5](http://algorithmicbotany.org/papers/abop/abop-ch5.pdf)). Fiddleheads are close to equiangular spirals ([Geometry Code](https://geometrycode.com/spirals-ferns-universal-unfolding-motifs/)).

For this style, the recommended fern LOD0 (judgement) is **opaque geometric pinnae** of 2–4 tris each, instanced along a resampled GN rachis curve:

- **Crown**: 10–18 fronds at the golden angle, with 10–20% of slots skipped so the crown doesn't read as a perfect star.
- **Shape**: pitch 75° inner to 20° outer, quadratic droop, and a pinna length profile such as `sin(π·t^0.8)^0.6`.
- **Fiddleheads**: 1–3 central ones built with Vector Rotate + Accumulate Field.
- **Budget**: about 1–1.5k tris.

An alpha frond ribbon is the 15–30 m LOD. Polycount consensus that "a fern might be mostly geo" while dense trees need alpha supports this split ([Polycount](https://polycount.com/discussion/144594/foliage-alphas-vs-geometry)). The free Martinelli fern GN asset is worth mining for graph ideas, but its licence text was not seen ([Gumroad](https://sagado.gumroad.com/l/ozxrlu)).

### Large leaves and rosettes

Large leaves (hosta, taro and elephant ear, monstera, banana) and agave-type rosettes should share one **parametric blade function**:

- **Blade**: a midrib curve plus a half-width profile w(u), meshed as a small (u,v) grid of about 8–14 rows × 2–3 columns per half.
- **Deformations**: V-fold, arch or droop, camber, ruffle and twist. These are the same set the leaf-modelling literature uses ([NUS leaf-space paper](https://www.comp.nus.edu.sg/~leowwk/papers/gmp2012.pdf); [NeuraLeaf](https://arxiv.org/html/2507.12714v1)).
- **Store u and v before deforming.** They drive procedural veins, colour and wind for free.
- **Holes and tears as geometry**: monstera fenestrations and banana tears become deleted faces and split vein strips, not alpha. At 5–15 m, a 1 m leaf's holes are several pixels wide, and staying opaque keeps early-Z and shadows clean.

Commercial stylized packs put monstera at **284–1,409 tris** and banana at **691–1,433** ([ArtStation Tropical Plants Pack](https://www.artstation.com/marketplace/p/jjv9r/tropical-plants-pack)), consistent with the 1–2.5k LOD0 budgets proposed here. Agave needs real thickness: a lens cross-section, `cull_back`, and no wind.

### Back faces flip your proxy normals

One engine fact changes the shading. Godot's Forward+ shader **negates the normal on back faces whenever culling is disabled**. The code is `if (!gl_FrontFacing) normal = -normal;` under `DO_SIDE_CHECK`, which both `cull_disabled` and `cull_front` define ([scene_forward_clustered.glsl](https://github.com/godotengine/godot/blob/master/servers/rendering/renderer_rd/shaders/forward_clustered/scene_forward_clustered.glsl); [scene_shader_forward_clustered.cpp](https://github.com/godotengine/godot/blob/master/servers/rendering/renderer_rd/forward_clustered/scene_shader_forward_clustered.cpp)). With proxy normals, every frond underside would light as if facing into the plant. Re-flip the normal in `fragment()` so both sides light alike, as Meadows did. For taro and hosta, keep the flip and tint back faces a paler underside colour, so a wind-flipped leaf shows its light side.

Use **low spherize** on big leaves (0.2–0.4 toward a per-leaf proxy) so the midrib fold still splits each blade into a lit half and a shadow half. Matkovski smooths normals "per group of leaves" rather than per plant precisely because "the light and shadow parts add more definition" ([80.lv](https://80.lv/articles/stylized-nature-vegetation-animation-shaders)). Ferns and hostas can go higher, 0.5–0.8 toward a crown ellipsoid, so they read as soft mounds.

Paint veins as faint stripes computed from (u,v) and skip normal maps. Alisavakis found leaf normal maps "were just adding noise" ([halisavakis.com](https://halisavakis.com/my-take-on-shaders-stylized-tree-leaves/)). Wind should *rotate* each frond or leaf about its baked base pivot by `strength·t²` rather than shear it, following the Crysis main-plus-detail scheme ([GPU Gems 3](https://developer.nvidia.com/gpugems/gpugems3/part-iii-rendering/chapter-16-vegetation-procedural-animation-and-shading-crysis)).

### Keep proxy normals out of a second colour attribute (judgement)

The fern notes propose moving the proxy normal into a second FLOAT_COLOR attribute. Godot's mapping of extra glTF attributes to CUSTOM0–3 is still unverified, so avoid that where possible. A fern or hosta proxy is one ellipsoid per plant, so compute it analytically in `vertex()` from a crown-centre uniform, the way grass computes up-normals. Keep COLOR_0 for wind, UV for (u,v), and UV2 for the pivot. Only per-leaf proxies on taro, monstera and banana need a second channel, and they can derive from the UV2 pivot plus an offset.

## Small flowers read as coloured drifts, not stems

At 5–40 m and 45°, stems barely register. A 4–10 cm head is roughly 1–3 px at 40 m (judgement estimate), so **clusters of heads** carry the read. Build flowers as small clump assets, not singles, in a new `FarmFlowerClump` GN group:

- **Stems**: 3–9 per clump on a Vogel/Poisson disc, as curve stems 0.15–0.6 m tall with an outward lean.
- **Heads**: rings of 5–8 petal quads, tilted **60–90° toward +Z** so the camera sees faces. Daisy-type centres use Vogel florets ([MathWorld](https://mathworld.wolfram.com/VogelSpiral.html)).
- **Leaves**: 2–4 base leaf blades reused from the grass generator.
- **Budget**: 60–300 tris per clump.

Martinelli's GN blossoming flowers show the phyllotaxis-plus-instancing pattern ([80.lv](https://80.lv/articles/magical-procedural-blossoming-flowers-created-with-blender-s-geometry-nodes)). Head normals should be up-biased at about 70/30 up versus head sphere, the EmacEArt anti-flicker principle, and shaded with the compressed flower-head response. Flowers cast no shadows.

Stylized heads can run 1.5–2× real size. Species choice for a seaside village should pair round and pointy forms, again after Sea of Thieves. Chamomile, sea thrift, lavender or lupin spikes, buttercup and cornflower give 5–6 species × 3 variants.

Colour discipline matters most. Pick a palette swatch **per drift**, not per instance, with 10–20% of clumps taking a neighbour swatch, and keep jitter within ±5% value and ±4° hue. Mixed per-instance colour is the classic "confetti" failure. The flower wind is height bend plus a per-stem bob of 2–6 cm at the head, sampling the same gust noise as grass, so a gust visibly crosses grass, then flowers, then bushes. That is the cheapest single "alive" cue in the scene (judgement). The flower LOD chain is the clump mesh to 20 m, up-facing head cards to 40 m, then a colour splat in the terrain.

## Scatter in layers, seed by world cell, and cluster everything but grass

### Production systems share one layered pattern

Horizon Zero Dawn placed "all nature" procedurally: **500+ asset types and 100,000+ objects at about 250 µs average GPU load**, with "Ecotopes made by 1 person". Its stated design goals include "Deterministic / Locally stable" ([van Muijden, GDC 2017](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)). Four of its mechanics transfer directly:

- **Footprints**: trees 6.0, bushes 3.0, undergrowth such as grass, sedge and fern 1.0 (units presumed metres).
- **Clearings**: undergrowth is driven by the *inverse* of the tree map, so clearings fill with ground cover.
- **Layered dithering**: layers that share a footprint split one precomputed dither pattern's threshold range. Layer A takes [0, dA) and layer B takes [dA, dA+dB), so they **cannot collide and weights become exact proportions**.
- **Painted bands**: one painted Placement_Trees value decodes into sparse edge → edge forest → inner trees bands.

Far Cry 5 contributes per-species viability from terrain masks, a priority system in which trees keep distance from each other while bushes may sit close, and an **"age" ramp from a signed distance field** that shrinks trees toward forest borders ([Mills notes on the GDC 2018 talk](https://christianjmills.com/posts/procedural-tools-far-cry-5-notes/index.html)). Ghost Recon Wildlands cascades placement, with "big trees spawn medium trees, which spawn small trees and bushes", and preferred more variants of fewer species ([80.lv](https://80.lv/articles/vegetation-generation-in-ghost-recon-wildlands)). Unreal PCG formalizes soft exclusion ("Minimum" difference mode) and position-mutated seeds ([UE PCG reference](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-framework-node-reference-in-unreal-engine)).

### Point generation

Point generation has two sourced building blocks. **Bridson's Poisson disk** uses a background grid of cell r/√n, k≈30 annulus candidates and an active list, and runs in linear time ([Bridson 2007](https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf)). **Kopf's recursive Wang tiles** give deterministic, tile-based blue noise with density by prefix truncation of progressive point sets ([ACM TOG 2006](https://dl.acm.org/doi/10.1145/1141911.1141916)).

The judgement layer on top has four parts:

- **Bridson is order-dependent**, so run it per **world cell** seeded by `hash(world_seed, cell, layer)`, then clip to the polygon. Moving a vertex then only changes points near that edge.
- **Grass and flowers use a pre-baked rank tile.** Bake a tileable 16 m blue-noise tile with a progressive rank per point and keep a point when `rank < density(x)`. Repainting density then adds or removes points without moving any.
- **Jittered grids read as planted rows from above**, so use Poisson for anything with a silhouette.
- **Per-instance randomness is keyed to the point**, as HZD keys it to "pattern idx/id". Yaw, scale and variant must never depend on draw order.

### Art-direction rules

The sourced art direction is short but consistent:

- Place in collections (Sea of Thieves).
- Clusters and no straight lines. For CGMA's Quinn, "abrupt transitioning and straight lines completely break the immersion" ([CGMA](https://www.cgmasteracademy.com/blog/5-ways-to-make-your-foliage-filled-scene-believable/)).
- A big/medium/small hierarchy. BotW's triangles are landmarks, view-blockers and tempo ([Nintendo Life](https://www.nintendolife.com/news/2017/10/zelda_breath_of_the_wilds_ingenious_design_is_all_about_triangles_apparently)).

Encoded for a cozy farm game (judgement):

- Trees in 2–5-tree Matérn clumps with overlapping canopies, so they read as one painted mass.
- One to three clearings of at least 6–10 m per biome polygon.
- Bushes and flowers attracted to rocks, fences, walls and tree bases.
- One or two flower species per drift, chosen by low-frequency noise.
- Polygon edges jittered 1–3 m, with Far Cry-style size taper.
- A **camera-side margin** for tall classes. At a fixed 45° view, anything south of a doorway or crop plot occludes it. No source covers this; it follows from the camera alone.

### Seaside mask rules

The seaside mask rules are all judgement, starting points to tune by eye:

| Zone | Rule |
|---|---|
| Wet sand | No vegetation |
| Dune grass | 3–15 m from shore, in Thomas clusters, peaking around 6–10 m |
| Reeds | 0–2 m from fresh water, placed along the bank tangent |
| Wind-bent shrubs | 10–30 m from shore, leaning downwind |
| Flowers | On flat ground, at least 1.5 m off paths |
| Trees | Kept 3–4 m from buildings and 2 m from paths, out of farm plots |

Rasterize exclusions as distance fields so every cut can feather. A soft edge reads as trodden ground; a binary one reads as a CAD boolean.

### Density budgets

Density budgets are where the notes disagree most, because they count different units. The grass research proposes **60–120 small tufts/m²** for lawn, about 300–800 blades/m² and comparable to hexaquo's 400. The scattering research proposes **3–8 larger 20–60-blade clumps/m²**. Both land in the low hundreds of blades per m².

Given the 45° argument that terrain paint carries the lawn, start at the low end: about 20–40 tufts/m² within 0–25 m. Raise density only if the front-lit shot looks bare. Visible ground at this camera is roughly 1,500–6,000 m² (judgement), so even generous budgets stay well inside MultiMesh territory. Trees at about 0.01/m² put around 60 on screen.

## The biome scatterer: a small custom tool beats every addon

### The existing addons

**ProtonScatter** is the closest existing tool. It is MIT-licensed, got a 4.7 compatibility commit on 2026-07-26 and a last push on 2026-09-27, and has Box, Sphere and Path domains with negative shapes, a Bridson Poisson modifier, raycast projection, clustering, chunked MultiMesh, threaded rebuilds and a transform cache ([GitHub](https://github.com/HungryProton/scatter)). Two code facts disqualify it for your trees:

- It **merges every MeshInstance3D of an item scene into one mesh and applies one cast-shadow setting per MultiMesh**. Your canopy cards (shadow off) and core and trunk (shadow on) cannot coexist without a shader workaround ([scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd)).
- Its item weighting is a proportional split of one point set (`count = round(proportion/total × transforms)`), not a per-point weighted draw ([scatter.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter.gd)).

It has also had no tagged release since 2023, and open issues follow each Godot minor version (4.6 modifier saving, 4.7 chunk size, cache load crashes) ([issues](https://github.com/HungryProton/scatter/issues)).

The other addons fit worse:

- **Spatial Gardener** is a brush painter with no maintainer commits since March 2025 and no sub-mesh scene support ([#83](https://github.com/dreadpon/godot_spatial_gardener/issues/83)).
- **Terrain3D's instancer** requires Terrain3D ground and single-mesh assets, and gives no collision ([docs](https://terrain3d.readthedocs.io/en/latest/docs/instancer.html)).
- **SimpleGrassTextured** (MIT, v2.1.0 April 2026) is grass-only ([GitHub](https://github.com/IcterusGames/SimpleGrassTextured)).
- **ScatterShot** (2026, Godot 4.5) was not inspected and deserves a ten-minute look first.

If forests are needed this week, ProtonScatter is a workable stopgap. Pair it with `if (IN_SHADOW_PASS) discard;` in the card shader, since `IN_SHADOW_PASS` is documented ([spatial shader reference](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html)), and use 32–64 m chunks.

### The Godot facts the design rests on

- **Editor APIs.** `EditorPlugin._forward_3d_gui_input` is called only for objects the plugin `_handles()`, and returns STOP to consume a click ([EditorPlugin](https://docs.godotengine.org/en/latest/classes/class_editorplugin.html)). `EditorNode3DGizmoPlugin` provides `_redraw`, `_set_handle` (whose camera "can be used to convert it to raycasts") and `_commit_handle` for undo ([gizmo plugin](https://docs.godotengine.org/en/latest/classes/class_editornode3dgizmoplugin.html)). `EditorUndoRedoManager` routes node properties to the edited scene's history ([docs](https://docs.godotengine.org/en/latest/classes/class_editorundoredomanager.html)).
- **MultiMesh.** It has **no per-instance culling**. Set `custom_aabb` to avoid recalculation. `use_colors` and `use_custom_data` can only change while `instance_count` is 0 ([MultiMesh](https://docs.godotengine.org/en/latest/classes/class_multimesh.html); [Using MultiMesh](https://docs.godotengine.org/en/latest/tutorials/performance/using_multimesh.html)).
- **Visibility ranges.** They work per node, and dithering is cheaper than alpha fade ([visibility ranges](https://docs.godotengine.org/en/latest/tutorials/3d/visibility_ranges.html)).
- **`NODE_POSITION_WORLD` under MultiMesh.** In the 4.7-stable vertex stage it compiles to `read_model_matrix[3].xyz`, where `read_model_matrix = model_matrix * matrix` for multimesh instances, so it is per-instance in single-precision Forward+. In `fragment()` it is the node's position ([scene_shader_forward_clustered.cpp @4.7-stable](https://github.com/godotengine/godot/blob/4.7-stable/servers/rendering/renderer_rd/forward_clustered/scene_shader_forward_clustered.cpp); [scene_forward_clustered.glsl @4.7-stable](https://github.com/godotengine/godot/blob/4.7-stable/servers/rendering/renderer_rd/shaders/forward_clustered/scene_forward_clustered.glsl)). Verify with two instances swaying out of phase. Anything per-instance needed in `fragment()` must pass through a varying.

### Design sketch

The sketch below merges the scattering and tooling research. Everything in it is judgement.

**Nodes.** Three node types make up the tool:

- `@tool class_name BiomeArea extends Node3D` stores `polygon: PackedVector2Array` (local XZ, plus optional hole rings), `seed`, `recipe: BiomeBag` and `bake: BiomeBake`. It is drawn by clicking on the ground.
- `BiomeExclusion` children (polygon or circle), plus a global `"scatter_blocker"` group for farm plots, roads and buildings, carve holes.
- An `EditorPlugin` handles both node types. It adds a spatial-editor toolbar with **Draw / Bake / Clear / Reseed**.

**Editing.** In Draw mode:

- LMB raycasts `project_ray_origin`/`project_ray_normal` against the ground collision mask, falling back to the node's Y plane, and appends a vertex inside an undo action.
- Shift-click inserts a vertex on the nearest edge; RMB deletes.
- A gizmo draws the outline hugging the terrain and exposes one handle per vertex. Handles re-raycast while dragging and commit an undo action on release.
- **Never rebuild during a drag.** Bake on commit or on the button.

**Recipe resources.** The bag is layered, and array order is priority:

```gdscript
class_name BiomeBag extends Resource            # e.g. beach_dune.tres, meadow.tres
@export var layers: Array[BagLayer]             # big → small: rocks, trees, bushes, ferns, flowers, grass
@export var edge_width_m := 3.0                 # density falloff inside the (noise-jittered) outline
@export var edge_jitter_m := 2.0
@export var clearing_noise: FastNoiseLite       # thresholded → clearings for tall layers only
@export var tint_noise: FastNoiseLite           # 5–20 m painterly colour washes

class_name BagLayer extends Resource
@export_enum("poisson_cluster", "blue_noise_tile", "attach_to_layer", "along_curve") var method := 0
@export var entries: Array[BagEntry]            # the weighted bag
@export var base_density := 0.01                # per m² before masks
@export var density_noise: FastNoiseLite        # 5–15 m modulation
@export var cluster_radius_m := 4.0             # Matérn R / Thomas σ
@export var cluster_count := Vector2i(2, 5)
@export var attach_layer := ""                  # understory: seed parents at this layer's points
@export var attach_dist := Vector2(0.6, 1.5)    # × parent footprint, shade-side biased
@export var suppress_below := 0.4               # soft multiplier on later layers inside footprint (UE "Minimum")
@export var rules: Array[MaskRule]              # slope, height, dist_shore/water/path/building/edge, painted, noise
@export var edge_density: Curve                 # can bump UP at ecotones (flowers at forest edge)
@export var edge_scale: Curve                   # FC5 "age" taper
@export var chunk_size_m := 32.0                # 64 for trees
@export var visibility_end_m := 0.0
@export var cast_shadow_override := -1          # -1 = use source part settings

class_name BagEntry extends Resource
@export var scene_or_set: Resource              # PackedScene or TreeSet
@export var weight := 1.0
@export var radius_same_m := 5.0                # spacing within its own layer
@export var radius_other_m := 1.0               # footprint later layers must respect
@export var scale_range := Vector2(0.85, 1.2)   # log-uniform
@export var tilt_max_deg := 3.0
@export var normal_align := 0.1                 # trees ~0.1, grass/rocks 0.6–1.0
@export var sink_m := Vector2.ZERO
@export var tint_jitter := Vector3(0.01, 0.0, 0.06)
@export var neighbour_dedupe := 0.85            # weight penalty if a near neighbour uses this variant
@export var collision_radius := 0.0             # >0 → trunk cylinder
@export var as_scene_instance := false          # interactables (starfruit tree) stay real scenes
```

`MaskRule` is `{source, range, remap: Curve, op: multiply|min|max}`. Within a `blue_noise_tile` layer, entry weights split the rank interval cumulatively, so they are **exact proportions**, per HZD's layered dithering. Within a cluster layer, they are expectations.

**Bake algorithm.**

1. **Clip.** Build the world-XZ polygon and subtract exclusions with `Geometry2D.clip_polygons`.
2. **Rasterize fields.** For each 32 m chunk touched, rasterize signed distance to the jittered edge, shore, water, paths and buildings at 0.5 m, cache it, and invalidate only dirty chunks.
3. **Candidates.** For each layer in order, and each world cell of about 2× the layer's maximum radius, seed the RNG with a stable FNV hash of (seed, layer, cx, cz), not Godot's `hash()`, whose cross-version stability is unverified. Generate candidates with a **fixed draw count**:
   - Poisson cluster parents with Matérn/Thomas offspring for big layers;
   - parents seeded at an earlier layer's points or along a curve for attached layers;
   - the rank tile for ground layers.
4. **Density.** Density = painted × Π(rule remaps) × noise × edge curve × soft suppression. Accept by `hash01(point) < density`, or `rank < density` for tile layers.
5. **Exclusion.** Enforce hard exclusion against an occupancy spatial hash: `radius_other` of earlier layers and `radius_same` within the layer. Earlier layers claim; later layers only read.
6. **Ground.** Raycast down on the main thread in batches, since physics-space access from worker threads is unsafe. Reject on a miss, a blocker hit, slope above the maximum, or a point below sea level.
7. **Variant and transform.** Pick the variant by weighted draw with the neighbour-dedupe penalty, and call `TreeSet.pick()` with a position hash. Compute yaw, tilt, scale (× edge age × cohort noise) and sink.
8. **Store.** Write compact placements (position, yaw, scale, normal xz, asset index) and a `source_hash` into a **binary `.res` per BiomeArea**.

On storage size, a text `.tscn` holding raw MultiMesh buffers runs about 200–250 B per instance per sub-mesh, against about 80 B in binary (arithmetic from the tooling notes). Placement lists shared across sub-meshes are about 3× smaller again.

**Render build.** The same code runs in the editor and at runtime, with no physics:

- Instantiate each unique scene once and record each `MeshInstance3D` part's mesh, `material_override`, `cast_shadow`, `extra_cull_margin` and local transform.
- Create **one MultiMeshInstance3D per (chunk × variant × part)**, sharing the instance transforms. Assign the buffer as one `PackedFloat32Array`, set `custom_aabb` to the chunk bounds plus the maximum extent and a wind margin, and copy each part's shadow and cull settings, so cards stay shadowless while the core and trunk cast.
- Leave these nodes **unowned** so they never save into the scene.
- Build one `StaticBody3D` per chunk with trunk cylinders for entries that have `collision_radius > 0`.
- Instantiate `as_scene_instance` entries as normal scenes.
- Name nodes *before* `add_child`. ProtonScatter measured "about a 100x slowdown" otherwise ([scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd)).

With 10 tree variants × 3 parts, that is up to 30 MultiMeshes per tree chunk, a few hundred draw calls on a village map.

**Tests and effort.** Three tests cover the tool:

1. A fixed polygon and seed must produce an identical placement hash across runs.
2. Moving one vertex must change placements only within about two chunks.
3. A render smoke test checks MultiMesh counts, that the cards MultiMesh has shadows off, and that `custom_aabb` contains every instance.

The tooling notes estimate 1–2 days for the core and one more for exclusions, edge falloff, colliders and polish. Ship five preset bags: `beach_dune`, `coastal_scrub`, `meadow`, `orchard_edge` and `stream_bank`.

## Recommended recipes per category

Every number in these tables is **engineering judgement** to calibrate in the lookdev contact-sheet loop, and none has been built yet. Shared contract for all categories:

- **Export**: FLOAT_COLOR `COLOR_0` for wind or proxy data (the proven channel); UV for atlas or (u,v); UV2 for pivots or card offsets; `force_disable_compression`; auto-LOD and shadow meshes off.
- **Shading**: painterly `light()` with brush noise and teal shadows, `SPECULAR = 0`.
- **Wind**: global wind uniforms shared with the trees.
- **Instancing**: placement via `BiomeArea`, with per-instance tint in `INSTANCE_CUSTOM`, not `COLOR`.

### Grass and ground cover

| | Lawn | Meadow | Dune grass | Reeds |
|---|---|---|---|---|
| Build | Opaque tufts, 5–7 blades, ~30–40 tris, 4–6 variants | Opaque tufts, 7–12 blades, 60–100 tris; about 1 in 15 carries a seed head | Fountain clumps, 12–30 arching blades + 0–3 spikes, 150–400 tris | Vertical clumps, 6–15 stems, some cattail heads, 100–250 tris |
| Height | 0.08–0.18 m | 0.35–0.8 m, Voronoi-patchy | 0.5–1.0 m | 1.0–2.0 m |
| Density | 20–40 tufts/m² to start (max 60–120) | 25–50/m² in patches | 0.5–3 clumps/m² in drifts | 5–15/m² in 1–3 m bank bands |
| Normals | Terrain/up only | Up + 0.2 rounding, fading to up by 12 m | Per-clump proxy 50–70% + up | Per-clump proxy ~50% + up |
| Colour | Terrain texture at root → tree `col_mid` → `col_top` | Wider gradient, warm tips | Grey-green → straw (≈0.78, 0.74, 0.50) | Blue-green → tan; brown heads |
| Wind | ≤5–8 cm; gust colour sheen does most of the work | Visible ridged gust waves, 8–15 m wavelength | Baked leeward lean + gusts | Rigid root pivot, 2–4 s period, phase along bank |
| Shadows | Off | Off | Near ring or proxy | Near ring |
| Visibility end | 30 m, sink into the ground over the last 10 m | 45 m | 55 m | 55 m |
| Far fallback | Terrain grass-zone paint | Terrain paint + cooler patch mask | Sand with faint grey-green speckle | Wet-edge colour band |

The GN tuft generator sweeps a Bézier ribbon per blade. It bakes R = height along the blade, G = per-blade random, B = width coordinate and A = stiffness mask, with UV2 = blade root offset. Build order:

1. Terrain grass-zone paint.
2. Lawn tufts and the shared `grass.gdshaderinc`.
3. Global wind, with the trees hooked to it.
4. Dune grass, then reeds, then meadow.
5. Pushers, then profile.

### Bushes, hedges and flowering shrubs

| | Round shrub | Wild hedge | Topiary hedge | Flowering bush |
|---|---|---|---|---|
| Base | `gn_tree.py`, Wood off, envelope library of 4–6 mounds (h ≈ 0.6–0.8 × width) | Swept lumpy tube, height ±20% | Rounded box → Mesh-to-SDF + faint inset bumps | Round shrub recipe |
| Core | Voxel 0.04–0.06, flatten 0.8, sink 10–20% | Per-clump SDF | SDF Mean, no holes | As shrub |
| Cards | 60–150 at 0.25–0.45 m + 8–20 skirt cards; core dab projection ~35% | Tree-style, skirt both sides | Short (0.15–0.25 m), dense, ≤3 cm overhang | As shrub |
| Proxy | 60% clump / 40% whole | 50% clump / 50% swept cylinder | Blurred box normals | As shrub |
| Tris (LOD0) | 500–1,300 | 600–1,200 per m | 300–600 per m | + flowers |
| Delivery | MultiMesh, 32 m chunks; LOD1 = core + 30% cards past 20 m | 2 m and 3 m modules, caps, corners (overlong, clipped) via Path3D placer; hero hedges as GN sweeps | As wild | Flower layer on its own slot or MultiMesh |

For flowering bushes, place 2–5 clusters on the sun side and leave at least 40% green:

- **Hydrangea**: 5–15 mophead blobs of 40–120 tris, blue, violet or pink per bush.
- **Hibiscus**: 6–15 flowers of 0.12–0.18 m.
- **Beach rose**: 8–20 magenta or white flowers of 0.07–0.1 m, with hips in season.

Buds are the same flowers at 0.3 scale, tinted green. The far LOD bakes the bloom colour into the core. Wind sway is 1–3 cm with card flutter only, and a new gate requires the core's minimum z to be at or below 0.

### Ferns and large-leaf plants

| Plant | Construction | Normals | Tris (LOD0 → LOD1 → LOD2) |
|---|---|---|---|
| Fern (0.5–1.2 m) | 10–18 golden-angle fronds, 14–22 alternate pinna pairs of 4-tri creased diamonds, V-fold 15–25°, 1–3 fiddleheads | Crown ellipsoid, spherize 0.7 + 0.2 spread | 1–1.5k → 400–600 alpha ribbons → ~100 cards or mound |
| Hosta (0.3–0.7 m) | 15–25 ovate-cordate blades (6×(2+2) grid), ruffle, arching petioles | Mound, spherize 0.5 | 0.7–1.2k → 300 → 60–100 |
| Taro / elephant ear (0.8–1.8 m) | 5–9 sagittate blades (10×(3+3) grid, lobes to negative u, optional peltate attachment), long petioles | Per-leaf 0.3 + plant 0.15; pale back face; strong translucency | 0.6–1k → 300 → 80 |
| Monstera (0.8–1.5 m) | 6–12 blades (12×(3+3) grid), 4–9 slits per side and 0–2 hole rows as deleted faces; juveniles unholed | Per-leaf 0.3 | 1–2k → 500 (holes kept) → 100 |
| Banana shrub (1.5–3 m) | Pseudostem + 6–10 oblong blades (L/W 3–4) with 3–8 tears, each strip on its own phase; rolled cigar leaf in the centre | Per-leaf 0.25 | 1.2–2.5k → 600 → 150 |
| Agave / succulent (0.3–1.2 m) | 18–40 thick lens-section leaves, golden angle, pitch 85° → 20–35°; echeveria variant uses Vogel √n | ≤0.2, `cull_back` | 0.5–1k → 250 → 80 |

All thin leaves use `cull_disabled` plus the fragment back-face re-flip. Veins come from (u,v) stripes. Wind rotates about the UV2 pivot by `strength·t²` with edge flutter; agave and fiddleheads get none. Shadows come only from LOD0 inside about 15 m, or from a proxy dome.

### Small flowers and wildflowers

The clump is 3–9 curve stems of 0.15–0.6 m with heads of 5–8 petal quads tilted 60–90° up and oversized 1.5–2×, plus 2–4 base blades, at 60–300 tris. Build 5–6 species × 3 variants, mixing round and pointy forms.

Place them in drifts of 5–30 clumps along paths, fences and house fronts, with a palette swatch per drift and 10–20% taking a neighbour swatch. Chunks are 16–32 m: the clump mesh shows to 20 m, head cards to 40 m, and a terrain splat takes over beyond. Heads use 70/30 up-biased normals and compressed toon shading, cap white at about 0.85, and cast no shadows. Wind is height bend plus a 2–6 cm head bob at 1.5–3 Hz, sampling the grass gust noise.

## What is sourced and what is judgement

**Sourced:**

- The construction families: up-normal grass, blob-core-plus-card bushes, spine-plus-leaflet ferns, and opaque geometry favoured for ferns.
- Godot engine behaviour: back-face normal negation, per-instance `NODE_POSITION_WORLD` in the vertex stage (read from source, not yet tested), MultiMesh culling and flag constraints, visibility ranges, and the editor APIs.
- The addon facts, from code and issue trackers.
- The production scattering patterns: HZD footprints, layered dithering and clearings; Far Cry 5 viability, priority and age ramps; the Wildlands cascade; UE PCG soft exclusion.
- A few performance anchors: hexaquo's 10M grass tris in under 2 ms, and commercial tropical-plant tri counts.

**Engineering judgement:**

- Every per-category number: densities, tri counts, spherize blends, colour values, visibility distances and chunk sizes.
- The seaside mask rules.
- The camera-side occlusion margin.
- The bush and flower transfer percentages.
- The terrain colour-texture fix for the `COLOR` conflict.
- The analytic fern proxy.
- The decision to build rather than adopt a scatterer.

**Weakly sourced:**

- Game-specific claims about Breath of the Wild, Genshin, Sea of Thieves and Ghost of Tsushima come from reverse-engineering or secondary summaries, and several Sea of Thieves quotes come from search snippets of a page that returned 403.
- Nothing citable exists on how Ghibli, A Wonderful Life, Ni no Kuni, Story of Seasons or Animal Crossing technically build any of these plants.

**Untested:**

- None of the Blender 5.2 node steps for the new generators was executed; the Blender MCP was down throughout this research.
- ScatterShot was not inspected.
- The 4.7 fixes for the global-sampler bugs were not confirmed.

## Conclusion

The tree build showed that a stylized tree is structured data the shader consumes. This research extends that finding to everything else: each foliage category is a **different primitive feeding the same data contract**. The pipeline therefore grows by adding small GN groups (tuft, frond rosette, blade rosette, flower clump) and a few inputs to `gn_tree.py`, not new shaders or new tools. The camera shifts effort as well. At 45°, most of the painterly read of a meadow or lawn lives in the terrain shader and in *where* things cluster, not in blade detail. The two highest-leverage early investments are therefore the terrain grass-zone paint and the biome scatterer's cluster, clearing and edge logic, both cheaper than any asset.

The scatterer is also a better evaluation surface than single assets. The world-cell determinism that keeps designer edits local also makes placements diffable and testable. The Addendum's cheap gates carry over directly as per-class instance counts, clearing-area fractions and flower-drift coherence, measured on a top-down mask of a baked `BiomeArea`. A one-day spike settles the biggest open risks:

- Two MultiMesh instances swaying out of phase (the `NODE_POSITION_WORLD` question).
- A lawn tuft over terrain paint at the front-lit shot (the density question).
- A 0.2×-scaled tree bush with a skirt (the ground-contact question).
- One baked `meadow` polygon, checked for local stability after a vertex drag.

---

## Addendum (2026-09-28): painted ground textures, first-hand

What follows was learned building the seaside ground textures for this project (board cards G1/G1a; code in `FarmGameGodot/tools/ground/`). None of it came from a source; all of it was tested in Godot 4.7 under the beach light from the fixed 45° game camera. The approved result is the dry-sand texture (`sand_dry.py`, 8 m tile, 2048², 256 px/m).

### Process: focus beats batch, and thumbnails lie

- **Making six materials at once produced generic noise.** The first pass painted six ground textures in one sweep (sands, lawn, meadow, path). It passed every numeric check and was rejected as "unacceptably bad". The textures were procedural camouflage and carpet: no designed detail, no idea of what a painter would put there. The same tooling, pointed at one material with a written design brief and about ten full-size critique passes, was approved. It is the same lesson as the hero tree: quality came from attention on one subject.
- **Judge at the game camera, at full resolution, with context.** 400×300 contact-sheet cells and flat 2×2 tiles hid every important failure. What caught them:
  - a 1600×900 render with the hero tree and a 1.7 m capsule for scale,
  - four fixed framings: front-lit, back-lit, a 3.5 m close-up, and a 30 m wide shot,
  - 1:1 crops of the texture itself.
- **The 30 m wide shot is the repeat detector.** Every tiling problem (blotch grids, stamped patches, props repeating) was invisible at the game camera and obvious at 30 m.

### Light: author albedo for the engine, and bake relief with the real sun

- **The lit scene multiplies albedo by about 1.6 and warms it.** That comes from the beach sun at energy 1.35 with colour (1, 0.91, 0.79), plus 0.35 ambient. Textures that looked right in an image viewer came out pale-yellow (sand) and orange (dirt) in game. The dry-sand base albedo that reads as the style bible's `sand_light` in game is **#D6C8AE**, noticeably greyer and cooler than the target. A palette check on the albedo cannot catch this. Colour has to be measured in the lit render, not the texture.
- **Paint relief, not noise.** The breakthrough was the same idea that made the tree work: forms with light and shadow families. The painter now builds a tileable height field (gentle mounds, weak hummocks, wind-ripple fields). It lights that field with the scene's actual sun into three posterized bands (warm lit, base, lavender shade) with brush-noise-jittered edges, which is the same model as the runtime toon `light()`, so baked and live shading agree.
- **Mapping the sun into texture space.** Ground UV is world (x, z) / tile size, with image +x = world +x and image +y = world +z. The Godot to-sun vector (−0.452, 0.616, 0.645) therefore becomes image-space light from (−0.57, +0.82), elevation sin = 0.616. Cast shadows in the texture fall right and up.
- **Caveat.** Baked relief is only correct for a fixed sun and world-aligned, unrotated UVs. A day/night cycle or rotated ground UVs would break it, and would need the relief as a normal/height map lit at runtime instead.

### Brushwork: follow form where there is form

- Strokes that follow the height field's contours describe ripples and mound sides beautifully. On nearly flat sand, the same rule follows random micro-contours and the ground looks **furry**.
- The fix is what a painter does. Where the slope is low, drag every stroke in one consistent, slowly wandering direction (a dry-brush pass), slightly lighter or darker than the ground (±2.5–5%), long and thin (7–18 cm × 1–3 cm). Only follow contours where form exists: inside ripple fields, or slopes above about 0.35.

### Wind ripples that don't look procedural

- **One sine train looks like corduroy.** Two trains a few degrees apart (9° here), with slightly different wavelengths (72 and 81 cycles per 8 m tile) and weights (1.0 and 0.55), interfere into the forks and pinch-offs of real ripple fields. Integer wave vectors keep them exactly tileable.
- **Profile and meander.** `sin(q) + 0.35·sin(2q + 0.6)` gives a steep lee and a gentle stoss side. A multi-octave warp of about ±1.3 cycles (at 3, 5 and 8 cycles per tile) makes them meander.
- **The shape of the ripple-field mask decides whether it looks painted or stamped.**
  - A mask from 1–2 cycles-per-tile noise makes one diagonal band per tile, which repeats as stripes.
  - A single mid-frequency band gives stamped ovals, like repeated fingerprints.
  - What works: a multi-scale shape (1, 2, 3, 5 cycles), a low-coverage threshold (smoothstep 0.55–0.85), and strength modulated inside the field so ripples fade in and out.
- **Numbers.** Ripple amplitude about 1 cm on a 10 cm wavelength. The shade band starts at N·L < 0.48, so only lee faces go lavender; lower thresholds let weak hummocks make grey smudges.

### Tiling: keep only non-distinct detail in the tile

- **Any distinct feature betrays the repeat.** Pebble groups, a shell or one pale blotch show up at exactly the tile period. Designed props looked lovely in the tile and had to come out of it.
- **Keep them as props instead.** The pebble, shell and twig painters are kept for the scatterer's small-props/decal layer, placed per world position.
- **Tile scale.** Big colour variation (1–8 m) belongs to a world-space ground shader, not the tile; inside the tile, macro contrast stays tiny (8×8 block-mean std about 0.4 L*). An 8 m tile at 2048² (256 px/m) was a good trade for sand at this camera.

### Small props for a 45° camera at 8 m

- **Matte stones: no glint.** Build them as a shadow-coloured body, then re-paint the body shifted toward the light, leaving a lavender crescent on the far side. Add a broad subtle lit patch and a thin crisp cast-shadow sliver. A specular dot makes pebbles read as glass beads or pearls.
- **Illustrator edges.** Anti-aliased filled polygons (supersampled) read as drawn props. Soft radial dabs read as dirt.
- **Exaggerate scale by roughly 1.5–2×.** True-scale 2–3 cm pebbles are 3–5 px from the game camera. At about 6–10 cm they read. Painters do the same.

### Tooling notes

- **Route comparison for tileable painterly ground.**
  - Material Maker's stylized sand: airbrushed clouds that repeat visibly from 30 m.
  - Z-Image concepts: painted in side perspective, unusable as top-down ground.
  - A custom numpy painter on a torus: exact tiling by construction, locked to the palette, about 5 s per 2048² texture. It won.
- **Periodic noise from cosines needs many terms.** Sums of integer-frequency cosines are exactly tileable, but with about 6 terms per octave they show directional banding. About 14–16 is isotropic enough.
- **Give each layer its own random stream.** Otherwise tuning one layer reshuffles everything after it, and a critique loop can't compare like with like.
- **Checks are floors, not goals.** Two lessons from the checker itself:
  - A seam metric must compare each axis with its own neighbour-step statistics. Mixing them made the directional ripple texture fail falsely.
  - A "brushwork" contrast floor tuned on noisy textures rejects deliberately calm designs. Per-material floors, documented next to the design brief, are honest. The number should only guard against a pure airbrush; the eye judges the rest at 1:1.

**Update (dune sand, same day).**
- **Macro flattening.** A texture where one feature covers parts of the tile and not others (rippled vs wind-scoured sand) averages to different brightness in those zones, and that reads as an 8 m checker from 30 m even when every local detail is good. A reliable fix is to take a periodic (FFT) Gaussian low-pass of the luminance (σ ≈ 25 cm here) and scale each pixel toward the tile mean by the ratio. That keeps all local detail and removes only the large-scale variation, which belongs to the world-space ground shader anyway. It is `flatten_macro()` in `tools/ground/ground_brush.py`.
- **Dune ripples.** Ripple fields covering most of the ground look like woodgrain unless the warp is low-frequency and large (about ±3.5 cycles at 1–3 cycles per tile), so the ripples sweep around implied topography rather than wobble.

**Update (meadow ground, same day).**
Clump-scale forms are where textures most easily turn into something else. Three failures in a row, each identifiable by what it looked like:
- **Band-limited noise at tussock scale** (20–40 cycles per tile) makes labyrinth ridges, like a topographic map or brain coral.
- **Discrete clumps of even size with high-contrast lit tops** read as leopard spots from the game camera.
- **Overlapping clumps combined with `max()`** leave crisp creases that the lighting turns into cell outlines, like moss or lichen.

What worked:
- Many clumps (radius skewed so many are small and a few big), combined as a smooth union `1 − exp(−k·Σ bumps)`.
- Soft, wide light bands.
- Directional blade strokes (6–12 cm, leaning with the wind, ±5–14% value) strong enough that the grain of the grass carries the read, with the clumps only as soft masses underneath.
- For tree-vs-ground separation, the ground green sits warmer and lighter (base #7B9C58) than the canopy mid-tone (~#528C4D).

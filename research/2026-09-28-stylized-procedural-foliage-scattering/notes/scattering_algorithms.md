# Scattering algorithms and art-direction rules for "place a biome in a polygon" (Godot 4.7, cozy Ghibli / HM:AWL seaside farm)

Scope: point distribution, multi-class ecological placement, masks, art direction, per-instance variation, and density budgets. The notes end with a recommended algorithm stack and bag data model. Tags used throughout:
- **[SOURCED]**: backed by the linked source.
- **[JUDGEMENT]**: engineering or art judgement by the researcher. No studio or paper states it.

Context carried over from the tree report (`reports/Stylized procedural trees for Godot.md`). Trees are instanced through MultiMeshInstance3D chunks of about 32–64 m with visibility ranges. Per-instance tint and wind phase travel in the MultiMesh colour and custom data. The 10 deciduous variants live in `FarmGameGodot/art/trees/deciduous_set/` as `deciduous_set_01..10.glb/.tscn`, behind `TreeSet.pick(key)`. The scatter tool should output per-chunk MultiMesh transforms that feed that system.

---

## 1. Point distributions (Poisson disk, variable radius, tiles, jittered grid; polygons with holes; determinism)

### Takeaway
Use **Bridson Poisson disk** (or an equivalent dart-throw against a grid) per size class. Place big items first and smaller classes against the accumulated occupancy. For determinism and local stability, seed by **world cell hash**, not by polygon. Both Horizon Zero Dawn and Kopf's recursive Wang tiles explicitly design for "deterministic, locally stable" output. For ground cover (grass, flowers), a **precomputed blue-noise / dither tile thresholded by a density map** (HZD-style) is cheaper and gives density control for free.

### Cited Findings
- **Bridson 2007 (fast Poisson disk, arbitrary dimension)** [SOURCED, verified from the PDF text]. It uses a background grid with cell size r/√n, so each cell holds at most one sample, plus an "active list". For a random active sample, it generates up to k candidates "chosen uniformly from the spherical annulus between radius r and 2r" (typically k = 30). It accepts a candidate if no existing sample lies within r, and otherwise removes the sample from the active list. Each iteration adds or removes a sample, so the algorithm is linear in the number of samples — [Bridson, SIGGRAPH 2007 sketch](https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf)
- **Recursive Wang tiles (Kopf, Cohen-Or, Deussen, Lischinski, SIGGRAPH 2006)** produce non-periodic blue-noise point sets over arbitrarily large areas. Local density follows "an arbitrary target density function". The method is "deterministic and tile-based", so "any local portion of a potentially infinite point set [can] be consistently regenerated", with cost proportional to the integral of density over the area. It relies on progressive point sets inside each tile, which gives spatially varying density by prefix truncation — [ACM TOG / SIGGRAPH 2006](https://dl.acm.org/doi/10.1145/1141911.1141916); [SIGGRAPH history summary](https://history.siggraph.org/learning/recursive-wang-tiles-for-real-time-blue-noise-by-kopf-cohen-or-deussen-and-lischinski/)
- **HZD's discretisation is a precomputed dither pattern, not per-frame Poisson** [SOURCED, from the slides]. The GENERATE step is "Discretizing step / Dither based / Responsible for collision". The pattern comes from an offline generation tool with the rules "Even spread thresholds", "Maximize 2D distance", "Uniform 2D distance w", and "Scale to w = footprint". Each thread takes one sample and runs a range test, a threshold test (pattern threshold < density), position generation, and normal construction. This takes about 10 µs. Placement then uses the "pattern idx/id for RNG", so per-instance randomness is keyed to the pattern point, not to spawn order — [van Muijden, GDC 2017 slides (PDF)](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- HZD's stated design goals include "Data driven / Deterministic / Locally stable" — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- **Unreal PCG Surface Sampler is a jittered grid with rejection** [SOURCED]. "Point Extents: Defines the basic grid cell size". "Looseness: … cell size is point extents * (1 + Looseness)". "Points Per Square Meter: Computes the ratio of kept cells". PCG mutates point seeds "according to its position, previous seed, this node's seed, and the component's seed", which makes them position-keyed and therefore locally stable — [UE PCG node reference](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-framework-node-reference-in-unreal-engine); [UE PCG overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-overview)
- UE PCG points carry "transforms, bounds, color, density, steepness, and seed", and "Point Density represents the probability of the point existing at that position" — [UE PCG overview](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-overview)
- **Godot prior art: ProtonScatter (MIT, Godot 4)** supports Box, Sphere and Path shapes. Shapes can be marked negative to form exclusion zones. It has a modifier stack (random, grid, edge creation; randomize and relax transforms; project on colliders; remove overlap; cluster), proportional item weighting, a global seed, and MultiMesh with chunking — [GitHub HungryProton/scatter](https://github.com/HungryProton/scatter)

### Inferences
- [JUDGEMENT] **Local stability with Bridson.** Standard Bridson is order-dependent: moving one polygon vertex re-flows the entire active-list growth. For "small edits change results only locally", generate candidates per **world cell** (for example 8 m cells). Seed each cell with `hash(world_seed, cell_x, cell_z, class_id)`, draw a fixed candidate list per cell (dart throwing or a pre-baked Poisson tile), and **then** clip to the polygon and masks. Moving the polygon edge then only adds or removes points near the edge, because interior cells keep identical candidates. This is how HZD (pattern-tile idx → RNG) and UE PCG (position-mutated seeds) behave.
- [JUDGEMENT] **Cross-cell conflicts.** Resolve conflicts at cell borders deterministically: process cells in a fixed order (sorted by cell coordinate) and check a 3×3 neighbourhood. Alternatively, use a pre-baked **toroidal (tileable) Poisson tile** with minimum distance r, stamped per cell with a hashed rotation/flip. Tile seams can then be slightly under-spaced. Cozy art direction hides that, and the later exclusion pass fixes overlaps between big items.
- [JUDGEMENT] **Pre-baked tile with rank thresholds (recommended for grass and flowers).** Bake one 16×16 m tileable blue-noise point set of about 4,000 points. Give each point a progressive rank in [0,1), as in Kopf's progressive sets and HZD's "even spread thresholds". Keep a point iff `rank < density(x)`. This gives painted and noise-driven density with zero runtime search. It stays stable when density is repainted, because points only appear or disappear and never move.
- [JUDGEMENT] **Point in polygon with holes.** Use the even–odd ray-crossing rule over all rings (outer ring plus hole rings), which handles holes automatically. Alternatively, triangulate once with Godot's `Geometry2D.triangulate_polygon()` and area-weighted triangle sampling, although triangulation does not handle holes directly. Use `Geometry2D.is_point_in_polygon()` per ring, or `Geometry2D.clip_polygons()` / `offset_polygon()` to build holes and inset rings. Store **signed distance to the polygon boundary** per candidate. It drives edge falloff (section 2) and edge-size tapering. (These Godot API names come from the researcher's knowledge of Godot 4.x and were not re-verified against 4.7 docs.)
- [JUDGEMENT] **Choosing among samplers.** A jittered grid (UE Surface Sampler style) is acceptable for grass. It reads as "planted rows" for trees and bushes at a 45° top-down camera because regularity is very visible from above. Use Poisson for anything with a readable silhouette.

### Gaps
- No public numbers were found on how often HZD's tile pattern repeats, or on the tile size used.
- No Godot 4.7-specific scatter benchmarks were found.

---

## 2. Multi-class / ecological placement (HZD, Far Cry 5, Wildlands, UE PCG, EcoBrush; suppression, understory, clustering, edges)

### Takeaway
Every production system found uses the same pattern: **layers ordered big to small, where each placed object claims a footprint that suppresses or modulates later layers**. On top of that sit **per-species viability from masks** and **density maps shaped by logic graphs**. Far Cry 5 adds an **age / size ramp from distance-to-edge**, which produces smaller trees at forest borders and is a direct fit for polygon edge falloff. Wildlands uses a **cascade** ("big trees spawn medium trees, which spawn small trees and bushes"), which is the understory rule.

### Cited Findings
- **HZD ecotopes** [SOURCED]. An "Ecotope describes environment" and determines asset types, distribution, colourisation, weather, effects, sound and wildlife. HZD used procedural placement "for all nature": 500+ asset types, 100,000+ objects in scene, about 250 µs average GPU busy load. "Nature assets created by 3 people, Ecotopes made by 1 person" — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- **HZD footprints per asset** [SOURCED]. The ecotope asset slide lists Trees (Lodgepole Pine 6.0, Douglas Fir 6.0), Bushes (Grease Wood 3.0) and Undergrowth (Carex grass 1.0, Payson's Sedge 1.0, Fern 1.0). The generation slide says to "Scale to w = footprint", which makes these numbers the per-asset dither spacing (footprint) in metres (inference on units, since the slide does not state them). A "Clearing" node in the logic graph uses an "Inverse" of Placement_Trees to drive undergrowth, so clearings get undergrowth where trees are absent — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- **HZD collision between layers** [SOURCED]. "Different footprint? → Read-back, dependencies". "Same footprint? → Layered Dithering". Layered dithering keeps "Two values in density map" with a "Two-Sided threshold test": layers stack in density space on the same dither pattern, so layer A takes thresholds [0, dA) and layer B takes [dA, dA+dB). They cannot collide, and weights become exact proportions. HZD used "ordering heuristics" to reduce dependencies — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- **HZD painted world data decoding** [SOURCED]. A single painted Placement_Trees map value 0→1 is decoded into bands: sparse edge → edge forest → inner trees. One brush thus paints a whole forest-edge gradient. World data is a set of 2D maps at 0.5–2 m resolution (Placement_Trees / BlockBush / Undergrowth at 1.0 m; Topo_Roads / Topo_Water / Topo_Objects at 0.5 m; Water_Flow; Erosion; Ecotopes A–H at 2.0 m). About 4 MB/km², "All Generated / All Paintable" — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- **Far Cry 5 biome recipes** [SOURCED via secondary notes on the GDC 2018 talk by Etienne Carrier]:
  - Sub-biome recipes contain "ingredients (trees, saplings, bushes, grass)".
  - Terrain inputs are "Occlusion, flow map, slope, curvature, illumination, altitude, latitude, longitude, wind vector map".
  - Species accumulate **viability** scores, and the higher-scoring species "wins" at a location.
  - A viability radius stops other species growing too close.
  - A **priority system** is used: trees (priority 10) with smaller priority radii let bushes (priority 0) sit close while trees do not overlap each other.
  - A "Density ramp controls the scattering density based on size, age, or viability".
  - **Age from a signed distance field**: "Age maximum distance controls the depth of the forest border tapering effect", which puts smaller trees at the edges.
  - A biome painter allows manual overrides such as clearing roads.

  Source: [Christian Mills notes](https://christianjmills.com/posts/procedural-tools-far-cry-5-notes/index.html); original [GDC Vault](https://www.gdcvault.com/play/1025557/Procedural-World-Generation-of-Far) / [slides PDF](https://ubm-twvideo01.s3.amazonaws.com/o1/vault/gdc2018/presentations/ProceduralWorldGeneration.pdf). Recipes "react to water proximity, altitude, and cliff erosion lines" — [PlayStation Blog](https://blog.playstation.com/2018/03/22/the-procedural-world-generation-of-far-cry-5/)
- **Ghost Recon Wildlands** [SOURCED, secondary]:
  - A Houdini tool spreads bushes and trees in a cascade: "big trees spawn medium trees, which spawn small trees and bushes".
  - Rules cover density, scale, slope, spacing, sun direction, material matching and flow maps.
  - Vegetation is auto-removed from roads, railways, water and footpaths.
  - A separate grass system spawns patterns from terrain splatting.
  - The team preferred multiple "steps" (variations) of one species over many species.

  Source: [80.lv – Vegetation Generation in GRW](https://80.lv/articles/vegetation-generation-in-ghost-recon-wildlands). The game has 11 biomes with per-ecosystem rock-scatter rules (curvature, slope alignment, cliff and road detection) — [80.lv – Procedural Technology in GRW](https://80.lv/articles/procedural-technology-in-ghost-recon-wildlands)
- **UE PCG pruning and exclusion** [SOURCED]. Self Pruning prioritises "Large to Small, etc.", has a "Radius Similarity Factor", and supports "randomized pruning to prevent patterns from emerging". The Difference node has two density modes. **Binary**: density becomes 0 if the difference density is > 0. **Minimum**: source density minus the max difference density, which gives soft exclusion. Spatial Noise writes a "spatially-consistent noise pattern (such as Perlin noise)". Density Remap applies a linear transform — [UE PCG node reference](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-framework-node-reference-in-unreal-engine)
- **EcoBrush (Gain, Long, Cordonnier, Cani, EG 2017)** [SOURCED, abstract level]. It runs ecosystem simulation offline and stores distribution statistics in biome databases indexed by terrain conditions (temperature, rainfall, sunlight, slope). Those databases then drive interactive synthesis with brushes, up to 5×5 km — [Wiley CGF](https://onlinelibrary.wiley.com/doi/abs/10.1111/cgf.13107); [HAL](https://hal.science/hal-01519852). The same group's follow-up, "Data-driven authoring of large-scale ecosystems", is at [ACM TOG 2020](https://dl.acm.org/doi/10.1145/3414685.3417848) (not read in depth).
- **Cluster processes** [SOURCED, standard stats definitions]:
  - **Matérn cluster**: Poisson parents with intensity λ. Each parent has Poisson(μ) offspring, placed uniformly in a disc of radius R.
  - **Thomas**: the same, but offspring have isotropic Gaussian displacement with σ.
  - Both are Neyman–Scott processes.

  Source: [spatstat rMatClust](https://www.rdocumentation.org/packages/spatstat.random/versions/3.3-2/topics/rMatClust); [arXiv 2210.06065](https://arxiv.org/pdf/2210.06065)

### Inferences
- [JUDGEMENT] **Three suppression mechanisms, used by class:**
  1. **Hard exclusion** (Poisson with per-asset radius, big first): tree vs tree, tree vs bush, rock vs rock.
  2. **Soft modulation** (UE "Minimum" / HZD "Inverse"): trees reduce grass density under the canopy to about 30–60% instead of to 0. Ghibli meadows still have grass under trees, just shorter and darker.
  3. **Attraction** (cascade / understory): ferns and shade bushes sample a Thomas cluster centred on placed trees. Offspring distance is in about [0.6, 1.5] × canopy radius, biased to the side facing away from the sun.
- [JUDGEMENT] **Two radii per asset**, mirroring Far Cry 5's viability radius plus priority radius. `radius_same` is the spacing against its own class. `radius_other` is the footprint that lower-priority classes must respect. For example, a tree has radius_same 5 m (canopies don't merge unless clumped) but radius_other 0.8 m (bushes may tuck under the canopy near the trunk).
- [JUDGEMENT] **Edge falloff from polygon SDF**, adapting Far Cry 5's age ramp. With `d` = distance inside the polygon edge:
  - `density *= smoothstep(0, edge_width, d)`
  - `scale *= lerp(edge_scale_min, 1, smoothstep(0, age_depth, d))`
  - Class-specific edge behaviour: flowers and tall grass can **increase** at the forest edge (an ecotone band), while trees taper.
  - **Jitter the edge** with low-frequency noise (amplitude about 1–3 m) so the polygon outline never reads as a straight line. CGMA/Quinn notes "abrupt transitioning and straight lines completely break the immersion" (section 4).
- [JUDGEMENT] **Blending between two adjacent biome polygons**: overlap them and cross-fade densities by SDF, rather than abutting them at a hard seam. Layered dithering (HZD) guarantees no collisions within the overlap for same-footprint classes.

### Gaps
- The HZD video likely covers exact footprint units and how the "Read-back, dependencies" collision works across different footprints. Only the slide text was available.
- Far Cry 5 details come from third-party notes, not the primary slides.
- No sourced numeric guidance was found on how much grass density should drop under canopies.

---

## 3. Masks and rules (slope, height, shore distance, paths/buildings, painting, noise, rule formats)

### Takeaway
Masks should be **2D world-space scalar fields** sampled per candidate: height, slope, distance-to-water, distance-to-path/building, painted density and noise. Each asset has **remap curves** from mask to density or viability. This is the common denominator of HZD (2D world data maps at 0.5–2 m, logic graphs), Far Cry 5 (viability from terrain attributes plus exclusion masks for water, cliffs and roads) and UE PCG (Distance, Difference, Density Remap, Spatial Noise nodes).

### Cited Findings
- HZD world data includes Height_Terrain / Height_Objects / Height_Water (0.5 m), Topo_Roads, Topo_Water and Topo_Objects (0.5 m, generated), Water_Flow, Erosion_Flow / Deposition, Terrain_Cavity, and paintable Placement_* maps (1 m). A logic graph combines Placement_Trees with Topo_Objects, Topo_Roads and Topo_Water into the final density ("Result") — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- Far Cry 5 uses exclusion masks for water, cliffs and roads from earlier tools, and viability from slope, altitude, flow, illumination and so on — [Mills notes](https://christianjmills.com/posts/procedural-tools-far-cry-5-notes/index.html). It also orients grass along a wind vector map and reflects terrain humidity through the grasses — [PlayStation Blog](https://blog.playstation.com/2018/03/22/the-procedural-world-generation-of-far-cry-5/)
- Wildlands excludes vegetation from roads, railways, water and footpaths, and uses flow maps and material matching — [80.lv](https://80.lv/articles/vegetation-generation-in-ghost-recon-wildlands)
- UE PCG Distance node: "calculates the distance to the nearest point". Difference node offers Binary and Minimum modes. Density Remap: D' = (OutMax−OutMin)(D−InMin)/(InMax−InMin)+OutMin — [UE PCG node reference](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-framework-node-reference-in-unreal-engine)
- ProtonScatter supports negative (exclusion) shapes and project-on-colliders — [GitHub](https://github.com/HungryProton/scatter)

### Inferences
- [JUDGEMENT] **Seaside rule set** (starting points to tune by eye). d_shore = horizontal distance to the waterline or sand boundary.
  - **Wet sand / tide zone**: no vegetation. Only shells and driftwood decals.
  - **Dune grass (marram-like)**: d_shore 3–15 m on sand, clumped (Thomas σ about 0.6 m, 6–15 per clump). Density peaks around 6–10 m and fades inland as meadow grass takes over.
  - **Reeds**: along freshwater (stream or pond edges), d_water 0–2 m, and slightly in-water up to about 0.3 m depth. Linear clusters along the bank, created by sampling offspring along the shoreline tangent rather than in a disc.
  - **Wind-bent coastal shrubs / pines**: d_shore 10–30 m, lean or rotation biased away from the prevailing wind. Far Cry 5's wind vector map makes this a principled choice.
  - **Meadow flowers**: flat (slope < 15°) and sunny. Avoid within 1.5 m of paths so paths stay readable. Boost in clumps near fences and walls (garden-edge "cozy" read).
  - **Trees**: slope < 25–30°. Exclude within about 3–4 m of buildings (canopy clearance) and 2 m of paths. Exclude from farm plots entirely.
  - **Rocks**: prefer slope > 20° and shore lines. Partially bury them (y offset −10 to −30% of height).
- [JUDGEMENT] **Exclusion shapes**: paths as Curve3D/Path3D with a width, and buildings as footprint polygons plus a margin. Rasterise them into a **distance field** at 0.25–0.5 m per chunk, so every rule can use a soft falloff (`smoothstep(margin, margin+feather, d)`) instead of a binary cut. Paths lined with slightly *denser* grass and flowers read as trodden and cozy. A binary cut reads as a CAD boolean.
- [JUDGEMENT] **Painted density**: a per-polygon (or per-level) low-resolution image (0.5–1 m/px) that multiplies all classes, or one channel per class group (trees / bushes / ground / flowers), like HZD's per-group Placement_* maps. Painting a single tree channel and decoding bands (sparse edge → edge → inner), as HZD does, is a powerful and simple authoring trick.
- [JUDGEMENT] **Rule format**: serialize as a Godot `Resource` (`.tres`) so it is diffable and inspector-editable. Each rule is `{mask: enum, remap: Curve, op: multiply|min|max|replace}`. Evaluate as a product of remapped masks times painted density times noise. Godot's `Curve` resource matches UE's Density Remap but is non-linear.

### Gaps
- No sourced shoreline-specific vegetation rules for games were found. The seaside rule set above is judgement informed by coastal dune zonation in general.

---

## 4. Art direction (Ghibli / painterly composition, clumps and clearings, readability; Sea of Thieves, BotW, Sable, Nintendo)

### Takeaway
The sourced art-direction guidance agrees on the following:
- Place vegetation **in collections, not individually**.
- Mix mid-height and low plants for silhouette contrast.
- **Cluster** instead of sprinkling.
- Avoid **straight lines and abrupt transitions**.
- Use a **big / medium / small hierarchy** for what blocks the view (BotW triangles).
- Use colour-coded foliage and marker plants to steer the player (Sea of Thieves).

A cozy farm game adds the need for **readable open play space**. Clearings and paths are first-class shapes, not leftovers.

### Cited Findings
- **Sea of Thieves (Rare)**:
  - Assets "should always be tested and placed in collections rather than individually". Trees are tested with bushes and rocks for "specific sympathetic relationships".
  - "Mid height plants work with low level flowers to bulk out areas with their differing overall and internal silhouettes".
  - Foliage colour and shape were used to aid gameplay, with island areas themed to specific colour schemes.
  - A red fern was the main "marker" foliage to attract player attention.

  These come from a search summary of Lee Piper's ArtStation breakdown; the page returned 403, so the quotes were not verified verbatim — [Lee Piper – Sea of Thieves: building the environment](https://www.artstation.com/artwork/bKP0Kg). For broader art-direction rules, see the GDC 2018 talk by Ryan Stevenson — [GDC Vault](https://gdcvault.com/play/1025015/Visual-Adventures-on-Sea-of); [80.lv summary](https://80.lv/articles/gdc18-visual-adventures-on-sea-of-thieves)
- **BotW "triangle rule"** (CEDEC 2017): three triangle sizes. "The largest are landmarks that serve as visual markers, the medium-sized triangles serve to obstruct the player's view … and the smallest triangles serve the tempo." Topography and "gravity" pull players toward points of interest — [Nintendo Life](https://www.nintendolife.com/news/2017/10/zelda_breath_of_the_wilds_ingenious_design_is_all_about_triangles_apparently); [Game Developer](https://www.gamedeveloper.com/design/5-design-lessons-learned-from-i-the-legend-of-zelda-breath-of-the-wild-i-); [Radiator Blog analysis](https://www.blog.radiator.debacle.us/2017/10/open-world-level-design-spatial.html)
- **Foliage placement craft (Fernando Quinn, CGMA)**: "pay attention to how objects tend to cluster in nature … when placing rocks and grass". "abrupt transitioning and straight lines completely break the immersion". In dense scenes, "it is really difficult to notice repetition if you rotate the trees before placing them next to one another" — [CG Master Academy](https://www.cgmasteracademy.com/blog/5-ways-to-make-your-foliage-filled-scene-believable/)
- HZD's clearing logic ("Clearing", "Inverse" of the tree map feeding undergrowth) and its sparse-edge → edge-forest → inner-trees banding show that clearings and edges are authored shapes in a AAA pipeline — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)

### Inferences
- [JUDGEMENT] **Ghibli / HM:AWL composition rules to encode:**
  1. **Few big masses, not many small ones.** Group trees into 2–5-tree clumps (Matérn, R ≈ 1–1.5 × canopy diameter) with a lone "hero" tree now and then. Trees in a clump may overlap canopies (radius_same inside a clump about 0.5–0.7 × canopy radius) so they read as one soft painted mass. That matches the "few soft masses" tree construction in the tree report.
  2. **Clearings as negative shapes.** Carve 1–3 low-frequency noise-threshold holes (or authored hole polygons) per biome polygon, each at least 6–10 m across. Grass and flowers fill them; trees and bushes do not. At a 45° camera, 5–40 m, the ground plane dominates the frame, so clearings are what make it read as painted rather than carpeted.
  3. **Big–medium–small hierarchy** (BotW triangles mapped to foliage). Trees are landmark or occluder, bushes break sightlines and frame paths, and flowers and ferns set the tempo along routes.
  4. **Hug edges.** Bushes and flowers cluster against rocks, fences, tree bases and building walls (attraction rules), not in open field centres.
  5. **Colour-zoned flowers.** One or two flower species per patch (a Thomas cluster with a large σ for drifts). Use a low-frequency noise to pick the species per patch rather than per instance. Mixed confetti per instance is the classic "sprinkled" failure.
  6. **Readable gameplay space.** The farm plot, paths, doorways and interactable approaches get hard exclusion. The tall-vegetation classes (trees, big bushes) get a larger camera-side margin, because at 45° a tall object *south* of a point of interest (toward the camera) occludes it. So far no sourced reference covers asymmetric margins like this. It is purely judgement, but it follows directly from the fixed camera.
- [JUDGEMENT] **Avoid "sprinkled" by construction.** Every non-grass class should be generated as clusters (Matérn/Thomas) whose parents are Poisson with low-frequency noise-modulated intensity. Only grass should use near-uniform blue noise, and even grass should have density modulated by noise at 5–15 m wavelength.

### Gaps
- No public, citable breakdown was found on vegetation placement rules in Sable, Kirby, Animal Crossing, or Harvest Moon / Story of Seasons. Nothing was found that documents Ghibli background-painting composition as rules.
- The Sea of Thieves claims come from a search snippet; the source page could not be fetched (HTTP 403).

---

## 5. Per-instance variation (rotation, scale, tint, neighbour de-duplication)

### Takeaway
Variation should come from a per-point hash, not sequential RNG: HZD derives randomness from the "pattern idx/id", and UE PCG mutates seeds by position. That way, re-running or local edits don't reshuffle rotations and scales. Randomized Y rotation hides repetition, and Wildlands favoured more variants of fewer species.

### Cited Findings
- HZD PLACEMENT step: "Needs pattern idx/id for RNG / Basis generation / Bounding box generation" (about 7 µs) — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- UE PCG seeds mutate by position, previous seed, node seed and component seed — [UE PCG node reference](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-framework-node-reference-in-unreal-engine)
- Rotating trees placed next to each other hides repetition — [CGMA / Quinn](https://www.cgmasteracademy.com/blog/5-ways-to-make-your-foliage-filled-scene-believable/)
- Wildlands used multiple variations ("steps") per species rather than many species, for memory — [80.lv](https://80.lv/articles/vegetation-generation-in-ghost-recon-wildlands)
- UE Self Pruning's "randomized pruning to prevent patterns from emerging" — [UE PCG](https://dev.epicgames.com/documentation/en-us/unreal-engine/procedural-content-generation-framework-node-reference-in-unreal-engine)
- Far Cry 5 ties size to "age" from distance-to-border — [Mills notes](https://christianjmills.com/posts/procedural-tools-far-cry-5-notes/index.html)
- Per-instance tint and wind phase go in MultiMesh colour and custom data (from the tree report) — [wave_forge PR](https://github.com/AntonTegnelov/wave_forge/pull/158)

### Inferences
- [JUDGEMENT] **Per instance:**
  - **Yaw**: full 0–360° for bushes, grass and flowers. For trees, restrict yaw if the variants have an authored "front" facing the 45° camera. Otherwise use a full range.
  - **Tilt**: 0–4° for trees; up to 8–10° for grass, aligned partly to the terrain normal (grass 0.7, trees 0.1). Trees should stay near-vertical.
  - **Scale**: log-uniform. Trees 0.85–1.2, bushes 0.8–1.25, grass 0.7–1.3, flowers 0.8–1.15.
  - **Size correlation**: multiply scale by the edge "age" factor and by a low-frequency noise, so neighbours are similar sizes (nature has cohorts). Otherwise the result reads as random noise.
  - **Tint**: jitter hue ±3–5°, value ±5–8%. Also apply a *spatially coherent* tint noise (5–20 m wavelength) so colour patches read as painterly washes. HZD had dedicated Variance_Foliage_Color maps at 1 m, which supports making tint a map rather than purely per-instance random [SOURCED existence of the maps; the painterly rationale is judgement].
- [JUDGEMENT] **Variant de-duplication** (10 tree variants): on assignment, pick variant = weighted draw with the weight of any variant used by the k = 2–3 nearest already-placed neighbours reduced by 80–90%. This is deterministic because placement order is deterministic (cell-sorted). A cheaper alternative is to hash the variant from the cell coordinate with a Latin-square offset. `TreeSet.pick(key)` should accept that hash as `key`.
- [JUDGEMENT] **Diversity check**: reuse the Vendi-score idea from the tree report at scene level. Render top-down 32 m crops and count "effective distinct crops". This is optional and unproven for scatter.

### Gaps
- No sourced numeric ranges for tint or scale jitter in stylized games were found. All ranges above are judgement.

---

## 6. Budget (density per m² per class; instance counts at 45°, 5–40 m)

### Takeaway
No studio publishes per-m² densities for a cozy game. The best sourced anchors are HZD's footprints (trees 6 m, bushes 3 m, undergrowth 1 m) and Ghost of Tsushima's blade counts, which are realistic-grass-blade scale and not comparable to clump meshes. Budget by footprint and by visible-area-per-frame, then enforce with chunked MultiMesh plus visibility ranges.

### Cited Findings
- HZD footprints: trees 6.0, bushes 3.0, undergrowth 1.0 (units presumed metres). HZD scene total: 100,000+ objects, 500+ asset types, about 250 µs GPU placement — [HZD slides](https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf)
- Ghost of Tsushima grass: about 83,000 blades on screen in about 2.5 ms. Distant tiles are twice as large with the same blade count, and 3 of 4 blades are dropped at the LOD switch. Tiles carry height, grass type, clumping factor, blade size and wind. These figures come from secondary summaries of the GDC 2021 Wohllaib talk and have not been checked against the primary — [tigerabrodi blog](https://tigerabrodi.blog/grass-in-ghost-of-tsushima); talk at [GDC Vault](https://gdcvault.com/play/1027033/Advanced-Graphics-Summit-Procedural-Grass)
- Wildlands tightened LOD distances more aggressively in dense areas — [80.lv](https://80.lv/articles/vegetation-generation-in-ghost-recon-wildlands)
- Godot: chunk per species into MultiMeshInstance3D of about 32–64 m, because visibility range is per node, not per instance (from the tree report) — [Godot visibility ranges docs](https://docs.godotengine.org/en/latest/tutorials/3d/visibility_ranges.html)

### Inferences
- [JUDGEMENT] **Starting densities for a stylized seaside village** (clump meshes, not blades):

| Class | Typical spacing (Poisson r) | Density in its zone | Notes |
|---|---|---|---|
| Trees | 5–7 m (0.5–0.7× canopy inside clumps) | 0.005–0.02 /m² (1 per 50–200 m²) | clumped 2–5, clearings carved |
| Large bushes | 2.5–3.5 m | 0.02–0.06 /m² | attracted to tree bases, walls, rocks |
| Ferns / small shrubs | 1–1.5 m | 0.1–0.3 /m² in patches | Thomas clusters under canopy |
| Grass clumps (each a 20–60-blade card clump) | 0.35–0.6 m | 3–8 /m² | blue-noise tile + density map |
| Flowers | 0.25–0.5 m within drifts | 2–10 /m² inside drift, ~0 outside | 1–2 species per drift |
| Rocks | 1.5–4 m | 0.005–0.03 /m² | shore/slope biased, half-buried |

- [JUDGEMENT] **Visible-area estimate.** A fixed 45° camera at 5–40 m distance sees roughly 20×15 m of ground near and about 120×70 m far. The visible ground area is on the order of **1,500–6,000 m²**. At 5 grass clumps/m² that is **7,500–30,000 grass instances**, which is fine for MultiMesh with a vertex-shader wind. Grass beyond about 30–40 m should thin (Tsushima-style 1/4 drop) or be replaced by a terrain colour and texture. Trees at 0.01/m² over 6,000 m² number about 60 on screen, which is trivial.
- [JUDGEMENT] **Enforce budgets in the tool.** Show live instance counts per class and per chunk, plus a per-polygon estimate (area × mean density). Warn above configurable caps, for example 20k grass per 32 m chunk.

### Gaps
- No sourced per-m² densities exist for Harvest Moon/Story of Seasons/Animal Crossing-style games.
- The Tsushima numbers have not been verified against the primary talk.
- No Godot 4.7 MultiMesh vegetation benchmarks were found.

---

## 7. Recommended algorithm stack and bag data model [JUDGEMENT, synthesised from the sourced patterns above]

This section is folded into the structure required by the note format. It is all engineering judgement.

### Algorithm stack (editor-time, deterministic, output baked to chunked MultiMesh)
1. **Input.** Biome polygon: `PackedVector2Array` outer ring plus hole rings. Optional painted density texture. Global exclusion sources: paths (Path3D + width), buildings (footprint polygons), farm plots, water / shoreline curves. `world_seed`.
2. **Field prep, per 32 m chunk intersecting the polygon.** Rasterise signed distance to the polygon edge (with noise-jittered edge), to shore, to water, to paths, and to buildings, at 0.5 m. Sample terrain height and slope. Cache the rasters and invalidate only chunks touched by an edit, which gives local changes only.
3. **Layer order** (HZD / Wildlands / UE "Large to Small"): rocks → trees → large bushes → small shrubs / ferns → flowers → grass.
4. **Per layer, per world cell** (cell = 2× the layer's max radius):
   1. Build a deterministic candidate set:
      - Big layers: cluster parents from hashed Poisson; offspring by Matérn/Thomas; per-asset Poisson reject against the occupancy grid.
      - Ground layers: a pre-baked tileable blue-noise tile with progressive ranks (Kopf/HZD), with `rank < density` as the keep test.
   2. Compute density = painted × Π(mask remaps) × low-freq noise × edge falloff × (soft suppression from earlier layers' footprints).
   3. Accept a candidate if `hash01(point_id) < density` for cluster/Poisson layers, or `rank < density` for tile layers.
   4. Apply hard exclusion against the occupancy grid with `radius_other` of earlier layers and `radius_same` within the layer. Earlier layers claim; later layers only read, so there are no dependencies backwards.
   5. Handle attraction layers (ferns near trees, reeds along banks) by seeding cluster parents *at* earlier-layer points or along curves, instead of Poisson parents.
5. **Variant pick.** Weighted by the bag, with the neighbour de-duplication penalty. The key is `hash(world_seed, round(pos*100), layer_id)`, and it is passed to `TreeSet.pick(key)`.
6. **Transform.** Yaw, tilt, scale (× edge age × cohort noise), terrain snap (raycast or height sample), and burial offset. Tint goes into the MultiMesh colour and wind phase into custom data.
7. **Output.** One MultiMeshInstance3D per (asset variant × 32 m chunk). Set visibility ranges per class (for example grass end at 35–45 m with dither fade, flowers 40–60 m, trees none, since the camera never sees far). Store the generated transforms in the scene (baked) and keep the recipe in a `.tres`, so git diffs stay small and regeneration is reproducible.
8. **Editor UX.** Regenerate only dirty chunks. Show a per-class count overlay. Include a "lock" or "stamp" option to convert an instance to hand-placed, which is then treated as an occupancy source for later regenerations.

### Bag data model (Godot Resources)
```gdscript
# BiomeBag.tres
class_name BiomeBag extends Resource
@export var layers: Array[BagLayer]          # evaluated in array order = priority (big → small)
@export var edge_width_m := 3.0              # density falloff inside polygon edge
@export var edge_jitter_m := 2.0             # noise amplitude on the outline
@export var clearing_noise: FastNoiseLite    # thresholded to carve clearings
@export var clearing_threshold := 0.65
@export var tint_noise: FastNoiseLite        # low-freq painterly colour patches

# BagLayer.tres
class_name BagLayer extends Resource
@export var name := "trees"
@export_enum("poisson_cluster", "blue_noise_tile", "attach_to_layer", "along_curve") var method := 0
@export var entries: Array[BagEntry]
@export var base_density := 0.01             # per m² before masks
@export var density_noise: FastNoiseLite     # modulates intensity (5–15 m wavelength)
@export var cluster_radius_m := 4.0          # Matérn R (or Thomas sigma)
@export var cluster_count := Vector2i(2, 5)  # offspring per parent
@export var attach_layer := ""               # for understory: parent layer name
@export var attach_dist := Vector2(0.6, 1.5) # × parent footprint
@export var suppress_below := 0.4            # soft multiplier this layer applies to later layers inside its footprint (1 = none, 0 = hard)
@export var rules: Array[MaskRule]           # multiplied together
@export var edge_behaviour := Curve.new()    # density vs normalized edge distance (can bump up at ecotone)
@export var edge_scale := Curve.new()        # scale vs edge distance (FC5 "age" taper)
@export var visibility_end_m := 0.0          # 0 = no culling

# BagEntry.tres (one asset or asset-set)
class_name BagEntry extends Resource
@export var scene_or_set: Resource           # PackedScene / Mesh / TreeSet
@export var weight := 1.0                    # relative within layer
@export var radius_same_m := 5.0             # spacing vs own layer
@export var radius_other_m := 1.0            # footprint that later layers respect
@export var scale_range := Vector2(0.85, 1.2)
@export var yaw_range_deg := Vector2(0, 360)
@export var tilt_max_deg := 3.0
@export var normal_align := 0.1
@export var sink_m := Vector2(0, 0)          # burial range
@export var tint_jitter := Vector3(0.01, 0.0, 0.06)  # h, s, v
@export var neighbour_dedupe := 0.85         # weight penalty if a close neighbour uses same variant
@export var rules: Array[MaskRule]           # per-entry viability (FC5-style), optional

# MaskRule.tres
class_name MaskRule extends Resource
@export_enum("slope_deg", "height", "dist_shore", "dist_water", "dist_path", "dist_building", "dist_edge", "painted", "noise", "sand_mask") var source := 0
@export var remap: Curve                     # source (normalized by range) → multiplier 0..1
@export var range := Vector2(0, 20)
@export_enum("multiply", "min", "max") var op := 0
```

- Weights inside a layer are exact proportions when using the HZD layered-dither idea: split the rank interval by cumulative weight. For cluster layers, they are expectations.
- Keeping `radius_same` and `radius_other` separate is the Far Cry 5 priority-radius idea. `suppress_below` is UE's "Minimum" difference mode and HZD's "Inverse" clearing logic.
- A preset bag library for the seaside village might include: `beach_dune` (dune grass clusters, sparse shore rocks, driftwood), `coastal_scrub` (wind-leaning shrubs and pines, lean from a wind vector), `meadow` (grass tile, flower drifts, rare lone tree), `orchard_edge` (tree clumps, fern understory, bushes on walls), and `stream_bank` (reeds along the curve, rocks, ferns).

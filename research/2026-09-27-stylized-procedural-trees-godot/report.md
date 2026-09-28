# Grow painterly trees from parameters, not photos

The trees that fit a Ghibli / Harvest Moon: A Wonderful Life / Ni no Kuni look are not detailed trees simplified. They are built on purpose from a few soft masses. The recommended construction is a **hybrid canopy**: an opaque, SDF-fused blob core that holds the silhouette and the shadow colour, a thin fringe of painted leaf-cluster cards on the sun-facing edges, and **normals projected from a smooth proxy** so the toon terminator sweeps across each clump like a brush stroke instead of fizzing across hundreds of cards. That construction is exactly what an image-to-3D model such as Hunyuan3D cannot deliver. A remeshed scan has no clump structure, no proxy normals, no wind masks and no LOD story, which is the most likely reason those trees looked poor. All five archetypes (round deciduous, conifer, palm, bush, starfruit hero) should come from **one custom, JSON-driven Python + Geometry Nodes generator** in headless Blender 5.2. It uses Blender 5.x's new SDF grid nodes to fuse canopy puffs, bakes wind and AO data into vertex colour, and exports glTF. In Godot 4.7, spherized normals should be recomputed in the vertex shader rather than trusted to the glTF import. Wind comes from a global uniform plus per-instance phase. Trees are instanced through chunked MultiMeshes with visibility-range LODs. Only the canopy blob core and trunk of interactables (the starfruit tree) get the inverted-hull ink. Evaluation should be tiered. Hard geometric gates and silhouette metrics go first. Then embedding similarity to concept art (DreamSim, CSD with a CSLS readout). Parameter search (Optuna/CMA-ES, or MAP-Elites for a varied forest) comes next. A local VLM is used only as a **pairwise, order-swapped tournament judge**, never as a 1–10 scorer, which matches both the team's earlier finding and the 2026 literature. Many of the specific numbers below (clump counts, band widths, card overhang, distance bands) are engineering judgement, not published studio rules. The public record on Ghibli, Harvest Moon or Ni no Kuni tree construction is essentially empty, so treat these numbers as starting targets to calibrate against the team's own concept art.

## Art direction: paint the canopy as a handful of soft masses, not as leaves

### Oga's two-brush method is the design brief

The most useful primary source for the look is Kazuo Oga, the background painter on Totoro, Mononoke and Spirited Away. He paints wet-into-wet to get a **"soft atmospheric base into which he can place smaller details of branches and leaves,"** with large soft passages first and details last ([Gurney Journey](http://gurneyjourney.blogspot.com/2017/03/demo-by-kazuo-oga.html)). He uses essentially two brushes. The flat hira-fude does everything rough, and he "paints leaves roughly with hira-fude and adds a few detailed leaves on it" ([Gurney Journey](http://gurneyjourney.blogspot.com/2011/02/kazuo-oga.html)). He describes his goal as getting "the essential point across effectively" rather than rendering comprehensively ([Animation Obsessive](https://animationobsessive.substack.com/p/what-kazuo-oga-thinks-about-when)). Ghibli-style painting guides say the same thing: cluster leaves into bunches that convey form, and manage where cloud edges meet tree edges so foreground and background stay distinct ([GVAAT](https://gvaat.com/blog/how-to-paint-ghibli-backgrounds/)).

In 3D terms, the flat brush is the **blob core** and the "few detailed leaves" are the **card fringe**. This is also how the documented Ghibli-inspired game projects were built. David Holland's "Meadows" used simple Sapling trees with spheres on the branches, sculpted into canopy shapes and then scattered with cards ([80.lv](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)). Kids With Sticks studied Totoro, Mononoke, Spirited Away and Nausicaä and landed on intersecting planes over spheres ([Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/)).

### Silhouette rules the generator must encode

No source gives explicit clump counts or sky-hole rules, so the following are design targets derived from Oga's big-then-small hierarchy and ordinary big/medium/small composition practice.

A mid-size deciduous canopy should have **one dominant mass, two to four secondary masses, and three to eight small accent puffs** on the silhouette edge, at roughly a 3:2:1 size ratio. Equal, evenly spaced puffs read as broccoli. The envelope should lean toward the light and have a flatter, darker underside, giving the stacked "cloud-shelf" profile typical of Ghibli trees. Punch **one to three deliberate, irregular sky holes** of about 10–20% of canopy width so they survive at 40 m. Keep tiny holes for the card fringe only. The trunk needs one clear S- or C-gesture, a base about 2–3× the diameter at the first fork, a root flare into the ground, and one to three primary branches that visibly disappear *into* clumps. The trunk showing through sky holes and under the canopy is a key Ghibli signifier.

Rare's Sea of Thieves rules give the species-level principle. Each asset needs a differentiated silhouette within and across species, trees should be tested next to bushes and rocks for "sympathetic relationships," and structures should be "realistically wonky" ([Habrador summary of GDC talks](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html); [GDC Vault](https://gdcvault.com/play/1025015/Visual-Adventures-on-Sea-of)). Fortnite's rule of removing all parallel lines from references is a useful generator constraint: no perfectly vertical trunks, no evenly spaced tiers ([Habrador](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html)). Separate the five archetypes by **silhouette class first and colour second**: round dome, tiered spire, arched fan crown, low mound, and a distinctive drooping umbrella for the starfruit hero.

### Two value families, warm light and saturated cool shadow

The core of the Ghibli read is **separation of the light family from the shadow family**. Darks recede and suggest canopy interior, lights advance, and temperature and saturation carry mood ([GVAAT](https://gvaat.com/blog/how-to-paint-ghibli-backgrounds/)). Oga worked from about **21 poster colours** and argued that subtle mixing of few colours yields a wide range ([Animation Obsessive](https://animationobsessive.substack.com/p/what-kazuo-oga-thinks-about-when)). That argues for a restricted per-biome palette with small jitter, not free per-tree hues.

Harry Alisavakis's stylized leaves shader dropped its extra shadow band as "barely visible (or possible useful) in most cases" ([Harry Alisavakis](https://halisavakis.com/my-take-on-shaders-stylized-tree-leaves/)). That supports **two main bands plus one accent**: a sun-side highlight cap, or a rim/translucency glow when backlit. Three hard bands on leaf geometry tend to fragment.

The painterly shader already has lavender shadows, so the practical reconciliation is this. Let the lavender ambient tint the shadow band, but keep foliage shadows **saturated and not black**, landing on a cool teal-to-blue-green after the lavender multiply. Hue-shift the lit side toward warm yellow-green.

Stack two gradients under the ramp. The first is a vertical base-to-tip gradient: darker and cooler at the canopy underside, warmer at the tips. This is the vertex-colour trick reported for BotW ([Polycount snippet](https://polycount.com/discussion/209623/smooth-foliage-like-in-breath-of-the-wild-europa-by-helder-pinto-mini-tutorial), unverified) and the "highlight height mask" Kids With Sticks used ([Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/)). The second is a radial interior-darkening AO. aVersion of Reality's stylized tree shader adds per-island random values so each cluster varies, AO used both to darken and to brighten, and distance-from-camera masks ([aVersion of Reality](http://www.aversionofreality.com/blog/2022/8/7/stylized-tree-shader)). Keep per-clump jitter at about ±3–6% value and a few degrees of hue, with per-tree tint from an instance hash.

### Hybrid canopies win for Ghibli; solid blobs win for Harvest Moon

The three construction families trade clearly:

| Approach | Look | Cost | Best fit here |
|---|---|---|---|
| Solid sculpted/fused blob | Clean, toy-like, reads with flat colour and outlines | Lowest, no overdraw | Bushes, far LODs, AWL-flavoured props |
| Pure alpha cards | Fluffiest, most painterly | Heavy overdraw, noisy without normal editing | Rare hero close-ups only |
| Hybrid blob core + card fringe | Painterly broken edge, solid read | Moderate; core culls most cards | Deciduous, starfruit, conifer near LOD |

Simon Trümpler's analysis of the Airborn trees shows why the hybrid works. The big central blob **"culls most" of the transparent planes**, so "you only need to care about the stuff of the front/sides," and normals are projected from the base sphere onto the leaf planes ([simonschreibt.de](https://simonschreibt.de/gat/airborn-trees/)). An anime foliage pipeline in Blender does the same with camera-facing planes instanced over a subdivided emitter, normals taken from the emitter, and an "inflate" step ([Trung Duy Nguyen](https://trungduyng.substack.com/p/tutorial-blender-anime-foliage-pipeline)). Tunic's early prototypes went the other way, "entirely flat colour on low poly geometry, relying mostly on lighting" ([Wireframe](https://wireframe.raspberrypi.com/articles/wireframe-cover-star-tunic-and-the-art-of-keeping-a-secret)). That solid-blob family fits the Harvest Moon: AWL side of the brief.

Because the reference mix spans both, **make the card fringe a parameter** (`fringe_density` 0 → pure blob, 1 → full hybrid) so art direction can dial it per species and per distance band.

Fringe rules (inference) are as follows. Cards overhang the blob by about 10–30% of local clump radius, on the outer shell only. They concentrate on upper and sun-side edges, and the underside stays a clean dark plane. Each card's alpha should be **a cluster of three to seven painted leaf dabs**, not single leaves or photo sprays, so the edge reads as brushwork and mipmaps stay clean. Tint the blob core with the shadow colour so gaps between cards read as depth, not holes. With a fixed 45° camera, partial or Y-locked billboarding avoids the "swimming" of full billboards.

### Proxy normals are the single most important shading trick

Every credible breakdown converges on **normal transfer from a smooth proxy**. Without it, "the rotation of the cards will make ugly, sudden changes in the shading" ([simonschreibt.de](https://simonschreibt.de/gat/airborn-trees/)). Meadows transferred a tree-enclosing sphere's normals to the cards with Blender's Data Transfer modifier and flipped back-face normals so both sides light identically ([80.lv](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)). Kids With Sticks note that the technique "has existed long before modern game development" and "still works great for stylized assets" ([Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/)). Habrador's "Witness-style" Blender recipe does the same with a Normal Edit modifier aimed at a sphere ([Habrador](https://blog.habrador.com/2018/05/how-to-make-stylized-witness-trees-in.html)). Alisavakis dropped normal maps on leaves entirely because "normal maps were just adding noise," and warns that rim terms produce "weird cutoff artifacts" on procedural trees ([Harry Alisavakis](https://halisavakis.com/my-take-on-shaders-stylized-tree-leaves/)).

For this project the proxy choice sets the look. One sphere per tree gives a single painted gradient. **Per-clump proxies** give each puff its own light and shadow, which is closer to Ghibli cloud-shelves. A blend of about 50–80% per-clump and the rest whole-tree is a good default. The brush-noise terminator in the painterly shader will look best when it rides on these smooth proxy normals. Anchor its noise in object or world space so it does not swim as the camera moves. Soften self-shadows on cards, and drive AO from baked vertex colour rather than SSAO, since shadow-map acne on cards is a major noise source (inference).

### Ink belongs on the blob core, and should fade with distance

Toon character outlines are typically inverted hulls. The Genshin recreations use Solidify with negative thickness and flipped normals ([Ben Ayers](https://bjayers.com/blog/9oOD/blender-npr-recreating-the-genshin-impact-shader)). Outlining alpha cards outlines every quad. Sable is the documented environment-outline case. It fades outline opacity with distance to emphasise perspective and hide pop-in, and calls fog "really, really key" for mid- to long-distance readability ([Game Developer](https://www.gamedeveloper.com/marketing/how-shedworks-refined-the-art-of-sable-in-pursuit-of-readability)).

The team's split is already right: painterly shader for environment trees, cel+ink for interactables. Extend it this way. Environment trees get **no ink**, separated by value, rim and fog. The starfruit hero, as an interactable, gets an inverted hull on its **blob core and trunk only**, tinted toward a dark foliage colour rather than black and faded with distance. A smooth fused blob produces a clean cloud-edge contour, which is exactly the Ni no Kuni storybook read.

### Distance readability comes from simplifying toward masses

Kids With Sticks followed Ghibli's own hierarchy. Background trunks get "one or two colors and slight detail," while foreground trunks are "intricately detailed, stratified" ([Kids With Sticks](https://kidswithsticks.com/creating-stylized-art-inspired-by-ghibli-using-unreal-engine-4/)). Sea of Thieves avoided granular texture noise ([Habrador](https://blog.habrador.com/2018/08/stylized-graphics-fortnite-sea-of-thieves.html)).

For a 45° camera at 5–40 m, a workable banding (inference) is:

- **5–15 m:** full hybrid with visible dab alpha and bark strokes.
- **15–30 m:** fewer, larger merged cards with fringe only on the silhouette.
- **30 m and beyond:** blob-only or impostor, values compressed toward fog colour.

Because cards and blob share the same proxy normals, shading is identical across swaps, which hides pops. At forest scale, group canopies into masses with clearings, keep tint variation low-frequency, push distant rows cooler and lighter, and avoid high-contrast tiny sky holes in background trees.

## Procedural generation: a custom JSON-driven generator beats every off-the-shelf tool

### Curve skeletons plus fused puffs suit stylized trees; realistic growth algorithms mostly do not

Weber-Penn is what Blender's Sapling Tree Gen implements, and it is maintained as an extension ([Blender Extensions](https://extensions.blender.org/add-ons/sapling-tree-gen/)). It has roughly 40 parameters per level and emits many thin branches. That is useful only at one or two levels with a custom canopy on top. Space colonization (Runions et al. 2007) grows branches to fill a given hull. A **GPL-3.0 Blender extension (v1.0.0, July 2025, Blender 4.2+)** implements it ([Blender Extensions](https://extensions.blender.org/add-ons/space-colonization-tree-generator/)), and jomiq/space_col is a numpy version ([GitHub](https://github.com/jomiq/space_col)). It is the right tool for the **hero tree**, where the branches must reach an art-directed canopy shape. L-systems take about 150 lines of Python and suit bushes and palm frond rachises, but they are unintuitive for silhouette control. Light- and gravity-driven growers like The Grove are the most realistic and least controllable.

The default for everything else should be a **curve trunk with taper and recursive child curves**. It works like this: build a noise-offset Bezier trunk, run Curve to Mesh with a 6–8-sided profile and radius falloff along the spline, then spawn children at N points with angle and length decay for two or three levels. Branch tips become canopy puff centres. It is fully deterministic from a seed plus JSON. Blender Studio's official Geometry Nodes course includes a curve-based tree generator chapter as a first-party example of the pattern ([Blender Studio](https://studio.blender.org/training/geometry-nodes-from-scratch/example-tree-generator/)).

### Blender 5.x SDF grid nodes make fused cloud-puff canopies a headless one-liner

Blender 5.0 added a volume-grid socket type and **27 OpenVDB-based grid nodes**, including SDF booleans and SDF Grid Mean smoothing ([Blender Developers Blog](https://code.blender.org/2025/10/volume-grids-in-geometry-nodes/)). The 5.2 LTS manual lists SDF Grid Boolean, SDF Fillet, SDF Grid Mean/Median, SDF Grid Offset, Grid to Mesh and Volume to Mesh ([Blender 5.2 manual](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/volume/index.html)). Points to Volume "generates a fog volume sphere around every point" and pairs with Volume to Mesh ([Blender 5.2 manual](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/point/points_to_volume.html)).

The canopy recipe is:

1. Give puff centres a radius attribute (0.6–1.2 × base).
2. Convert them to an SDF or density grid.
3. Run SDF Mean/Median or Fillet for one to three iterations to melt the seams.
4. Optionally apply SDF Offset.
5. Mesh at a voxel size of about canopy-diameter/20.
6. Merge by distance, decimate to 300–1,500 tris, and shade smooth.

The result is a single watertight surface with outward normals, which is exactly the toon-friendly blob. The exact Python identifiers for the mesh/points-to-SDF nodes were not confirmed, so introspect `bpy.types` in 5.2 before hard-coding them. Metaballs and voxel remesh plus smooth are version-proof fallbacks.

### The whole thing runs headless

Geometry Nodes groups can be built from Python (`bpy.data.node_groups.new(..., 'GeometryNodeTree')`), attached through a NODES modifier, and evaluated or applied under `blender -b --python` ([CGWire](https://blog.cg-wire.com/blender-scripting-geometry-nodes-2/)). **NodeToPython (GPL-3.0, v4.2.0) supports Blender 4.2 through 5.2** and converts a hand-built node graph into readable Python for version control ([GitHub](https://github.com/BrendanParmer/NodeToPython)).

The maintainable pattern is to author the GN group once in a `tree_gn.blend`, or generate it via NodeToPython. Then each headless run loads the group, sets modifier inputs from JSON (socket IDs from `node_group.interface.items_tree`), calls an update, applies, and exports glTF. One known wrinkle: modifier input changes from Python historically did not trigger an update, and touching the object or calling `interface_update` is the workaround ([Blender bug #87006](https://developer.blender.org/T87006)). Route a single `seed` into every Random Value and Distribute node. The Blender MCP server was down during this research, so none of this was executed live against 5.2.

### Tools and licences

| Tool | Licence | Status | Headless | Verdict for this project |
|---|---|---|---|---|
| Custom Python + GN generator | Yours | n/a | Full | **Primary** for all five archetypes |
| Sapling Tree Gen | GPL-3.0+ | 0.3.7, Dec 2025, Blender 4.4+ ([versions](https://extensions.blender.org/add-ons/sapling-tree-gen/versions/)) | Operator call | Optional skeleton variety for deciduous/conifer |
| Space colonization ext. | GPL-3.0+ | v1.0.0, Jul 2025 ([page](https://extensions.blender.org/add-ons/space-colonization-tree-generator/)) | Operator call | Hero (starfruit) skeleton |
| MTree forks | GPLv3 addon / MIT C++ core | Active forks for 4.2+/4.3+ ([ethanporcaro](https://github.com/ethanporcaro/modular_tree_py311), [GoodPie](https://github.com/GoodPie/modular_tree)) | Custom node tree; 5.x unverified | Skip; realistic, compiled binary |
| NodeToPython | GPL-3.0 | v4.2.0, 4.2–5.2 ([GitHub](https://github.com/BrendanParmer/NodeToPython)) | Yes | Use for versioning GN graphs |
| proctree | BSD-3 | C++, no Python binding ([GitHub](https://github.com/jarikomppa/proctree)) | Port needed | Reference for a permissive twig-card generator |
| The Grove | Proprietary core, Apache addon | Commercial, Blender 5-compatible | Unverified | Skip; licence forbids distributing grown tree models ([licence](https://www.thegrove3d.com/license/)) |
| SpeedTree | Subscription (Indie < $200K) | Active ([Unity](https://support.unity.com/hc/en-us/articles/15723241438228-What-s-the-difference-between-SpeedTree-Learning-Edition-Indie-Pro-and-Enterprise)) | Not Blender-headless | Skip |
| Tree It | Freeware, closed | Windows GUI ([80.lv](https://80.lv/articles/tree-it-a-free-tree-generator)) | No | Skip |

GPL add-ons used as internal tooling do not encumber the generated meshes. The pipeline script only becomes GPL if it copies GPL code and the script itself is distributed. Free stylized GN setups such as RC12's generator ([80.lv](https://80.lv/articles/free-stylized-tree-generator-made-with-geometry-nodes-in-blender)) are worth mining for ideas, but check their licences before copying graphs.

### Bake Crysis-style wind data into vertex colour at generation time

The Crysis scheme in GPU Gems 3 stores leaf-edge stiffness, per-leaf phase, overall stiffness and AO in RGBA. It layers main bending (whole tree, scaled by normalized height) with detail bending ([NVIDIA GPU Gems 3](https://developer.nvidia.com/gpugems/gpugems3/part-iii-rendering/chapter-16-vegetation-procedural-animation-and-shading-crysis)). Godot's official tree tutorial and Victor Karp's Godot 4 wind shader both drive sway from vertex-colour gradients ([Godot docs](https://docs.godotengine.org/en/stable/tutorials/shaders/making_trees.html); [Victor Karp](https://victorkarp.com/godot-foliage-wind/)).

The generator knows the structure, so it should write these channels procedurally with Store Named Attribute:

- **R** = main sway, `(z/height)^1.5–2`.
- **G** = per-branch or per-puff random phase.
- **B** = parameter along the branch (0 at attach, 1 at tip).
- **A** = flutter mask or baked AO (canopy shell and frond tips).

Optionally add UV2 with the branch pivot. Store data channels as FLOAT_COLOR to avoid sRGB conversion. That precaution is inferred; verify it with a test export. Prefer COLOR_0 and UV2, because Godot's mapping of arbitrary glTF attributes to CUSTOM0–3 was not verified.

## Godot-side rendering: recompute normals in the shader, instance by chunk, ink only interactables

### Alpha scissor plus a hand-written back-light term

Godot's docs are explicit on alpha modes. **Alpha blend cannot cast shadows. Alpha scissor can, and suits "foliage and fences."** Alpha hash dithers and still casts shadows. Alpha antialiasing (alpha-to-coverage) needs MSAA of at least 2× ([Godot docs, StandardMaterial3D](https://docs.godotengine.org/en/latest/tutorials/3d/standard_material_3d.html)). The official Making Trees tutorial combines `cull_disabled`, `world_vertex_coords`, SSS transmittance for backlit leaves, and a depth pre-pass ([Godot docs](https://docs.godotengine.org/en/stable/tutorials/shaders/making_trees.html)). A community tip is to set `ALPHA_SCISSOR_THRESHOLD` below 0.5 so cards don't thin out at distance ([godotshaders, J Hell, MIT](https://godotshaders.com/shader/simple-cheap-stylized-tree-shader/)). The spatial-shader reference confirms `BACKLIGHT`, `FRONT_FACING`, `IN_SHADOW_PASS`, `NODE_POSITION_WORLD`, and `ATTENUATION` inside `light()` ([Godot docs, Spatial shaders](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html)).

Because the project overrides `light()`, the engine's default diffuse no longer applies. The painterly `light()` must do three things. It bands N·L through the ramp with the brush-noise terminator. It **posterizes `ATTENUATION`** so cast-shadow edges match the bands. And it adds its own translucency term, `BACKLIGHT * clamp(-NdotL,0,1) * ATTENUATION * LIGHT_COLOR`, which reserves Alisavakis's view·(−light) "fake SSS" glow for backlit moments. Whether `BACKLIGHT` interacts with an overridden `light()` in 4.7 is undocumented, so test it.

For shadows, use `IN_SHADOW_PASS` to disable billboarding in the shadow pass. For the softest painterly result, set leaf cards to `cast_shadow = off` and let a low-poly canopy proxy with `shadows_only` cast a clean blobby shadow.

### Author proxy normals in Blender, but make the shader the source of truth

A Godot forum user who transferred proxy normals with Blender's Data Transfer modifier saw only base normals in Godot. Blend shapes also broke custom normals, and the thread has no confirmed fix ([Godot Forum](https://forum.godotengine.org/t/how-to-get-custom-normals-into-godot/112350)). Related threads report glTF normal discrepancies ([Godot Forum](https://forum.godotengine.org/t/godots-gltf-importer-breaks-normals/86623); [godot#41514](https://github.com/godotengine/godot/issues/41514)). Godot's "Generate LODs" import option can also alter low-LOD normals (inference).

The robust route for a procedural pipeline is to **bake each vertex's clump-centre offset** into vertex data (UV2, or spare colour channels on the blob core) and compute `NORMAL = normalize(mix(NORMAL, normalize(VERTEX - clump_center), spherize))` in `vertex()`. Keep the exported custom normals as a fallback. This also makes the per-clump vs whole-tree blend a live material parameter that art direction can tune without regenerating meshes. Disable Godot's auto LOD generation on foliage and supply the generator's own LODs.

### Wind: global uniform for weather, per-instance phase for desync

Godot 4 `global uniform`s are set once project-wide. `instance uniform`s give per-node values with a practical maximum of 16 per shader ([Godot blog](https://godotengine.org/article/godot-40-gets-global-and-instance-shader-uniforms/)). Victor Karp's shader pans FastNoiseLite textures, desyncs trees by world position, and uses the model matrix so rotated trees sway in the same world direction ([Victor Karp](https://victorkarp.com/godot-foliage-wind/)).

Combine these into a three-layer sway:

- **Trunk bend** = `wind_dir * strength * R`.
- **Branch sway** = slow sine or noise with phase G, scaled by B.
- **Flutter** = high-frequency noise × A.

Drive them from a `global uniform vec4 wind` that a weather script updates. Per-tree phase comes from `NODE_POSITION_WORLD` for single MeshInstances and from `INSTANCE_CUSTOM` for MultiMesh instances. `instance uniform` does not vary per MultiMesh instance (inference). Cozy games want low-amplitude, slow canopy motion that moves whole clumps. Palm fronds and conifer tier tips are where the per-branch channel earns its keep.

### Chunked MultiMesh, visibility ranges, and a toon-aware impostor tier

Visibility ranges provide begin/end distances with margins, fade modes (Self, or Dependencies for HLOD), and a recommendation to prefer dithering over alpha fades and simpler materials on distant LODs ([Godot docs, Visibility ranges](https://docs.godotengine.org/en/latest/tutorials/3d/visibility_ranges.html)). Put each species into **MultiMeshInstance3D chunks of about 32–64 m**. One huge MultiMesh culls as a single AABB, and visibility range works per node, not per instance. Carry per-tree tint and wind phase in MultiMesh colour and custom data, as in the wave_forge vegetation example ([GitHub PR](https://github.com/AntonTegnelov/wave_forge/pull/158)).

For the far tier, **zhangjt93/godot-imposter** (MIT, octahedral, verified on 4.5 beta) is the maintained Godot 4 option ([GitHub](https://github.com/zhangjt93/godot-imposter)). It bakes albedo and normal, so toon lighting must be re-applied in the impostor shader, and 4.7 compatibility is unverified. At a 45° camera the trees are mostly seen from above, so a cheaper two- or three-card crossed LOD with baked toon albedo, or the blob-only mesh, may be enough.

Godot 4.7's release notes (AreaLight3D, HDR output, clearcoat changes) show no changes to alpha modes, MultiMesh, visibility ranges, glTF normals or `light()` ([Godot 4.7 release](https://godotengine.org/releases/4.7/)). Test toon banding under AreaLight3D if you use it.

Useful building blocks, all to be adapted to the custom `light()`:

- J Hell's MIT billboard "fluffy tree" shader and Miu's CC0 extension with fresnel, wind and flip-normal ([godotshaders](https://godotshaders.com/shader/stylized-fluffy-tree-leaves/)). These swap well into the fringe-card material once the "unshaded + fresnel" part is replaced by the painterly `light()` on spherized normals.
- FaRu85/Godot-Foliage ([GitHub](https://github.com/FaRu85/Godot-Foliage)).
- SimpleGrassTextured (MIT, MultiMesh) for ground cover ([GitHub](https://github.com/IcterusGames/SimpleGrassTextured)).

### Outlines: inverted hull on the proxy, never on cards

Inverted-hull outlines are built by expanding along normals and rendering back faces only ([toon-rp wiki](https://github.com/Delt06/toon-rp/wiki/Inverted-Hull-Outline)). Godot 4.5's stencil Outline mode, like Grow, requires connected, smooth-shaded meshes ([Godot docs](https://docs.godotengine.org/en/latest/tutorials/3d/standard_material_3d.html); [godot-docs PR #11373](https://github.com/godotengine/godot-docs/pull/11373/files)). Neither fixes alpha cards, because both follow the quad, not the alpha shape. So for the starfruit tree, attach the cel+ink inverted-hull material as a next pass on the **fused blob core and trunk**, with `cast_shadow` off. The generator's watertight SDF mesh is ideal for this. If screen-space ink is ever wanted on environment trees, drive it by depth discontinuities only and mask foliage out of the normal-edge term, or it fizzes.

## Automatic evaluation: gate with geometry, match with embeddings, rank with a VLM

### Cheap geometric gates catch most failures before any GPU work

Perception research finds vertex count and **P²/A** (perimeter squared over area) among the strongest predictors of perceived 2D shape complexity, along with outline-to-convex-hull perimeter ratios ([Visual Computer](https://link.springer.com/article/10.1007/s00371-022-02634-8)). A study of 22,301 pairwise 3D-shape preferences found **symmetry, curvature and compactness** to be the main geometric drivers of preference, though it covered furniture and mugs, not trees ([arXiv 2505.12373](https://arxiv.org/abs/2505.12373)). Stava et al. established the inverse-procedural-tree loop: fit generator parameters to a target tree via a shape similarity and MCMC ([Stava 2014](https://onlinelibrary.wiley.com/doi/abs/10.1111/cgf.12282)).

The project should compute a small set of metrics from Workbench or Eevee mask renders **taken from the actual 45° game camera pitch**, plus front and side views:

- Silhouette IoU or chamfer distance against concept turnarounds.
- Sky-hole fraction, meaning (hull − silhouette)/hull on the canopy, with hole count and size.
- Compactness P²/(4πA) and mirror-symmetry IoU.
- Canopy-to-trunk proportions, read straight from the generator's parameters.

Score each as **distance to a target band** derived from three to five approved reference trees, never as higher-is-better. Mesh validity (tri budget per LOD, non-manifold edges, loose parts, material-slot count, presence of COLOR_0) is a hard reject, not a score. None of these metrics has been validated against human judgement on stylized trees specifically, so calibrate the bands locally.

### Embeddings: DreamSim for "looks like the concept," CSD with CSLS for "same style"

**DreamSim** fine-tunes a CLIP/OpenCLIP/DINO ensemble on human similarity judgements and reaches **96.16% agreement with human 2AFC choices**. It captures mid-level layout, pose and appearance, and installs with pip ([arXiv 2306.09344](https://arxiv.org/html/2306.09344v3); [GitHub](https://github.com/ssundaram21/dreamsim)). Use it as the primary render-vs-concept score.

**CSD** (MIT, ViT-L) is purpose-built for style independent of content ([arXiv 2404.01292](https://arxiv.org/pdf/2404.01292); [GitHub](https://github.com/learn2phoenix/CSD)). A July 2026 study, however, found that **raw CSD cosine fails for 25.3% of artists**, with same-artist similarity falling below cross-artist similarity. Switching to a **CSLS readout cut failures from 23/91 to 4/91**, and the failure affects CLIP, SigLIP and DINOv2 backbones too ([arXiv 2605.09030](https://arxiv.org/html/2605.09030v2)). So score style as a CSLS-normalized *margin* between a pool of approved references and a pool of off-style negatives. The Hunyuan3D trees make a ready-made negative pool.

CLIP text scores are weak on 3D plausibility. Kendall τ was as low as about 0.28 on that criterion in GPTEval3D's table ([arXiv 2401.04092](https://arxiv.org/pdf/2401.04092)), so give them low weight. Two render passes help. Render with lighting and background matched to the concept art, and also run the actual Godot shaders for final candidates, since the look lives in the painterly `light()`, not in Blender. Godot's `--headless` mode uses a dummy renderer, so screenshotting needs a real GPU context, such as a hidden window or a virtual display. This is from general Godot knowledge and was not verified in the research notes.

### VLM judges: pairwise, both orders, six views, aggregated with Bradley-Terry

The team's earlier finding is now well supported. "VLM Judges Can Rank but Cannot Score" measured **only 32–34% exact agreement with humans on a 5-point scale**, with poor items overscored by up to +1.98 and excellent ones underscored by up to −1.04. It recommends pairwise comparison ([arXiv 2604.25235](https://arxiv.org/html/2604.25235v1)). GPTEval3D likewise compares two assets at a time, builds Elo ratings, and ensembles over seeds, layouts and left/right flips to counter documented position bias ([arXiv 2401.04092](https://arxiv.org/pdf/2401.04092)). A 2026 factorial study of 12 VLM judges found that **a compact six-view RGB protocol performs comparably to denser views or added depth/normals**, and that model choice is the largest factor ([arXiv 2607.10826](https://arxiv.org/abs/2607.10826)). Trained scorers such as 3DGen-Score (0.725 pairwise agreement versus CLIP's 0.661) target generic text-to-3D quality and know nothing about this art direction ([arXiv 2503.21745](https://arxiv.org/html/2503.21745v1)). They are at most a "not broken" prior.

The local recipe therefore works like this. Load **Qwen3-VL-8B** at 4- or 8-bit, which fits a 16 GB card, and prompt it with ([Hugging Face](https://huggingface.co/Qwen/Qwen3-VL-8B-Instruct)):

- the concept sheet;
- two candidate six-view grids;
- named criteria: silhouette, clump hierarchy, value separation, trunk gesture, and "does this read as a Ghibli painting".

Ask each pair in both orders and discard inconsistent verdicts. Fit Bradley-Terry across candidates. Use absolute VLM answers only as binary defect gates, such as floating fringe cards, trunk poking through the canopy, or fruit intersecting leaves.

### Search: Optuna or CMA-ES for one tree, MAP-Elites for a forest

Quality-diversity search suits content libraries. MAP-Elites partitions a behaviour space and keeps the best candidate per cell, yielding a large diverse set in one run ([Gravina et al., arXiv 1907.04053](https://arxiv.org/pdf/1907.04053)). CMA-ME adds CMA-ES emitters to the archive ([arXiv 2505.06617](https://arxiv.org/pdf/2505.06617)). ProcFunc (2026) is a Python library that analyses Blender-based procedural generators to expose and optimize their parameters. It is worth evaluating as infrastructure ([arXiv 2604.26943](https://arxiv.org/html/2604.26943v2)).

For set health, the **Vendi Score** on DINOv2 ViT-L/14 embeddings gives an "effective number of distinct trees" ([arXiv 2210.02410](https://arxiv.org/html/2210.02410v2); [arXiv 2310.12952](https://arxiv.org/html/2310.12952)). Require style tightness in CSD-CSLS space at the same time. A good forest is **diverse in shape and coherent in style**. No published QD study on procedural trees exists, so this is transfer from level design and image generation, not proven practice.

## Recommended pipeline: one generator, five presets, a tiered loop

### The generator contract

Every tree is a JSON document validated against a schema. The agent edits JSON only, and a single `blender -b --python gen_tree.py -- spec.json` call produces:

- `tree_LOD0/1/2.glb`, where LOD0 is hybrid, LOD1 has merged cards, and LOD2 is blob-only;
- a proxy-shadow mesh;
- mask renders;
- a metrics JSON.

The pipeline stages are:

1. Skeleton: curve recursion, or space colonization for the hero.
2. Puff centres from branch tips plus hull-seeded extras.
3. SDF fuse, smooth and decimate into the blob core.
4. Fringe cards scattered on the upper and sun-side shell.
5. Attributes: wind RGBA in COLOR_0, clump-centre offset in UV2, AO.
6. LOD variants and glTF export.

Shared parameters for all types: `seed`, `height`, `lean_deg`, `trunk{base_radius, taper, bend, gesture: S|C, root_flare, profile_sides: 6–8}`, `canopy{puff_count, size_ratio: [3,2,1], envelope_scale_xyz, underside_flatten, sky_holes, voxel_div: ~20, smooth_iters: 1–3, target_tris}`, `fringe{density, overhang: 0.1–0.3, sun_bias, underside_suppress}`, `normals{spherize, clump_vs_tree_blend}`, `color{palette_id, clump_jitter, tree_jitter}`, `wind{sway_exp, flutter}`, `lod{bands_m: [15, 30]}`.

Per-type starting ranges (engineering judgement, to be calibrated):

| Parameter | Round deciduous | Conifer | Palm | Bush | Starfruit hero |
|---|---|---|---|---|---|
| Height (m) | 4–8 | 5–12 | 5–9 | 0.6–1.5 | 4–6 |
| Skeleton | Curve recursion, 2–3 levels | Straight tapered trunk, mostly hidden | Quadratic-bend curve, tip offset 0.1–0.3×H, 8–14 ringed segments (sawtooth radius) | 3–7 short stems or none | Space colonization in an authored umbrella hull |
| Canopy | 1 dominant + 2–4 secondary + 3–8 accent puffs, SDF-fused | 3–7 stacked cone tiers, tier height 1.2–1.8× step, skirt droop `sin(nθ)`, per-tier yaw jitter | 6–12 arched frond strips, pitch 20–60°, quadratic droop, young fronds upright in centre | 5–15 puffs in flattened ellipsoid (h≈0.6r), top-biased, base sunk | 3–5 large drooping cloud-shelves with 2–3 sky holes showing the trunk |
| Fuse method | SDF fuse | Faceted or SDF-softened per preset | None (strips) | SDF fuse, coarse | SDF fuse |
| Fringe density | 0.6–1.0 | 0.2–0.4 (tier edges only) | 0 (geometric leaflets or alpha fronds) | 0–0.3 | 0.8–1.0 |
| Tris LOD0 | 1.5–4k | 0.8–2k | 1–2.5k | 300–800 | 4–8k |
| Wind emphasis | Clump sway | Tier-tip bob (G per tier, B radial) | Frond wave (B along frond, G per frond) | Minimal | Clump sway plus fruit dangle |
| Shader | Painterly env | Painterly env | Painterly env | Painterly env | **Cel+ink, inverted hull on core + trunk** |
| Extras | Root flare | Optional snow slot on normal.z > 0.6 | 3-sphere coconut cluster | Optional flowers via normal.z filter | Fruit instances: 5-point extruded star sections, stored as separate harvestable nodes |

The starfruit tree should be the one asset where hand direction is explicit. Give it an authored hull mesh for space colonization, a larger tri budget with 50% and 20% decimated LODs, and fruit as separate instances so gameplay can pick them.

### The evaluation loop

Run cheapest first.

- **Gate (milliseconds):** schema sanity, tri budgets, non-manifold and loose geometry, attribute presence, proportion bands. Failures never render.
- **Silhouette (tens of milliseconds):** mask renders at the 45° game pitch plus front and side. IoU/chamfer against concept turnarounds, sky-hole fraction, compactness, symmetry band.
- **Embedding (batched GPU):** DreamSim to the concept views and a CSD-CSLS style margin against approved and negative pools. Combine into one fitness whose weights are fitted by Bradley-Terry on **about 50–100 human pairwise labels**. The same labelled set verifies each metric's correlation with the team's taste.
- **Search:** Optuna TPE for mixed discrete and continuous single-tree tuning. MAP-Elites or CMA-ME for species libraries, with behaviour axes such as height/width ratio × sky-hole fraction, or clump count × trunk lean. Reject clones by DreamSim nearest-neighbour distance and by parameter distance.
- **Judge (seconds per pair):** render the top-k elites in Godot with the real shaders. Run a both-orders Qwen3-VL-8B pairwise tournament against the concept sheet and aggregate by Bradley-Terry.
- **Human:** review only the top N and every case where the VLM and embedding rankings disagree. Feed those verdicts back into the weight fit.

Keep the VLM tournament and human spot checks as held-out judges, because optimizing hard against DreamSim or CLIP invites adversarial-looking trees.

## Conclusion

The failure of the Hunyuan3D trees points to the underlying issue. In this style the parts that make a tree look right cannot be recovered from a surface scan: which clump owns which normal, which vertex sways, where the ink goes, what drops at 30 m. A stylized tree is a structured object whose structure the shader consumes, so the generator must emit that structure (clump centres, wind channels, LOD tiers, proxy hulls) as first-class data. That is also what makes automatic evaluation tractable. Most quality can be judged from the generator's own parameters and flat silhouettes before any neural model runs, and the fuzzy "is it Ghibli" question shrinks to a small pairwise tournament where local VLMs are reliable.

Two things remain genuinely uncertain and should be tested first:

- Whether proxy normals survive the Blender 5.2 → glTF → Godot 4.7 path. The shader fallback removes this risk.
- Whether DreamSim and CSD track the team's own taste on painted-concept-versus-toon-render comparisons, a domain gap no published study covers.

A one-day spike can settle both. Generate twenty deciduous variants, label fifty pairs by hand, and measure metric agreement before building the full search loop.

---

## Addendum (2026-09-28): what building it taught us

These notes come from building one hand-directed tree (`art/trees/deciduous_01`, `tools/tree/gen_deciduous.py`), then a Geometry Nodes space-colonization version (`deciduous_02`, `tools/tree/gn_tree.py`), then a gated set of ten (`deciduous_set`, `tools/tree/export_set.py` + `assemble_set.gd`). Everything below was tested, not inferred.

### One open question is settled

- **Proxy normals survive the Blender 5.2 → glTF → Godot 4.7 trip if you carry them yourself.** Encode them in a `FLOAT_COLOR` point attribute as `n * 0.5 + 0.5`, converted by hand to Godot axes `(x, z, −y)`, since the exporter can't know it's a vector. After import they decode to unit length within 0.013. Set `force_disable_compression`, turn off `generate_lods` and `create_shadow_meshes`, and in the shader `NORMAL = mix(NORMAL, proxy, spherize)`. The billboard corner offset for each leaf card also survives in UV2. The shader-side route the report proposed works. We never relied on custom split normals.
- DreamSim and CSD were **not** tested, because they weren't needed for one tree or a set of ten. Judging by eye against the style brief and cheap gates was enough at this scale.

### The failure modes, in the order they appeared (a usable eval rubric)

1. **Lollipop:** the canopy core is far too small and reads as separate balls on sticks. Blender metaballs put the surface at about 0.59·radius at threshold 0.6 / stiffness 2.2, and about 0.685·radius at 0.3 / 2.0, so calibrate or use SDF. Blender 5.2's `Points to SDF Grid` takes a per-point radius and avoids the problem.
2. **Broccoli:** the masses merge into one lump. Fix it with wider clump spacing, notches between shelves (negative metaballs or protruding clumps), and flatter shelf undersides (flatten factor 0.6–0.72).
3. **Blob with stickers:** a smooth core with sparse cards on top looks like plastic. The fix was to project the same leaf-dab atlas onto the core in world space at ~35% strength and use more, larger cards (0.8–1.2 m, 300–500 per tree). This mattered more than any lighting change.
4. **Front-lit flatness:** the beach sun sits behind the default game camera, so N·L lights every visible clump and the masses merge. Lighting can't separate them; the albedo has to. Darken the **seams** between clumps and keep the **shelf undersides free of cards** so a dark line shows. Seam detection trick: blur the stand-in normal across the core mesh. The length of the blurred vector drops where clumps meet, so `seam = clamp((1 − |blur(n)|) · 5)`.
5. **Stick trunk (procedural only):** if the canopy sits beyond the influence radius from the top of the initial trunk, space colonization grows nothing. The trunk is then the starting line with one tip, and the pipe model makes it a thread. Derive trunk height from the canopy's actual bottom plus 0.5 m.
6. **Floating clumps (procedural only):** Poisson clumps on the envelope edge can miss the main mass. Delete core islands under 25% of the largest (`Mesh Island` → `Accumulate Field` → `Attribute Statistic`).

### Numbers calibrated by the build (they replace the report's guesses)

- **Budget:** 5–6k triangles for the whole tree at LOD0 (wood 2–3k, core 2–3k, cards ~600). Set members span 4.7k–7.0k.
- **Stand-in normal blend:** 80% per-clump / 20% whole-tree. Card offset outward from the core surface: 0.03–0.22 m.
- **Clumps:** radius = mix(min, max, rand^1.8) gives the many-small / few-big hierarchy. Poisson spacing is 1.4–1.8 m on a ~5.5 m canopy.
- **Pipe model:** radius = 0.03 · √(tips downstream) · (1 + flare·e^(−z/0.35)) gives a ~0.25 m trunk at 1 m height.
- **Space colonization (Step / Influence / Kill):** 0.22 / 1.8 / 0.45 m, 60 iterations, attractor density 25/m³, up-bias 0.2.
- **Colours:** shadow tint (0.44, 0.56, 0.84) with underside colour (0.12, 0.32, 0.36) gives the saturated teal shadows. Bark tint (0.56, 0.50, 0.56) keeps the trunk from reading orange next to the teal.

### Geometry Nodes specifics (Blender 5.2)

- **Space colonization in a Repeat Zone** is about 25 nodes. Per iteration, each attractor finds its nearest node with `Sample Nearest`, and attractors within the kill distance are deleted. Votes are summed per node with `Accumulate Field` grouped by the nearest-node index. `Leading == 1` picks one representative attractor per group to become the new node; store the group sums before deleting the rest. Storing the **parent position** instead of the parent index avoids depending on Join order. Edges come from instancing a two-vertex line per node, then `Merge by Distance`.
- **Pipe-model counts:** run `Shortest Edge Paths` to the root, then `Edge Paths to Curves` from every tip, then `Accumulate Field` grouped by original vertex id on the curve points. `Sample Nearest` can't read curves, so convert them to points first.
- **Other traps:**
  - `Merge by Distance` in "All" mode merges **transitively** and collapses dense point clouds. Don't use it for spacing.
  - `Grid to Mesh` outputs flat-shaded faces, which triples the vertex count on glTF export. Add `Set Shade Smooth`.
  - Any float2 corner attribute exports as a UV map, so remove helpers like `uvb`.
  - Modifier inputs in 5.2 are `md.properties.inputs.<Socket_N>.value`, not `md[identifier]`.
- **Cost:** a full rebuild takes about 0.7 s, and ten gated trees export in 2 s headless.

### Quality: procedural vs hand-directed

- **Cheap geometric gates did the job,** confirming the report's first tier. Four gates (trunk radius ≥ 0.13 m at 1 m, ≥ 200 cards, ≤ 7.5k triangles, width ≥ 3.8 m) caught every broken candidate in the set run, with 2 of 12 rejected. No neural judge was needed to reject bad trees. Ranking good ones is where a judge would earn its keep.
- **Hand placement still wins on the best tree.** The hand-placed 12-clump layout (deciduous_01) beats Poisson-on-an-ellipsoid on silhouette. Procedural trees drift toward round "mushroom" crowns unless the envelope is asymmetric. **The envelope shape is the main art-direction lever**; seeds only vary the details. Next steps: sculpted or library envelopes, and a silhouette gate (e.g. compactness P²/4πA, which should catch the round ones).

### Pipeline practicalities

- **No `.blend` inside the Godot project.** Godot tries to import it through Blender, and headless `--import` hangs. Keep blends in a folder with `.gdignore`.
- **Rendering on Xvfb works.** Godot renders on the real GPU inside a private Xvfb display. The lookdev loop is generate → import → four fixed framings (front-lit, back-lit, 18 m, 40 m grove) → one contact sheet. Keep the front-lit game-camera shot as the must-pass view.
- **Not yet tested:**
  - Detail levels and impostors
  - Chunked MultiMesh instancing
  - Godot's built-in `BACKLIGHT` under a custom `light()` (we wrote our own translucency term)
  - AreaLight3D
  - DreamSim, CSD and the VLM judge tiers

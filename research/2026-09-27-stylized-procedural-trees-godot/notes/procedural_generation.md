# Procedural tree generation for headless Blender (5.2): algorithms, canopy construction, tools, recipes, wind data

Research date: 2026-09-27. "Verified" = confirmed on a source fetched this session. "Inferred" = my engineering judgement or background knowledge that was not confirmed against a fetched source this session.

## Algorithms: Weber-Penn, L-systems, space colonization, attractor methods, curve trunks with recursion — which suit STYLIZED low-poly trees?

### Takeaway
Weber-Penn (Sapling) and space colonization both have maintained Blender extensions today, but both aim at realistic branching and are GPL. For stylized game trees the key parts are a few thick, tapered branches and a separate blob canopy. The most controllable, headless-friendly approach is a simple custom recursive curve-based generator in Python or Geometry Nodes: a trunk curve with a radius falloff, then recursive child curves. Space colonization is best kept for a single "hero" tree whose silhouette should fill a given hull.

### Cited Findings
- Sapling Tree Gen "uses the algorithm presented by Jason Weber and Joseph Penn in their paper 'Creation and Rendering of Realistic Trees'", can add armatures (animated or not), and supports saving and loading presets (verified) — [Blender Extensions: Sapling Tree Gen](https://extensions.blender.org/add-ons/sapling-tree-gen/)
- A dedicated Blender extension, "Space colonization tree generator" v1.0.0 (July 18, 2025, Blender 4.2 LTS+, GPL-3.0-or-later), grows a tree skeleton inside a mesh the user selects. It is based on Runions, Lane & Prusinkiewicz (2007). It is 6.6 KB, has about 7.9k downloads, and a reviewer asked for a preview feature (verified) — [Blender Extensions: Space colonization tree generator](https://extensions.blender.org/add-ons/space-colonization-tree-generator/)
- Other space colonization implementations (verified to exist):
  - varkenvarken/spacetree: an older Blender add-on that was part of Blender contrib; a commercial version exists — [GitHub varkenvarken/spacetree](https://github.com/varkenvarken/spacetree)
  - elfnor/spacetree-sverchok: a Sverchok scripted node for Sverchok 0.6 / Blender 2.83 (legacy) — [GitHub elfnor/spacetree-sverchok](https://github.com/elfnor/spacetree-sverchok)
  - jomiq/space_col: a parallel numpy implementation in plain Python — [GitHub jomiq/space_col](https://github.com/jomiq/space_col)
- Blender Studio's official "Geometry Nodes from Scratch" course has a "Tree Generator" example chapter. This shows that curve-based GN tree building is a documented first-party workflow (verified that the chapter exists; contents not fetched) — [Blender Studio](https://studio.blender.org/training/geometry-nodes-from-scratch/example-tree-generator/)
- proctree (C++ port of proctree.js, BSD-3 generator) is a compact parametric generator. It produces a trunk mesh plus a separate "twig" mesh of leaf cards and is "several orders of magnitude faster" than the JS original. It has no Python binding (verified) — [GitHub jarikomppa/proctree](https://github.com/jarikomppa/proctree)

### Inferences
- **Weber-Penn** (Sapling): about 40 parameters per level (split counts, curve, downAngle, rotate, taper, branch counts per level). It can reproduce many species, but the output is a curve object with many thin branches. For a stylized look you would use levels 1–2 only, set low curve resolution, and convert to mesh. It can be driven headless by calling its operator with a preset dict. It is best for the conifer and deciduous skeletons if you accept GPL tooling.
- **L-systems**: grammar strings plus a turtle interpreter. They are trivial to implement in about 150 lines of Python (no dependency needed) and deterministic from a seed plus rules in JSON. They are good for bushes, stylized ferns and palm frond rachis, but tuning them for silhouette control is unintuitive. No maintained Blender 4.2+/5.x L-system extension was confirmed (gap).
- **Space colonization**: silhouette-driven. You give it a canopy hull (sphere, cone, or artist mesh) and branches fill it. This is excellent for a hero tree and for making branches reach the canopy blobs. It is O(attractors × nodes) per iteration, so keep attractors in the low thousands and use numpy or KD-trees (`mathutils.kdtree`).
- **Particle/attractor methods** (e.g. The Grove's light- and gravity-driven growth simulation): the most realistic, the least stylized, and the least controllable.
- **Curve trunk + taper + recursive branches (recommended default)**:
  1. Build a Bezier or poly curve with a noise-offset trunk, then use Curve to Mesh with a circle profile of 6–8 segments. Radius comes from a "parameter along spline" falloff.
  2. At N points along each parent curve, spawn child curves with angle and length decay, going 2–3 levels deep.
  3. Use child end points as canopy puff centres.

  This gives full JSON control (seed, levels, angles, taper, puff radius) and is fully deterministic.

### Gaps
- Did not fetch the Weber & Penn (1995) or Runions et al. (2007) papers directly; the attributions are via the extension pages.
- No 2025–2026 maintained L-system Blender extension was verified.

## Canopy construction for a soft "cloud-puff" look (metaballs, SDF/volume blobs, voxel remesh, icospheres) and fusing clumps

### Takeaway
In Blender 5.x the cleanest headless approach is inside Geometry Nodes:
1. Scatter puff centres (branch tips or points in a hull).
2. Turn them into a density or SDF grid (Points to Volume, or the new 5.0 SDF grid nodes).
3. Smooth or fillet the grid.
4. Convert it back to a mesh with Volume to Mesh / Grid to Mesh.
5. Decimate and smooth-shade.

This fuses clumps into one watertight soft surface. Metaballs and voxel remesh are viable fallbacks.

### Cited Findings
- The Points to Volume node "generates a fog volume sphere around every point"; the manual says it usually makes sense to combine it with Volume to Mesh (verified, Blender 5.2 LTS manual) — [Blender 5.2 manual: Points to Volume](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/point/points_to_volume.html)
- The Blender 5.2 LTS manual lists these volume operation nodes (verified) — [Blender 5.2 manual: Volume nodes](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/volume/index.html):
  - Grid to Mesh, Grid to Points, Volume to Mesh
  - SDF Grid Boolean, SDF Fillet, SDF Grid Laplacian, SDF Grid Mean, SDF Grid Mean Curvature, SDF Grid Median, SDF Grid Offset
  - Field to Grid, Clip Grid, Grid Dilate & Erode, Grid Mean/Median, Prune Grid, Voxelize Grid
  - Sample/Advect/Curl/Gradient nodes
- Blender 5.0 added a volume grid socket type and 27 new grid nodes built on OpenVDB. These include booleans and SDF filters, and the SDF Grid Mean node smooths an SDF grid. The blog post is dated Oct 7, 2025 (verified) — [Blender Developers Blog: Volume Grids in Geometry Nodes](https://code.blender.org/2025/10/volume-grids-in-geometry-nodes/)
- Using the new nodes, artists "can convert meshes or point clouds to SDF grids, then perform Boolean operations on them" (secondary summary) — [CG Channel: Blender 5.0](https://www.cgchannel.com/2025/11/blender-5-0-is-out-check-out-its-5-key-features/); [DIGITAL PRODUCTION](https://digitalproduction.com/2025/11/20/blender-5-0-its-here/)
- Blender 5's grid system allows volume displacement, which turns any mesh into a cloud-like volumetric form in GN (verified that the article exists; the details are from a secondary summary) — [Mark Kingsnorth: Volume Displacement in Blender 5](https://www.markkingsnorth.com/2025/11/19/turning-objects-into-clouds-in-blender-5-a-look-at-volume-displacement/)

### Inferences
- **Recommended recipe for Blender 5.2**:
  1. Take the puff-centre points and give each a radius attribute (random 0.6–1.2 × base).
  2. Run Points to SDF Grid if present (exact node name not verified in this session; the older "Points to Volume" with Amount density plus the Volume to Mesh threshold is verified).
  3. Apply SDF Grid Mean or Median (1–3 iterations) or SDF Fillet to melt the seams between clumps.
  4. Optionally apply SDF Grid Offset to grow or shrink the canopy.
  5. Convert with Grid to Mesh / Volume to Mesh at a voxel size of about 1/20 of canopy diameter.
  6. Clean up: Merge by Distance, a Decimate or remesh step to about 300–1500 tris, Set Shade Smooth, and optionally Subdivision Surface at level 1 before decimating.

  Normals then point outward from a single fused surface. This is exactly the lighting you want for "cloud puff" toon shading. You can also transfer normals from a sphere or ellipsoid proxy for an even softer look.
- **Metaballs**: they fuse naturally and can be scripted easily (`bpy.data.metaballs`, `elements.new()`), then converted to mesh with `bpy.ops.object.convert(target='MESH')`. Operators need a context, but they work in background mode with an object selected. They are slower and less controllable than SDF grids, and not part of GN.
- **Voxel remesh + smooth**: union icospheres (Join), then use the Remesh modifier in voxel mode (or GN "Mesh to Volume" → Volume to Mesh), then Corrective Smooth or Laplacian Smooth, then Decimate. This is robust and version-independent (works on 4.x).
- **Icospheres with noise displacement** (not fused): cheapest, and gives an intentionally "clumpy" low-poly look. Seams between blobs show, which may be fine for a faceted low-poly style but not for soft puffs.
- **Style note**: stylized puffs are often shaded with a normal transfer from a smooth proxy. Canopy vertex colour or AO baked into COLOR_0 helps the look in Godot.

### Gaps
- The exact 5.2 node identifiers for mesh/points → SDF (e.g. `GeometryNodeMeshToSDFGrid`, `GeometryNodePointsToSDFGrid`) were not confirmed on a fetched page; verify with `bpy.types` introspection in Blender 5.2.
- No benchmark of grid-to-mesh time per tree was found.

## Geometry Nodes workflows: can a GN tree be built entirely from Python and evaluated/applied headless?

### Takeaway
Yes. Node groups can be created with `bpy.data.node_groups.new(name, 'GeometryNodeTree')`, nodes and links added by Python, attached through a NODES modifier, and inputs set through modifier keys. You then evaluate with `obj.evaluated_get(depsgraph)` / `bpy.data.meshes.new_from_object`, or call `bpy.ops.object.modifier_apply`, all under `blender -b --python`. NodeToPython can turn a hand-built GN tree into Python code to version in the pipeline.

### Cited Findings
- You can create GN trees in Python by adding nodes (NodeGroupInput, GeometryNodeSubdivideMesh, NodeGroupOutput), linking them, and assigning the group to a modifier (verified via a 2026 tutorial summary) — [CGWire blog: How to Script Geometry Nodes in Blender with Python (2026)](https://blog.cg-wire.com/blender-scripting-geometry-nodes-2/)
- NodeToPython (GPL-3.0) v4.2.0 supports Blender 4.2 through 5.2 and converts Geometry, Shader and Compositing node graphs into readable Python. It can output as a script or as an add-on zip, and handles layout, default values, subgroups and naming (verified) — [GitHub BrendanParmer/NodeToPython](https://github.com/BrendanParmer/NodeToPython)
- There is a known issue where changing GN modifier input values from Python did not trigger an update. The workaround mentioned is `mod.node_group.interface_update(context)` or touching the object (partially verified via a search summary of the bug tracker) — [Blender bug #87006](https://developer.blender.org/T87006)
- Built-in node references (Curve to Mesh, Instance on Points, Distribute Points on Faces, Points to Volume, Volume to Mesh) are all in the Blender 5.2 LTS manual — [Blender 5.2 manual: Geometry Nodes Modifier](https://docs.blender.org/manual/en/latest/modeling/modifiers/geometry_nodes.html)

### Inferences
- **Pipeline pattern**:
  1. Keep a `tree_gn.blend` asset containing the GN group, authored by hand or generated by a NodeToPython script.
  2. In the headless script, `bpy.data.libraries.load` the group, create an empty mesh object, and add the modifier.
  3. Set inputs via `mod["Socket_2"] = value`. Socket identifiers come from `node_group.interface.items_tree`.
  4. Call `obj.data.update()` and `bpy.context.view_layer.update()`.
  5. Apply the modifier, then export with `bpy.ops.export_scene.gltf`.

  Only the JSON-driven parameters vary per run. This is more maintainable than building 100+ nodes in raw Python each run.
- **Reproducibility**: expose a `Seed` input and route it into every Random Value and Distribute node.
- **Instances**: realize instances before export (Realize Instances node). Alternatively, keep leaf cards as instances and let the glTF exporter use EXT_mesh_gpu_instancing if Godot's importer support is confirmed (not verified).
- **Operators in background mode**: most `bpy.ops.object.*` operators work in `-b` when an active object is set via `bpy.context.view_layer.objects.active`. Use `bpy.context.temp_override` where a context is needed.

### Gaps
- Headless behaviour was not tested live: the Blender MCP server failed to connect this session, so nothing was run in Blender 5.2.

## Concrete tools: licence, last release, Blender support, headless scriptability, stylized output

### Takeaway
There is no single off-the-shelf tool that is permissive, maintained, headless-first, and stylized. Among Blender tools that can be scripted headless, Sapling (GPL, maintained as an extension, 0.3.7 for 4.4+) and MTree (GPL addon plus MIT C++ core, forks for 4.2+/4.3+) are the realistic options. The Grove is commercial, with licence limits on redistributing tree models. SpeedTree is subscription-based and not Blender-headless. The pragmatic choice is a custom Python/GN generator, possibly borrowing ideas from Sapling or proctree (BSD).

### Cited Findings
- **Sapling Tree Gen**: version history is 0.3.5 (May 14, 2024, 4.2 LTS+), 0.3.6 (May 22, 2024), and 0.3.7 (Dec 25, 2025, Blender 4.4+, adds "action slots" support). Licence is GPL-3.0-or-later and it has about 278k downloads. It "was part of Blender 4.1 bundled add-ons, but is now offered as is, with limited support." Location: Add → Curve (verified) — [Extensions page](https://extensions.blender.org/add-ons/sapling-tree-gen/); [Version history](https://extensions.blender.org/add-ons/sapling-tree-gen/versions/); [Source repo](https://projects.blender.org/extensions/add_curve_sapling)
- **Improved Sapling** (abpy fork) exists on GitHub; its status was not checked — [GitHub abpy/improved-sapling-tree-generator](https://github.com/abpy/improved-sapling-tree-generator)
- **MTree / Modular Tree**:
  - The original is at [MaximeHerpin/modular_tree](https://github.com/MaximeHerpin/modular_tree).
  - Fork ethanporcaro/modular_tree_py311 targets Blender 4.2+ (Python 3.11) and ships per-platform release zips. It is dual-licensed: GPLv3 addon and MIT core C++ library (verified) — [GitHub ethanporcaro/modular_tree_py311](https://github.com/ethanporcaro/modular_tree_py311)
  - Fork GoodPie/modular_tree requires Blender 4.3+ and is also GPLv3 addon plus MIT core. It has about 605 commits and a modern toolchain (uv, Ruff, pre-commit) and uses a custom node tree (Trunk, Branches, Tree Mesher nodes), not GN (verified) — [GitHub GoodPie/modular_tree](https://github.com/GoodPie/modular_tree)
  - Fork tuomaslarjama/modular_tree adds Unreal Pivot Painter export fixes — [GitHub](https://github.com/tuomaslarjama/modular_tree)
- **The Grove**:
  - The Blender add-on part is Apache-2.0, but the Grove Core is proprietary.
  - Licensed users may use grown trees in commercial projects but "may not sell or distribute simulations or 3D models of trees grown with The Grove Core."
  - The latest version is described as Blender 5 compatible (back to 4.5).
  - Version 2.2 (April 2025) added a Skeleton tool that builds game-friendly low-bone armatures.

  Verified via — [The Grove License](https://www.thegrove3d.com/license/); [CG Channel: The Grove 2.2](https://www.cgchannel.com/2025/04/f12-releases-the-grove-2-2-for-blender-and-houdini/); [Install page](https://www.thegrove3d.com/learn/install/)
- **SpeedTree** (Unity):
  - The Indie tier is for individuals or companies under $200K annual revenue and funding. It is a monthly subscription, single machine at a time, needs internet, and is tied to a user.
  - The exact 2026 price was not retrievable because the store page returned 403.

  Sources — [Unity support: SpeedTree editions](https://support.unity.com/hc/en-us/articles/15723241438228-What-s-the-difference-between-SpeedTree-Learning-Edition-Indie-Pro-and-Enterprise); [SpeedTree licensing guide](https://docs.unity3d.com/speedtree-modeler/manual/licensing-guide.html)
- **Tree It** (Evolved Software): free, and exported trees may be used commercially. It exports OBJ, .X and DBO (FBX mentioned by some sources), has an LOD slider, and auto-generates vertex colours usable for wind. It is a Windows GUI app, not scriptable (verified via secondary sources) — [80.lv](https://80.lv/articles/tree-it-a-free-tree-generator); [CG Channel](https://www.cgchannel.com/2018/01/download-free-tree-generation-tool-tree-it/); [Evolved: TreeIt](https://www.evolved-software.com/treeit/treeit); [Godot TreeIt shader](https://godotshaders.com/shader/treeit-tree-shader/)
- **proctree / SnappyTree**: the proctree generator is BSD-3, the editor/support libraries use BSD and zlib licences, and it is C++ with no Python binding. SnappyTree (the web app for proctree.js) is GPL-2 and outputs proctree JSON (verified) — [GitHub jarikomppa/proctree](https://github.com/jarikomppa/proctree); [GitHub supereggbert/SnappyTree](https://github.com/supereggbert/SnappyTree); [GitHub procedural/proctree](https://github.com/procedural/proctree)
- **Stylized GN tree generators (free or paid, mostly .blend node setups rather than extensions)**:
  - RC12's free stylized tree GN generator includes procedural bark and generated UVs — [80.lv](https://80.lv/articles/free-stylized-tree-generator-made-with-geometry-nodes-in-blender)
  - Satendra Saraswat's "Stylized Tree Generator" / VRIKSH is a curve-based GN setup for Blender 4.0+, sold on Gumroad; licence not stated — [BlenderNation](https://www.blendernation.com/2024/05/25/free-download-stylized-tree-generator/); [Gumroad VRIKSH](https://allthework17.gumroad.com/l/nwkts)
  - "Easy Tree" is a GN-based extension on the official platform — [Blender Extensions: Easy Tree](https://extensions.blender.org/add-ons/easy-tree/)
- **Space colonization extension**: GPL-3.0+, v1.0.0, July 2025, 4.2+ — see Algorithms section.

### Inferences
Comparison summary:

| Tool | Licence | Status (2026) | Blender 5.x | Headless scripting | Stylized fit |
|---|---|---|---|---|---|
| Sapling Tree Gen | GPL-3.0+ | Maintained (0.3.7, Dec 2025) | 4.4+ declared; 5.x inferred OK | Yes: `bpy.ops.curve.tree_add(**params)` after enabling the extension (operator name inferred from classic addon) | Medium: realistic, needs few levels + custom canopy |
| MTree forks | GPL addon / MIT core | Community forks active | 4.2+/4.3+; 5.x not verified | Custom node tree; scriptable via node properties + operator (inferred); compiled binary per OS | Medium: good trunks/roots, realistic leaves |
| The Grove | Proprietary core, Apache addon | Commercial, active | Yes (5 compat claimed) | Possible via Python addon (not verified) | Low for cartoon; redistribution restrictions |
| SpeedTree | Commercial subscription | Active (Unity) | N/A (standalone) | Not Blender-headless | High quality, but not the target style/licence |
| Tree It | Freeware (closed) | Old, sporadic | N/A | No (GUI) | Low-medium |
| proctree (C++) | BSD-3 | Small, unmaintained-looking | N/A | Port to Python ~300 lines, or call a compiled CLI | Good for low-poly "twig card" trees |
| Space colonization ext. | GPL-3.0+ | v1.0.0 Jul 2025 | 4.2+ | Yes (operator), tiny codebase | Good for hero skeletons |
| Custom GN/Python | Own | — | Native | Full | Best control |

- **GPL implications**: running GPL add-ons as internal tooling is fine. Generated meshes are output, not derivative code, so they are not GPL-encumbered. If you copy Sapling/MTree Python code into your own pipeline script and distribute that script, the script becomes GPL. This matters only if the pipeline code itself is shipped.
- **Recommendation**: build a custom Python generator (JSON → curves → GN canopy/fuse → glTF) for all five archetypes. Optionally call the Sapling operator for conifer or deciduous skeleton variety, and the space colonization extension for the hero tree.

### Gaps
- Last-commit dates for the MTree forks, abpy improved-sapling, and proctree were not displayed on fetched pages.
- SpeedTree 2026 pricing is unknown (store returned 403).
- Houdini tree tools (Labs Tree tools, SideFX) were not researched due to the tool-call budget.
- Easy Tree licence/version details were not fetched.

## Palms, conifers, bushes: procedural recipes

### Takeaway
All three are easy to do deterministically with curves and instancing:
- **Palm**: a curved trunk made of stacked, slightly scaled rings, plus a crown of arched frond cards.
- **Conifer**: stacked, jittered cones or discs along a vertical axis, each fused or simply overlapped.
- **Bush**: an SDF-fused cluster of puffs around a few short stems.

No authoritative source was fetched for these specific recipes, so everything below is inferred from standard practice.

### Cited Findings
- No dedicated sources were fetched for palm, conifer or bush recipes in this session (see Gaps).

### Inferences
- **Palm (curved segmented trunk + frond cards)**:
  - **Trunk**: a Bezier curve from a quadratic bend (tip offset of 0.1–0.3 × height, lean from JSON). Use Curve to Mesh with a 6–8 sided circle profile. Resample to N segments (8–14). Per segment, scale the radius with a sawtooth (e.g. `0.9 + 0.1 * fract(t*N)`) to get the ringed "stacked" look, or use Instance on Points with a slightly flared cylinder per segment and realize them. The radius tapers slightly toward the top and flares at the base.
  - **Crown**: 6–12 fronds instanced at the curve end point, rotated evenly around Z with jitter. Each frond is a curve (rachis) that arcs up then droops: pitch 20–60°, with gravity droop from a quadratic Z offset along its length. Build it as a strip mesh via Curve to Mesh with a line profile (width tapering), or as a mesh with leaflets as V-shaped cards. The frond card can use an alpha-tested leaf texture or pure geometry leaflets (sawtooth outline) for a fully geometric stylized look.
  - **Extras**: a coconut cluster of 3 low-poly spheres; young fronds pointing up in the middle.
- **Conifer (tiered cones)**:
  - **Trunk**: a straight tapered cylinder, mostly hidden.
  - **Tiers**: k = 3–7 tiers. Tier i sits at height `h_i = base + i * step * (1 - 0.1*i)`, with radius decreasing roughly linearly toward the top and height 1.2–1.8 × step, so each tier overlaps the one below.
  - **Tier shape**: use a Cone primitive with 8–12 vertices, or a cone whose bottom ring is displaced down in a "skirt" wave (`sin(n*theta)` droop) for a stylized fir look.
  - **Variation**: add a random yaw per tier, plus slight noise on the lower ring vertices.
  - **Snow variant**: a separate material slot on the upward-facing faces (normal.z > 0.6).
  - **Merging**: optionally fuse the tiers through the SDF pipeline for a soft variant, or keep them faceted for a low-poly look.
- **Round deciduous**: 2–3 level curve skeleton with branch tips as puff centres, then SDF fuse (see canopy), then decimate. Use a separate trunk/branch mesh with 6-sided profiles.
- **Bush**: 3–7 short stems from the origin, or none. Scatter 5–15 points in a flattened ellipsoid (radius r, height 0.6r), with a bias to the upper hemisphere. Then Points to Volume / SDF, smooth, and mesh at a coarse voxel size (about 300–800 tris). Randomly sink the base below ground. Optional flower instances via Distribute Points on Faces with a normal.z filter.
- **Hero tree**: space colonization inside an artist-authored hull mesh gives a characterful trunk. Then do larger canopy puffs, multiple puff clusters with gaps for silhouette readability, and higher tri budgets with LODs produced by Decimate at 50% and 20%.
- **JSON parameter schema idea**: `{archetype, seed, height, trunk:{radius, taper, bend, segments, profile_sides}, branches:{levels, count, angle, length_decay}, canopy:{method, puff_count, puff_radius, voxel_size, smooth_iters, target_tris}, palm:{fronds, frond_length, droop}, conifer:{tiers, base_radius, skirt_waves}, wind:{…}}`

### Gaps
- No primary tutorial sources were fetched for palm or conifer GN recipes (e.g. Erindale's videos were not checked); these recipes are engineering inference.

## Wind animation data baked into vertex colours/UVs for glTF → Godot

### Takeaway
Use the Crysis/SpeedTree-style hierarchy:
- **Main bending**: whole-tree sway, driven by normalized height.
- **Branch bending**: a per-branch weight plus a phase.
- **Leaf flutter**: an edge-stiffness mask.

Encode it procedurally in COLOR_0 (RGBA), plus optionally UV2/TEXCOORD_1, because Godot reliably imports these glTF attributes and a vertex shader can read `COLOR` and `UV2`.

### Cited Findings
- The Crysis (GPU Gems 3, ch. 16) vertex colour scheme is: R = leaf edge stiffness, G = per-leaf phase variation, B = overall leaf stiffness, A = precomputed AO. Main bending displaces the whole object along the wind direction, scaled by normalized height. Detail bending splits into edge bending (along XY normals, R) and per-leaf vertical bending (B stiffness, G phase) (verified) — [NVIDIA GPU Gems 3, Ch. 16](https://developer.nvidia.com/gpugems/gpugems3/part-iii-rendering/chapter-16-vegetation-procedural-animation-and-shading-crysis)
- Godot vertex-colour wind shaders exist that use a vertex-paint mask to control sway. Models are exported as glTF with vertex colours enabled. One approach uses a linear gradient in the red channel as the flexibility factor so the trunk stays still while the top sways (verified via summaries) — [Godot Shaders: vertex paint controlled wind sway](https://godotshaders.com/shader/vertex-paint-controlled-simple-wind-sway-shader/); [Victor Karp: Shader based foliage wind in Godot 4](https://victorkarp.com/godot-foliage-wind/)
- Tree It auto-generates vertex colours for tree flexibility, and there is a matching Godot shader (verified) — [Godot Shaders: TreeIt tree shader](https://godotshaders.com/shader/treeit-tree-shader/)
- An MTree fork focuses on Unreal Pivot Painter export, i.e. per-branch pivot data baked into UVs/textures (verified that the repo exists) — [GitHub tuomaslarjama/modular_tree](https://github.com/tuomaslarjama/modular_tree)

### Inferences
- **Procedural authoring in GN (all as Store Named Attribute, then exported as a colour attribute)**:
  - **R = main sway weight**: `pow(z / height, 1.5–2)` so the base stays pinned.
  - **G = branch phase**: a random value per branch island, computed from the branch curve index captured before Curve to Mesh (Capture Attribute of Index / Random Value with the spline ID). For canopy puffs, use random per puff via the nearest puff centre.
  - **B = branch/detail bend weight**: the parameter along the branch spline, 0 at the attach point and 1 at the tip. Set it to 1 for the canopy with noise, and to 0 on the trunk.
  - **A = leaf flutter mask or AO**: e.g. 1 on frond tips and canopy outer shell (from distance-to-canopy-centre), 0 elsewhere.
- **Optional UV2 (TEXCOORD_1)**: store the branch pivot's XY or its local length for SpeedTree-like pivot bending. Godot exposes UV2 in spatial shaders.
- **Export**: in Blender 4.2+/5.x the glTF exporter exports the active or rendered colour attribute as COLOR_0. The mesh colour attribute must exist as a real colour attribute (`FLOAT_COLOR` or `BYTE_COLOR`, point or corner domain) after the GN modifier is applied; convert with `mesh.color_attributes` if needed. Custom attributes prefixed with an underscore can also be exported (exporter option), but Godot's importer mapping of arbitrary `_ATTR` to CUSTOM0–3 was not verified. Prefer COLOR_0 and UV2.
- **Palm fronds**: use B for the along-frond parameter and G for a per-frond phase, which gives nice independent frond waving.
- **Conifer tiers**: use a per-tier phase in G and radial distance in B, so the tier tips bob.
- **Linear vs sRGB**: the glTF COLOR_0 is linear. When storing data (not colours) with BYTE_COLOR, watch for sRGB conversion; FLOAT_COLOR avoids it (inferred; verify).

### Gaps
- It was not verified which glTF custom attributes Godot 4.x maps to CUSTOM0–3 on import.
- No 2025–2026 source was fetched on Blender 5.x glTF exporter colour-attribute options.
- SpeedTree's exact wind vertex layout was not documented publicly in the sources fetched.

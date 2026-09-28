# Godot 4.7 scatter and foliage tooling: existing addons and building an in-editor "biome scatterer"

Research date: 2026-09-28. Project: FarmGameGodot (Godot 4.7.2, Forward+). Repo facts (licence, stars, last push, releases) come from the GitHub API on this date. Engine-internals claims come from reading Godot source at the `4.7-stable` tag. Anything labelled **[judgement]** is engineering opinion, not a sourced claim.

Local context I checked:
- Installed plugins are boujie_water_shader 1.0.0, dialogue_manager 4.1.0, gloot 3.0.2, godot_mcp 4.1.11 and phantom_camera 0.11.0.3. **No scatter or foliage addon is installed** (`project.godot [editor_plugins]`, `addons/`).
- `scripts/tree_set.gd`: `TreeSet extends Resource`, `@export var scenes: Array[PackedScene]`, `pick(key) -> scenes[posmod(key, size)]`.
- `art/trees/deciduous_set/deciduous_set_01.tscn`: root `Node3D` with three `MeshInstance3D` children:
  - `canopy_cards`: `material_override` = mat_canopy_cards.tres, `cast_shadow = 0`, `extra_cull_margin = 1.0`.
  - `canopy_core`: material_override only.
  - `trunk`: material_override only.
  - Meshes are embedded ArrayMesh sub-resources, about 360–410 KB per .tscn.
- `shaders/tree.gdshaderinc`: `tree_phase = dot(NODE_POSITION_WORLD, …)` on line 101, **inside `vertex()`**. `MODEL_MATRIX` is also used in vertex (wind direction, `cam_to_model`, world_pos varying).

---

## Q1. Which existing Godot 4 scatter/foliage addons exist, and what is their licence, version, 4.5+/4.7 compatibility, feature set and maintenance status?

### Takeaway
Of the addons checked, only ProtonScatter has the "polygon domain + weighted items + Poisson + projection + chunked MultiMesh" feature set. It is MIT, got a 4.7 compatibility commit in July 2026 and was pushed as recently as 2026-09-27, but it has had no tagged release since 2023 and has a long tail of crash, cache and version-upgrade issues. Spatial Gardener is a brush painter, not a polygon scatterer. Terrain3D's instancer only works on Terrain3D terrain. SimpleGrassTextured covers grass only. All of them are MIT.

### Cited Findings
**ProtonScatter (HungryProton/scatter)**
- MIT licence, about 2,990 stars, last push 2026-09-27, 70 open issues. The newest GitHub *release/tag* is `4.0` (2023-10-23), but `plugin.cfg` on main says `version="4.2.0"` — [GitHub repo](https://github.com/HungryProton/scatter).
- Asset Library lists ProtonScatter v4.1 (Godot 4.1 min), updated 2026-01-02, MIT — [Godot Asset Library](https://godotengine.org/asset-library/asset?filter=scatter&godot_version=4.5&sort=updated).
- Recent commits:
  - 2026-07-26 "adds uid files to the repo, fix compatibility issue with 4.7".
  - 2026-07-26 "Adds scatter cache demo".
  - 2026-07-26 "removes github workflow … outdated and I don't have the skills to maintain it".
  - 2026-09-27 "remove the uniform scaling requirement when using Jolt physics".
  - Source: [commits](https://github.com/HungryProton/scatter/commits/main).
- Still-open issues include:
  - #255 "Not compatible with 4.7" (2026-06-03, no detail given; predates the 07-26 fix).
  - #260 "chunk_dimension value isn't applied in-game" (4.7.2rc double-precision; a large path shape crashes with the default chunk size of 15).
  - #253 "Game crashes at runtime while loading ProtonScatter objects".
  - #248 "Scatter not saving modifier stack in 4.6".
  - #242 "Cannot add Scatter Nodes in 4.6 beta".
  - #243 Metal/macOS breakage.
  - #238 "Clear Cache button does nothing".
  - #236 freeze when duplicating a node.
  - #213 "Could not load cache every time I open a scene".
  - Source: [issues](https://github.com/HungryProton/scatter/issues).
- Architecture: a `ProtonScatter` node holds a Blender-like modifier stack, with `ScatterItem` children (assets) and `ScatterShape` children (domain). Shapes are Box, Sphere and Path, and any shape can be marked negative for exclusion — [README](https://github.com/HungryProton/scatter).
- Modifiers in `src/modifiers/`: create_inside_{grid,poisson,random}, create_along_edge_*, project_on_geometry, relax, remove_outside_shapes, remove_random, clusterize, randomize_transforms, look_at, snap_transforms, proxy, array, and others — [source tree](https://github.com/HungryProton/scatter/tree/main/addons/proton_scatter/src/modifiers).
- Poisson modifier: `radius` and `samples_before_rejection=15`, Bridson-style grid with `cell = radius/sqrt(2)` — [create_inside_poisson.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/modifiers/create_inside_poisson.gd).
- `project_on_geometry` exposes ray_direction, ray_length, ray_offset, remove_points_on_miss, align_with_collision_normal, max_slope, collision_mask and exclude_mask. It works by batched `PhysicsRayQueryParameters3D` raycasts — [project_on_geometry.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/modifiers/project_on_geometry.gd).
- A closed path shape tests containment in 2D (XZ) with `PolygonPathFinder.is_point_inside` — [path_shape.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/shapes/path_shape.gd).
- Deterministic output: "Using the same seed with the same settings will produce identical results" (`global_seed`). Render modes are Use Instancing (MultiMesh), Create Copies and Use Particles. Chunking is on by default with `chunk_dimensions = Vector3.ONE*15`. Rebuild runs on a `Thread`. `force_rebuild_on_load := true` by default. `keep_static_colliders` creates static collision through PhysicsServer3D RIDs, not nodes — [scatter.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter.gd).
- Item weighting: `count = round(item.proportion / total_item_proportion * transforms_count)`. This is a proportional split of one point set across items, not a per-point weighted draw — [scatter.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter.gd).
- `ScatterCache` node saves transforms to a `.res`/`.tres` file, with `auto_rebuild_cache_when_saving` — [scatter_cache.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/cache/scatter_cache.gd).
- Per-item settings: `override_cast_shadow`, `visibility_range_begin/end/margins`, `visibility_range_fade_mode` (Disabled/Self) and optional LOD generation — [scatter_item.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter_item.gd).
- Code comment: "if set_name is used after add_child it is crazy slow … About a 100x slowdown was observed" — [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).

**Spatial Gardener (dreadpon/godot_spatial_gardener)**
- MIT, about 1,310 stars. Latest release v1.4.1 (2025-03-05, "Convert to Godot 4.4"). No commits to master since 2025-03-05 — [GitHub](https://github.com/dreadpon/godot_spatial_gardener).
- Features: paints foliage and props on arbitrary 3D surfaces with a sphere brush, uses an octree plus MultiMesh with frustum culling, has import/export, and handles "per-mesh collision and shadow". Pitched at "finite medium-sized scenes", not procedural or heightmap worlds — [README](https://github.com/dreadpon/godot_spatial_gardener).
- Version-support gaps are handled by community PRs rather than releases:
  - #76/#77 4.5 compatibility.
  - #78 "Regression with Godot 4.5 and 4.5.1 MMIOctreeNode corruption after project reload".
  - #82 4.6 theme fix.
  - #84 "Compatibility with Godot 4.7" (2026-06-30). A contributor "got the addon to work in Godot 4.7" with explicit return types, but reported leftover issues when opening an existing Gardener.
  - #72 "Update_LODs eating a ton of frame time".
  - #68 mutex deadlock.
  - Source: [issues](https://github.com/dreadpon/godot_spatial_gardener/issues).
- #83 (2026-06-12) is an open feature request: "Add scene instancing for trees with sub-mesh instance leaves and branches". That means it does not handle multi-MeshInstance tree scenes today — [issue #83](https://github.com/dreadpon/godot_spatial_gardener/issues/83).

**Terrain3D instancer (TokisanGames/Terrain3D)**
- MIT, about 4,300 stars, v1.0.2-stable (2026-05-19), last push 2026-09-26 — [GitHub](https://github.com/TokisanGames/Terrain3D).
- Stores instances as MultiMesh in a "32x32m grid of cells" and has painting through an Asset Dock. Supports up to 10 LODs (recommended max 4) by visibility range, plus `last_shadow_lod` and `shadow_impostor`. `Terrain3DInstancer.add_transforms()` / `add_multimesh()` are available for code placement — [Terrain3D instancer docs](https://terrain3d.readthedocs.io/en/latest/docs/instancer.html).
- Limitations from the same docs:
  - "Multi-mesh objects (e.g., tree trunk + leaves) require combining into single meshes".
  - No per-instance culling within a cell.
  - "No collision".
  - Scene transforms in the source files are ignored.
  - Requires Terrain3D as the ground.

**SimpleGrassTextured (IcterusGames)**
- MIT, about 570 stars, v2.1.0 (2026-04-03), last push 2026-09-17 — [GitHub](https://github.com/IcterusGames/SimpleGrassTextured).
- In-editor painting onto StaticBody3D terrain, custom meshes, shadow toggle, heightmap bake, LOD/auto-centre optimisation and player interaction. Grass/ground cover only — [README](https://github.com/IcterusGames/SimpleGrassTextured).

**Smaller or less active scatterers**
- The Asset Library "scatter" search (min Godot 4.5 filter) lists:
  - ScatterShot v1.0.5 (Godot 4.5, MIT, 2026-06-22).
  - Yamms v1.2.0 (4.3, MIT, 2025-03-19).
  - Scatter Tool v1.0 (4.2, CC0, 2024-11-30).
  - MultiMesh Scatter v1.1.0 (4.0, MIT, 2023-02-12).
  - Source: [Asset Library](https://godotengine.org/asset-library/asset?filter=scatter&godot_version=4.5&sort=updated).
- GitHub search: arcaneenergy/godot-multimesh-scatter (MIT, about 91 stars, last push 2024-01), Mattiny/yamms (MIT, about 47 stars, 2025-03) and a few near-zero-star repos — [GitHub search API](https://github.com/search?q=godot+scatter+multimesh&type=repositories).

### Inferences
- ProtonScatter is the best of these addons to *learn from*. Its modifier set, Poisson implementation, projection modifier, chunk splitting, thread-based rebuild and cache resource are all close to what we need, and the MIT licence lets us copy code with attribution.
- Adopting ProtonScatter wholesale brings risk **[judgement]**:
  - It is maintained by one person with no CI; the author removed the workflow.
  - There are no tagged releases for 3 years.
  - A cluster of issues follows each minor Godot version (4.3, 4.6, 4.7).
- Spatial Gardener and Terrain3D both fail our key constraint, which is multi-MeshInstance tree scenes with per-mesh shadow settings.
- None of the surveyed addons does "draw a polygon by clicking on the ground" as its primary UX. ProtonScatter's Path shape is the closest: you edit a Path3D curve with handles.

### Gaps
- ScatterShot (2026, Godot 4.5) was not inspected beyond its Asset Library listing. Its features and repo are unknown.
- I found no published performance benchmarks with numbers for any of these addons.
- I did not verify whether ProtonScatter main currently loads cleanly in 4.7.2. The only evidence is the 2026-07-26 commit message plus issue #255 (open, pre-fix).

---

## Q2. How well do the existing addons fit our three-mesh trees with per-mesh shadow settings?

### Takeaway
ProtonScatter's MultiMesh mode **merges every MeshInstance3D of a scene into one mesh** and applies **one** cast-shadow setting per item. Our canopy_cards would therefore cast shadows, or the trunk and core would lose theirs, unless we either split the tree scene into separate items or discard cards in the shadow pass with `IN_SHADOW_PASS`. Terrain3D and Spatial Gardener do not support multi-mesh scenes at all.

### Cited Findings
- ProtonScatter's `get_merged_meshes_from(item)` gathers all MeshInstance3Ds under the item scene:
  - Surfaces are grouped by material and merged into a single MeshInstance (up to 8 surfaces).
  - With more than 8 unique materials, "everything will be merged into a single surface. Material and custom data will NOT be preserved".
  - Source: [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).
- `get_or_create_multimesh[_chunk]` makes **one MultiMeshInstance3D per item (per chunk)** and applies `mmi.set_cast_shadows_setting(item.override_cast_shadow)` plus `material_override = get_final_material(...)` to it. It does not copy the source's `extra_cull_margin` — [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).
- In ProtonScatter, `use_colors` / `use_custom_data` are only enabled when the item has a `custom_script` — [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).
- Godot spatial shaders expose `IN_SHADOW_PASS`, "true when the shader is being rendered in a shadow mapping pass" — [Spatial shader reference](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html). In 4.7 it maps to `scene_data_block.data.flags & SCENE_DATA_FLAGS_IN_SHADOW_PASS` — [scene_shader_forward_clustered.cpp @4.7-stable](https://github.com/godotengine/godot/blob/4.7-stable/servers/rendering/renderer_rd/forward_clustered/scene_shader_forward_clustered.cpp).
- Terrain3D: multi-mesh objects "require combining into single meshes" — [Terrain3D docs](https://terrain3d.readthedocs.io/en/latest/docs/instancer.html). Spatial Gardener: sub-mesh scene instancing is still an open feature request — [#83](https://github.com/dreadpon/godot_spatial_gardener/issues/83).

### Inferences
- With ProtonScatter, the tree would become one MMI per chunk with three surfaces (cards, core, trunk) and one shadow setting. Two workarounds, both **[judgement]**:
  - (a) Add `if (IN_SHADOW_PASS) discard;` in the card shader. Put it in fragment, or collapse the vertex to zero-area in vertex, which is cheaper. Then set the item to cast shadows ON. The shadow pass still processes card vertices, which wastes some work but is simple.
  - (b) Fork the item logic so it emits one MMI per source MeshInstance with shared transforms. At that point we are rewriting the part of the addon that matters.
- Because merging picks the *materials*, our ShaderMaterials survive the merge: 3 unique materials is fewer than 8. Per-surface culling margin is lost. Cards that are displaced by wind in the vertex shader may pop at screen edges unless we set `MultiMesh.custom_aabb` or `extra_cull_margin` on the MMI. The addon exposes neither per item **[judgement from code read]**.

### Gaps
- I did not measure the shadow-pass cost of approach (a) against separate MMIs.

---

## Q3. How do you implement the in-editor polygon tool in Godot 4.7 (EditorPlugin, gizmos, click-to-place on ground, undo/redo)?

### Takeaway
Use an `EditorPlugin` that `_handles()` a custom `@tool class_name BiomeArea extends Node3D` storing `PackedVector2Array polygon` (local XZ). Add points with `_forward_3d_gui_input` by raycasting from the viewport camera. Drag vertices with an `EditorNode3DGizmoPlugin` (handles plus `_commit_handle` undo). Wrap every mutation in `EditorUndoRedoManager` actions. Everything below is standard, documented API.

### Cited Findings
- `_forward_3d_gui_input(viewport_camera: Camera3D, event: InputEvent) -> int` returns:
  - `AFTER_GUI_INPUT_PASS` (0): forward to other plugins.
  - `AFTER_GUI_INPUT_STOP` (1): consume.
  - `AFTER_GUI_INPUT_CUSTOM` (2): pass to other plugins but not the main Node3D editor.
  - It is only called when `_handles(object)` returns true for the edited object, which also triggers `_edit(object)` / `_make_visible(visible)`.
  - `_forward_3d_draw_over_viewport(control)` plus `update_overlays()` draw 2D overlays on the viewport.
  - `get_undo_redo()` returns the `EditorUndoRedoManager`.
  - `add_custom_type(...)` and `add_node_3d_gizmo_plugin(plugin)` register types and gizmos.
  - Source: [EditorPlugin docs](https://docs.godotengine.org/en/latest/classes/class_editorplugin.html).
- `EditorNode3DGizmoPlugin`:
  - `_has_gizmo(node)` attaches the gizmo.
  - `_redraw(gizmo)`: "common to call EditorNode3DGizmo.clear() at the beginning … then add visual elements", using `add_lines`, `add_handles`, and `create_material` / `create_handle_material`.
  - `_set_handle(gizmo, id, secondary, camera, screen_pos)`: "the camera can be used to convert it to raycasts".
  - `_get_handle_value` provides the restore value.
  - `_commit_handle(..., restore, cancel)`: "creating an UndoRedo action for the change, using the current handle value as 'do' and the restore argument as 'undo'."
  - Source: [EditorNode3DGizmoPlugin docs](https://docs.godotengine.org/en/latest/classes/class_editornode3dgizmoplugin.html).
- `EditorUndoRedoManager`:
  - `create_action(name, merge_mode, custom_context, backward_undo_ops)` → `add_do_property/add_undo_property`, `add_do_method/add_undo_method`, `add_do_reference/add_undo_reference` → `commit_action(execute=true)`.
  - The history is chosen from the first operation's object: nodes go to the edited scene's history, built-in resources to their scene's history, external resources to global history.
  - "Only … editor plugins"; games use `UndoRedo`.
  - Source: [EditorUndoRedoManager docs](https://docs.godotengine.org/en/latest/classes/class_editorundoredomanager.html).
- Working reference: ProtonScatter's `scatter_gizmo_plugin.gd` and path shape gizmo (MIT) implement handles on a curve domain — [src/](https://github.com/HungryProton/scatter/tree/main/addons/proton_scatter/src).

### Inferences
**Recommended UX [judgement]**
- Select a BiomeArea. A small toolbar appears (added with `add_control_to_container(CONTAINER_SPATIAL_EDITOR_MENU, …)`) with Edit points / Add / Bake / Clear.
- In Add mode:
  - LMB: raycast `camera.project_ray_origin(pos)` / `project_ray_normal(pos)` against `get_world_3d().direct_space_state` using the ground collision mask. Fallback is intersecting the plane y=area.global_position.y.
  - Append the point in local XZ. Return STOP.
  - Shift+LMB on an edge inserts a point.
  - RMB or Delete removes the nearest point.
  - Enter or Escape exits.
- Handles are one per vertex. `_set_handle` re-raycasts to the ground for dragging. `_commit_handle` does `add_do_property(area, "polygon", new) / add_undo_property(area, "polygon", restore)`.
- Draw the outline with `add_lines`, sampling each edge's ground height so the line hugs the terrain. Optionally show a density preview (dots) of the next bake.

**Why a polygon rather than Path3D [judgement]**
- `PackedVector2Array` gives exact `Geometry2D.is_point_in_polygon`, `Geometry2D.triangulate_polygon` (area-weighted uniform sampling), `offset_polygon` (edge falloff and inner margins) and `clip_polygons` (exclusion holes, e.g. farm plots, paths, buildings).
- A Path3D gives free curve handles but needs baking to a polyline first, as ProtonScatter does with `PolygonPathFinder`.

**Exclusion [judgement]**
- Add `BiomeExclusion` child nodes (polygon or circle) and optionally a group `"scatter_blocker"`. Points inside these are removed.
- Test against existing ProtonScatter-like negative shapes, or `clip_polygons` the domain.

### Gaps
- There are no official docs or tutorials specifically on "click to place polygon vertices on terrain" in 4.7. The snippets above are composed from the API docs.
- Whether `direct_space_state.intersect_ray` is reliable from editor input callbacks, outside `_physics_process`, is not documented. ProtonScatter does its raycasts from editor code in practice, but I found no doc statement either way.

---

## Q4. How should placements bake to MultiMesh: per sub-mesh MMIs, chunking, visibility ranges, per-instance data, and the NODE_POSITION_WORLD question?

### Takeaway
Bake one MultiMeshInstance3D per (chunk × variant × sub-mesh), with the sub-meshes sharing the transform list. Copy each source MeshInstance's material_override, cast_shadow and extra_cull_margin onto its MMI. Use 32–64 m chunks with visibility_range_end per layer.

Important correction to the brief: **in Godot 4.7-stable Forward+ (single precision), `NODE_POSITION_WORLD` read inside `vertex()` already returns the per-instance position for MultiMesh instances.** Our tree shader's wind phase therefore works per tree without changes. Use `INSTANCE_CUSTOM` / `COLOR` for tint and variant data rather than for phase.

### Cited Findings
- MultiMesh has no per-instance culling. "Every single instance will always render (they are spatially indexed as one, for the whole object)". The guidance is to "create several MultiMeshes for different areas of the world". MultiMesh handles "up to millions of objects". The docs suggest allocating the max and using `visible_instance_count` — [Using MultiMesh](https://docs.godotengine.org/en/latest/tutorials/performance/using_multimesh.html); [MultiMesh class](https://docs.godotengine.org/en/latest/classes/class_multimesh.html).
- MultiMesh class notes:
  - `custom_aabb`: "Setting this manually prevents costly runtime AABB recalculations".
  - `use_colors` / `use_custom_data` can only change "when instance_count is 0".
  - "Once the maximum lights are consumed by one or more instances, the rest … will not receive any lighting".
  - Blend shapes are ignored.
  - No max instance count is documented.
  - Source: [MultiMesh class](https://docs.godotengine.org/en/latest/classes/class_multimesh.html).
- Visibility ranges:
  - Per GeometryInstance3D node, with begin/end plus margins (hysteresis).
  - Fade modes: Disabled (fastest), Self (alpha fade, costs), Dependencies.
  - "dithering transparency is faster to render compared to alpha blending".
  - `visibility_parent` for HLOD.
  - Source: [Visibility ranges](https://docs.godotengine.org/en/latest/tutorials/3d/visibility_ranges.html).
- The project's tree report already recommends "MultiMeshInstance3D chunks of about 32–64 m", notes that visibility range works per node, not per instance, and says to carry tint and phase in colour and custom data — reports/Stylized procedural trees for Godot.md §"Chunked MultiMesh, visibility ranges", which cites [wave_forge PR](https://github.com/AntonTegnelov/wave_forge/pull/158). Terrain3D independently uses 32×32 m cells — [Terrain3D docs](https://terrain3d.readthedocs.io/en/latest/docs/instancer.html). ProtonScatter defaults to 15 m chunks — [scatter.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter.gd).
- **NODE_POSITION_WORLD with MultiMesh (source-read, 4.7-stable):**
  - `scene_shader_forward_clustered.cpp` renames `NODE_POSITION_WORLD` → `read_model_matrix[3].xyz`.
  - In the vertex stage of `scene_forward_clustered.glsl`, `read_model_matrix` starts as `model_matrix` and, for multimesh, becomes `read_model_matrix = model_matrix * matrix;`, where `matrix` is the instance transform.
  - The guard is `#if !defined(USE_DOUBLE_PRECISION) || … || defined(MODEL_MATRIX_USED)`.
  - So in single-precision builds the vertex-stage NODE_POSITION_WORLD is the *instance's* world origin, and `MODEL_MATRIX` also includes the instance transform.
  - In the fragment stage, `read_model_matrix` is rebuilt from `instances.data[instance_index].transform`, the per-node transform, so it does **not** include the MultiMesh instance.
  - Source: [scene_shader_forward_clustered.cpp @4.7-stable L783](https://github.com/godotengine/godot/blob/4.7-stable/servers/rendering/renderer_rd/forward_clustered/scene_shader_forward_clustered.cpp); [scene_forward_clustered.glsl @4.7-stable L263, L341, L1315](https://github.com/godotengine/godot/blob/4.7-stable/servers/rendering/renderer_rd/shaders/forward_clustered/scene_forward_clustered.glsl).
- The spatial shader docs describe NODE_POSITION_WORLD only as "Node position, in world space" and do not say how it behaves with MultiMesh. INSTANCE_CUSTOM is "Instance custom data (for particles, mostly)" — [Spatial shader reference](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html). The documentation is silent where the source is explicit.
- Instance custom data is read from the same transform buffer (`instance_custom = transforms.data[offset]` when `sc_multimesh_has_custom_data()`) — [scene_forward_clustered.glsl @4.7-stable](https://github.com/godotengine/godot/blob/4.7-stable/servers/rendering/renderer_rd/shaders/forward_clustered/scene_forward_clustered.glsl).

### Inferences
**Shader change needed? [judgement, based on the source read]**
- None for wind phase under Forward+ with a single-precision build, since `tree.gdshaderinc` uses NODE_POSITION_WORLD only in `vertex()`.
- For robustness and readability, prefer `MODEL_MATRIX[3].xyz` in vertex, which has the same behaviour and is less surprising. Optionally add `INSTANCE_CUSTOM.x` as an explicit phase override, with `use_custom_data` on.
- Things that would break:
  - Reading NODE_POSITION_WORLD in `fragment()` or `light()` (e.g. per-tree hue) → pass a varying from vertex instead.
  - Double-precision builds, where the instance transform is not baked into read_model_matrix unless MODEL_MATRIX is used.
  - The Mobile/Compatibility renderers, which I did not check.
- **Verify with a quick A/B:** two instances in one MultiMesh should sway out of phase.

**Per-instance data layout [judgement]**
- `use_colors = true` → `COLOR` = tint (rgb) plus a spare (a).
- `use_custom_data = true` → `INSTANCE_CUSTOM` = (phase, scale01 / seasonal age, variant hash, reserved).
- Cost is +8 floats per instance on top of the 12 for TRANSFORM_3D.
- The shaders must multiply `ALBEDO *= COLOR.rgb` only for MultiMesh, or default the colour to white. `COLOR` in vertex is vertex colour × instance colour when use_colors is on. Our wind masks live in vertex colour, so **don't** put a tint in instance colour if the shader reads COLOR for wind. Use INSTANCE_CUSTOM for the tint instead.

**Multi-mesh scene → MMIs [judgement]**
- At bake time, instantiate each unique PackedScene once, `find_children("*", "MeshInstance3D")`, and record for each part:
  - `mesh`
  - `material_override` (or the per-surface overrides)
  - `cast_shadow`
  - `extra_cull_margin`
  - `transform` relative to the scene root
  - `gi_mode`
  - `layers`
- For each chunk and variant, create one MMI per part.
- Instance transform = `placement_xform * part_local_xform`.
- Copy `cast_shadow`, so cards get OFF and trunk/core get ON, plus `material_override` and `extra_cull_margin`. Set `multimesh.custom_aabb` = chunk bounds grown by the max canopy extent plus the wind margin, so the AABB is not recomputed.
- For 10 variants × 3 parts that is up to 30 MMIs per chunk. At about 30 m chunks on a village map this is a few hundred draw calls at most. Alternatives:
  - Merge variants per part into one MultiMesh by baking all 10 variants' cards into one mesh and selecting by instance: not possible, since a MultiMesh has one mesh.
  - Accept per-variant MMIs.
  - Reduce to fewer variants per area.

**Visibility ranges for a 5–40 m fixed 45° camera [judgement]**
- Ground cover (grass, flowers): `visibility_range_end` ≈ 35–45 m with a margin of 3–5 m, fade Disabled or a dither in shader.
- Bushes and rocks: ≈ 60 m.
- Trees: effectively unlimited, or 80–120 m, since trees define the skyline.
- With a max camera distance of 40 m, plus the view frustum at 45°, far visibility matters less than chunk culling. Choose chunk size so a screen covers about 4–9 chunks: **32 m for grass/flowers, 64 m for trees**.

**Instance counts [judgement]**
- A cozy village is likely to have hundreds to low-thousands of trees and tens of thousands of grass tufts. That is well inside MultiMesh limits. The bottleneck is overdraw from alpha-scissor cards and shadow casters, not instance count.
- Set grass `cast_shadow = OFF`. For bushes, consider shadows ON only for core blobs.

### Gaps
- Behaviour of NODE_POSITION_WORLD under the Mobile and Compatibility renderers was not checked in source.
- I did not verify empirically in-engine that the vertex-stage NODE_POSITION_WORLD varies per instance. This is a source read, so treat it as highly likely but test it.
- There is no documented hard maximum for MultiMesh instance_count or buffer size.

---

## Q5. How should we handle ground projection, collision for trees, and persistence (saving baked results vs regenerating at load)?

### Takeaway
Raycast in the editor at bake time against the ground's collision (mask-filtered) to get height and normal, then save the result. Store the baked MultiMesh resources as **external binary `.res` files per BiomeArea**, not inline in the `.tscn`. Only trees and big rocks get collision: simple cylinder or capsule shapes through PhysicsServer3D RIDs or a single StaticBody3D with many CollisionShape3D children per chunk. Grass and flowers get none.

### Cited Findings
- ProtonScatter projects with batched `PhysicsRayQueryParameters3D` raycasts. It offers `max_slope`, `align_with_collision_normal` and `exclude_mask`, and removes points on a miss — [project_on_geometry.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/modifiers/project_on_geometry.gd).
- ProtonScatter `keep_static_colliders` "Uses the Physics server directly instead of creating actual collision nodes": one `body_create()` in STATIC mode, plus sphere/box/capsule shape RIDs copied from the item's collision data — [scatter.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter.gd).
- ProtonScatter defaults to `force_rebuild_on_load := true` (regenerate on load) and offers `ScatterCache` (saves transforms to .res) for "faster scene loading times". Several of its open bugs are cache or load related (#213, #215, #238, #253) — [scatter.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter.gd); [issues](https://github.com/HungryProton/scatter/issues).
- Terrain3D instances have no collision — [Terrain3D docs](https://terrain3d.readthedocs.io/en/latest/docs/instancer.html). SimpleGrassTextured requires a StaticBody3D ground for painting — [README](https://github.com/IcterusGames/SimpleGrassTextured).
- Text scene files are human-readable but slower to load than binary. Large numeric data should be stored in binary resources rather than embedded in text — [TSCN file format docs](https://docs.godotengine.org/en/4.4/contributing/development/file_formats/tscn.html); [Godot forum: storing PackedFloat32Array](https://forum.godotengine.org/t/best-way-of-storing-and-loading-a-packedfloat32array-on-disk/91833).

### Inferences
**Serialization size (arithmetic) [judgement]**
- TRANSFORM_3D is 12 floats. With colours and custom data it is 20 floats = 80 bytes per instance in binary.
- In a text `.tscn`, a PackedFloat32Array is written as decimal text, roughly 8–12 chars per float, so about 200–250 B per instance.
- For each sub-mesh MMI the buffer is duplicated. A 3-part tree is 3 × 80 B = 240 B binary, or about 700 B as text.
- Estimates:
  - 2,000 trees ≈ 0.5 MB binary / 1.4 MB text.
  - 100k grass ≈ 8 MB binary / 20+ MB text.
  - Text in the main level scene hurts load time, VCS diffs and editor save time.
- Options:
  - (a) Save each chunk's MultiMesh with `ResourceSaver.save(mm, "res://…/biome_<id>/chunk_x_z_part.res")` and reference it by path.
  - (b) Store only the compact placement list (position, yaw, scale, variant idx, about 6 floats) in one `.res` per BiomeArea, and rebuild the MMIs in `_ready()`. This takes milliseconds with no physics, because height is already baked. It shares one transform list across parts, cutting the stored data by about 3× compared with (a).
  - **Recommend (b)**: store placements plus seed, and build MMIs at load from the placement list. The same rebuild path is used in the editor, and the `.tscn` diff stays small (BiomeArea polygon + seed + recipe).

**Regenerate at load vs bake [judgement]**
- Do not re-scatter at runtime with raycasts. The physics world may not be ready at `_ready`, it costs load time, and any change to terrain or collision would silently move every tree.
- Bake placements in the editor. Rebuilding render nodes from baked placements at load is fine.

**Tree collision [judgement]**
- Cozy farming needs the player to bump into trunks. One `StaticBody3D` per chunk with a `CylinderShape3D` of about 0.3–0.5 m radius per tree trunk is cheap. Jolt handles thousands of static shapes easily.
- Direct PhysicsServer3D RIDs, as ProtonScatter does, save nodes but need manual freeing and are invisible in the editor.
- For choppable or interactable trees (starfruit hero), keep real scene instances rather than MultiMesh. The scatterer can have a per-asset "instantiate as scene" flag.

**Ground source [judgement]**
- If the ground is a MeshInstance with a trimesh collider, a raycast is fine.
- If we adopt a heightmap terrain later, sample the heightmap directly: it is faster, deterministic and needs no physics.
- Use `max_slope` and a normal alignment factor (e.g. trees stay upright with 0–10% normal blend; rocks and grass get 60–100% normal blend).
- Reject points whose ray hits a non-ground layer, such as a house, fence or water. For the seaside village, also reject points below a shoreline height or inside a water polygon.

### Gaps
- I did not inspect the FarmGameGodot level to learn what the ground is (MeshInstance plus trimesh, CSG, or heightmap), so the projection backend choice stays open.
- Load time for rebuilding N MMIs from a placement list at `_ready` was not measured. It is expected to be small, but that is untested.

---

## Q6. What are the Godot-specific pitfalls: AABB and culling, shadows, editor performance, determinism?

### Takeaway
The main pitfalls are:
- One AABB per MultiMesh, so chunk it and set custom_aabb.
- One shadow setting per MMI, so split by sub-mesh.
- `use_colors` / `use_custom_data` locked once instance_count > 0.
- Slow node naming after `add_child`.
- The editor freezing on large rebuilds, so rebuild async and debounce.
- Determinism breaking if RNG draws depend on iteration order or on raycast outcomes.

### Cited Findings
- AABB: one AABB and all-or-nothing culling per MultiMesh; `custom_aabb` avoids "costly runtime AABB recalculations" — [MultiMesh class](https://docs.godotengine.org/en/latest/classes/class_multimesh.html); [Using MultiMesh](https://docs.godotengine.org/en/latest/tutorials/performance/using_multimesh.html).
- Lights: once per-object light limits are consumed, remaining instances get no lighting — [MultiMesh class](https://docs.godotengine.org/en/latest/classes/class_multimesh.html). Chunking also mitigates this for local lights such as lanterns in the village.
- Setting `use_colors` / `use_custom_data` requires `instance_count == 0`. ProtonScatter resets `instance_count = 0` before changing them — [MultiMesh class](https://docs.godotengine.org/en/latest/classes/class_multimesh.html); [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).
- `set_name` after `add_child` gave a "100x slowdown". ProtonScatter names nodes before adding them and defers `add_child` — [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).
- ProtonScatter rebuilds on a `Thread` and debounces with `_rebuild_queued` / `call_deferred` — [scatter.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/scatter.gd). Its issue tracker still shows freezes and crashes around duplication and large shapes (#236, #260, #211) — [issues](https://github.com/HungryProton/scatter/issues).
- Chunk-size bug: a chunk_dimension set in the editor was not applied in-game, and a small chunk size on a huge shape crashed (4.7.2rc, double precision) — [issue #260](https://github.com/HungryProton/scatter/issues/260).
- ProtonScatter owner handling: in the editor it sets `owner = get_tree().get_edited_scene_root()` so that generated nodes get saved — [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).

### Inferences
**Determinism [judgement]**
- Use one `RandomNumberGenerator` per (area seed, layer index). For example, `rng.seed = hash([area.seed, layer_idx])`, ideally with a stable custom hash such as FNV over integers rather than `hash()`. Treat `hash()` output as a possible cross-version risk; this is unverified.
- Draw a **fixed number of RNG values per candidate point**, whether or not the point is later rejected by the polygon test, raycast or slope test. That way editing the polygon or exclusions only changes points locally.
- Better: make the candidate pattern **world-anchored**. Run Poisson or a jittered grid per world cell, seeded by `hash(cell_x, cell_z, layer_seed)`, then clip to the polygon. Moving one vertex then only changes trees near that edge, which matters for designers.
- Choose variants with `TreeSet.pick(hash_of_cell_and_index)`, which matches the existing "stable key" design.

**Editor performance [judgement]**
- Never rebuild on every handle drag. Rebuild on `_commit_handle` or an explicit Bake button. During drag, show only the outline.
- Do sampling and raycasts on the main thread in batches, or use `WorkerThreadPool` for pure-math sampling. PhysicsDirectSpaceState access from threads is unsafe unless physics runs on a separate thread, so do raycasts on the main thread.
- Assign `multimesh.buffer` in one go (a PackedFloat32Array) instead of calling `set_instance_transform` per instance. This is much faster from GDScript.

**Generated nodes [judgement]**
- Option 1: set `owner` so the nodes save into the scene. With (b) above, don't: make them runtime-only children (no owner) rebuilt from the placement resource in both editor and game. This keeps the `.tscn` small and avoids stale duplicates. It is the same idea as ProtonScatter's cache, but simpler.

**Max instance counts [judgement]**
- There is no documented cap. Keep grass chunks under about 20–30k instances each so buffer uploads on rebuild stay snappy.

**Shadows [judgement]**
- For the fixed 45° camera, the tree cards should not cast shadows (current scene setting), and grass, flowers and small rocks should not either.
- Also consider `GeometryInstance3D.cast_shadow = SHADOWS_ONLY` proxy MMIs, using a simplified canopy core for shadow if the core is heavy.

### Gaps
- I found no documented thread-safety guarantees for editor-time `intersect_ray`.
- I did not test `hash()` stability across Godot versions for our seeding.

---

## Q7. Should we build our own biome scatterer or adopt an addon, and what should our own look like?

### Takeaway
**Build our own small tool (about 600–1,000 lines of GDScript [judgement]), borrowing algorithms from ProtonScatter (MIT, with credit).**

Reasons to build:
- Our hard requirements are polygon-drawn-on-ground UX, weighted bags of multi-MeshInstance PackedScenes, per-sub-mesh shadow and cull settings, a world-anchored deterministic seed tied to TreeSet, and compact baked placements.
- Each of those is either missing from or awkward in ProtonScatter.
- ProtonScatter carries upgrade-churn risk across Godot minors.
- Spatial Gardener and Terrain3D don't support multi-mesh trees.

Adopting ProtonScatter is a reasonable **stopgap** if we need forests this week. Pair it with the `IN_SHADOW_PASS` discard in the card shader and larger chunks (32–64 m).

### Cited Findings
- ProtonScatter merges sub-meshes and applies one shadow setting per item — [scatter_util.gd](https://github.com/HungryProton/scatter/blob/main/addons/proton_scatter/src/common/scatter_util.gd).
- Spatial Gardener has no sub-mesh scene support (#83), no maintainer commits since 2025-03, and its 4.5 and 4.7 compatibility comes via community PRs (#76, #84) — [GitHub](https://github.com/dreadpon/godot_spatial_gardener/issues).
- Terrain3D requires Terrain3D ground, single-mesh assets and no collision — [Terrain3D docs](https://terrain3d.readthedocs.io/en/latest/docs/instancer.html).
- All candidate code is MIT, so algorithms such as Poisson, projection and gizmo patterns can be reused with a licence notice — [ProtonScatter](https://github.com/HungryProton/scatter), [Spatial Gardener](https://github.com/dreadpon/godot_spatial_gardener), [SimpleGrassTextured](https://github.com/IcterusGames/SimpleGrassTextured), [Terrain3D](https://github.com/TokisanGames/Terrain3D).
- SimpleGrassTextured (MIT, active 2026) remains a reasonable option for *painted* grass detail if a brush is preferred over polygons — [GitHub](https://github.com/IcterusGames/SimpleGrassTextured).

### Inferences — implementation sketch [judgement throughout]

**Resources**
- `ScatterAsset extends Resource`:
  - `scene: PackedScene`, or `tree_set: TreeSet` (expands to its scenes)
  - `weight: float`
  - `min_scale`, `max_scale`, `yaw_random := true`
  - `align_to_normal: float` (0..1)
  - `sink: float` (push into the ground)
  - `collision_radius: float` (0 = none)
  - `as_scene_instance: bool` (for interactables)
  - `tint_jitter: Color`
- `ScatterLayer extends Resource`:
  - `assets: Array[ScatterAsset]` (the weighted bag)
  - `spacing_m` (Poisson radius) and `density` (0..1 keep ratio)
  - `edge_falloff_m` (fewer points near polygon edge, using `Geometry2D.offset_polygon` or distance to edge)
  - `max_slope_deg`, `ground_mask`, `blocker_mask`
  - `chunk_size_m` (32 for small, 64 for trees)
  - `visibility_end_m`, `visibility_margin_m`
  - `cast_shadows_override` (Use source / Off)
- `BiomeRecipe extends Resource`: `layers: Array[ScatterLayer]`, evaluated in order, so trees first, then bushes with a min distance from trees, then grass.
- `BiomeBake extends Resource`:
  - per layer: `PackedFloat32Array placements` (x, y, z, yaw, scale, nx, nz …) and `PackedInt32Array asset_idx`
  - `source_hash` (polygon + recipe + seed) to detect stale bakes
  - saved as `.res` next to the level.

**Nodes**
- `@tool class_name BiomeArea extends Node3D`:
  - `@export var polygon: PackedVector2Array`, `@export var seed: int`, `@export var recipe: BiomeRecipe`, `@export var bake: BiomeBake`.
  - On `_ready()` (editor and game), call `_build_render()`. This creates non-owned child MMIs per (layer, chunk, variant, part) plus one StaticBody3D per chunk for trunk colliders.
- `@tool class_name BiomeExclusion extends Node3D`: a polygon or circle hole. Also honour global group `"scatter_blocker"`, such as farm plots and roads.

**Editor plugin (`addons/biome_scatter/plugin.gd`)**
- `_enter_tree`: `add_custom_type("BiomeArea", "Node3D", …)`, `add_node_3d_gizmo_plugin(BiomeGizmo.new())`, and a toolbar control in `CONTAINER_SPATIAL_EDITOR_MENU` with [Draw] [Bake] [Clear] [Seed ↻].
- `_handles(o)`: `o is BiomeArea or o is BiomeExclusion`.
- `_forward_3d_gui_input(cam, ev)`: in Draw mode, LMB → raycast ground → `undo.create_action("Add biome point")`, `add_do_property(area, "polygon", new)`, `add_undo_property(area, "polygon", old)`, `commit_action()` → `AFTER_GUI_INPUT_STOP`. Shift-click inserts on the nearest edge. RMB deletes.
- `BiomeGizmo extends EditorNode3DGizmoPlugin`:
  - `_redraw`: outline via `add_lines` at ground height, plus vertex `add_handles`.
  - `_set_handle`: ground raycast.
  - `_commit_handle`: undo action (per docs).
- The Bake button runs, as one undo action, `do: area.bake = new_bake; area._build_render()` and `undo: old bake`.

**Bake data flow**
1. Build polygon(s) in world XZ and subtract exclusions with `Geometry2D.clip_polygons`.
2. For each layer, take the polygon's world-cell grid (cell = 2×spacing). For each cell, `rng.seed = hash(seed, layer, cx, cz)` and generate Poisson or jittered candidates (fixed draw count).
3. Keep a candidate if it is inside the polygon (`Geometry2D.is_point_in_polygon`), passes the density and edge-falloff test, and is clear of min-distance to earlier layers (spatial hash).
4. Raycast down from `max_y` on `ground_mask`. Reject on a miss, a blocker hit, slope > max, or y < sea level.
5. Pick the asset by weighted draw (cumulative weights, deterministic `rng.randf()`). For a TreeSet, `pick(hash(cx, cz, i))`.
6. Store `(pos, yaw, scale, normal)` and the asset index in `BiomeBake`, then `ResourceSaver.save`.

**Render build (editor and runtime, no physics needed)**
- Cache per asset scene a `parts` list of `{mesh, material, cast_shadow, extra_cull_margin, local_xform}` by instantiating once and freeing.
- Bucket placements into `Vector2i(floor(x/chunk), floor(z/chunk))`.
- For each (chunk, asset, part), create an MMI with:
  - `multimesh.transform_format = TRANSFORM_3D`, `use_custom_data = true` (phase, tint idx), then `instance_count = n`, then `buffer =` a packed array built in one go.
  - `custom_aabb` = chunk bounds + asset max extent.
  - `cast_shadow` from the part, or the layer override. `material_override`, `extra_cull_margin` and `visibility_range_end/margin` from the layer. `gi_mode` as the source.
- Colliders: per chunk, one StaticBody3D with CylinderShape3D children for assets with `collision_radius > 0`.
- For `as_scene_instance` assets, instantiate the PackedScene normally.

**Shader touch-ups**
- Keep `NODE_POSITION_WORLD` in vertex (per-instance in 4.7 Forward+ per the source read), or switch to `MODEL_MATRIX[3].xyz`.
- Add optional `INSTANCE_CUSTOM` usage for tint and seasonal variation behind a `uniform bool use_instance_custom`, because non-MultiMesh MeshInstances read INSTANCE_CUSTOM as zero (to verify).
- Do not put the tint in instance `COLOR` while the shader reads vertex COLOR for wind masks.

**Testing**
- A headless GUT/gdUnit-style test that bakes a fixed polygon plus seed and asserts an identical `placements` hash across runs.
- A second test: moving one vertex changes placements only within ~2 chunks.
- A render smoke test: MMI count, cast_shadow on the cards MMI is OFF, and the custom_aabb contains all instances.

**Effort estimate [judgement]**
- 1–2 days for the core: resources, polygon editing, bake, MMI build.
- +1 day for exclusions, edge falloff, colliders and polish.
- This compares favourably with forking ProtonScatter's item and chunk logic to preserve sub-mesh shadows and then tracking upstream across Godot upgrades.

### Gaps
- I did not assess ScatterShot (2026, 4.5+). Worth a 10-minute look before building, in case it already supports multi-part scenes.
- Actual draw-call and frame-time numbers for 30 MMIs per chunk on the target hardware are unmeasured. Profile after the first bake.

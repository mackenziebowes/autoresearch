# Rendering stylized/painterly foliage in Godot 4.x (target 4.7, Forward+, custom toon light())

Labels: **[V]** = verified against the cited page during this research; **[I]** = inferred / from general Godot knowledge, not confirmed by a fetched source in this session.

## 1. Leaf-card shaders: alpha modes, double-sided lighting, backlight/translucency, shadows

### Takeaway
Use alpha scissor (or alpha-to-coverage on top of scissor, with MSAA) for leaf cards: it stays in the opaque pipeline, casts shadows and sorts correctly. Alpha hash also casts shadows but is noisy. Real alpha blending cannot cast shadows. For translucency use the `BACKLIGHT` built-in (or the Forward+-only SSS transmittance), and in a custom `light()` handle the back side yourself using `FRONT_FACING` and `cull_disabled`.

### Cited Findings
- [V] Spatial shader built-ins (latest docs = 4.7 dev line): `ALPHA_SCISSOR_THRESHOLD`: "If written to on any branch, values below a certain amount of alpha are discarded." `ALPHA_HASH_SCALE`: "Alpha hash scale when using the alpha hash transparency mode. Defaults to 1.0." `ALPHA_ANTIALIASING_EDGE`: "The threshold below which alpha to coverage antialiasing should be used." `ALPHA_TEXTURE_COORDINATE`: "The texture coordinate to use for alpha-to-coverage antialiasing." — [Godot docs, Spatial shaders](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html)
- [V] `BACKLIGHT` (fragment, inout): "Color of backlighting (works like direct light, but it's received even if the normal is slightly facing away from the light)." `BACKLIGHT` can also be read inside `light()`. `FRONT_FACING`: "true if current face is front facing". `cull_disabled`: "Culling disabled (double sided)." `IN_SHADOW_PASS`: "true when the shader is being rendered in a shadow mapping pass". `NODE_POSITION_WORLD` is available (useful for per-tree canopy centre and wind phase). `light()` runs once per light and exposes `LIGHT`, `ATTENUATION` ("Attenuation based on distance or shadow"), `DIFFUSE_LIGHT`. — [Godot docs, Spatial shaders](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html)
- [V] StandardMaterial3D transparency modes: Alpha (blend) "can't cast shadows, and are not visible in screen-space reflections". Alpha Scissor "can cast shadows" and suits "foliage and fences". Alpha Hash dithers, still casts shadows, "suited for realistic-looking hair". Depth Pre-Pass suits "transparent grass or tree foliage". Alpha Antialiasing (Alpha Edge Blend / Alpha Edge Clip) smooths edges and needs MSAA 3D of at least 2x. Cull mode Disabled gives double-sided rendering, and Blender-exported materials come with culling disabled, which can cost performance. SSS is "only available in the Forward+ renderer". — [Godot docs, StandardMaterial3D](https://docs.godotengine.org/en/latest/tutorials/3d/standard_material_3d.html)
- [V] The official "Making trees" tutorial uses `cull_disabled`, `world_vertex_coords` (so wind stays coherent across duplicated trees), `SSS_TRANSMITTANCE_COLOR = transmission.rgba;` for backlit leaves, and an alpha pre-pass for leaves so they sort and shadow correctly. — [Godot docs, Making trees](https://docs.godotengine.org/en/stable/tutorials/shaders/making_trees.html)
- [V] Community tip: set `ALPHA_SCISSOR_THRESHOLD` below 0.5 so cards don't thin out at distance. Values closer to 0 make leaves look "fluffier" up close. — [godotshaders: Simple, cheap stylized tree shader (J Hell, MIT)](https://godotshaders.com/shader/simple-cheap-stylized-tree-shader/)
- [V] The "Stylized Fluffy Tree Leaves" shader uses unshaded + alpha-to-coverage, fresnel rim, and a `flip_normal` toggle for double-sided leaves. — [godotshaders: Stylized Fluffy Tree Leaves (Miu, CC0, 13 Jun 2025)](https://godotshaders.com/shader/stylized-fluffy-tree-leaves/)

### Inferences
- [I] Custom `light()` recipe for toon cards: in `fragment()`, flip the normal when `!FRONT_FACING` (or keep the canopy-sphere normal for both sides, which usually looks better for toon). Set `BACKLIGHT = leaf_translucency_color`. In `light()`, compute `NdotL = dot(NORMAL, LIGHT)`, band it with a ramp texture, then add `BACKLIGHT * clamp(-NdotL,0,1) * ATTENUATION * LIGHT_COLOR` (a cheap wrap/translucency term). Godot does not apply BACKLIGHT automatically once you override `light()`, so you have to add the term yourself.
- [I] A custom `light()` stops the engine's default diffuse/specular, so shadows only reach you through `ATTENUATION`. Posterize `ATTENUATION` too, so shadow edges on foliage match the toon bands.
- [I] Shadows from alpha-cut cards: the shadow pass runs `fragment()` alpha, so scissor discards also show up in the shadow map. Billboarding/vertex offset code that depends on the camera will orient cards toward the *light* camera in the shadow pass. Use `IN_SHADOW_PASS` to disable billboarding there, or accept the look. For soft painterly results, many devs turn off cast shadows on leaf cards (use a cheap proxy shadow caster or blob shadows) to avoid noisy, dotted shadows.
- [I] Alpha hash plus TAA/FSR2 gives soft dithered edges at low cost. Without TAA it looks noisy. Alpha-to-coverage is only worth it if you already run MSAA.

### Gaps
- No fetched source documents exactly how `BACKLIGHT` interacts with an overridden `light()` in 4.7. Test it.
- Could not verify that 4.7 changed anything about alpha modes. The 4.7 release notes fetched below do not mention it.

## 2. Soft toon foliage normals (spherized normals, Blender → glTF → Godot vs in-shader)

### Takeaway
There are two workable routes: (a) author spherized/transferred normals in Blender (Data Transfer modifier from a sphere/blob proxy) and export them as the glTF NORMAL attribute, or (b) compute them in the vertex shader from a canopy centre. Route (b) is more robust in Godot because it survives LOD generation, compression and blend-shape problems, and it works on procedural meshes.

### Cited Findings
- [V] A Godot forum user who transferred custom normals from a proxy mesh via Blender's Data Transfer modifier reported that only base normals showed up in Godot. Blend shapes also break custom normals. The thread has no confirmed solution. — [Godot Forum: How to get custom normals into Godot?](https://forum.godotengine.org/t/how-to-get-custom-normals-into-godot/112350)
- [V] Other threads report normal discrepancies between Blender and Godot glTF imports. — [Godot Forum: GLTF imports differ greatly from Blender](https://forum.godotengine.org/t/gltf-imports-differ-greatly-from-blender-uv-and-normals/53024); [Godot Forum: Godot's gltf importer breaks normals](https://forum.godotengine.org/t/godots-gltf-importer-breaks-normals/86623); [godot#41514 Importing GLTF files with correct shading](https://github.com/godotengine/godot/issues/41514)
- [V] Blender's glTF exporter has a "Normals" option (and Tangents) under geometry export. — [Blender manual glTF 2.0](https://docs.blender.org/manual/en/2.80/addons/io_scene_gltf2.html)
- [V] `NODE_POSITION_WORLD` is a built-in and gives the object origin in world space, usable as the canopy centre when the tree origin sits at the canopy. — [Godot docs, Spatial shaders](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html)

### Inferences
- [I] glTF stores per-vertex normals, so Blender custom split normals are exported as NORMAL when "Normals" is enabled (and modifiers are applied on export). Godot uses the imported normals rather than recomputing them when they are present. Common failure causes: (1) the Data Transfer modifier isn't applied or "Apply Modifiers" is off, (2) shape keys are present (the forum case), (3) older Blender versions need Auto Smooth enabled for custom normals to exist. Also check the import dock: "Generate LODs" makes simplified meshes that can alter normals on low LODs. Consider disabling it on foliage and supplying your own LODs/impostors.
- [I] In-shader alternative (robust, no import risk): in `vertex()`, `NORMAL = normalize(VERTEX - canopy_center_local);` where `canopy_center_local` is a uniform, or per-vertex data baked into `COLOR.rgb`/`UV2` (for multi-blob canopies, bake each vertex's blob centre offset). Blend with the card's own normal (`mix(NORMAL, sphere_n, spherize)`) to keep some leaf breakup. Godot's glTF import maps COLOR_0 → `COLOR` and TEXCOORD_1 → `UV2`. Extra attributes may map to CUSTOM0–3, but this is unverified (see Gaps).
- [I] For toon shading with billboarded "fluffy" cards (the j-hell / Miu technique), the spherized normal is effectively required: the cards rotate toward the camera, so their geometric normals are meaningless for lighting.

### Gaps
- No fetched source confirms how Godot 4.x imports extra glTF attributes (TEXCOORD_2+, custom `_ATTR` names) into CUSTOM0–3. Test with a sample file.
- No confirmed statement found on whether the 4.2+ vertex compression (octahedral normal packing) visibly degrades spherized normals. Probably negligible [I].

## 3. Wind: vertex wind, global params, per-instance data, hierarchical sway

### Takeaway
The standard Godot approach combines three things: Blender-painted vertex colour masks (R = overall sway from trunk base to top, G = branch-tip/leaf flutter), `world_vertex_coords` or a world-position-derived phase to desync trees, and a `global uniform` for wind direction/strength that a script updates via `RenderingServer.global_shader_parameter_set()`.

### Cited Findings
- [V] The official tutorial paints vertex colours in Blender, uses red intensity for sway magnitude, and uses sine-based displacement, e.g. `VERTEX.x += sin(VERTEX.x * sway_phase_len * 1.123 + TIME * sway_speed) * strength;`. `world_vertex_coords` keeps wind consistent across duplicated trees. — [Godot docs, Making trees](https://docs.godotengine.org/en/stable/tutorials/shaders/making_trees.html)
- [V] Victor Karp's Godot 4 foliage wind (28 Jan 2025, updated 26 May 2025): the R vertex-colour gradient drives trunk-to-top sway and the G gradient isolates branch-tip movement. FastNoiseLite textures panned over time provide the motion. Tree world position goes into a random range to desync trees. A model-matrix transform keeps rotated trees swaying in the same world direction. Shader shared as code and graph, attribution "welcome but not required". — [victorkarp.com: Shader based foliage wind in Godot 4](https://victorkarp.com/godot-foliage-wind/)
- [V] Global and instance uniforms (Godot 4.0+): `global uniform vec4 my_color;` is defined in Project Settings, with no stated count limit. `instance uniform` gives per-node values without duplicating materials, with a practical maximum of 16 per shader, no textures, and optional `instance_index(n)`. — [Godot blog: global and per-instance shader uniforms](https://godotengine.org/article/godot-40-gets-global-and-instance-shader-uniforms/)
- [V] `INSTANCE_CUSTOM` ("Instance custom data (for particles, mostly)") and `INSTANCE_ID` are vertex built-ins. — [Godot docs, Spatial shaders](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html)
- [V] Example project pattern: a MultiMesh carries each instance's phase (hashed from id) and stiffness as custom data, and a vegetation shader bends plants downwind using those plus a global wind parameter. — [AntonTegnelov/wave_forge PR #158](https://github.com/AntonTegnelov/wave_forge/pull/158)
- [V] FaRu85/Godot-Foliage adds a "wiggle" layer on top of wind (credits Rainware for wind and Nekoto for wiggle). — [GitHub FaRu85/Godot-Foliage](https://github.com/FaRu85/Godot-Foliage)

### Inferences
- [I] Hierarchical sway recipe: (1) trunk bend = `wind_dir * strength * pow(height01, 2)` with height taken from vertex colour R (or computed from `VERTEX.y / tree_height`). (2) Branch sway = sine/noise with phase from `COLOR.b` (per-branch random, baked in Blender) scaled by G. (3) Leaf flutter = high-frequency noise × G (or × UV2 "distance from branch"). The per-tree phase comes from `NODE_POSITION_WORLD` for MeshInstances or `INSTANCE_CUSTOM.a` for MultiMesh. Apply displacement in world space (`MODEL_MATRIX`), or use `world_vertex_coords`.
- [I] `instance uniform` does not give per-instance values inside a MultiMesh (it is per GeometryInstance3D node). Use MultiMesh `use_colors`/`use_custom_data` (`COLOR`/`INSTANCE_CUSTOM`) for per-tree tint and phase.
- [I] For a toon look, keep wind amplitude low and frequency slow on canopies. Painterly games usually sway whole leaf clumps rather than individual cards. Offsetting the billboard cards' UV noise gives "shimmer" without big vertex motion.

### Gaps
- No licence was found for the wave_forge example (check the repo's licence before copying).

## 4. Scale: MultiMesh, instance data, visibility ranges/HLOD, impostors, occlusion

### Takeaway
Use MultiMeshInstance3D (or a scatter addon) per tree species/chunk, with per-instance colour and custom data for tint and phase. Use `visibility_range_*` with dither fade for LOD/HLOD, and add an octahedral impostor tier via the MIT `zhangjt93/godot-imposter` plugin for Godot 4. Occlusion culling (OccluderInstance3D) helps mainly for terrain/buildings, and leaf cards make poor occluders.

### Cited Findings
- [V] Visibility ranges: Begin/End distances plus Begin/End Margin for hysteresis or fade. Fade modes are Disabled (fastest), Self ("Uses alpha blending to smoothly fade"), and Dependencies (works with Visibility Parent for HLOD). HLOD lets "a single larger mesh ... replace several smaller meshes, so that the number of draw calls can be reduced at a distance, but culling opportunities can be preserved when up close". Recommended: simpler materials on distant LODs, and dithering instead of alpha blending for fades because it is "faster to render compared to alpha blending". — [Godot docs, Visibility ranges (HLOD)](https://docs.godotengine.org/en/latest/tutorials/3d/visibility_ranges.html)
- [V] zhangjt93/godot-imposter: octahedral impostors for Godot 4.x, MIT, verified on 4.5.beta5, last activity around Aug 2025, 169 stars. Adds an "Imposter" button in the 3D toolbar for GeometryInstance3D. Limitations: baking resets the node transform, and batch processing is disabled. — [GitHub zhangjt93/godot-imposter](https://github.com/zhangjt93/godot-imposter)
- [V] The original wojtekpil/Godot-Octahedral-Impostors is Godot 3 only and not ported to 4. bongblender/godot4-Octahedral-Imposters is a shader-only Godot 4 projection shader. There is also an open proposal for built-in impostors (godot-proposals #5351). — [wojtekpil repo](https://github.com/wojtekpil/Godot-Octahedral-Impostors); [bongblender repo](https://github.com/bongblender/godot4-Octahedral-Imposters); [Proposal #5351](https://github.com/godotengine/godot-proposals/issues/5351)
- [V] SimpleGrassTextured: MIT, Godot 4, MultiMesh-based, editor painting tool, interactive grass (`SimpleGrass.set_interactive(true)`), custom meshes, options to disable shadows, bake heightmaps and set LOD bias. 574 stars, 95 commits. — [GitHub IcterusGames/SimpleGrassTextured](https://github.com/IcterusGames/SimpleGrassTextured)

### Inferences
- [I] For a 45° third-person camera, view distances are moderate and trees are seen mostly from above. Impostors need the upper hemisphere (hemi-octahedral mode is enough if the plugin supports it). A cheaper "2–3 crossed cards" LOD with a baked toon albedo may be enough for the far band.
- [I] Chunk the world into MultiMesh cells (e.g., 32–64 m) so frustum culling still works. One huge MultiMesh is culled as a single AABB. Visibility range works per MultiMeshInstance3D node, not per instance, which is another reason to chunk.
- [I] For budget: disable shadow casting on leaf cards beyond near range (a separate LOD mesh with `cast_shadow = off`), keep trunk shadows, and use a shadow-only low-poly canopy proxy (`cast_shadow = shadows_only`) for soft blobby toon shadows.

### Gaps
- No benchmark numbers found for Godot 4 foliage (cards vs impostors vs MultiMesh counts).
- Did not verify whether godot-imposter works unchanged on 4.7, or whether it supports custom `light()` shaders on the impostor (probably not: impostors bake albedo/normal, so toon lighting would have to be re-applied in the impostor shader).

## 5. Existing Godot 4 addons/examples for stylized trees/foliage

### Takeaway
There is no single complete toon tree solution. The useful building blocks are the j-hell billboard "fluffy" technique (MIT) and Miu's CC0 extension, FaRu85's foliage shader, Victor Karp's wind, the official Making Trees tutorial, SimpleGrassTextured for ground cover, and godot-imposter for LOD.

### Cited Findings
- [V] "Simple, cheap stylized tree shader": J Hell, 5 Nov 2023, MIT. It billboards quads via `vec2 viewspace_offset = UV.xy - vec2(0.5); vec4 modelspace_offset = inverse(MODELVIEW_MATRIX) * vec4(viewspace_offset.xy, 0.0, 0.0); VERTEX += modelspace_offset.xyz;` with alpha-to-coverage. Requires quad meshes with fully unwrapped 0–1 UVs. Comments: "if the vertices are too far away from each other in model space, the illusion breaks", and some issues were reported on 4.1.1. — [godotshaders](https://godotshaders.com/shader/simple-cheap-stylized-tree-shader/)
- [V] "Stylized Fluffy Tree Leaves": Miu, 13 Jun 2025, CC0. Based on j-hell's shader: billboard_strength, wind, fresnel rim, alpha from the green channel, flip_normal, unshaded + A2C. It includes leaf textures and an Illustrator source. The GitHub mirror (TheMIU/Stylized-Fluffy-Tree-Shader, 13 stars, 2 commits) shows no licence file in the fetched view and notes it was partly made with ChatGPT help. — [godotshaders](https://godotshaders.com/shader/stylized-fluffy-tree-leaves/); [GitHub](https://github.com/TheMIU/Stylized-Fluffy-Tree-Shader)
- [V] FaRu85/Godot-Foliage: 125 stars, 11 commits. Quad-only meshes, UV X used to randomly rotate faces, red channel as leaf alpha, wind + wiggle + colouring. Credits Pontus Karlsson (base concept), Victoria Zavhorodnia (colouring), Rainware (wind), Nekoto (wiggle). FaRu's meshes/textures are "freely usable". The fetched page did not state a formal licence or Godot version. — [GitHub FaRu85/Godot-Foliage](https://github.com/FaRu85/Godot-Foliage)
- [V] godotshaders "foliage" tag lists more community shaders. The "Stylized Multimesh Grass Shader" is one of them. — [godotshaders foliage tag](https://godotshaders.com/shader-tag/foliage/); [Stylized Multimesh Grass Shader](https://godotshaders.com/shader/stylized-multimesh-grass-shader/)
- [V] Paid/itch option: "Godot 4: Tree wind shader Asset" (JobLab/Creative Core Studio) with transmission colour for backlit glow. Licence not checked. — [itch.io](https://joblab-studio.itch.io/godot-4-tree-wind-shader-asset)
- [V] Desarrh/Procedural-Tree-Godot: a @tool procedural pixel-art tree generator for Godot 4 (a different style, but it shows procedural generation in-editor). — [GitHub](https://github.com/Desarrh/Procedural-Tree-Godot)

### Inferences
- [I] The billboard fluffy-tree technique pairs naturally with a procedural Blender/Geometry-Nodes pipeline: scatter quads on a canopy volume, set each quad's UV to 0–1, and bake canopy-centre offset / wind masks into vertex colour. Then replace the "unshaded + fresnel" part with your custom toon `light()` using spherized normals (section 2). This keeps it consistent with the rest of the toon pipeline.
- [I] GDQuest publishes shader courses and open-source demos, but no specific GDQuest foliage/tree shader repo was found in this session.

### Gaps
- Last-commit dates for TheMIU, FaRu85 and SimpleGrassTextured could not be read from the fetched pages. Check GitHub directly.
- None of these were verified on 4.7 specifically.

## 6. Outlines on foliage in a Godot toon pipeline

### Takeaway
Inverted-hull outlines on alpha cards outline each quad rectangle, which looks wrong. The usual approaches are: (a) no outlines on leaves, only on trunks; (b) inverted hull on a smooth low-poly canopy *proxy* mesh (blob), rendered with a next-pass/second mesh, so the canopy silhouette gets a soft outline while the cards provide the painted breakup; (c) a screen-space post-process outline (depth/normal edge detection) that naturally follows alpha-cut silhouettes; (d) Godot 4.5+ stencil outline mode, which also needs smooth connected meshes.

### Cited Findings
- [V] Inverted hull: expand the mesh along normals, render only back faces (`cull_front`) in a solid colour, then render the object normally on top. — [toon-rp wiki: Inverted Hull Outline](https://github.com/Delt06/toon-rp/wiki/Inverted-Hull-Outline)
- [V] Godot 4.5 added a stencil-based Outline mode (and X-Ray) in StandardMaterial3D as an alternative to Grow. It assigns a preconfigured stencil material to Next Pass and, like Grow, requires meshes with "connected faces with shared vertices, or 'smooth shading'". — [Godot docs, StandardMaterial3D](https://docs.godotengine.org/en/latest/tutorials/3d/standard_material_3d.html); [godot-docs PR #11373](https://github.com/godotengine/godot-docs/pull/11373/files); [godotshaders: Stencil-based silhouette](https://godotshaders.com/shader/stencil-based-silhouette/)
- [V] Godot 4 cel/outline guides and a "Complete Toon Shader" exist on godotshaders for reference. — [supermatrix.studio guide](https://supermatrix.studio/blog/creating-a-stylized-3D-cel-shader-in-godot-4-from-scratch); [Complete Toon Shader](https://godotshaders.com/shader/complete-toon-shader/)

### Inferences
- [I] Stencil outlines on alpha-scissor cards: the stencil write happens where the fragment survives, so the grown outline pass would still be based on the card geometry (quads), not on the alpha shape. It is not a fix for leaf cards. It is only useful on the canopy proxy or trunks.
- [I] Screen-space outlines (a full-screen quad reading `hint_depth_texture` and `hint_normal_roughness_texture` in Forward+) are the most common Godot answer for foliage. Alpha-scissored and depth-prepassed cards write depth, so edges follow leaf shapes. Tune the depth threshold with distance to avoid noisy interiors, or run edge detection only on the depth discontinuities at canopy silhouettes. Painterly looks often skip interior foliage lines entirely.
- [I] Proxy-hull approach: a smooth icosphere/metaball canopy (the same one used to transfer spherized normals) shrunk slightly inside the cards, with an inverted-hull outline material and `cast_shadow` off. It gives a clean "cel" silhouette that hand-painted games (e.g., Genshin-like trees) approximate.

### Gaps
- No Godot-specific forum thread or breakdown was found that explicitly compares outline strategies for alpha-card foliage. The recommendations above are inferred from general technique knowledge.

## Appendix: Godot 4.7 notes (applies across questions)

### Takeaway
Godot 4.7 (2026) brings AreaLight3D, HDR output, inline text-shader previews, clearcoat changes and per-pass environment UBOs. Nothing found in its release notes changes alpha modes, MultiMesh, visibility ranges, glTF normals or `light()`, so the 4.3–4.5 behaviour described above should carry over.

### Cited Findings
- [V] 4.7 highlights: AreaLight3D (rectangle lights), HDR output, inline previews of text-based shader operations, clearcoat closer to the Disney reference, per-pass environment uniform buffers, and 3D particle scale/rotation tweaks. The fetched release page mentioned no glTF/MultiMesh/LOD/stencil/`light()` changes. — [Godot 4.7 release page](https://godotengine.org/releases/4.7/); [GameFromScratch: 4.7 beta](https://gamefromscratch.com/godot-4-7-beta-released/)
- [V] Stencil support landed in 4.5. — [Godot docs, StandardMaterial3D](https://docs.godotengine.org/en/latest/tutorials/3d/standard_material_3d.html)

### Inferences
- [I] AreaLight3D will call a custom `light()` like other lights. Test toon banding with it, since area-light `LIGHT` vectors may band differently.

### Gaps
- The exact 4.7 stable release date was not captured. The full changelog was not reviewed for shader-language additions.

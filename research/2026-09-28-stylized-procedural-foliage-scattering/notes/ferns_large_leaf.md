# Ferns and large-leaf plants (ferns, hosta, taro/elephant ear, monstera, banana shrub, agave/succulent rosettes) for the painterly Godot 4.7 pipeline

Scope note: the pipeline is the one described in `reports/Stylized procedural trees for Godot.md` (+ its 2026-09-28 addendum): Blender 5.2 Geometry Nodes generator (`FarmGameGodot/tools/tree/gn_tree.py`) -> glTF -> Godot 4.7 Forward+, painterly toon `light()` with brush-noise terminator and teal shadows, data in COLOR_0 / UV2, proxy normals carried by hand in a FLOAT_COLOR attribute and applied as `NORMAL = mix(NORMAL, proxy, spherize)`. Tags used below: **[sourced]** = backed by the linked source; **[EJ]** = engineering judgement / inference, not published practice; **[verified-src]** = checked by me directly in engine source code.

## 1. Frond construction: what replaces blob+cards for ferns

### Takeaway
Ferns are not canopies. They are a rosette of 6-20 **curved rachis curves** with **leaflets (pinnae) instanced along each curve**. This maps directly onto the palm row already in the tree report ("frond strips, B along frond, G per frond"), not onto the SDF blob + card fringe. Industry fern workflows (SpeedTree, Blender GN fern assets) all use spine + leaflets + curl. For a toon/Ghibli read at 5-40 m, the recommended LOD0 is **opaque geometric pinnae** (2-4 tris each) on a GN curve. An alpha-cutout ribbon ("frond card") is the LOD1/far option. ABOP supplies the arrangement maths (golden-angle rosette, compound-leaf L-systems), but a GN curve graph is more controllable than an L-system interpreter.

### Cited Findings
- SpeedTree's Frond generator "creates a mesh that lies along a branch spine" ([Unity SpeedTree docs](https://docs.unity3d.com/speedtree-modeler/manual/frond-generator.html)). Its properties are the useful parameter vocabulary for a GN frond. **Fold** "folds each side of the frond toward each other (much like folding a piece of paper)". **Curl** "curls both of the outside mesh edges in towards the spine". **Gravity** "pulls the edges of both sides ... toward the ground". **Roll** rotates the mesh around the spine. Start/End are 0-1 along the parent spine. Segment controls are "Accuracy" (segments per rib) and a minimum "Width" segment count. Lighting options are **Alignment** (vertex normals point along spine growth), **Spread** (normals point away from the spine) and **Smooth** ([SpeedTree frond properties](https://docs.unity3d.com/speedtree-modeler/manual/frond-generator-properties.html)). "Spread" is the built-in equivalent of our proxy-normal trick.
- Louis Sullivan (Electric Square), The Green Chapel: ferns were built in SpeedTree with trunk nodes set to "Spine only" to lay out the fern. Multiple trunk nodes gave layers "from the larger dead fronds on the bottom up to the young, curling ones at the top". A curl force was applied to the young leaves, and "individual leaves with anchor points" were added "to break up the flat look of the frond cards". The pinna/frond atlases were generated in Substance Designer using the fractal self-similarity of fern structure ([80.lv](https://80.lv/articles/creating-a-mysterious-scene-with-lifelike-foliage-using-speedtree)). Polycounts are not given.
- Alex Martinelli's free fern Geometry Nodes asset (Blender 4.3+) uses **recursive leaf distribution** for "the fractal-like quality of real ferns", with adjustable and randomizable roll, leaf count/angle/shape/scale/inclination, global noise, and circle or area distribution templates. It is free / name-your-price on Gumroad with "no usage restrictions". It must have modifiers applied or be baked before engine export ([Digital Production](https://digitalproduction.com/2025/06/23/procedural-ferns-in-blender/); [80.lv](https://80.lv/articles/generate-procedural-fern-like-plants-in-blender); [Gumroad](https://sagado.gumroad.com/l/ozxrlu)). No polycount is given. It is a realistic/Cycles-oriented asset, so mine it for graph ideas rather than output.
- A YouTube GN tutorial on fern unfurling manipulates curves with **Vector Rotate and Accumulate** nodes ([YouTube](https://www.youtube.com/watch?v=KNBBfg3eZac), from the search summary; not watched). A commercial GN "unfolding spiral" setup exists: "The curl", Superhive, paid ([Superhive](https://superhivemarket.com/products/the-curl---unified-unfolding-spiral-setup-in-geometry-nodes)).
- Fiddlehead biology and geometry: a fern frond develops from a tightly coiled spiral (fiddlehead, crozier, koru) and unfurls as it develops. The coil comes from outer-side cells elongating more than inner-side cells, and the shape is close to an equiangular (logarithmic) spiral ([Geometry Code](https://geometrycode.com/spirals-ferns-universal-unfolding-motifs/); [Te Ara](https://teara.govt.nz/en/photograph/10852/silver-fern-koru)).
- **ABOP (Prusinkiewicz & Lindenmayer), Ch. 4 Phyllotaxis.** Vogel's formula is φ = n·137.5°, r = c·√n. The divergence angle 137.5° = 360°·τ⁻² is the Fibonacci angle. The book shows that 137.3° and 137.6° give visibly wrong patterns. The L-system is `A(n) → +(137.5)[f(n^0.5)~D]A(n+1)`. The "cylindrical model" places organs on a stem at a constant divergence plus a vertical displacement ([ABOP ch.4 PDF](http://algorithmicbotany.org/papers/abop/abop-ch4.pdf)).
- **ABOP Ch. 5 Models of plant organs** has three approaches:
  - (a) **predefined surfaces** (bicubic patches placed with contact point + heading + up vector). "The majority of organs presented in this book have been modeled this way."
  - (b) **developmental surfaces** traced by the turtle. The book's "fern" (Fig 5.3) uses polygon leaves `L → {-FX+X-FX-|-FX+X+FX}`, `X → FX` with a 20° angle increment. It warns that tracing "produces acceptable effects only in the case of small, flat surfaces", and says a leaf blade built "as a union of triangles rather than a single polygon ... is advantageous if the blade bends".
  - (c) **compound leaves**: `A(d): d>0 → A(d-1); d=0 → F(1)[+A(D)][-A(D)]F(1)A(0); F(a) → F(a*R)`, with apical delay D and elongation rate R. Table values run from (D=0, R=2.0) to (D=7, R=1.17). Alternating pinnae use two apices A/B. "A change of 0.01 visibly alters proportions" ([ABOP ch.5 PDF](http://algorithmicbotany.org/papers/abop/abop-ch5.pdf)).
- Polycount forum consensus (search-summary only; the page returned 403 to my fetch): "something like a fern might be mostly geo while something like a tree with thousands of leaves would have alpha". Alpha brings overdraw and "no early z-rejection", and pushing extra polygons can be more efficient than alpha triangles, at the cost of harder edges ([Polycount](https://polycount.com/discussion/144594/foliage-alphas-vs-geometry); [Polycount](https://polycount.com/discussion/112248/alphas-vs-geo-for-foliage)).
- Godot runtime generation: SurfaceTool and ArrayMesh both suit static generated geometry. ArrayMesh is slightly faster with a harder API, and SurfaceTool offers `generate_normals()`/`index()` ([Godot docs, procedural geometry](https://github.com/godotengine/godot-docs/blob/master/tutorials/3d/procedural_geometry/index.rst); [SurfaceTool](https://github.com/godotengine/godot-docs/blob/master/tutorials/3d/procedural_geometry/surfacetool.rst)). I found no maintained Godot 4 GDScript fern or plant generator on GitHub.

### Inferences
- **[EJ] Three frond constructions, ranked for this art style:**
  1. **Geometric pinnae on a rachis curve (recommended LOD0).** Each pinna is a lanceolate "diamond" of 4 verts / 2 tris, or 6 verts / 4 tris with a centre crease so it can fold. Pinnae are instanced alternately or oppositely along the curve. Everything is opaque with a crisp cel silhouette, no alpha shimmer, full early-Z and depth prepass, and an inverted-hull outline works if a fern is ever interactable. A 14-frond fern × 18 pinna pairs × 2 tris ≈ 1,000 tris plus about 14×8×2 for rachis ribbons ≈ 1.2k tris. That fits the "bush 300-800 / palm 1-2.5k" band of the tree report.
  2. **Alpha-cutout ribbon ("frond card").** Make a 2-column × 6-10-row strip that follows the rachis, with width = 2 × max pinna length, V-fold on the centre column, and a painted pinnate outline in alpha. About 24-40 tris per frond, so 14 fronds ≈ 500 tris. This is cheaper in tris but brings alpha-scissor shimmer and thinning at 30-40 m, and overdraw. Use it as LOD1 at roughly 15-30 m.
  3. **Individually modelled pinnules (2-pinnate).** A painterly look does not want this. Pinnule detail should live in the pinna outline or texture, following the Oga "few detailed leaves" principle from the tree report.
- **[EJ] Generation method.** Use GN curves, not an L-system interpreter. The pipeline already has GN tooling and the ABOP L-systems mostly *encode* profiles that are easier to write as float curves. Transfer from ABOP:
  - the golden angle for frond yaw;
  - the compound-leaf "phase effect", where older (basal) pinnae are longer. In GN this is a length profile over spline parameter t, e.g. `L(t) = Lmax · sin(π·t^0.8)^0.6 · (1 − 0.3t)`, zero at the rachis tip, with a short bare stipe for t < 0.12-0.2.
- **[EJ] GN graph sketch (Blender 5.2; node names from general 4.x/5.x knowledge, introspect before hard-coding as the tree report advises):**
  1. `Points` (N fronds). Yaw = index·137.5° + jitter. Pitch = mix(pitch_inner≈70°, pitch_outer≈20°, rank^0.7). Length = mix(0.6, 1.0, rank).
  2. `Instance on Points` of a `Curve Line`, then `Realize`. `Resample Curve` to 12-16 points.
  3. `Set Position` with droop: offset −Z by `droop · length · t²` (quadratic, like the palm fronds), plus slight lateral S-noise.
  4. Fiddlehead fronds (1-3 central, and optionally in spring). Rotate each point about the frond's local side axis by an accumulated angle `θ(t) = curl · smoothstep(t0, 1, t)`, integrated with `Accumulate Field` over segment directions. This is the "Vector Rotate + Accumulate" approach. Use an equiangular spiral target (radius shrinks geometrically). Pinnae on the curled part are scaled to around 0.2 and folded flat.
  5. `Curve to Points` (count = pinna pairs × 2), or `Sample Curve` at t_i. Side = ±1 alternating (with half-step offset for alternate pinnae). Rotation = `Align Rotation to Vector` (X to tangent, then Z to frond "up") + side yaw of 50-70° away from the tip + fold (roll the two sides up by 10-30° to form a shallow V, i.e. SpeedTree "Fold").
  6. `Instance on Points` of the pinna mesh. Scale from the `Float Curve` L(t). Apply random ±8% length and ±5° yaw. Realize.
  7. `Store Named Attribute` for the wind and normal data listed in sections 3 and 4.
  8. `Set Shade Smooth`. Delete helper float2 attributes (the addendum warns they export as UVs).
- **[EJ] Rosette/crown rules for ferns:**
  - the outer ring is older: longer, lower pitch, droops more, darker and more yellow;
  - the inner ring is young: upright and brighter;
  - add 0-3 fiddleheads at the centre;
  - drop a few fronds randomly (skip 10-20% of golden-angle slots) so the crown does not look like a perfect radial star, which is the Fortnite "no parallel lines" rule from the tree report;
  - give the crown an asymmetric lean of 5-15° so the silhouette is not radially symmetric from the 45° camera.
- **[EJ] GDScript ArrayMesh** is only worth it if you want per-instance runtime variation (e.g. fiddleheads unfurling over game days). The same maths runs in about 150 lines of GDScript, but wind/normal attributes must be written identically to the Blender path. For static set dressing, keep everything in the Blender GN generator for one source of truth. For "unfurling as seasons pass", a cheaper route is 3-4 baked growth-stage variants swapped by a day counter.

### Gaps
- Tri counts or construction details for ferns in Ghibli-adjacent games (Ni no Kuni, AWL, Tunic) were not found. No public breakdown exists that I could locate.
- Alex Martinelli asset: the formal licence text was not seen (only the Digital Production paraphrase "no usage restrictions"). Check the Gumroad page before copying node groups.
- ABOP PDF redistribution terms were not verified. algorithmicbotany.org hosts the chapters publicly, but I did not find an explicit licence statement.
- Exact Blender 5.2 Python identifiers for `Align Rotation to Vector`, `Accumulate Field`, `Sample Curve` etc. were not verified live (Blender MCP was unavailable).

## 2. Large leaves (hosta, taro/elephant ear, monstera, banana) and succulent rosettes

### Takeaway
Build every big leaf from one parametric **blade function**: a midrib curve plus a half-width profile w(u), meshed as a small (u,v) grid (roughly 8-12 rows × 2-3 columns per half, 30-70 tris). Deform it in (u,v) space with V-fold, longitudinal arch/droop, edge ruffle and twist, then attach it to a petiole curve. The research leaf-modelling literature uses exactly these deformations (outline, camber, longitudinal bend, twist, V-fold). At a 45° camera, large holes and splits (monstera fenestrations, banana tears) should be **geometry**, not alpha, because they are large enough to read and opaque geometry avoids the alpha costs.

### Cited Findings
- Leaf-modelling literature (search-summary level): a leaf blade is decomposed into midrib, blade surface and veins, with the outline and margin defining shape. Outlines can come from the **Superformula** or from piecewise polynomial curves with control points. Deformations include **camber** (cross-sectional bending), **longitudinal bend** ("gravitational droop for dicots, upward arch for monocots"), **twist about the midrib**, and **V-fold**. Variation comes from random perturbation of the outline control points ([FloraForge, ScienceDirect 2026](https://www.sciencedirect.com/science/article/pii/S2772375526005411), full text 403 to my fetch; [UCT honours thesis, Foster 2012](https://projects.cs.uct.ac.za/honsproj/cgi-bin/view/2012/foster_mazzolini_pieterse.zip/Yggdrasil%20Webpage/Documents/Foster%20-%20Thesis.pdf); [Leaf modeling in leaf space, NUS](https://www.comp.nus.edu.sg/~leowwk/papers/gmp2012.pdf); [NeuraLeaf arXiv 2507.12714](https://arxiv.org/html/2507.12714v1)).
- ABOP: a blade built as a union of triangles in a framework from the midrib (cordate leaf, Fig 5.4-5.5) is preferred over a traced polygon "if the blade bends" ([ABOP ch.5](http://algorithmicbotany.org/papers/abop/abop-ch5.pdf)).
- SpeedTree's frond Fold / Curl / Gravity / Roll are the same deformation set applied to a spine-following strip ([SpeedTree frond properties](https://docs.unity3d.com/speedtree-modeler/manual/frond-generator-properties.html)).
- Commercial stylized/low-poly tropical packs give tri budgets. "Tropical Plants Pack" (ArtStation) lists Banana, 5 variations at **691-1,433 tris**, and Monstera, 4 variations at **284-1,409 tris**, across 113 meshes of 27-3,426 tris with LODs ([ArtStation](https://www.artstation.com/marketplace/p/jjv9r/tropical-plants-pack), per search summary). A CGTrader stylized tropical pack uses one 512×512 gradient texture for everything ([CGTrader](https://www.cgtrader.com/3d-models/plant/leaf/stylized-tropical-plants-pack)).
- Polycount (search-summary level): large leaves like palm or banana "benefit from 3D normalmaps". That is realistic-pipeline advice. For our toon look, Alisavakis found normal maps on leaves "were just adding noise" (cited in the tree report: [Harry Alisavakis](https://halisavakis.com/my-take-on-shaders-stylized-tree-leaves/)).
- Dragos Matkovski, "The Illustrated Nature" (stylized): leaves are "a simple intersecting mesh of 3 quads", with normals edited "to point outwards" and smoothed **per group of leaves** rather than per whole plant, because "the light and shadow parts add more definition". Deliberately abstract 256×256 white alpha textures are coloured by material parameters ([80.lv](https://80.lv/articles/stylized-nature-vegetation-animation-shaders)).
- Hexbreaker (Unity, stylized marsh with lilypads, lotus, ferns): foliage is sculpted then baked to 2D cards "with some depth to them", with veins sculpted and baked to normal maps ([itch.io devlog](https://cfringerwtcc.itch.io/hexbreaker/devlog/544906/hexbreaker-how-to-create-foliage)). This is an example of the card path for small plants.

### Inferences
- **[EJ] Blade generator (shared by hosta, taro, monstera, banana, agave):**
  - Parameters: `length L`, `max_width W`, `profile w(u)` as a float curve (or a Superformula preset), `base_shape` (tapered / cordate / sagittate / peltate), `tip_acuminate`, `fold_deg` (V-fold at the midrib, 0-40°), `arch` (midrib curvature: droop for dicots; banana arches then droops), `camber` (edges curl up/down across v), `ruffle_amp/freq` (hosta/taro margin wave, sin along u scaled by |v|), `twist_deg`, `rows 8-14`, `cols_per_half 2-3`, `splits[]` / `holes[]`.
  - Mesh: build a (u,v) grid with u ∈ [0,1] along the midrib and v ∈ [−1,1] across, with x = v·w(u). Keeping the midrib as an actual edge loop makes the V-fold a clean crease that splits light and shadow. **Store u, v as attributes before any deformation.** Wind and shading both want them.
  - Cordate/sagittate base (elephant ear/taro): let w(u) extend to negative u, so the lobes run back past the petiole attachment. Real taro (*Colocasia*) is **peltate**: the petiole attaches inside the blade, about 1/4-1/3 from the notch. That changes the pivot and droop direction (the blade hangs tip-down like a shield). *Alocasia* "elephant ear" is often held more upright. The botany here is general knowledge, not from a fetched source.
  - Monstera: pinnatifid slits = remove grid columns between lateral-vein lines from the edge inward. Fenestrations (holes) = delete faces whose (u,v) falls inside hole ellipses placed between veins. At 45°/5-15 m a 1 m leaf's holes are 5-15 cm, several pixels wide, so they read as geometry. Snap the boundary verts of deleted faces onto the ellipse to avoid stair-steps, or accept the faceting as stylized.
  - Banana: long oblong blade (L/W ≈ 3-4), parallel lateral veins. "Tears" = cut the grid along 3-8 random vein lines (duplicate edges, split) and give each strip independent droop/flutter. This is the signature look and should be geometry.
  - Hosta: ovate-cordate, pronounced parallel curved veins (vein hints go in albedo/vertex colour), ruffled margin, arching petioles from a dense basal mound. Many leaves (12-30) with smaller blades (15-35 cm), so use the coarser grid (6 rows × 2 cols per half ≈ 24 tris per leaf).
  - Agave/beach succulent rosette: leaves are thick, so blades need **thickness**. Use a lens/triangular cross-section (3-4 sided extrusion) with a sharp tip and a spine, not a double-sided sheet. Arrange by golden angle with pitch from about 80° (centre) to 20-35° (outer). Outer leaves may recurve at the tip. Keep it opaque and single-sided, which gets back-face culling. Aloe/echeveria-like "beach succulents": use the same rosette with shorter, fatter leaves and Vogel r = c√n placement for a flat echeveria.
- **[EJ] Petioles:** use a curve with 3-4-sided `Curve to Mesh` profile (or a flat 2-tri ribbon with proxy normals below about 1 cm radius). Length and arch come from a float curve. The blade attaches at the petiole end, oriented by the petiole tangent + `blade_pitch`.
- **[EJ] Poly budget per plant at LOD0:**

  | Plant | Blades | Tris/blade | Petioles | Total |
  |---|---|---|---|---|
  | Hosta | 15-25 | ~24 | 15-25 × 12 | 700-1.2k |
  | Taro/elephant ear | 5-9 | 60-90 | 5-9 × 24 | 600-1k |
  | Monstera | 6-12 | 80-150 (holes add tris) | — | 1-2k |
  | Banana shrub | 6-10 | 100-200 including tears; pseudostem 150-300 | — | 1.2-2.5k |
  | Agave | 18-40 | 12-24 (thick) | — | 500-1k |

  These agree with the ArtStation pack's 284-1,433 tri range for monstera and banana.
- **[EJ] Why not alpha for these:** big leaves make large planar alpha areas mostly opaque anyway. Alpha scissor would only buy edge detail that a (u,v) outline already gives, while costing shimmer and the loss of opaque-pass benefits. Use alpha only on hosta/fern **far LODs** (merged cards) or for tiny margin serrations, which are not needed in this style.

### Gaps
- How Genshin, BotW, Tunic, Ni no Kuni or AWL actually model big-leaf plants: no primary breakdown found. The Genshin search results were fan/marketing articles with no technical content. Do not claim studio practice.
- FloraForge full text (paywall/403): the exact parametrisation and code licence were not read.
- Whether GN `Fill Curve` robustly triangulates outlines with holes into bendable meshes (it tends to produce long thin triangles with no interior verts) was not tested. That is why the (u,v)-grid + face-delete route is recommended.

## 3. Shading thin double-sided leaves and fronds under the painterly toon `light()`

### Takeaway
Godot **automatically negates NORMAL on back faces** when a material uses `cull_disabled`. With proxy/spherized normals this makes the underside of every frond and leaf light as if facing into the plant, so re-flip it in `fragment()` (`if (!FRONT_FACING) NORMAL = -NORMAL;`) to get the Meadows / Illustrated-Nature "both sides light identically from the proxy" behaviour. Use a **lower spherize** on big leaves than on trees so the midrib fold still splits each leaf into a lit half and a shadow half. That split is the key anime big-leaf read. Paint veins as low-contrast albedo/vertex-colour stripes and skip normal maps.

### Cited Findings
- **[verified-src]** Godot's forward-clustered scene shader contains `#if defined(DO_SIDE_CHECK) if (!gl_FrontFacing) { normal_highp = -normal_highp; }`, and `cull_disabled` and `cull_front` both define `DO_SIDE_CHECK` ([scene_forward_clustered.glsl](https://github.com/godotengine/godot/blob/master/servers/rendering/renderer_rd/shaders/forward_clustered/scene_forward_clustered.glsl), line ~1226; [scene_shader_forward_clustered.cpp](https://github.com/godotengine/godot/blob/master/servers/rendering/renderer_rd/forward_clustered/scene_shader_forward_clustered.cpp), lines ~873-874; checked on master, Sept 2026).
- Godot spatial-shader docs: `FRONT_FACING` is "true if current face is front facing". `BACKLIGHT` is "Color of backlighting (works like direct light, but it's received even if the normal is slightly facing away from the light)". `depth_prepass_alpha` does an "opaque depth pre-pass for transparent geometry". `ALPHA_SCISSOR_THRESHOLD` discards below a value ([Godot docs, spatial shaders](https://docs.godotengine.org/en/latest/tutorials/shaders/shader_reference/spatial_shader.html)).
- Meadows flipped back-face normals so both sides light identically, after transferring sphere normals (via the tree report: [80.lv Meadows](https://80.lv/articles/meadows-creating-stylized-nature-in-ue4)). Matkovski smooths normals **per leaf group**, not per plant, to keep light/shadow definition ([80.lv](https://80.lv/articles/stylized-nature-vegetation-animation-shaders)). SpeedTree exposes the same as frond normal "Spread" (away from spine) vs "Alignment" ([SpeedTree docs](https://docs.unity3d.com/speedtree-modeler/manual/frond-generator-properties.html)).
- Alisavakis: normal maps on stylized leaves "were just adding noise", and a third shadow band was barely visible (via the tree report, [halisavakis.com](https://halisavakis.com/my-take-on-shaders-stylized-tree-leaves/)).
- The Godot Stylized Fluffy Tree Leaves shader (MIT/CC0 lineage per the tree report) includes a "normal flipping for double-sided rendering" option and fresnel ([godotshaders](https://godotshaders.com/shader/stylized-fluffy-tree-leaves/); [GitHub TheMIU](https://github.com/TheMIU/Stylized-Fluffy-Tree-Shader)).

### Inferences
- **[EJ] Proxy choice per type** (mirroring the tree addendum's 80/20 per-clump/whole-tree):
  - **Fern:** proxy = a flattened ellipsoid centred at the crown, raised 30-40% of height. Spherize 0.6-0.8 on pinnae so the whole fern reads as one soft mound with a painted gradient. Add 0.2 of per-frond "spread" (normal away from the rachis, like SpeedTree Spread) so each frond still separates.
  - **Hosta:** mound proxy with spherize 0.4-0.6. Many small leaves should read as a mound.
  - **Taro, monstera, banana:** spherize 0.2-0.4 toward a per-leaf proxy (leaf centre offset along the leaf's up side by about 0.5 × width), plus 0.1-0.2 toward the plant proxy. This keeps the V-fold light/dark split and a smooth gradient across the blade instead of per-triangle facets.
  - **Agave:** real (closed, thick) geometry, so normal spherize is at most 0.2. Use cull_back and no flipping.
- **[EJ] Back-face rule for the shader:** with `cull_disabled`, write `if (!FRONT_FACING) NORMAL = -NORMAL;` *after* applying the proxy mix in `fragment()`, if you want both sides lit the same. Proxy normals are applied in `vertex()` and interpolate into NORMAL, which Godot then negates for back faces. For big leaves, a stylized alternative is to keep the flip and tint the back face a lighter, cooler, less saturated underside colour (`mix(albedo, underside_col, float(!FRONT_FACING))`). That gives the pale leaf-underside read of taro and hosta when wind flips a leaf. Test both in the lookdev contact sheet.
- **[EJ] Translucency:** reuse the report's custom term `BACKLIGHT * clamp(-NdotL,0,1) * ATTENUATION * LIGHT_COLOR`, computed with the **geometric** (unspherized) normal for big leaves so backlit banana and taro blades glow per leaf. Scale it by a vertex mask that is 0 on the midrib and petiole and 1 toward the blade edge. Ferns get a weaker glow (0.3-0.5).
- **[EJ] Vein hints without noise:**
  - Midrib = a stripe in the albedo from |v| < 0.04, slightly lighter than the blade.
  - Lateral veins = `smoothstep` on `fract((u − k·|v|)·N_veins)` at 5-10% value contrast, faded out with distance (a camera-distance uniform or `length(VERTEX)`).
  - Hosta gets parallel curved veins. Banana gets dense parallel veins, rendered as a faint stripe only. Monstera gets pinnate veins that line up with the holes and splits.
  - Veins are computed from the stored (u,v), so no texture is needed.
- **[EJ] Painterly texture:** reuse the brush-dab atlas projected in object space at about 25-35%, per the addendum's "blob with stickers" fix. Add a base-to-tip gradient (darker, cooler at the petiole/rachis base; warmer at tips) from B/u. Give older outer fern fronds a slight yellow/olive jitter.
- **[EJ] Shadow noise:** thin leaves self-shadowing through the fold create acne and flicker. Consider `cast_shadow` on for the plant and softening ATTENUATION posterization. Or use a proxy shadow mesh as in the tree report, for ferns and hostas, which are fine as a blobby shadow.

### Gaps
- Whether Godot's built-in `BACKLIGHT` does anything under an overridden `light()` is still untested (flagged in the addendum as well).
- Mobile/Compatibility renderer (GLES3) side-check behaviour was not checked; only Forward+ was.

## 4. Wind for fronds and big leaves

### Takeaway
Bake per-vertex **position along the frond/leaf (t)**, a **per-frond phase**, a **distance-from-midrib flutter mask** and the **frond base pivot**. Bend each frond as a rotation about its base, scaled by t², not a positional shear, and flutter the pinna and leaf tips at high frequency. This is the Crysis main + detail bending scheme already planned for palms.

### Cited Findings
- The Crysis/GPU Gems 3 scheme stores edge stiffness, per-leaf phase, overall stiffness and AO in vertex RGBA. It layers main bending, scaled by normalized height, with detail bending ([NVIDIA GPU Gems 3](https://developer.nvidia.com/gpugems/gpugems3/part-iii-rendering/chapter-16-vegetation-procedural-animation-and-shading-crysis)), as used in the tree report.
- Matkovski: a vertex-colour alpha gradient top-to-bottom drives bend strength, and a green channel drives noisier secondary motion for leaves ([80.lv](https://80.lv/articles/stylized-nature-vegetation-animation-shaders)).
- Tree-report channel layout: R = main sway, G = phase, B = parameter along branch, A = flutter; UV2 for a pivot. Per-instance phase comes from `NODE_POSITION_WORLD` or MultiMesh `INSTANCE_CUSTOM`. There is a `global uniform` wind vector (see the report's Godot wind section and its sources).

### Inferences
- **[EJ] Channel assignment for frond/leaf plants** (keeps compatibility with the tree shader):
  - **R** = plant-level sway, (z/height)^1.5. Small for ferns, near 0 for agave.
  - **G** = per-frond or per-leaf random phase (0-1).
  - **B** = t, the parameter along the frond or petiole+blade (0 at crown, 1 at tip; for big leaves, u remapped over petiole+blade).
  - **A** = flutter mask: for pinnae, distance from the rachis normalized by pinna length; for blades, |v| (edge) × u.
  - **UV2** = frond base pivot in object space (xy), with pivot z in a spare channel. Or bake the pivot into COLOR as in the addendum's proxy-normal encoding. That is a FLOAT_COLOR attribute, so a second colour attribute may be needed, which Godot maps to CUSTOM slots. Verify mapping with a test export; the tree report flagged CUSTOM0-3 mapping of glTF attributes as unverified.
- **[EJ] Vertex shader:**
  - Frond bend = rotate (VERTEX − pivot) about axis cross(up, wind_dir) by angle `bend = strength · B² · (0.6 + 0.4·sin(TIME·f + G·6.28))`. This keeps frond length and avoids the stretching of a pure offset.
  - Flutter = `A · amp_f · noise(TIME·f_hi + G)` along the local normal (≈ proxy normal). Keep it low amplitude for cozy.
  - Big leaves: add a slow "flap" rotation of the blade about the petiole tip, gated by u > u_petiole_end. Taro and banana get the most flap. Banana tear strips get their own phases, so store the strip id in G.
  - Agave: sway 0 and flutter 0; only the flower spike, if any, sways.
  - Fiddleheads: no wind.
- **[EJ] Normals under wind:** bending rotates geometry but the stored proxy normal does not rotate. At the small cozy amplitudes (<15°) this is invisible. If needed, rotate the NORMAL with the same rotation matrix.

### Gaps
- No published stylized-fern wind shader for Godot 4 was found. The above is adapted from the tree/palm scheme.

## 5. Performance in Godot 4.7 Forward+: tri budget, instancing, alpha scissor vs opaque geometry, overdraw

### Takeaway
With 45° views at 5-40 m and ferns, hostas and agaves in the hundreds, opaque geometric leaflets at 0.5-2k tris per plant in chunked MultiMeshes are cheaper overall and cleaner-looking than alpha cards. Reserve alpha scissor for LOD1/LOD2 merged frond cards. The evidence on overdraw cost is thin and partly contradictory, so profile rather than assume.

### Cited Findings
- Godot alpha modes: alpha blend cannot cast shadows. Alpha scissor can and "suits foliage and fences". Alpha hash dithers. Alpha-to-coverage needs MSAA ≥ 2× ([Godot docs, StandardMaterial3D](https://docs.godotengine.org/en/latest/tutorials/3d/standard_material_3d.html), via the tree report).
- Visibility ranges and chunked MultiMesh recommendations come from the tree report ([Godot docs, visibility ranges](https://docs.godotengine.org/en/latest/tutorials/3d/visibility_ranges.html)).
- Polycount (search-summary level): alpha foliage brings "overdraw and no early z-rejection", and more polygons can be more efficient than alpha triangles ([Polycount](https://polycount.com/discussion/144594/foliage-alphas-vs-geometry)). Tightly cutting card shape to the leaf "will reduce overdraw" ([Polycount](https://polycount.com/discussion/204716/foliage-polycount-vs-alpha-cutout-which-one-to-favour-in-trees)).
- Counterpoint: Eastshade's developer says they do not optimize foliage overdraw at all ("I don't even think about this ... I'm not going to modify the silhouette of my trees"). The one measured win they report was moving the vegetation shader from forward to deferred, which "shaved 30% off my render time" in Unity ([Eastshade Studios](https://eastshade.com/foliage-optimization-in-unity/)). That is a Unity finding, not directly applicable to Godot Forward+.
- Commercial stylized packs put monstera at 284-1,409 tris and banana at 691-1,433 tris with LODs ([ArtStation](https://www.artstation.com/marketplace/p/jjv9r/tropical-plants-pack)).

### Inferences
- **[EJ] Budgets (LOD0 / LOD1 / LOD2):**
  - Fern: 1-1.5k / 400-600 (alpha ribbon fronds) / 60-120 (3-5 crossed cards or low mound).
  - Hosta: 0.7-1.2k / 300 / low mound 60-100.
  - Taro: 0.6-1k / 300 / 80.
  - Monstera: 1-2k / 500 (holes removed; albedo-painted holes or alpha) / 100.
  - Banana: 1.2-2.5k / 600 / 150 crossed cards.
  - Agave: 0.5-1k / 250 / 80.
  - LOD bands as in the tree report: 15 m / 30 m. Many of these plants can drop straight to LOD2 beyond 30 m, since a 0.8 m fern is about 20-30 px at 40 m.
- **[EJ] Why opaque geometry is favoured here:** Forward+ depth prepass plus opaque leaflets get full early-Z, shadows without alpha discard, stable silhouettes under TAA/FXAA, and no mip thinning of alpha at distance. Alpha scissor keeps shadows but `discard` shaders lose some early-depth optimizations. This is a general GPU principle; I found no Godot-specific benchmark.
- **[EJ] Instancing:** one mesh per species-variant (4-8 variants per species from seeds), placed via chunked MultiMeshInstance3D (32-64 m chunks) with per-instance colour (tint jitter) and custom data (wind phase, scale). Hand-placed hero plants (garden hostas at a house) can be plain MeshInstance3D. Disable Godot auto-LOD and shadow meshes on import, per the addendum. Double-sided leaves need `cull_disabled`, which doubles rasterized fragments only for faces seen from behind (cheap). Agave stays `cull_back`.
- **[EJ] Shadows:** ferns and hostas carpeting the ground: set `cast_shadow = off` on LOD1+, and consider shadow casting only from LOD0 within about 15 m, or a proxy-dome shadow mesh.

### Gaps
- No Godot 4.x benchmark of alpha-scissor vs opaque-geometry foliage was found. Profile in the project (Godot profiler / RenderDoc) with a 200-fern test chunk.
- The Polycount thread text could not be fetched directly (403). Attributions are from the search summary.

## 6. Existing references, tools and licences (plus recommended recipe per type)

### Takeaway
There is no drop-in permissive fern or big-leaf generator for Godot. The practical route is to extend the existing `gn_tree.py` generator with two new node-group modules, **`frond_rosette`** and **`blade_rosette`**, sharing the blade/frond primitives and the COLOR/UV2 contract. Mine the Martinelli fern asset and the SpeedTree frond parameter set for ideas, and ABOP for rosette and compound-leaf maths.

### Cited Findings
- **Alex Martinelli Procedural Fern (GN, Blender 4.3+):** free / name-your-price, "no usage restrictions" (paraphrase) ([Digital Production](https://digitalproduction.com/2025/06/23/procedural-ferns-in-blender/); [Gumroad](https://sagado.gumroad.com/l/ozxrlu)).
- **"The Curl" unfolding spiral GN setup:** commercial ([Superhive](https://superhivemarket.com/products/the-curl---unified-unfolding-spiral-setup-in-geometry-nodes)).
- **SpeedTree:** subscription and not Blender-headless (tree report). Its frond docs are a good parameter reference ([SpeedTree docs](https://docs.unity3d.com/speedtree-modeler/manual/frond-generator-properties.html)).
- **ABOP:** chapters are hosted free at algorithmicbotany.org ([ch.4](http://algorithmicbotany.org/papers/abop/abop-ch4.pdf), [ch.5](http://algorithmicbotany.org/papers/abop/abop-ch5.pdf)). Algorithms are not copyrightable; the licence of the text was not checked.
- **Godot shaders:** Stylized Fluffy Tree Leaves (MIT/CC0 per the tree report) ([godotshaders](https://godotshaders.com/shader/stylized-fluffy-tree-leaves/); [GitHub](https://github.com/TheMIU/Stylized-Fluffy-Tree-Shader)); J Hell simple stylized tree shader (MIT) ([godotshaders](https://godotshaders.com/shader/simple-cheap-stylized-tree-shader/)).
- **Godot procedural mesh helpers:** gdprocmesh (GDNative, Bastiaan Olij) ([GitHub](https://github.com/BastiaanOlij/gdprocmesh/)) and script-mesh ([GitHub](https://github.com/matjlars/script-mesh)). Licences were not checked; neither is plant-specific. The official SurfaceTool/ArrayMesh docs are sufficient.
- **NodeToPython (GPL-3.0, Blender 4.2-5.2)** for versioning GN graphs (tree report; [GitHub](https://github.com/BrendanParmer/NodeToPython)).

### Inferences: recommended recipes (all [EJ]; calibrate against concept art with the addendum's contact-sheet loop)

**Shared contract.**
- COLOR_0 = (R sway, G phase, B t-along, A flutter), as FLOAT_COLOR.
- Proxy normal in a second FLOAT_COLOR attribute, encoded n·0.5+0.5 in Godot axes (the addendum's proven route).
- UV0 = (u,v) of blade/pinna for procedural veins and colour.
- UV2 = frond/leaf pivot xy.
- Material: painterly env `light()` with `cull_disabled` for thin leaves, back-face re-flip in fragment, custom translucency term, vein stripes from UV0, world-space dab overlay at about 30%.
- Import: `force_disable_compression`, no auto-LOD, no shadow meshes.
- Gates, extending the addendum's cheap geometric gates: tri budget per LOD; no floating parts; silhouette compactness/asymmetry from the 45° mask render; frond count ≥ min; for monstera, hole count ≥ 3 on adult leaves.

**1. Fern (sword/Boston-type ground fern, 0.5-1.2 m).**
- Crown: 10-18 fronds, yaw = i·137.5° + ±12° jitter, 10-20% slots skipped. Pitch 75°→20° inner→outer, length 0.6→1.0 inner→outer, droop 0.25-0.5·L·t². 1-3 central fiddleheads (equiangular curl, 1.5-2.5 turns, scale 0.25).
- Frond: rachis resampled to 12-16 pts; 14-22 pinna pairs, alternate. Pinna = 6-vert/4-tri creased lanceolate. Length profile sin(π t^0.8)^0.6, 15% bare stipe, pinna angle 55-70° from the rachis toward the tip, V-fold 15-25°.
- Normals: ellipsoid proxy, spherize 0.7 + spread 0.2.
- Colour: outer fronds olive/darker; tips warm; underside teal via the shadow band.
- LODs: 1-1.5k tris → alpha ribbon fronds ~500 → 3-5 crossed cards or mound ~100.

**2. Hosta (0.3-0.7 m mound).**
- 15-25 leaves on arching petioles (golden angle, pitch 60°→25°).
- Blade: ovate-cordate, L 15-35 cm, L/W 1.3-1.8. Grid 6×(2+2), V-fold 20-30°, ruffle amp 0.02·L, arch droop at the tip, parallel curved vein stripes.
- Normals: mound proxy, spherize 0.5.
- Two colourways: blue-green and variegated (edge stripe from |v| > 0.8).
- About 1k tris.

**3. Taro / elephant ear (0.8-1.8 m).**
- 5-9 leaves. Long petioles 0.5-1.2 m, arching out from a short clumped base.
- Blade: sagittate-cordate, L 0.4-0.9 m. Grid 10×(3+3) extended to negative u for the basal lobes. Peltate attachment option at u≈0.2 for true taro. Blade pitch hanging 30-60° below the petiole tangent, V-fold 15°, gentle ruffle.
- Normals: spherize 0.3 per-leaf + 0.15 plant.
- Strong translucency. Lighter underside tint via the back face.
- About 0.8k tris.

**4. Monstera (0.8-1.5 m clump).**
- 6-12 leaves on long petioles from a short stem cluster.
- Blade: broad cordate-ovate, L 0.4-0.8 m. Grid 12×(3+3). Pinnatifid slits = 4-9 per side, remove face columns from the edge to 40-75% of half-width along lateral vein lines. Fenestrations = 0-2 rows of elliptical holes near the midrib (deleted faces, boundary snapped). Juvenile leaves (inner/young) have no holes. V-fold 10°.
- Normals: spherize 0.3 per-leaf.
- 1-2k tris.
- Hole geometry is the signature, so keep it through LOD1. Paint holes as dark albedo only at LOD2.

**5. Banana-leaf shrub (1.5-3 m).**
- Pseudostem: 6-8-sided tapered cylinder, 150-300 tris, with vertical colour streaks.
- 6-10 leaves emerging from the top: one young rolled "cigar" leaf upright in the centre (a cylinder or tightly V-folded blade), older leaves arching then drooping (arch then droop over u).
- Blade: oblong L 1-2 m, L/W 3-4. Grid 14×(2+2). 3-8 tears along vein lines, each strip with its own phase. Older outer leaves partly browned or with torn tips via the colour ramp.
- Normals: spherize 0.25 per-leaf.
- Wind: the most flap and flutter of any type.
- 1.2-2.5k tris.

**6. Agave / beach succulent rosette (0.3-1.2 m).**
- 18-40 thick leaves, golden-angle cylindrical placement. Pitch 85° (centre) → 20-35° (outer), outer tips recurving.
- Leaf: lanceolate, thick lens cross-section (3-4 sides), 5-8 segments, sharp tip. Optional margin teeth as painted albedo, not geometry.
- Opaque, `cull_back`, normals spherize ≤ 0.2 to a rosette proxy.
- Colour: glaucous blue-green with a pale edge stripe, which suits a seaside village.
- Wind: none.
- 0.5-1k tris.
- Variants: echeveria-like flat rosette (Vogel r = c√n, pitch 10-40°) for low beach groundcover, and aloe (fewer, longer, spotted leaves).

### Gaps
- None of these recipes has been built and viewed in the project yet. Tri budgets and parameters are starting points only.
- No Ghibli, Ni no Kuni or AWL primary source on big-leaf or fern construction was found. The art-direction inference relies on the Oga "few detailed leaves" principle from the tree report.

# Automatic Evaluation of Procedural Trees / 3D Assets Against an Art Direction

Scope: a menu of automatic metrics and judge loops that let an AI-agent pipeline (Blender procedural tree generator, then Godot) accept, reject, or iterate on variants without a human reviewing each one. Must run locally on a 16 GB GPU. Researched 2026-09-27. "Verified" means the claim is stated in the cited source. "Inferred" means my own reasoning or engineering judgement, not a claim from a source.

## Geometric / silhouette metrics (cheap checks run before any neural model)

### Takeaway
Hand-computed geometric metrics are the fastest filter and the only ones that directly enforce hard constraints (budget, manifoldness, proportions). Their link to human judgement comes from perception research: perimeter²/area, convex-hull ratios, symmetry, and compactness predict perceived complexity and aesthetic preference. None of the studies I found tested trees, so the thresholds have to be calibrated against the project's own concept art.

### Cited Findings
- In perception research, the number of vertices was the strongest predictor of perceived 2D shape complexity. P²/A (perimeter squared over area) also correlated strongly with judged complexity. Other measures in use include the number of independent turns, angular variability, and the ratio of outline perimeter to convex-hull perimeter. — [Perceptually grounded quantification of 2D shape complexity (Visual Computer)](https://link.springer.com/article/10.1007/s00371-022-02634-8), via search summary (verified at snippet level only)
- A large 3D aesthetic study collected 22,301 pairwise comparisons on MTurk across 5 categories (chairs, tables, mugs, lamps, dining chairs). It fit Bradley-Terry scores, then used Random Forest + SHAP, and found symmetry, curvature, and compactness to be influential geometric drivers of preference. The study included no trees or plants. — [Modeling Aesthetic Preferences in 3D Shapes (arXiv 2505.12373)](https://arxiv.org/abs/2505.12373)
- Known 3D shape features include multi-view silhouette complexity (counting silhouette edges from several views), surface-to-volume ratio, and a convexity ratio. — [arXiv 2505.12373 via search summary](https://arxiv.org/pdf/2505.12373)
- Inverse procedural modelling of trees (Stava et al. 2014, CGF) contributes three things. (1) A compact parametric tree model. (2) An efficient similarity measure between two trees. (3) Parameter estimation with MCMC, so that the generator reproduces an input tree. This is the classic "fit procedural tree parameters to a target via a shape distance" loop. — [Stava et al. 2014, Wiley CGF](https://onlinelibrary.wiley.com/doi/abs/10.1111/cgf.12282); [Purdue PDF](https://cs.purdue.edu/homes/bbenes/papers/Stava14CGF.pdf)
- Follow-up work notes that the shape distance between the model tree and the sample tree strongly affects similarity quality. The best results came from space colonization combined with inverse procedural modelling. — [Realistic Tree Modelling, Springer](https://link.springer.com/chapter/10.1007/978-3-319-52308-8_6) (search snippet)
- Game-art practice: a strong silhouette reads at a glance. Small details should add interest without distorting the major silhouette read, and excessive detail becomes visual noise. — [80Level: Shape Language and Readability](https://medium.com/@EightyLevel/character-design-shape-language-and-readability-6ee4bb6f98a6) (practitioner opinion, not measured)

### Inferences
- (Inferred) Suggested cheap metric set, all computable in Blender Python / numpy in milliseconds per tree:
  - **Silhouette IoU vs concept turnaround**: render a flat black alpha mask from the same camera and orthographic framing as each concept view (front/side/3-4). Normalize both masks to a common bounding box (align bbox height and base), then compute IoU. Use chamfer distance or distance-transform distance on the outlines as a softer alternative, because IoU punishes small offsets of thin branches.
  - **Negative-space ("sky hole") fraction**: (convex hull area − silhouette area) / convex hull area, measured on the canopy region only. Stylized/Ghibli-like canopies want a low value with a few deliberate holes. Realistic canopies want a high value. Also count connected background components inside the hull to get the number of holes and their size distribution.
  - **Compactness / complexity**: P²/(4πA) (1 = circle), convex-hull perimeter ratio, and outline turning-angle count after polygon simplification. These are the perceptual correlates cited above.
  - **Canopy-to-trunk ratio**: the canopy bbox height vs the clear-trunk height, and the canopy width vs the trunk base diameter. Read these from the generator's own parameters or from vertex groups rather than from images.
  - **Symmetry**: compare the mask with its mirror about the trunk axis, as IoU(mask, flip(mask)). Stylized trees usually want moderate asymmetry, so score the distance from a target band rather than maximizing.
  - **Mesh budget / validity**: tri count vs budget per LOD, `bmesh` non-manifold edge count, loose verts, zero-area faces, and UV presence. Also check material slot count for Godot draw calls. These are hard reject gates, not scores.
- (Inferred) Normalize each geometric metric to a target band derived from 3–5 approved reference trees, not to "higher is better". Art direction is a target, not a maximum.

### Gaps
- I found no published study that validates silhouette IoU, sky-hole fraction, or canopy/trunk ratios against human judgement of stylized trees specifically.
- I did not retrieve the exact form of Stava's tree similarity measure. The paper PDF is linked but I did not read it.

## Image metrics against concept art (CLIP/SigLIP/DINOv2, CSD, LPIPS, DreamSim)

### Takeaway
Different embeddings measure different things. DreamSim is the best-validated "does this look like that" metric for mid-level similarity: layout, pose, object appearance. CSD is purpose-built for style independent of content, but its raw cosine is unreliable as an absolute score, so use it relatively (rank or CSLS-normalized). DINOv2 ViT-L/14 is the recommended backbone for set-level distribution and diversity metrics. CLIP is best for text-prompt alignment. All of these fit comfortably on a 16 GB GPU.

### Cited Findings
- **DreamSim** concatenates CLIP, OpenCLIP, and DINO embeddings and fine-tunes them (LoRA) on human judgements over the NIGHTS dataset (~20k synthetic image triplets). It reaches 96.16% accuracy at predicting human 2AFC judgements. It sits between low-level metrics (LPIPS, PSNR, SSIM) and high-level ones (CLIP), capturing mid-level attributes: object appearance, viewing angle, pose, layout. — [DreamSim paper (arXiv 2306.09344)](https://arxiv.org/html/2306.09344v3); [GitHub ssundaram21/dreamsim](https://github.com/ssundaram21/dreamsim); installable with `pip install dreamsim` ([PyPI](https://pypi.org/project/dreamsim/))
- **CSD (Contrastive Style Descriptors)** is trained with a multi-label contrastive loss (style tags) plus a self-supervised loss, using augmentations that preserve style while changing content. It was trained on LAION-Styles (511,921 images, 3,840 style tags) and beats self-supervised and style-attribution baselines on style retrieval (DomainNet, WikiArt). — [Measuring Style Similarity in Diffusion Models (arXiv 2404.01292)](https://arxiv.org/pdf/2404.01292)
- CSD availability: ViT-L backbone, weights on HF `tomg-group-umd/CSD-ViT-L`, MIT license. The repo warns: "We are currently investigating an issue with model weights due to which there is some discrepancy with the reported numbers in the paper." — [GitHub learn2phoenix/CSD](https://github.com/learn2phoenix/CSD)
- **Caveat on raw CSD cosine (July 2026)**: CSD output is 768-dimensional. With raw cosine, 23/91 artists (25.3%) show negative discrimination gaps pairwise, meaning same-artist similarity falls below the highest cross-artist similarity. 15/91 fail under aggregated pool scoring. The fix is CSLS (Cross-domain Similarity Local Scaling) as the default readout, which cuts failures to 4/91. Optionally, interpolating positional embeddings to 336×336 costs about 2.25× more compute. Verification AUC rises from 0.883 to 0.905. The failure is backbone-general: CLIP ViT-L/14, SigLIP-large, and DINOv2-Large fail the same way. The authors recommend running their diagnostic on your own corpus before trusting CSD scores. — [When Style Similarity Scores Fail (arXiv 2605.09030)](https://arxiv.org/html/2605.09030v2)
- **DINOv2 ViT-L/14 vs Inception**: a large human-evaluation study found DINOv2-ViT-L/14 gives much richer evaluation than Inception-V3. FID fails to reflect human-judged realism of diffusion models. Existing memorization metrics cannot separate memorization from underfitting or mode shrinkage. — [Exposing flaws of generative model evaluation metrics (arXiv 2306.04675)](https://arxiv.org/abs/2306.04675)
- In GPTEval3D's human-alignment table, CLIP-based and aesthetic-predictor metrics scored well on some criteria but poorly on others. Average Kendall's tau vs human expert rankings: PickScore ~0.562, CLIP-S ~0.568, CLIP-E ~0.628, Aesthetic-S ~0.671, Aesthetic-E ~0.611, GPTEval3D 0.710. CLIP's weakest criterion was "3D plausibility" (tau as low as ~0.28). (Numbers come from my text extraction of the PDF table, so column alignment is best-effort.) — [GPTEval3D paper (arXiv 2401.04092)](https://arxiv.org/pdf/2401.04092)
- 3DGen-Bench reports that plain CLIP gets 0.661 pairwise agreement with humans on text-to-3D, versus 0.725 for their trained 3DGen-Score. — [3DGen-Bench (arXiv 2503.21745)](https://arxiv.org/html/2503.21745v1)

### Inferences
- (Inferred) Recommended split of duties:
  - **DreamSim** (render vs concept image, same view): "does this tree look like the concept?" This is the primary image-similarity score for concept matching.
  - **CSD with CSLS readout or rank-based use**: "is this rendered in the same style as the concept / approved set?" Compare each render against a pool of approved references plus a pool of negatives (off-style trees), and score the margin rather than the raw cosine.
  - **CLIP/SigLIP text score**: sanity check against a style prompt ("stylized low-poly oak, chunky canopy clumps"). It is weak on 3D plausibility per GPTEval3D, so give it low weight.
  - **LPIPS**: only useful when renders are pixel-aligned with the reference, for example regression tests after a generator code change. It is too low-level for concept matching.
  - **Gram-matrix (VGG) style loss**: a classic texture/style statistic. It has no human-alignment evidence in the sources I read. Treat it as a tie-breaker at most.
- (Inferred) Render trees for embedding metrics with lighting, background, and camera matched to the concept art. Also render a flat-shaded/unlit variant. Embedding metrics are sensitive to background and lighting mismatch, which otherwise swamps the tree signal.
- (Inferred) VRAM: DreamSim (ViT-B-scale ensemble), CSD ViT-L, DINOv2 ViT-L/14 (~300M params), and CLIP ViT-L/14 all fit together in a few GB at fp16, well within 16 GB. Batch many renders.

### Gaps
- I found no study measuring which of DreamSim, CSD, or CLIP best tracks human judgement of *stylized 3D render vs 2D concept painting* (a domain gap: painted concept vs rendered mesh). Calibrate this locally with a small human-labelled set (e.g., 50 pairwise choices).
- I did not verify exact DreamSim parameter counts or VRAM.

## VLM-as-judge for 3D assets (GPTEval3D, 3DGen-Bench, T3Bench, MATE-3D, Eval3D, 3D-DefectBench)

### Takeaway
The field has converged on multi-view renders (RGB, often plus normal maps) fed to a VLM or a trained CLIP-based scorer, with pairwise comparisons aggregated by Elo or Bradley-Terry. Pairwise judging is consistently more reliable than absolute 1–10 scores. VLMs show position bias and score compression, so randomize order, ensemble, and calibrate against a small human set. A compact 6-view RGB protocol is about as good as denser views or added normals.

### Cited Findings
- **GPTEval3D (CVPR 2024)**
  - Setup: GPT-4V compares *two* 3D assets under user-defined criteria (text-asset alignment, 3D plausibility, texture details, geometry details, texture-geometry coherency), and Elo ratings come from pairwise results. The benchmark has 13 methods and 110 prompts. — [project page](https://gpteval3d.github.io/); [arXiv 2401.04092](https://arxiv.org/abs/2401.04092)
  - Inputs: RGB renders plus world-space surface-normal renders in the same layout. Normals help geometric reasoning and cross-view correspondence. The repo renders 120 views at 512×512. — [paper](https://arxiv.org/pdf/2401.04092); [GitHub 3DTopia/GPTEval3D](https://github.com/3DTopia/GPTEval3D)
  - Robust ensemble: GPT-4V's answers are stochastic, so the authors perturb seeds, render layout, view count (1, 2×2, or 3×3), criteria lists, and left/right order (horizontal flip), then ensemble. Variance drops as ensemble size grows. — [paper, Sec 5.2 / App. D](https://arxiv.org/pdf/2401.04092)
  - View-count ablation: a single view helps low-level criteria (texture/geometry detail). 4 or 9 views are needed for global criteria (alignment, 3D plausibility). — [paper, App. D](https://arxiv.org/pdf/2401.04092)
  - Known limitation stated by the authors: GPT-4V has systematic errors such as "bias toward certain image positions". — [paper, Limitations](https://arxiv.org/pdf/2401.04092)
  - The repo supports only the OpenAI GPT-4V API, with no local model. — [GitHub](https://github.com/3DTopia/GPTEval3D)
- **3DGen-Bench (2025)**
  - Dataset: 11,200 models from 19 generators, with 68,000+ preference votes and 56,000+ absolute score labels, over 5 dimensions (geometry plausibility, geometry details, texture quality, geometry-texture coherence, prompt-asset alignment).
  - Models: **3DGen-Score** uses a CLIP-ViT-H/14 backbone with separate normal and RGB multi-view encoders and 5 dimension heads. **3DGen-Eval** is MV-LLaVA based and takes a 2×2 grid including normals, supporting both absolute scoring and pairwise comparison.
  - Results: 3DGen-Score reaches 0.725 pairwise agreement on text-to-3D (CLIP 0.661, GPTEval3D 0.677) and 0.767 on image-to-3D. Kendall tau is 0.711 (T23D) and 0.856 (I23D).
  - Release: on HuggingFace, annotations under MIT. — [arXiv 2503.21745](https://arxiv.org/html/2503.21745v1); [project](https://zyh482.github.io/3DGen-Bench/)
- **MATE-3D / HyperScore (ICCV 2025)**: 1,280 textured meshes over 8 prompt categories, with 107,520 annotations across 4 dimensions. HyperScore extracts CLIP features, fuses them with dimension-conditioned attention, and uses a hypernetwork to produce a per-dimension quality-mapping function. It outperforms prior metrics on MATE-3D. Code: [GitHub zhangyujie-1998/HyperScore](https://github.com/zhangyujie-1998/HyperScore). — [arXiv 2412.11170](https://arxiv.org/abs/2412.11170); [project](https://mate-3d.github.io/)
- **Eval3D (CVPR 2025)**: rather than a black-box MLLM, it measures the *consistency among foundation models and tools* (semantic and geometric consistency across views, text-3D alignment). It aligns with human judgement as well as or better than prior work on all dimensions, but its accuracy depends on the underlying foundation models. — [arXiv 2504.18509](https://arxiv.org/abs/2504.18509)
- **3D-DefectBench (2026)**: a factorial study of 12 VLM judges over 84 pipeline designs and ~3.2M defect decisions. Findings:
  - "A compact six-view RGB protocol performs comparably to denser multi-view settings and inputs augmented with depth or surface normals."
  - Model choice is the largest factor.
  - The best judge still lags trained human labelers.
  - Texture agreement drops sharply with noisier labels.
  - Recommendation: evaluate judges as complete pipelines and calibrate them. — [arXiv 2607.10826](https://arxiv.org/abs/2607.10826)
- **"VLM Judges Can Rank but Cannot Score" (2026)**: tested LLaVA-Critic-7B, Phi-4-reasoning-vision-15B, and Gemini 2.5 Flash.
  - Only 32–34% exact agreement with humans on a 5-point scale.
  - Score compression: poor items are overscored by +0.89 to +1.98 and excellent ones underscored by −0.74 to −1.04.
  - High rank correlation can coexist with very wide conformal intervals, for example Pearson 0.507 with a 3.08-wide interval on a 1–5 scale.
  - Intervals were narrowest for aesthetics/natural-image tasks (~2.1–2.4) and widest for chart/maths tasks.
  - Recommendation: use pairwise comparison when intervals are wide. — [arXiv 2604.25235](https://arxiv.org/html/2604.25235v1)
- Pairwise-evaluation literature (MT-Bench, Chatbot Arena) reports position and verbosity biases, including a persistent ~5% position bias despite explicit instructions. — [search summary citing arXiv 2604.25235 / PairBench 2502.15210](https://arxiv.org/html/2502.15210v3) (I did not verify the ~5% figure in the primary source)
- **Local VLM option**: Qwen3-VL-8B (Instruct and Thinking) has open weights on HF, and GGUF quantizations exist (unsloth). It claims improved spatial perception (viewpoints, occlusion) and 3D grounding. — [HF Qwen/Qwen3-VL-8B-Instruct](https://huggingface.co/Qwen/Qwen3-VL-8B-Instruct); [unsloth GGUF](https://huggingface.co/unsloth/Qwen3-VL-8B-Instruct-GGUF)
- T3Bench: I did not retrieve its details in this session (see Gaps).

### Inferences
- (Inferred) Local 16 GB recipe:
  - Qwen3-VL-8B at 4-bit or 8-bit quantization. bf16 8B weights alone are ~16 GB, so full precision will not fit alongside anything else.
  - Input: a 6-view RGB turntable grid per tree. Per 3D-DefectBench, add normals only if geometry criteria underperform.
  - Protocol: *pairwise* comparisons ("Which tree better matches the attached concept sheet on: silhouette, canopy massing, branch rhythm, stylization?"). Ask each pair in both A/B orders and discard inconsistent verdicts, which cancels position bias.
  - Aggregation: Elo or Bradley-Terry across candidates, as in GPTEval3D.
  - Absolute scores: use them only as coarse gates (pass/fail on explicit defects such as floating leaves or trunk penetrating the canopy). Never use them as the optimization objective.
- (Inferred) Of the benchmark-trained scorers, 3DGen-Score/HyperScore target generic text-to-3D quality (plausibility, texture). They will not know the project's art direction. They are useful as a "not broken" prior, not as a style judge.
- (Inferred) Because the concept art is the target, include the concept image in the VLM prompt ("reference" plus two candidates). This turns the task into reference-guided pairwise comparison, which is the setting VLMs handle best per the rank-not-score finding.

### Gaps
- I did not retrieve T3Bench's scoring method or numbers.
- No source tested VLM judges on stylized foliage or trees specifically. Foliage (thin branches, alpha cards) may be a hard case. Unknown.
- I have not verified the VRAM figures for 3DGen-Eval (MV-LLaVA) or HyperScore on a 16 GB card. 3DGen-Score uses CLIP-ViT-H/14 (~1B params), which should fit (inferred).

## Variety/diversity of procedural variants and set coherence

### Takeaway
Use the Vendi Score on DINOv2 ViT-L/14 embeddings of the variant set as an "effective number of distinct trees" diversity measure. Measure coherence as tight clustering in style-embedding space (CSD or DreamSim) around the approved references. Diversity and coherence are two different embedding spaces: diverse in shape/structure, tight in style.

### Cited Findings
- The Vendi Score is the exponentiated entropy of the eigenvalues of a normalized similarity (kernel) matrix. A score of v means the set is as diverse as v completely dissimilar items. It is reference-free. — [The Vendi Score (arXiv 2210.02410)](https://arxiv.org/html/2210.02410v2)
- DINOv2 ViT-L/14 embeddings with a linear kernel are used to compute Vendi Scores for generative models, because that space aligns well with human evaluation. — [Cousins of the Vendi Score (arXiv 2310.12952)](https://arxiv.org/html/2310.12952); [Stein et al. 2306.04675](https://arxiv.org/abs/2306.04675)
- The Conditional Vendi Score separates prompt-induced diversity from the model's internal diversity. — [arXiv 2411.02817](https://arxiv.org/pdf/2411.02817)
- Stein et al. found that diversity does not explain the gap between human judgements and automated metrics, so diversity metrics should be complemented, not trusted alone. — [arXiv 2306.04675](https://arxiv.org/abs/2306.04675)

### Inferences
- (Inferred) Clone rejection: reject a new variant if its max DreamSim/DINOv2 similarity to any accepted variant exceeds a threshold, for example the nearest-neighbour distance observed between clearly different human-approved trees. Also check parameter-space distance so that near-duplicate seeds are caught cheaply before rendering.
- (Inferred) Set coherence: compute the CSD/DreamSim centroid of the approved style references. Require every accepted variant within a radius (using CSLS-adjusted similarity per 2605.09030). Report the set's mean pairwise style similarity alongside its DINOv2 Vendi Score. A good forest has a high Vendi Score (shape) and tight style similarity.
- (Inferred) Also report diversity on the geometric descriptors (height/width ratio, sky-hole fraction, canopy clump count). They are cheaper and more interpretable than embeddings, and double as MAP-Elites behaviour axes.

### Gaps
- I found no published "set coherence" metric for stylized asset libraries. The centroid/radius approach above is my own proposal.

## Practical loops: parameter search and quality-diversity over procedural generators

### Takeaway
The established pattern is: render, score with a cheap automatic objective, and search parameters with random search, Bayesian optimization, CMA-ES, or MCMC. For a *library* of varied trees, MAP-Elites/QD is the right fit: fitness = art-direction match, behaviour descriptors = interpretable shape features. The expensive VLM judge belongs only at the final pairwise tournament stage.

### Cited Findings
- Stava et al. estimate procedural tree parameters with MCMC, optimizing a similarity between generated and target trees. This is direct precedent for parameter search against a shape objective on trees. — [Stava 2014](https://onlinelibrary.wiley.com/doi/abs/10.1111/cgf.12282)
- PCG through Quality Diversity (Gravina et al. 2019) frames QD as finding a set of high-quality solutions covering a space defined by behaviour metrics. It uses either behaviour-space distance (novelty search) or behaviour-space partitioning (MAP-Elites grids), optionally with local competition and constraints (e.g., FI-2Pop for feasibility). The authors say QD suits mixed-initiative content design because it yields a large diverse set in one run. — [arXiv 1907.04053](https://arxiv.org/pdf/1907.04053)
- The same survey records MAP-Elites generating 2D images and 3D objects, using a DNN classifier's output both for quality and as the diversity partition. — [arXiv 1907.04053](https://arxiv.org/pdf/1907.04053)
- CMA-ME (Covariance Matrix Adaptation MAP-Elites) combines CMA-ES search with a MAP-Elites archive, aiming for solutions that are both high-quality and diverse along measure functions. — [search summary, arXiv 2505.06617](https://arxiv.org/pdf/2505.06617)
- Recent MAP-Elites PCG applications include FPS map generation (2026). — [arXiv 2605.30570](https://arxiv.org/html/2605.30570v1)
- ProcFunc (2026) is a Python library for Blender-based procedural generators that can automatically analyze generators to expose parameters and optimize their random distributions. It is relevant infrastructure for parameter search in Blender. — [arXiv 2604.26943](https://arxiv.org/html/2604.26943v2)
- CLIP score (cosine between prompt embedding and render embedding) is used as a semantic objective for text-guided 3D and for evaluating Blender-code 3D generation, alongside execution reliability and human preference. — [search summary, Proc3D arXiv 2601.12234](https://arxiv.org/pdf/2601.12234); [ProcFunc](https://arxiv.org/pdf/2604.26943)

### Inferences
- (Inferred) Proposed tiered loop for the tree pipeline, cheapest first:
  1. **Gate (ms)**: parameter sanity plus mesh checks (tri budget, non-manifold, loose parts, canopy/trunk ratio in band). Fail = reject without rendering.
  2. **Silhouette (tens of ms, Eevee/Workbench mask renders)**: silhouette IoU/chamfer vs the concept turnaround, sky-hole fraction, compactness, and symmetry, each scored as distance to target band.
  3. **Embedding (GPU, ~10–50 ms per image batched)**: DreamSim to the concept views, CSD margin (approved vs off-style pools, CSLS), and optionally CLIP text. Combine into a weighted fitness. Fit the weights on ~50–100 human pairwise labels with Bradley-Terry or logistic regression. That labelled set is also where each metric's human correlation gets verified.
  4. **Search**: CMA-ES (continuous params) or Optuna TPE (mixed/discrete params) for "one best tree". Use MAP-Elites/CMA-ME for a forest library, with behaviour axes = (height/width ratio, sky-hole fraction) or (canopy clump count, trunk lean). pyribs is a common Python QD library (not verified in this session).
  5. **Judge (seconds per pair)**: local Qwen3-VL-8B pairwise tournament among the top-k elites vs the concept sheet, both orders, then Elo. A human reviews only the final top-N and any VLM/embedding disagreements.
- (Inferred) Re-check that fitness does not get "gamed". Optimizing hard against DreamSim or CLIP can produce adversarial-looking trees. The VLM tournament and the occasional human spot check act as held-out judges.

### Gaps
- I found no published MAP-Elites or QD study applied specifically to procedural *trees/vegetation*. The QD searches returned robotics and level design only.
- I found no benchmark comparing CMA-ES, Bayesian optimization, and random search on Blender procedural-tree parameters.
- I did not verify throughput numbers (renders/sec in Blender headless, VLM latency on 16 GB) in this session.

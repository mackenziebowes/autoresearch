# Outcome-Based Pricing for AI Agents Outside Customer Support (as of 2026-09-29)

Source-quality legend: **[C]** = confirmed (vendor/press release/primary reporting); **[R]** = reported by credible press; **[A]** = aggregator/SEO pricing-guide estimate (treat as directional only). Many third-party "pricing guides" are vendor-competitor content (e.g., Precedent on EvenUp, Paraform on Juicebox, 11x on Artisan) — flagged where relevant.

## Cross-function context: how much outcome pricing exists at all?

### Takeaway
Outside CX, pure per-outcome pricing is rare in 2026. The market has converged on hybrid models (platform fee + credits/usage), and outcome pricing survives mainly where the outcome is already a monetized, auditable event with a pre-existing contingency tradition (recovered revenue, placements, collections, per-case legal work).

### Cited Findings
- Growth Unhinged 2026 State of B2B Monetization (survey of 230 companies, Apr–May 2026, published 2026-05-13): hybrid is the most common primary model at 37%, up from 25% a year earlier, expected 47% in three years; flat-fee + per-seat expected to fall from 42% to 8%. [C] — [Growth Unhinged](https://www.growthunhinged.com/p/the-state-of-b2b-monetization-in-2026); secondary summary [OpenMeter](https://openmeter.io/blog/monetization-benchmarks-2026)
- Same report: 29% of companies offer multiple AI pricing options (up from 21%); 26% of investors prefer outcome-based; target gross margins for outcome-based AI offerings ~50% vs 70–80%+ SaaS; "many enterprises still crave predictability." HubSpot announced outcome-based pricing for Breeze agents in April 2026 (CX-adjacent). [C] — [Growth Unhinged](https://www.growthunhinged.com/p/the-state-of-b2b-monetization-in-2026)
- Kyle Poyar has publicly argued "why outcome-based pricing is rare and flawed" (LinkedIn post, 2025). [C] — [LinkedIn](https://www.linkedin.com/posts/kyle-poyar_ai-monetization-pricing-activity-7318970307698970625-8qF-)
- Aggregator claim: <10% of AI vendors offer outcome-based pricing; a Bessemer projection of 61% by end-2026 is cited; a 1H 2026 buyer survey found 43% prefer consumption, 27% outcome-based. [A — secondary, original sources not verified] — [Pickaxe](https://pickaxe.co/post/ai-agent-pricing-models)
- Deloitte published accounting guidance specifically on revenue recognition for outcome-based pricing in agentic AI products (June 4, 2026) — a signal the model is material enough to need GAAP treatment. [C] — [Deloitte DART](https://dart.deloitte.com/USDART/home/publications/deloitte/industry/technology/accounting-outcome-based-pricing-agentic-ai)

### Inferences
- The 61% Bessemer projection looks inconsistent with Poyar's measured 2026 data (hybrid 37% primary); the report writer should treat it as aspirational.
- ~50% target gross margin for outcome-priced offerings explains why many vendors retreat: they bear model-cost and failure risk.

### Gaps
- No rigorous survey isolating outcome-pricing share by function (sales vs legal vs healthcare) was found.

## Sales/GTM: AI SDRs (per meeting / per qualified lead) vs seats and credits

### Takeaway
Despite marketing that AI SDRs are "digital workers," the major vendors bill annual subscriptions/credits, not per meeting. Qualified explicitly considered and rejected meetings/pipeline-based pricing. The category's credibility took a hit from 11x's 2025 ARR/churn scandal; per-meeting pricing exists mainly among smaller agencies/"pay-per-meeting" lead-gen shops.

### Cited Findings
- **Qualified (Piper)**: bundled Piper into all tiers (Premier/Enterprise/Ultimate) with capacity expanding by tier; base plan priced around the cost of a human SDR. Rejected per-conversation/token pricing ("unpredictable pricing for customers, which is a poor experience") and value-based pricing on meetings booked/pipeline ("varies wildly across industries, company sizes, and go-to-market strategies… impractical"). Post dated 2025-07-22. [C — interview-based case study] — [Sequence](https://www.sequencehq.com/blog/how-qualified-priced-their-ai-sdr-agent-from-add-on-to-all-in-strategy)
- Qualified reported starting price ~$42k/yr (Premier), $60k–$100k+ enterprise. [A, Vendr-derived] — [MarketBetter](https://marketbetter.ai/blog/qualified-pricing-breakdown-2026/); [Vendr](https://www.vendr.com/marketplace/qualified)
- **11x**: Growth plan from ~$3,750/mo billed annually; median contract ~$45k/yr ($39.75k–$65.6k range). [A] — [Cleanlist AI SDR Pricing Index, 2026-07-23](https://www.cleanlist.ai/blog/2026-07-23-ai-sdr-pricing-statistics). Some aggregators describe 11x as charging "for booked meetings rather than seats" [A, conflicts with contract data] — [Pickaxe](https://pickaxe.co/post/ai-agent-pricing-models).
- **11x scandal**: TechCrunch (2025-03-24) reported 11x claimed customers it didn't have (ZoomInfo said it was a one-month trial, "we are not a customer," threatened legal action; Airtable trial "never used in production"), counted one-year contracts with three-month break clauses as full ARR even after customers exited, and ex-employees cited ~70–80% churn. [R] — [TechCrunch](https://techcrunch.com/2025/03/24/a16z-and-benchmark-backed-11x-has-been-claiming-customers-it-doesnt-have)
- AI SDR churn broadly reported at 50–70% annually. [A] — [Cleanlist](https://www.cleanlist.ai/blog/2026-07-23-ai-sdr-pricing-statistics)
- **Artisan (Ava)**: ~$1,000–$2,500/mo by contact volume (subscription). [A] — [Cleanlist](https://www.cleanlist.ai/blog/2026-07-23-ai-sdr-pricing-statistics); [formanorden](https://formanorden.com/blog/ai-sdr-pricing/)
- Implied cost per booked meeting from AI SDR subscriptions ~$80–$150 vs $500+ for a human SDR (derived estimate, not a billing unit). [A] — [Cleanlist](https://www.cleanlist.ai/blog/2026-07-23-ai-sdr-pricing-statistics)
- **Salesforce Agentforce (incl. SDR agent)**: Flex Credits launched May 2025 at $0.10/action (20 credits; $500 per 100k credits), alongside earlier $2/conversation; per-user license $5/user/mo also offered. Salesforce's own SDR example: a $2 conversation could run 3–6 actions for $0.30–$0.60. [C] — [Salesforce press release, 2025-05-15](https://www.salesforce.com/news/press-releases/2025/05/15/agentforce-flexible-pricing-news/); [Salesforce Help](https://help.salesforce.com/s/articleView?id=004811240&language=en_US&type=1). Critics note SDR sequences can use 35+ actions (~$3.50/conversation). [A] — [Macha](https://www.getmacha.com/blog/agentforce-pricing-explained)
- Growth Unhinged notes Salesforce offers four different pricing models including outcome-based options. [C] — [Growth Unhinged](https://www.growthunhinged.com/p/the-state-of-b2b-monetization-in-2026)

### Inferences
- In GTM, "per action/credit" (Salesforce, Clay-style credits) is winning over "per meeting," because attribution of a meeting/pipeline to the agent is contested and meeting quality varies. Clay's credit model (not researched in depth here) is the paradigm of consumption pricing in this category.
- The 11x episode shows why outcome/ARR claims in AI SDR are viewed skeptically: short break clauses functioned like outcome-contingent trials, but were booked as ARR.

### Gaps
- No primary source found for Regie.ai's pricing model or Clay's current credit prices in this pass; no verified vendor at scale billing per meeting booked.
- No verified 2026 ARR for 11x or Artisan.

## Healthcare revenue cycle and admin

### Takeaway
Healthcare RCM is one of the clearest homes for outcome pricing, because RCM outsourcing has long used contingency (% of collections/recoveries). AI vendors (AKASA) market performance-based / % recovered pricing on denials and mid-cycle modules; the biggest player, Smarter Technologies, is a PE roll-up of a services BPO plus AI. Admin/voice agents (Infinitus) and ambient scribes (Abridge, Ambience) remain per-task/minute or per-provider seat.

### Cited Findings
- **AKASA**: markets performance-based pricing for its Mid-Cycle Prebill Optimization Suite with "no upfront fees," invoicing only after measurable financial improvement; denial-management modules reportedly priced on % of net revenue recovered; rate cards undisclosed. [C for marketing claim / A for rates] — [RevCycleAI deep dive](https://revcycleai.com/blog/akasa-vendor-deep-dive/); [RFP.wiki](https://www.rfp.wiki/specialty-industries/healthcare-life-sciences/healthcare/revenue-cycle-management-software/akasa); [AKASA](https://akasa.com/)
- **Smarter Technologies**: formed May 2025 by New Mountain Capital combining SmarterDx, Thoughtful.ai, and Access Healthcare; expected >$800M revenue, 27,000 employees, 200+ clients, >$200B revenue managed; acquired Pieces Technologies (Sept 2025) and launched SmarterNotes. [C] — [Healthcare Dive](https://www.healthcaredive.com/news/new-mountain-capital-creates-ai-revenue-cycle-management-company-smarter-technologies/748748/); [New Mountain](https://www.newmountaincapital.com/new-mountain-capital-forms-smarter-technologies-through-combination-of-smarterdx-thoughtful-ai-and-access-healthcare/)
- Hospital denied-claim rework averages ~$118/claim — the baseline cost that per-claim AI pricing is benchmarked against. [A] — [AKASA blog](https://akasa.com/blog/current-challenges-best-practices-for-hospital-revenue-cycle-management/)
- **Infinitus** (payor calls, benefits verification, prior auth follow-up): pricing combines annual platform/license fees with usage per task or per minute, or subscription; discounts tied to monthly minimums. [C, vendor page] — [Infinitus pricing](https://www.infinitus.ai/pricing/)
- **Tennr** (referral intake/prior auth): $101M Series C at $605M valuation, June 2025 (IVP-led). No public pricing. 2024 revenue est. $3–5M [A]. — [Fortune](https://fortune.com/2025/06/18/tennr-health-tech-ai-patient-referral-ivp-a16z-lightspeed-iconiq-series-c/); [Latka](https://getlatka.com/companies/tennr.com)
- **Ambient scribes are seat-priced**: Abridge ~$200–$800 per provider/month or ~$2,500/clinician/yr (negotiated, no public list); Ambience ~$200–$800+ per provider/month, up to $600–$1,200+ for full enterprise. Abridge reported $100M+ ARR, $5.3B valuation; Ambience $243M Series C at $1.25B (mid-2025). [A] — [Vero](https://www.veroscribe.com/blog/abridge-review-2026); [RFP.wiki Abridge](https://www.rfp.wiki/specialty-industries/healthcare-life-sciences/healthcare/ambient-clinical-documentation/abridge); [ValueAddVC](https://valueaddvc.com/blog/abridge-valuation-2026-5-3b-100m-arr-and-how-the-ai-scribe-beat-nuance-and-ambience)

### Inferences
- Where AI touches cash (coding accuracy, denials, underpayments), contingency pricing is natural and verification is the payer remittance. Where it touches labor time (scribes, calls), vendors price per provider or per minute.
- The largest "AI RCM" business by revenue is an AI-enabled services roll-up, which supports the thesis that outcome/services pricing wins via services businesses rather than pure software.

### Gaps
- Could not verify Cohere Health's pricing (payer-side prior auth; likely PMPM/per-review) or a per-prior-auth price at any vendor.
- No primary source on an ambient scribe charging per note; no confirmed 2026 price war data.

## Legal: per demand / per case vs seats

### Takeaway
Plaintiff personal-injury AI (EvenUp) is the clearest legal case of per-unit-of-work pricing, having moved to per-case pricing in May 2025 and in May 2026 to a fuller "pre-litigation-as-a-service." Big Law platforms (Harvey, Legora) stay on seats, with Legora adding consumption metering for heavy agent use in June 2026.

### Cited Findings
- **EvenUp** launched "all-in-one per-case pricing" with AI Drafts Suite on 2025-05-14/15 ("one clear, predictable cost per case"). [C] — [BusinessWire](https://www.businesswire.com/news/home/20250514536151/en/EvenUp-Launches-AI-Drafts-Suite-Smart-Workflows-Medical-Bill-Summary-and-Case-Based-Pricing-to-Redefine-Legal-Automation-for-Personal-Injury-Firms); [Legal IT Insider](https://legaltechnology.com/2025/05/15/evenup-launches-ai-drafts-suite-smart-workflow-and-per-case-pricing-model/)
- Reported per-demand rates ~$300 base, $500–$800+ with add-ons; other estimates $300–$1,500/case. [A — note one source is competitor Precedent] — [Precedent](https://precedent.com/comparing-the-best-ai-demand-letter-solutions-for-personal-injury-law-firms-precedent-vs-evenup/); [ProPlaintiff](https://www.proplaintiff.ai/post/evenup-pricing-breakdown?83395cbf_page=4)
- EvenUp raised $150M Series E at >$2B valuation (2025-10-07), total $385M raised. [C] — [Law.com](https://www.law.com/legaltechnews/2025/10/07/evenup-announces-150m-series-e-funding-round-at-valuation-over-2b/)
- EvenUp launched Pre-Litigation-as-a-Service (PLAAS) on 2026-05-14: AI plus US-based case management staff covering claim setup through settlement negotiation; >$10M in PLAAS subscription sales during early testing; used by 30% of the top 100 PI firms; >10,000 cases/week, >$14B damages; claims 95% of third-party policy limits recovered. Pricing basis (per case vs contingency) not disclosed. [C] — [LawSites](https://www.lawnext.com/2026/05/evenup-extends-beyond-software-with-launch-of-pre-litigation-as-a-service-offering-for-pi-law-firms.html)
- **Harvey**: quote-only per-seat pricing, reported ~$1,200–$2,000/seat/month (leaked tiers). [A] — [Vaquill](https://www.vaquill.ai/blog/harvey-legora-cocounsel-pricing-reality); [eesel](https://www.eesel.ai/blog/harvey-ai-pricing)
- **Legora**: ~$3,000/user/yr reported with 10-seat minimum; also a pay-as-you-go credit plan; in June 2026 moved its Agent Pro tier to consumption pricing. [A] — [Vaquill](https://www.vaquill.ai/blog/how-much-does-legora-cost); [Layer3](https://www.layer3labs.io/guides/legora-pricing)

### Inferences
- EvenUp's verification unit (a delivered demand per case) is naturally countable; PI firms themselves work on contingency, so per-case pricing aligns with their economics. Its move toward services (PLAAS) mirrors the "sell the work" thesis.
- Harvey/Legora's clients bill hourly; per-seat fits a buyer that doesn't want to price per output. Legora's shift is to usage, not outcomes.

### Gaps
- Supio's pricing model and per-contract-review pricing vendors were not verified in this pass.

## Finance/accounting: per close, per invoice; AI-native firms

### Takeaway
AI accounting software (Rillet, Campfire, Basis) is custom-quoted subscription, not per close. Outcome/fixed-fee pricing appears via AI-enabled accounting roll-ups (Crete/Current) that sell completed work at firm fees.

### Cited Findings
- **Rillet**: no per-seat or revenue-based pricing; custom quote, reported $15k–$40k/yr. **Campfire**: ~$20k–$40k/yr, no public price list (mid-2026). [A] — [Rillet blog](https://www.rillet.com/blog/the-best-ai-accounting-software-and-tools-for-2026); [ERP Scorecard](https://erpscorecard.com/articles/campfire-erp-review-2026)
- **Basis**: $100M Series B at $1.15B (2026-02-24, Accel & GV); agents used by ~30% of the Top 25 accounting firms across CAS, tax, audit. Pricing not disclosed. [C] — [BusinessWire](https://www.businesswire.com/news/home/20260224020999/en/Basis-Raises-$100M-at-a-$1.15B-Valuation-as-Accounting-Firms-Adopt-End-to-End-Agents-Across-Accounting-Tax-and-Audit); [SiliconANGLE](https://siliconangle.com/2026/02/24/ai-accounting-startup-basis-secures-100m-1-15b-valuation-firms-adopt-agent-based-workflows/)
- **Crete Professionals Alliance** (Thrive Holdings-backed, not General Catalyst): plans >$500M to acquire US accounting firms (June 2025); >$300M annual revenue, 20 businesses, 900 staff; Tax AI processed 7,000 returns with 31% average tax-prep time savings. Rebranded as **Current** (2026-06-02), with Thrive Holdings and OpenAI embedding AI engineers. [C/R] — [Accounting Today](https://www.accountingtoday.com/news/crete-pa-plans-500-million-spend-to-buy-and-upgrade-firms-with-ai); [BusinessWire](https://www.businesswire.com/news/home/20260602293257/en/Crete-Professionals-Alliance-Rebrands-as-Current-to-Equip-Independent-Accounting-Firms-to-Compete-at-Enterprise-Scale)

### Inferences
- Basis sells to accounting firms (who bill clients per engagement), so its buyer isn't looking for per-close pricing; fixed-fee pricing emerges at the firm level.

### Gaps
- Pilot's current fixed-fee bookkeeping pricing and any per-invoice AP-agent pricing were not verified in this pass.

## Recruiting: per hire / per placement

### Takeaway
Recruiting has true outcome pricing because agency contingency (20–30% of first-year salary) is the incumbent model. Paraform is the clearest AI-era contingency marketplace. Mercor's large revenue comes mostly from supplying expert contractors to AI labs rather than per-placement fees. Sourcing tools (Juicebox) sell seats plus credits.

### Cited Findings
- **Paraform**: companies post roles with set payouts (typically $10k–$30k), independent recruiters compete, pay only on hire; 20–25% contingency. $40M Series B, March 2026 (Scale Venture Partners), $65M total, 10,000+ recruiters. [A/R] — [HeroHunt](https://www.herohunt.ai/blog/paraform-pricing-alternatives-2026/); [Staffing Journal](https://www.staffingjournal.ca/priced-like-software-paid-like-a-headhunter/)
- **Mercor**: $2.0B annualized gross revenue (June 2026), $614M H1 2026 gross revenue, $760M run rate end of 2025; $350M Series C at $10B (2025-10-28). Contractors receive 60–70% of top-line, so H1 2026 net revenue est. ~$180–250M. Sacra cites a ~30% recruiting fee on direct placements as "primary"; the contractor revenue-share figures suggest most gross revenue is contractor (hourly) work, not placements. [R/A; internal inconsistency flagged] — [Sacra](https://sacra.com/c/mercor/)
- **Juicebox (PeopleGPT)**: $99/seat/mo (Starter, annual), $179 Growth, credit-metered contacts/exports, $199 per agent/month add-on. [A] — [Glozo](https://www.glozo.com/blog/juicebox-pricing); [HeroHunt](https://www.herohunt.ai/blog/juicebox-pricing-2026-cost-and-alternatives/)

### Inferences
- Verification in recruiting is simple (the hire starts), and buyers are already used to paying 20–30% contingency, so outcome pricing faces low buyer friction here.

### Gaps
- Moonhub (reportedly acqui-hired by Salesforce in 2025) not verified in this pass.

## Security: per alert / per investigation

### Takeaway
AI SOC vendors mostly sell annual capacity bundles denominated in investigations (a unit-of-work proxy), not guaranteed outcomes; pricing is increasingly opaque.

### Cited Findings
- **Dropzone AI**: reported flat pricing from ~$36,000/yr including up to 4,000 investigations per AI analyst (~$9/investigation); Dropzone removed public dollar amounts from its pricing page in 2025. [A; pricing page confirms investigation-capacity framing] — [Dropzone pricing](https://www.dropzone.ai/pricing); [UnderDefense](https://underdefense.com/blog/dropzone-pricing/)
- AI SOC market: per-investigation/per-alert (Dropzone, Prophet), per-endpoint, per-GB, flat platform fee; published figures cluster at ~$9–10/investigation with $36k–$50k annual commitments; 7AI quotes custom enterprise pricing. [A] — [UnderDefense](https://underdefense.com/blog/ai-soc-pricing/)
- **XBOW** (autonomous pentester) reached #1 on HackerOne's US leaderboard (June 2025), submitting ~1,060 vulnerabilities; raised $75M. Its bug-bounty earnings are per-valid-finding by definition, but its commercial product is priced per test (~$4k–$8k reported). [R/A] — [XBOW blog](https://xbow.com/blog/top-1-how-xbow-did-it); [Slashdot](https://it.slashdot.org/story/25/07/05/1847237/xbows-ai-powered-pentester-grabs-top-rank-on-hackerone-raises-75m-to-grow-platform); [Penetrify](https://www.penetrify.cloud/en/compare/penetrify-vs-xbow/)

### Inferences
- An "investigation" is an activity, not an outcome (a closed-false-positive is still billed); vendors avoid charging on "threat caught" because true positives are rare and contestable.

### Gaps
- Torq and Prophet pricing not verified with primary sources.

## Coding: per task / per PR / ACU vs usage

### Takeaway
Coding agents have converged on subscription + usage/credits/rate limits, not per-merged-PR. Devin's ACU is a compute/time unit, not an outcome. Cursor's June 2025 shift to usage-based credits triggered backlash and refunds; Factory moved from token tiers to rate-limited plans in April 2026.

### Cited Findings
- **Devin (Cognition)**: Devin 2.0 (April 2025) cut entry from $500/mo to $20 Core with ACUs at $2.25 each; on 2026-04-14 Cognition retired Core/Team for Free/Pro/Max/Teams/Enterprise, cutting minimum team entry from $500 to $80/mo. [A — multiple guides agree] — [Lindy](https://www.lindy.ai/blog/devin-pricing); [Omid Saffari](https://omidsaffari.com/blog/devin-pricing)
- **Cursor**: June 2025 replaced 500 fast requests with $20 of usage at API rates; CEO Michael Truell apologized on 2025-07-04 and offered refunds for unexpected charges June 16–July 4. [C] — [Cursor blog](https://cursor.com/blog/june-2025-pricing); [FinTech Weekly](https://www.fintechweekly.com/magazine/articles/cursor-pricing-change-user-backlash-refund)
- **Factory**: Pro $20/seat/mo; replaced token-metered tiers with rolling rate limits (5-hour, weekly, monthly windows) around April 2026. [A, docs page exists] — [Factory docs](https://docs.factory.ai/pricing); [Kunavo](https://kunavo.com/guides/factory-droid-pricing)

### Inferences
- Merged-PR pricing hasn't appeared at scale; merges depend on human reviewers and are easily gamed (tiny PRs), so compute-based units dominate. Per-bounty models exist only in marketplaces (bug bounties).

### Gaps
- No verified vendor billing per merged PR; Claude Code weekly limit specifics not captured in this pass.

## Collections, insurance claims, tax

### Takeaway
Collections is naturally contingency-priced; AI entrants claim to undercut agency percentages, but evidence comes mostly from vendor content.

### Cited Findings
- Traditional agencies charge 25–50% contingency (25–35% for <90 days, 40–50% >1 year); AI platforms claim 5–15% success-only fees (AgentCollect claims ~50% recovery in 20 days). [A — vendor self-published] — [AgentCollect](https://www.agentcollect.com/blog/how-much-do-collection-agencies-charge); [ARM Solutions](https://blog.armsolutions.com/blog/what-is-collection-agency-pricing-2026-fee-guide)

### Gaps
- No verified outcome-priced AI insurance-claims or tax vendor with traction found; recovery claims are unverified.

## AI-native services firms and roll-ups

### Takeaway
VCs have bet heavily that outcome pricing wins by owning the services business (selling completed work at firm fees) rather than persuading software buyers to pay per outcome. These are the largest revenue pools in this map, but much is acquired revenue.

### Cited Findings
- Sequoia (Julien Bek, "Services: The New Software," ~April 2026): "autopilots" sell the outcome (closed books, completed tax filing) to the buyer; $6 spent on services for every $1 on software; target categories include insurance brokerage, claims adjustment, IT managed services, tax, accounting/audit, simple legal, payroll, compliance. [C] — [Sequoia](https://sequoiacap.com/article/services-the-new-software); [Fortune, 2026-04-21](https://fortune.com/2026/04/21/services-are-the-new-software-sequoia-venture-capital-julien-bek-ai-native-eye-on-ai/)
- **General Catalyst**: ~$1.5B dedicated to buying accounting firms, call centers, property managers, IT service providers. **Long Lake** (GC-backed, founded 2023) acquired ~30 businesses starting with HOA management and agreed a $6.3B acquisition of American Express Global Business Travel, expected to close H2 2026. [R] — [PitchBook](https://pitchbook.com/news/articles/general-catalysts-6-3b-amex-deal-puts-its-ai-roll-up-strategy-on-display); [CNBC, 2026-06-08](https://www.cnbc.com/2026/06/08/silicon-valleys-new-buyout-playbook-is-hitting-wall-street.html); [GC](https://www.generalcatalyst.com/stories/the-future-of-services)
- Crete/Current (Thrive) >$300M revenue — see finance section. Smarter Technologies (New Mountain) >$800M — see healthcare section.

### Gaps
- Titan (GC's MSP roll-up) specifics not verified in this pass; no disclosed organic growth or margin data for roll-ups.

## Who is winning, and where vendors retreated

### Takeaway
Outcome/unit-of-work pricing has clearly won in (1) healthcare revenue recovery (contingency), (2) recruiting (placement fees: Paraform), (3) plaintiff PI legal (EvenUp per case, now moving to services), and (4) collections. It lost or was never adopted in AI SDRs (Qualified rejected it; 11x/Artisan are subscriptions; Salesforce uses per-action credits), coding (Cursor, Devin, Factory use usage/credits/rate limits), security (investigation-capacity bundles), ambient scribes and Big Law (seats, plus Legora metering).

### Cited Findings
- Qualified rejection of meetings/pipeline pricing (2025-07-22) — [Sequence](https://www.sequencehq.com/blog/how-qualified-priced-their-ai-sdr-agent-from-add-on-to-all-in-strategy)
- Cursor's 2025 usage-pricing backlash and refunds — [Cursor](https://cursor.com/blog/june-2025-pricing)
- Dropzone hid public prices (2025) — [UnderDefense](https://underdefense.com/blog/dropzone-pricing/)
- Hybrid rising to 37% primary model (2026) — [Growth Unhinged](https://www.growthunhinged.com/p/the-state-of-b2b-monetization-in-2026)
- Traction leaders by disclosed numbers: Mercor ($2.0B annualized gross, mid-2026 [R]); Smarter Technologies (>$800M expected revenue [C]); Crete/Current (>$300M [R]); Abridge ($100M+ ARR, seat-priced [A]); EvenUp ($2B valuation, 30% of top-100 PI firms [C]).

### Inferences
- The pattern: outcome pricing works when (a) the outcome is already a monetized event with an incumbent contingency market, (b) verification is external (payer remittance, hire start date, settlement), and (c) the vendor controls enough of the workflow to own the outcome — often by becoming a services firm.
- Where outcomes depend on the customer's own inputs (lead lists, code review, alert quality), vendors fall back to credits/usage.

### Gaps
- Few vendors disclose the split of revenue by pricing model; "winning" judgments rely on valuations and aggregate revenue, not unit economics.

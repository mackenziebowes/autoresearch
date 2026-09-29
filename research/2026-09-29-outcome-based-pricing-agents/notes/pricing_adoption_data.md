# AI Agent Pricing Models: Aggregate Adoption Data and Buyer-Side Evidence (seat, usage, credit, hybrid, outcome)

Research date: 2026-09-29. All sources dated where possible. Many "stat roundup" blogs (Nevermined, Pilot, Monetizely, eesel, Enterprise DNA, etc.) repeat figures without primary citations; those are flagged or excluded.

## 1. How widespread is outcome-based pricing (vendor-side surveys), and how is it trending?

### Takeaway
Vendor surveys from 2025–2026 put outcome-based pricing at roughly 5% to 23% of AI/software vendors. The spread comes from how each survey samples and how it defines the model. The dominant shift is from pure seat and flat-fee toward hybrid and credit models. Outcome-based pricing is growing quickly from a small base, but it is still a minority model and is usually layered onto a subscription.

### Cited Findings
- **Growth Unhinged / Kyle Poyar, 2025 State of B2B Monetization** (n=240 software and AI companies, surveyed April–May 2025; typical respondent US-based, $1–20M ARR). Flat-fee fell from 29% to 22%, seat-based from 21% to 15%, and hybrid rose from 27% to 41%. Outcome-based pricing was **5%** at the time, and respondents *expected* 25% by 2028. — [Growth Unhinged 2025](https://www.growthunhinged.com/p/2025-state-of-b2b-monetization); PDF co-published with Tremont VP: [Tremont](https://www.tremontvp.com/post/the-2025-state-of-b2b-monetization)
  - Search snippets elsewhere quote "7% of AI product monetization" as outcome-based, and say outcome-based pricing is "out of reach for 95% of the market." The fetched page gave 5%. Treat the figure as roughly 5–7% depending on which cut is used. — [Growth Unhinged 2025](https://www.growthunhinged.com/p/2025-state-of-b2b-monetization)
- **Growth Unhinged 2026 State of B2B SaaS and AI Monetization** (n=230 software and AI companies, surveyed April–May 2026; ARR mix: 22% under $1M, 28% $1–20M, 24% $20–150M, 25% over $150M):
  - Hybrid is the most popular model at 37%, up from 25% "12 months ago." This 25% baseline conflicts with the 41% hybrid figure in the 2025 report, which suggests a changed definition or sample. Flag this before comparing the two years.
  - AI credits are used by 29% of companies (up 126% YoY in 2025). A further 33% plan to add credits in 6–12 months, including "roughly 1-in-2" companies above $50M ARR.
  - 29% offer customers a choice of pricing models, up from 21%. 75% changed pricing or packaging in the past year.
  - Seat pricing persists at scale: 29% of companies above $150M ARR use per-seat.
  - The public summary did not give a precise outcome-based share for 2026.
  - Source: [Growth Unhinged 2026](https://www.growthunhinged.com/p/the-state-of-b2b-monetization-in-2026)
- **ICONIQ State of AI** (about 300 executives building AI products, ICONIQ portfolio and non-portfolio; waves in April 2025 and December 2025):
  - 58% still include a subscription or platform component.
  - Consumption-based pricing is at 35%, and outcome-based at **18%**. Secondary coverage says outcome-based rose from **2% in Q2 2025 to 18%** by the January 2026 publication.
  - 37% plan to change their AI pricing model in the next 12 months, driven by customer demand, competitive pressure and margin concerns.
  - Among outcome-based models, 36% tie price to cost savings and 18% to revenue generated.
  - Annual commitments and tiered overages are the most common safeguards on consumption and outcome models.
  - Source: [ICONIQ Bi-Annual Snapshot](https://www.iconiq.com/growth/reports/2026-state-of-ai-bi-annual-snapshot); [ICONIQ 2026 State of AI](https://www.iconiq.com/growth/reports/state-of-ai-2026)
- A secondary source reports a later ICONIQ wave: consumption rising from 35% to 42%, outcome-based from 18% to 23%, and an average of 1.7 pricing models per company. I did not verify this against ICONIQ's own page. — [AI Impact Foundation, May 2026](https://aiimpactfoundation.com/blog/2026-05/state-of-ai-monetization)
- ICONIQ's 2025 survey, cited by BCG, found 68% of vendors charge separately for AI or restrict it to premium tiers. — [BCG, Aug 13 2025](https://www.bcg.com/publications/2025/rethinking-b2b-software-pricing-in-the-era-of-ai)
- **Gartner (services contracts, not SaaS)**: "Only 19% of services buyers and 13% of service agreements on the seller side use such [outcome-based] arrangements today" (Tom Coshow, VP analyst). Gartner projects that through 2031, fewer than 25% of tech CEO services contracts will adopt outcome-based pricing. — [CIO Dive, Aug 31 2026](https://www.ciodive.com/news/agentic-ai-outcome-pricing-models/829023/)
- McKinsey reportedly earned about 25% of its 2025 global client fees from outcome-based contracts. This comes from a secondary summary; I did not find a primary McKinsey source. — cited via [TheStreet](https://www.thestreet.com/markets/ai-is-forcing-mckinsey-bcg-bain-to-rethink-consulting-fees) (search snippet)

### Inferences
- The surveys disagree (Growth Unhinged about 5–7%, ICONIQ 18–23%) mainly because of sampling. ICONIQ surveys AI-product builders, often AI-native companies in agentic categories such as support. Growth Unhinged covers a broader set of SaaS companies, many of which are small. ICONIQ's "outcome-based" category may also count a subset of a company's pricing, since companies blend about 1.7 models. The share of companies with *some* outcome-priced SKU is therefore much higher than the share whose revenue is *primarily* outcome-priced.
- Credits and hybrid models are the fastest-growing *mainstream* choice. Outcome-based pricing is the fastest-growing in percentage terms, but from a small base.
- Vendor bias: ICONIQ is a VC/growth investor. Growth Unhinged's report is co-sponsored by pricing and billing vendors (Tremont VP distributes the PDF). Billing vendors (Metronome, Orb, Chargebee, Paid.ai) have a commercial interest in promoting usage and outcome models.

### Gaps
- No 2025–2026 primary survey data was retrieved from OpenView (which appears to have stopped its SaaS benchmarks after the firm wound down), Metronome, Orb, Stripe, Chargebee, Maxio, Paid.ai, Ibbaka or Simon-Kucher, within the tool budget.
- The exact outcome-based share in Growth Unhinged 2026 was not visible in the public summary.
- Bessemer cohort data broken out by pricing model was not found.

## 2. Analyst forecasts and consulting views (Gartner, IDC, Forrester, McKinsey, BCG, Deloitte)

### Takeaway
Analysts are skeptical about near-term adoption. Gartner calls the rise of outcome-based pricing "more buzz than reality," and it separately warns that inference costs per agentic workflow will rise sharply, which pressures fixed-price and outcome models. BCG recommends hybrid (seat plus agentic) transitions rather than a jump to pure outcome pricing.

### Cited Findings
- Gartner's Tom Coshow: "Right now, what we see is that the increase in outcome-based pricing is more buzz than reality." He advises CIOs to ask, "If the vendor isn't taking on the risk, why are you bothering with outcome-based pricing?" — [CIO Dive, Aug 31 2026](https://www.ciodive.com/news/agentic-ai-outcome-pricing-models/829023/)
- Gartner, per the same article: fewer than 25% of tech CEO services contracts will be outcome-based through 2031. Gartner also says "vendors expect to be paid more, not less" as customers hand work to agents, and it expects the cost-to-value gap in process-centric service contracts to shrink at least 50% by 2027 because of agentic AI. — [CIO Dive / Channel Dive, Aug 2026](https://www.channeldive.com/news/agentic-ai-outcome-pricing-models-zendesk-gartner/829209/)
- Gartner (Aug 17 2026 press release) predicts AI inference costs per agentic workflow will rise more than fivefold through 2028. I could not open the page (HTTP 403), so this is the headline only. — [Gartner press release](https://www.gartner.com/en/newsroom/press-releases/2026-08-17-gartner-predicts-ai-inference-costs-per-agentic-workflow-will-increase-more-than-fivefold-through-2028)
- Gartner (Aug 26 2025): 40% of enterprise apps will feature task-specific AI agents by end of 2026, up from under 5% in 2025. — [Gartner](https://www.gartner.com/en/newsroom/press-releases/2025-08-26-gartner-predicts-40-percent-of-enterprise-apps-will-feature-task-specific-ai-agents-by-2026-up-from-less-than-5-percent-in-2025)
- BCG (Aug 13 2025) sets out five agentic pricing models: usage by resources, agent-based subscription, usage by interactions, outcome by jobs, and outcome by financial result.
  - Its recommendations are to run a hybrid of seats and agentic pricing, to transition gradually using leading indicators (agentic usage, fewer seats bought), and to avoid cost-plus pricing because model costs fall.
  - It also recommends building telemetry, value baselining and usage forecasting capabilities.
  - Source: [BCG](https://www.bcg.com/publications/2025/rethinking-b2b-software-pricing-in-the-era-of-ai)
- McKinsey has published on managing the cost versus value of agentic AI systems. I did not extract specific pricing-model statistics. — [McKinsey](https://www.mckinsey.com/capabilities/quantumblack/our-insights/cost-versus-value-managing-agentic-ai-system-performance)

### Inferences
- Taken together, the analyst view is that outcome pricing will grow but stay a minority (below 25%) through the end of the decade. Hybrid is the default.

### Gaps
- IDC and Forrester agent-pricing forecasts were not retrieved.
- No Deloitte figures were found.
- I could not verify a widely repeated "Gartner: X% of agentic AI will use outcome-based pricing by 202X" statistic from a primary Gartner source, so I have not included one.

## 3. Buyer sentiment: CFO and procurement views

### Takeaway
Buyers value outcome alignment in principle, but their top practical concern is predictability, and many struggle to define measurable outcomes. Buyer surveys conflict. One (Futurum) shows stated preference for consumption (43%) and outcome (27%) over seats. Procurement behavior and vendor earnings calls show buyers pushing for committed, budgetable envelopes. Vendors that tried pure metering (Salesforce $2 per conversation, HubSpot credits) moved toward predictability.

### Cited Findings
- a16z buyer survey, as cited by BCG:
  - "47% of buyers struggle to define clear, measurable outcomes"
  - "36% worry about cost predictability"
  - "25% face difficulty aligning on value attribution with vendors"
  - "24% acknowledge that outcomes often depend on factors outside of vendors' control"
  - Source: [BCG, Aug 2025](https://www.bcg.com/publications/2025/rethinking-b2b-software-pricing-in-the-era-of-ai)
- BCG IT buyers survey: 40% of buyers cite seat reduction as their main lever for cutting software spend, and 48% plan to increase AI/GenAI spend in the next 12 months. — [BCG](https://www.bcg.com/publications/2025/rethinking-b2b-software-pricing-in-the-era-of-ai)
- Futurum 1H 2026 Enterprise Software Decision Makers survey (published May 12 2026; sample size not disclosed): 43% of buyers prefer consumption-based pricing and 27% prefer outcome-based. "Fewer than one in five buyers still prefer classic per-user pricing models." — [Futurum](https://futurumgroup.com/press-release/are-outcome-based-and-hybrid-ai-pricing-models-rewriting-the-vendor-playbook/)
- A buyer example: Seton Hall University CIO Paul Fisher said outcome models "add complication to contract negotiation as you need to be very specific on what the desired outcomes are." He also noted they incentivize the partner to "get it right." — [CIO Dive, Aug 31 2026](https://www.ciodive.com/news/agentic-ai-outcome-pricing-models/829023/)
- Pegasystems charges fixed fees per completed case and absorbs AI costs to keep pricing predictable. This is a vendor responding to buyers' dislike of token variability. — [CIO Dive](https://www.ciodive.com/news/agentic-ai-outcome-pricing-models/829023/)
- HubSpot CEO Yamini Rangan (Q2 2026 release, Aug 5 2026): "Scaling companies want real outcomes and predictable pricing when adopting AI." — [Digital Applied summary of filed release](https://www.digitalapplied.com/blog/hubspot-q2-2026-ai-pricing-credits-agent-adoption)
- BlackLine's CFO has promoted a shift to outcome-based AI pricing. The CFO Dive article frames finance chiefs as seeking predictability. — [CFO Dive](https://www.cfodive.com/news/blackline-cfo-touts-outcome-based-ai-pricing-shift/825940/) (not fully fetched)
- Stats often repeated in blogs ("78% of IT leaders report unexpected charges from consumption/AI pricing," "90% of CIOs cite cost forecasting as top challenge") appeared only in aggregator content without a traceable primary source. Treat them as unverified.

### Inferences
- Stated preference in the Futurum poll leans toward flexible models. Revealed behavior at incumbents (HubSpot, Salesforce, ServiceNow) shows buyers want an outcome or usage *unit* inside a *capped or committed* budget envelope. "Predictable outcome pricing," meaning a per-resolution price with spend caps or commits, is the synthesis.
- The biggest reported barrier is defining outcomes (47%), which points to disputes over definitions. Resolution-based pricing in customer support works because "resolution" is relatively binary. Most other domains lack such a clean unit.

### Gaps
- I found no rigorous, large-sample CFO or procurement survey focused specifically on outcome pricing for AI agents.
- I found no quantitative data on contract disputes over outcome definitions, only anecdotes.

## 4. Incumbent moves as signals

### Takeaway
Most incumbents moved toward **credits and hybrid envelopes** (Salesforce Flex Credits, then AELA per-user; Microsoft Copilot Credits; Atlassian Rovo credits; ServiceNow assists). Outcome-based pricing is concentrated in **customer service**: Zendesk per automated resolution, HubSpot per resolved conversation, and Intercom and Sierra among AI-natives. The stated reason in every case is customer demand for predictability, together with seat cannibalization.

### Cited Findings
- **Salesforce Agentforce** has three concurrent models:
  - $2 per conversation (October 2024), largely abandoned for unpredictability.
  - Flex Credits at $0.10 per action (May 2025), sold as 100,000 credits for $500 with 20 credits per standard action.
  - Per-user AELA at $125+ per month (late 2025), with seats that "include your digital workforce."
  - Agentforce had $540M ARR by Q3 FY2026 (+330% YoY), with 18,500 deals of which 9,500 were paid, and about 8% penetration of 150K+ customers.
  - SaaStr's Jason Lemkin calls running parallel models "the only honest response to a market that hasn't made up its mind yet."
  - Sources: [SaaStr, Feb 16 2026](https://www.saastr.com/salesforce-now-has-3-pricing-models-for-agentforce-and-maybe-right-now-thats-the-way-to-do-it); [Salesforce Flex pricing PR, May 15 2025](https://www.salesforce.com/news/press-releases/2025/05/15/agentforce-flexible-pricing-news/)
  - Per secondary coverage, from Q2 FY27 Agentforce ARR also includes Slackbot and Headless 360, so part of the Q1-to-Q2 jump was a definitional change. — [jitendrazaa.com](https://www.jitendrazaa.com/blog/salesforce/salesforce-agentforce-credits-cost-model-complete-guide-2026/) (secondary)
- **Microsoft** uses metered Copilot Credits for Copilot Studio and agents, replacing per-message billing from Sept 1 2025. Credits cost $200 per 25,000-credit pack per month, or $0.01 per credit pay-as-you-go, pooled at tenant level. This sits alongside M365 Copilot seats at $360 per user per year (Sept 2026 list). — [Microsoft Copilot Studio pricing](https://www.microsoft.com/en-us/microsoft-365-copilot/pricing/copilot-studio); [Microsoft Learn billing](https://learn.microsoft.com/en-us/microsoft-copilot-studio/requirements-messages-management)
- **ServiceNow** uses a hybrid model with an envelope of included assists, reload packs, and about a 30% uplift on the Pro SKU. Amit Zavery on the call: "the hybrid model has seemed to be resonating with my customers. They know what the envelope they have, what they will be consuming beyond that…" About 50% of the business is non-seat while seats keep growing. AI ACV passed $1B in Q2 2026. — [ServiceNow Q2 2026 transcript (Investing.com)](https://www.investing.com/news/transcripts/earnings-call-transcript-servicenow-beats-q2-2026-forecasts-shares-rebound-after-hours-93CH-4807190); [TechTarget on ServiceNow AI pricing change](https://www.techtarget.com/searchitoperations/news/366641692/ServiceNow-AI-pricing-change-takes-on-enterprise-ROI-struggles)
- **HubSpot** moved from credit-metered Breeze agents toward outcome-based pricing. It cut the customer-conversation price from $1.00 to $0.50, charged only on resolution, lowered entry points and added spend thresholds (Q2 2026, Aug 2026). Commentators described this as conceding that credit pricing was holding back adoption. Agent metrics: Customer Agent at 10,000+ customers with a 72% resolution rate. — [Digital Applied, Aug 9 2026](https://www.digitalapplied.com/blog/hubspot-q2-2026-ai-pricing-credits-agent-adoption); [SaaS Intelligence](https://saasintelligence.substack.com/p/hubspot-sells-ai-outcomes-most-vendors)
- **Atlassian** is introducing metered usage including Rovo credits, with usage billing effective Dec 3 2026 for most meters. The Information groups Atlassian and HubSpot as joining a shift away from flat AI fees. — [The Information](https://www.theinformation.com/articles/atlassian-hubspot-join-shift-ai-flat-fees); [SaaSRise](https://www.saasrise.com/blog/the-2026-saas-pricing-guide-2)
- **Zendesk** charges per automated resolution, with payment only when the AI resolves the issue end to end. List price is about $1.50 per resolution, per secondary sources. Zendesk expanded the model with its Resolution Platform at Relate 2026. — [Futurum](https://futurumgroup.com/insights/zendesk-bets-on-autonomous-ai-agents-outcome-pricing-to-upend-service-models/); [Zendesk blog](https://www.zendesk.com/blog/ai/agentic-ai/outcome-based-pricing/)
- GitHub, Notion, monday.com, Gong and Docusign are reported to have rebuilt billing around credits. This is secondary and unverified individually. — [search summary of SaaSRise/The Information coverage](https://www.saasrise.com/blog/the-2026-saas-pricing-guide-2)

### Inferences
- Pattern: **outcomes where the unit is binary and attributable** (support resolutions), and **credits or hybrid where work is heterogeneous** (platforms such as Microsoft, Salesforce, ServiceNow and Atlassian).
- Salesforce went from per-conversation (quasi-outcome) to credits to seats. HubSpot went from credits to per-resolution. They moved in opposite directions, and both cited predictability. This suggests "predictability" is served by caps and commits more than by any particular metering unit.

### Gaps
- No Workday pricing details were retrieved; Workday Flex Credits are unverified.
- I did not find direct Benioff earnings-call quotes on AELA reasoning.

## 5. Evidence linking pricing model to growth, NRR or win rates

### Takeaway
The evidence is anecdotal and vendor-reported. Intercom Fin, Sierra and Salesforce show fast growth, but no cohort study controls for category. No ICONIQ, Bessemer or Stripe data compares NRR or growth by pricing model.

### Cited Findings
- **Intercom Fin** charges $0.99 per resolution.
  - ARR went from about $1M to $12M in its first year and is now about $100M, on pace to be roughly 50% of Intercom's approximately $400M ARR.
  - **New-customer NRR improved from 112% to 146%**, per CFO Dan Griggs.
  - The resolution rate rose from about 25% to "north of 65–70%."
  - Intercom deliberately sacrificed about $60M of legacy ARR to force migration.
  - Source: [Mostly Metrics (CJ Gustafson), 2025](https://www.mostlymetrics.com/p/how-intercom-reaccelerated-growth-with-outcome-based-pricing); later coverage of nearly $100M ARR: [Enterprise DNA, Aug 2 2026](https://enterprisedna.co/resources/ai-pulse/ai-pulse-2026-08-02-intercom-s-fin-ai-agent-is-nearing-100m-arr-roughly-half-of/)
- **Sierra** charges per resolved interaction (outcome-based). It is reported at a $15.8B valuation and about $100M ARR in seven quarters, rising to roughly $150–200M by early or mid 2026. These are secondary sources with inconsistent round labels (Series C vs Series E). — [Value Add VC](https://valueaddvc.com/blog/sierra-ai-valuation-2026-15-8b-series-e-enterprise-ai-agents)
- Salesforce Agentforce, a mixed model, reached $540M ARR (+330% YoY) by Q3 FY2026. — [SaaStr](https://www.saastr.com/salesforce-now-has-3-pricing-models-for-agentforce-and-maybe-right-now-thats-the-way-to-do-it)
- Growth Unhinged 2026 lists "not enough expansion revenue" as the top pricing concern overall. — [Growth Unhinged 2026](https://www.growthunhinged.com/p/the-state-of-b2b-monetization-in-2026)
- A widely repeated claim says seat-priced AI products have "40% lower gross margins and 2.3x higher churn" than usage- or outcome-priced ones. It appeared in a search-engine summary attributed loosely to Growth Unhinged and pricing blogs, but I could not locate it in the fetched Growth Unhinged content. **Unverified; do not use** without a primary source.

### Inferences
- Intercom's NRR jump is the strongest single data point that outcome pricing can drive expansion. Outcome pricing grows automatically with resolution rate, so product improvement converts directly to revenue. The result is confounded by a forced migration and by category tailwinds.
- Survivorship bias: the celebrated outcome-priced companies all sit in customer support, the category with the clearest outcome unit.

### Gaps
- No ICONIQ, Bessemer or Stripe cohort data ties pricing model to NRR, growth or win rate.
- No win-rate or deal-conversion data by pricing model was found.

## 6. Gross-margin implications for vendors paying inference costs

### Takeaway
AI gross margins are well below SaaS norms (a median target of about 50–52% versus 70–80%+). Margin variance across customers can be extreme. Outcome pricing shifts inference-cost risk onto the vendor, and Gartner's forecast of rising per-workflow inference costs makes that risk larger.

### Cited Findings
- The median AI margin target is 50%, against 70–80%+ for SaaS. 12% of companies target 80%+ AI margins, and about 12% target 20% or lower. Internal costs and margins are cited as the most important factor in AI pricing (2025). — [Growth Unhinged 2026](https://www.growthunhinged.com/p/the-state-of-b2b-monetization-in-2026); [Growth Unhinged 2025](https://www.growthunhinged.com/p/2025-state-of-b2b-monetization)
- ICONIQ: AI companies expect gross margins of about 52% on average in 2026. Margin concerns are one of the top three drivers of planned pricing changes, and annual commits and tiered overages are the main safeguards. — [ICONIQ](https://www.iconiq.com/growth/reports/2026-state-of-ai-bi-annual-snapshot)
- BCG: "One vendor of customer engagement software has experienced margin variance exceeding 70 percentage points across different customer accounts." BCG warns against cost-plus pricing because model costs fall. — [BCG](https://www.bcg.com/publications/2025/rethinking-b2b-software-pricing-in-the-era-of-ai)
- Gartner: inference costs per agentic workflow will rise more than fivefold through 2028 (headline only). — [Gartner, Aug 17 2026](https://www.gartner.com/en/newsroom/press-releases/2026-08-17-gartner-predicts-ai-inference-costs-per-agentic-workflow-will-increase-more-than-fivefold-through-2028)
- Pegasystems absorbs AI costs under fixed per-case fees, taking margin risk to give buyers predictability. — [CIO Dive](https://www.ciodive.com/news/agentic-ai-outcome-pricing-models/829023/)
- Intercom CFO: even at $5–6 cost per ticket, customers "are getting a great bargain" at $0.99 per resolution. The value gap leaves pricing headroom. — [Mostly Metrics](https://www.mostlymetrics.com/p/how-intercom-reaccelerated-growth-with-outcome-based-pricing)

### Inferences
- Under outcome pricing, unresolved attempts still consume inference but earn no revenue. Vendor margin therefore depends on resolution rate and cost per attempt. Fixed per-unit prices face pressure from both sides: falling token prices drive price competition, while rising tokens per workflow (Gartner) raise costs. Credits and usage pricing pass more of that variance to the buyer, which explains why platforms with heterogeneous workloads prefer credits.

### Gaps
- No quantitative comparison of gross margins between outcome-priced and usage-priced vendors was found.
- Intercom, Sierra and Zendesk do not disclose AI-specific gross margins.

# Outcome-Based Pricing Among Customer Support / CX AI Agent Vendors (as of 2026-09-29)

Source-confidence legend: **[confirmed]** = vendor primary source (pricing page, help doc, press release, vendor blog); **[reported]** = reputable press or analyst (CNBC, SaaStr, Futurum, Mostly Metrics); **[aggregator]** = SEO/comparison sites or competitor-authored pages (eesel, getmacha, myaskai, valueaddvc, GetLatka, fin.ai/learn, etc.). Treat aggregator figures as directional only. Funding rounds are intentionally not re-researched (see sibling notes in `2026-09-29-ai-agent-funding-landscape/notes/vertical_enterprise_agents.md`).

## Q1. How does each vendor price, and what exactly is a billable "outcome"?

### Takeaway
Only a handful of vendors actually bill per outcome at list price: Intercom/Fin ($0.99), Zendesk (~$1.50–$2.00), HubSpot ($0.50), Crescendo, and Sierra (custom contracts, reportedly ~$1.50). The rest bill per conversation (Decagon, Ada, Salesforce's $2 option), per action or credit (Salesforce Flex Credits, Microsoft Copilot Studio, ServiceNow "assists"), or per seat or platform fee (Salesforce AELA, Forethought, ServiceNow tiers). Even the outcome vendors define a "resolution" mostly as the customer not escalating and not coming back within a fixed window (24h at Fin, up to 72h at Zendesk and HubSpot). It is not verified problem-solving.

### Cited Findings

**Intercom / Fin (renamed from Intercom to "Fin" in May 2026; Salesforce is acquiring it)**
- Price: **$0.99 per outcome**. The base plan is $49/month and includes 50 resolutions; additional outcomes are $0.99 each. Unused resolutions do not roll over. At most one charge per conversation, however many questions or procedures it contains. [confirmed] — [Fin help: Fin pricing outcomes](https://fin.ai/help/en/articles/13975800-fin-pricing-outcomes)
- **2025–26 change: "resolution" became "outcome," and there are now two billable outcome types.**
  - (1) **Resolution**: the customer confirms satisfaction, or "exits the conversation without requesting further assistance".
  - (2) **Procedure handoff**: Fin "successfully executes a Procedure that you've configured to end in a handoff to a human or a workflow". This means some handoffs to humans are now billable. [confirmed] — [Fin help](https://fin.ai/help/en/articles/13975800-fin-pricing-outcomes)
- **Hard vs soft resolution.**
  - A "Confirmed resolution" is an affirmative customer response.
  - An "Assumed resolution" is: "If a customer disengages from the conversation for 24 hours after Fin's last answer." Both are billable.
  - If the customer returns to the same conversation seeking help, "that resolution will be deducted and not charged." [confirmed] — [Fin help](https://fin.ai/help/en/articles/13975800-fin-pricing-outcomes)
- **Not billable:**
  - escalations triggered by workspace rules or Fin's default behavior
  - procedure failures or technical errors
  - conversations where Fin's last message was a question rather than an answer
  - an explicit request for a human [confirmed] — [Fin help](https://fin.ai/help/en/articles/13975800-fin-pricing-outcomes)
- Intercom reportedly deliberately gave up about **$60M of legacy-pricing ARR** to force migration to the new model. CFO Dan Griggs: "We're trading dollars per seat at, say, a hundred bucks a seat for $300 of Fin. That's an easy trade to make." [reported] — [Mostly Metrics (CJ Gustafson)](https://www.mostlymetrics.com/p/how-intercom-reaccelerated-growth-with-outcome-based-pricing)

**Sierra (outcome-based since its 2024 launch)**
- Sierra's own framing: "Sierra gets paid only when we complete a task for you… With no reliance on seat-based pricing, we have no conflicting incentives." The original Dec 2024 post was republished or expanded on **2026-05-12**, and Sierra calls it "one of our most read posts and the subject of intense debate in our industry." [confirmed] — [Sierra blog: Outcome-based pricing for AI agents](https://sierra.ai/blog/outcome-based-pricing-for-ai-agents)
- Sierra's "Outcomemaxxing" post gives only abstract outcome examples: "complete the mortgage application, retain the subscriber, process the insurance claim, increase the basket size".
  - It concedes that outcome pricing "is more complex than seat-based or consumption pricing — operationally, contractually, accounting-wise."
  - It adds: "we're not dogmatic about it and do consumption-style work where it fits the shape of the problem."
  - It discloses no share of contracts on outcome pricing, no verification method and no margins. [confirmed] — [Sierra blog: Outcomemaxxing](https://sierra.ai/blog/outcomemaxxing)
- Reported contract structure:
  - roughly **$1.50 per resolved interaction**
  - no charge for unresolved or escalated conversations
  - low-value interactions (e.g., greeter conversations) billed on consumption
  - definitions "vary by contract"
  - since a **July 2026 "Horizon" launch**, the outcome can be a multi-week business goal
  
  [aggregator; unverified, and the Horizon claim could not be confirmed in a primary source] — [Fin.ai/learn: Sierra pricing (competitor-authored)](https://fin.ai/learn/sierra-ai-pricing); [valueaddvc](https://valueaddvc.com/blog/how-does-sierra-ai-make-money-outcome-based-pricing-enterprise-agents-and-the-business-model-breakdown)
- Bret Taylor has discussed the logic of outcome-based pricing publicly on podcasts (Cheeky Pint / Stripe; Lenny's Podcast). The transcripts were not fetched. — [Sierra podcast page](https://sierra.ai/resources/podcasts/bret-taylor-of-sierra-on-ai-agents-outcome-based-pricing-and-the-openai-board)

**Decagon: per conversation (default) or per resolution**
- Decagon offers both models and says **"the vast majority of Decagon customers choose per-conversation pricing."**
  - Its argument against outcome pricing: customers face "unpredictable invoices and the constant renegotiations often required with outcome-based pricing, where the nature of 'successful outcomes' are unclear." It asks: "If a frustrated customer stops responding, does that count as 'resolved'? If the agent provides a trivial answer, should it cost the same as a major technical fix?"
  - It also argues per-conversation pricing means "We're not incentivized to push partial resolutions or sidestep tough cases." [confirmed] — [Decagon blog: Pricing the AI Agent Economy](https://decagon.ai/blog/pricing-ai-agents)
- Price levels are not published. Reported figures:
  - ~$0.99/conversation
  - per-resolution estimates of ~$1.50 (Sacra) or ~$0.50 negotiated
  - a reported ~$50K/yr platform fee
  - Vendr-style median ACV of ~$433K (range $105K–$923K)
  
  [aggregator; much of it competitor-authored] — [fin.ai/learn: Decagon pricing](https://fin.ai/learn/decagon-ai-pricing); [featurebase](https://www.featurebase.app/blog/decagon-pricing)

**Salesforce Agentforce: $2 per conversation, then Flex Credits, then seats (AELA). All three are now sold in parallel.**
- Timeline [reported, SaaStr, 2026-02-16]:
  - **Oct 2024**: $2 per conversation.
  - **May 2025**: Flex Credits at $0.10/action (20 credits; 100K credits = $500).
  - **Late 2025**: Agentic Enterprise License Agreement (AELA), per-user at **$125+/user/month**.
  
  All three run simultaneously. — [SaaStr](https://www.saastr.com/salesforce-now-has-3-pricing-models-for-agentforce-and-maybe-right-now-thats-the-way-to-do-it/)
- Why it changed, per SaaStr:
  - With the $2 model, "buyers couldn't model their costs", and it "immediately priced out nonprofits, SMBs, and any organization without a large AI budget".
  - Credits were "more granular" but still "fundamentally unpredictable for enterprise procurement".
  - Seats gave CFOs "predictable budgeting". [reported] — [SaaStr](https://www.saastr.com/salesforce-now-has-3-pricing-models-for-agentforce-and-maybe-right-now-thats-the-way-to-do-it/)
- Voice actions are 30 credits ($0.15). A $2 conversation equals about 20 actions, so conversations are cheaper above roughly 20 actions each. The Flex Credits launch date was 2025-05-15. [aggregator] — [getmacha](https://www.getmacha.com/blog/agentforce-pricing-explained); [concret.io](https://www.concret.io/blog/new-agentforce-pricing-model)
- The $2 conversation is defined as a 24-hour session. Salesforce has also introduced an "Agentic Work Unit (AWU)" metric, but "correspondence between the AWU and the price is still unclear." [reported, 2026-06-09] — [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice)
- Note: none of Salesforce's models is truly outcome-based. The $2 conversation is billed whether or not the conversation resolves.

**Zendesk: per automated resolution (since Aug 2024), extended in 2026 to custom "outcome-based" contracts**
- Pricing: ~**$1.50 per automated resolution on committed volume, $2.00 pay-as-you-go**, launched Aug 2024. An automated resolution is a request resolved by the AI agent without escalation to a human. [aggregator, consistent across several sources] — [eesel](https://www.eesel.ai/blog/zendesk-ai-dynamic-pricing-resolution); [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice)
- Verification:
  - There is an inactivity window of **up to 72 hours**, which is channel-dependent and may be shorter for messaging.
  - A resolution also counts if the user confirms satisfaction or accepts a suggested help-center article, or if an AI evaluation of the transcript after the window judges it resolved. [reported] — [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice)
  - Zendesk says each resolution passes "rule-based checks and AI assessment". [aggregator paraphrasing Zendesk] — [eesel](https://www.eesel.ai/blog/zendesk-ai-dynamic-pricing-resolution)
- **May 2026 announcement (Futurum, 2026-05-20).**
  - Zendesk moved to "charging only for verifiably resolved interactions". It builds custom contracts with "clear, auditable success metrics and pricing guardrails", with engineering teams working alongside customers.
  - Supporting tools are "Quality Score" for continuous QA and a "Context Graph".
  - Proof points: internal "Zen on Zen" 60% autonomous resolution; BritBox 47% autonomous resolution; a DMV customer at 70% within 3 days.
  - Futurum's Keith Kirkpatrick called it "a decisive break from traditional SaaS support playbooks." [reported] — [Futurum Group](https://futurumgroup.com/insights/zendesk-bets-on-autonomous-ai-agents-outcome-pricing-to-upend-service-models/)
- Zendesk's framing piece. [confirmed, undated in fetch] — [Zendesk blog: Understanding outcome-based pricing](https://www.zendesk.com/blog/ai/agentic-ai/outcome-based-pricing/)

**HubSpot (comparison point)**
- Breeze Customer Agent is **$0.50 per resolved conversation** (no human handoff; up to 72h inactivity). Breeze Prospecting Agent is $1.00 per recommended lead. [reported, 2026-06-09] — [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice)

**ServiceNow: tiered seats plus "assist" consumption, not outcome-based**
- On **2026-04-09**, three AI-native tiers (Foundation, Advanced, Prime) replaced the legacy Standard/Pro/Pro Plus/Enterprise/Enterprise Plus tiers.
- Effective **2026-07-23**:
  - an incident summary costs 1 "assist"
  - one agentic workflow costs **25–150 assists**, depending on how many tools it calls
- There are no public list prices. [aggregator] — [workagent.ai](https://workagent.ai/servicenow-ai-agents-pricing); [Crossfuze](https://www.crossfuze.com/campaigns/servicenow-pricing-changes-2026); [Redress Compliance](https://redresscompliance.com/servicenow-now-assist-ai-pricing-guide.html)

**Microsoft Copilot Studio / Dynamics: credits (consumption), not outcome-based**
- Copilot Credits:
  - prepaid pack of $200/month for 25,000 credits, no rollover; or pay-as-you-go at $0.01/credit
  - a scripted answer is 1 credit; a generative answer is 2
  - external, customer-facing traffic always consumes credits
- Internal use is included with the $30/user/month M365 Copilot licence. [aggregator; official page linked] — [CloudZero](https://www.cloudzero.com/blog/copilot-studio-pricing/); [Microsoft pricing page](https://www.microsoft.com/en-us/microsoft-365-copilot/pricing/copilot-studio); [Microsoft Learn](https://learn.microsoft.com/en-us/microsoft-copilot-studio/billing-licensing)

**Crescendo: outsourcer-style, AI plus humans, per resolution**
- The pricing page lists plans "from **$1.25 per resolution**". G2 lists "**$2.99/per resolution, includes all**". Third parties cite $1.25/resolution plus about $2,900/month for the managed AI-plus-human service. Crescendo sells as a BPO replacement, so the price covers the outcome whether AI or its human staff resolve it. [confirmed pricing page / aggregator] — [Crescendo pricing](https://www.crescendo.ai/pricing); [G2](https://www.g2.com/products/crescendo-ai/pricing); [getmacha](https://www.getmacha.com/blog/crescendo-ai-complete-guide)

**Ada: per conversation (reportedly)**
- Ada reportedly bills by conversation and publishes no per-resolution price. Vendr-type data puts it at about $73.5K/yr. [aggregator] — [myaskai](https://myaskai.com/blog/ada-cx-forethought-comparison-2026); [aitoolsbakery](https://aitoolsbakery.com/blog/ai-customer-support-agents-pricing/)

**Forethought: tiered plans (Basic/Pro/Enterprise), reportedly per agent/month**
- About $59.5K/yr, with no published per-resolution price. [aggregator] — [myaskai](https://myaskai.com/blog/ada-cx-forethought-comparison-2026)

**Parloa (voice)**
- One summary states Parloa bills per successfully resolved conversation rather than per minute, with about $300K+/yr entry pricing and about $50M ARR in late 2025. **Treat as unverified.** No Parloa primary source was retrieved. [aggregator] — [getmacha](https://www.getmacha.com/blog/parloa-complete-guide); [eesel](https://www.eesel.ai/blog/parloa-pricing); [Chatarmin](https://chatarmin.com/en/blog/parloa-pricing)

**Kore.ai**
- No 2026 pricing details were retrieved. It is a Gartner Conversational AI MQ leader (see Q2).

### Inferences
- There is a spectrum from pure outcome pricing to pure consumption:
  - Crescendo, Intercom/Fin, Zendesk, HubSpot and Sierra: per outcome
  - Decagon, Ada and Salesforce's $2 option: per conversation
  - Salesforce Flex Credits, Microsoft credits and ServiceNow assists: per action
  - Salesforce AELA, ServiceNow tiers and Forethought: seats or platform fees
  
  Incumbent platforms (Salesforce, ServiceNow, Microsoft) have avoided true outcome pricing. The outcome-priced vendors are support-native players (Intercom, Zendesk, HubSpot) and AI-native startups.
- Intercom's 2025–26 move from "resolution" to "outcome" (making procedure handoffs billable) quietly widens the billable base. Fin now earns on some conversations that end with a human, which blurs the pure "pay only if resolved" promise.
- Even Sierra, the flagbearer, now hedges publicly ("not dogmatic… consumption-style work where it fits").

### Gaps
- Primary-source Sierra per-outcome prices and contract definitions are not public. The ~$1.50 figure and the "Horizon" July 2026 launch came only from aggregator or competitor pages.
- No primary pricing was found for Decagon, Ada, Forethought, Kore.ai or Parloa. The Parloa per-outcome claim is especially shaky, because most voice-AI vendors (Retell, Vapi, Bland) bill per minute, and this could not be confirmed.
- The exact date Intercom introduced "Procedure handoff" as a billable outcome was not found.
- The Zendesk primary pricing page and the exact 2026 list price were not fetched directly.

## Q2. Who is winning, and is outcome pricing why?

### Takeaway
On revenue, the leaders are Salesforce Agentforce (> $1.2B ARR, but consumption- and seat-priced and bundled), Sierra (~$150M ARR Feb 2026, outcome-priced) and Intercom/Fin (> $100M Fin ARR, outcome-priced, now being bought by Salesforce for $3.6B). Outcome pricing clearly helped Intercom re-accelerate by making the AI attach-rate easy to sell. But Decagon has scaled to a $4.5B valuation with mostly per-conversation contracts, and Gartner's leaders are platform players. So outcome pricing looks like a go-to-market accelerant and a positioning device rather than the decisive cause of winning. Resolution quality (65–76% rates) is what makes the per-outcome economics work.

### Cited Findings
- **Fin (Intercom).**
  - **2026-06-15**: Salesforce signed a definitive agreement to acquire Fin for **~$3.6B**.
  - Fin resolves **76%** of support volume end-to-end and has **30,000+** customers.
  - Closing is expected in Q4 of Salesforce's FY27.
  - The press release does not mention the pricing model. [confirmed] — [Salesforce press release](https://www.salesforce.com/news/press-releases/2026/06/15/salesforce-signs-definitive-agreement-to-acquire-fin/); [CNBC](https://www.cnbc.com/2026/06/15/salesforce-ai-customer-service-fin-acquistion.html); [MarTech](https://martech.org/salesforce-acquires-fin-formerly-known-as-intercom/)
- Intercom renamed itself Fin in May 2026, about a month before the deal. [reported] — [MarTech](https://martech.org/salesforce-acquires-fin-formerly-known-as-intercom/)
- **Fin ARR trajectory:**
  - $1M → $12M in year 1, then approaching ~$100M
  - expected to be ~50% of Intercom's ~$400M total ARR
  - NRR rose from 112% (pre-AI) to 146%
  - resolution rate rose from ~25% at the May 2023 launch to 65–70%
  
  [reported] — [Mostly Metrics](https://www.mostlymetrics.com/p/how-intercom-reaccelerated-growth-with-outcome-based-pricing)
- Aggregators report:
  - Intercom at $400M ARR in Apr 2026, up from $382M at end-2025
  - Fin > $100M ARR, growing ~350%/yr, with ~8,000 businesses
  - a 67% average resolution rate, and 23% at launch
  
  [aggregator] — [GetLatka](https://getlatka.com/companies/intercom-1); [Sacra](https://sacra.com/c/intercom/)
- Eoghan McCabe (X, Jun 2025): Q1 FY26 was "the largest quarter by net new" ARR, and he claimed Intercom was on course to be "the fastest growing large software company". [confirmed, CEO statement; truncated] — [X/@eoghan](https://x.com/eoghan/status/1932879226535096536)
- Deal multiple: $3.6B against ~$400M total Intercom ARR is **~9x ARR**, far below Sierra's reported ~100x run-rate. This is an inference from the figures above. The implication: the outcome-priced incumbent was valued like SaaS, not like an AI-native.
- **Sierra.** $100M ARR in Nov 2025, then **$150M ARR by early Feb 2026**. More than 40% of the Fortune 50 are customers. [reported, TechCrunch 2026-05-04, via sibling notes] — [TechCrunch](https://techcrunch.com/2026/05/04/sierra-raises-950m-as-the-race-to-own-enterprise-ai-gets-serious/). Around $200M ARR in 2026. [aggregator] — [valueaddvc](https://valueaddvc.com/blog/how-does-sierra-ai-make-money-outcome-based-pricing-enterprise-agents-and-the-business-model-breakdown)
- **Salesforce Agentforce:**
  - Q3 FY26: **$540M ARR, +330% YoY**; 18,500 total deals, 9,500 paid; about 8% of 150K+ customers. [reported] — [SaaStr](https://www.saastr.com/salesforce-now-has-3-pricing-models-for-agentforce-and-maybe-right-now-thats-the-way-to-do-it/); [Futurum Q3 FY26](https://futurumgroup.com/insights/salesforce-q3-fy-2026-ai-agents-data-360-lift-bookings-and-fy26-outlook/)
  - **Q1 FY27: $1.2B ARR, +205% YoY**. [confirmed] — [Salesforce PR](https://www.salesforce.com/news/press-releases/2026/06/15/salesforce-signs-definitive-agreement-to-acquire-fin/). MarTech misstates this as "+20%".
  - Q2 FY27: reportedly **> $1.5B ARR, +240%**, with "more than 50% of Agentforce bookings" tied to Flex Credits. [aggregator] — [valueaddvc](https://valueaddvc.com/pulse/salesforce-agentforce-1-5-billion-arr-benioff-2026)
  - These figures cover all Agentforce, not only service.
- **Decagon:** a $4.5B valuation (Jan 2026) and 100+ new enterprise customers in the year, while most customers are on per-conversation pricing. [confirmed Decagon blog / Bloomberg, via sibling notes] — [Decagon blog](https://decagon.ai/blog/pricing-ai-agents)
- **Zendesk** (private; no ARR disclosed): Futurum frames its outcome-pricing push as a strategic bet. There are no revenue results yet. [reported] — [Futurum](https://futurumgroup.com/insights/zendesk-bets-on-autonomous-ai-agents-outcome-pricing-to-upend-service-models/)
- **Analyst rankings.** The 2026 Gartner Magic Quadrant for Conversational AI Platforms (14 vendors) names **Google, Salesforce, SoundHound AI and Kore.ai** as Leaders. Kore.ai is described as the only vendor that is a Leader in both this MQ and the latest Forrester Wave. Sierra and Decagon were not found in coverage of the MQ. [reported] — [CX Today](https://www.cxtoday.com/customer-analytics-intelligence/gartner-magic-quadrant-conversational-ai-2026/); [Salesforce blog](https://www.salesforce.com/blog/salesforce-2026-gartner-magic-quadrant-conversational-ai-platforms/); [Google Cloud blog](https://cloud.google.com/blog/products/ai-machine-learning/google-is-a-leader-in-the-gartner-magic-quadrant-for-conversational-ai)

### Inferences
- Outcome pricing is a **causal accelerant for Intercom**. It turned AI into a low-friction add-on inside an installed base of tens of thousands of customers ("$100/seat for $300 of Fin"). The resolution rate climbing from ~25% to 65–76% is what made per-resolution revenue per customer grow. Outcome pricing only pays off when the resolution rate is high. At 25% it would have starved revenue.
- For Sierra, outcome pricing is part of the enterprise sales narrative (aligned incentives, no seats to cannibalize). But contracts are bespoke and mixed with consumption, so revenue cannot be attributed to the pricing model.
- **Decagon is the counter-example**: it grew fast mostly on per-conversation pricing, arguing that outcome pricing creates disputes. That suggests product and enterprise GTM, not the pricing unit, drive wins.
- **Salesforce's revealed preference**: it runs three non-outcome models, yet paid ~9x ARR for an outcome-priced leader. It is buying an SMB/mid-market self-serve engine and a proven resolution rate, not necessarily the pricing philosophy. How Fin's $0.99 pricing survives inside Agentforce is an open question.
- Analyst rankings (Gartner) still reward platform breadth and don't yet reflect the AI-native outcome-priced vendors.

### Gaps
- No primary Decagon ARR. No Zendesk AI revenue. No ServiceNow CX-agent-specific revenue (Now Assist ACV was not retrieved this run).
- No Forrester Wave (customer-service AI agents) 2026 details were retrieved. It is unclear whether Sierra or Decagon appear in any Gartner or Forrester evaluation.
- Notable customer losses or churn between vendors (e.g., switches from Fin to Decagon) were not found.
- Kyle Poyar's (Growth Unhinged) 2025–26 pricing surveys were not retrieved this run.

## Q3. Disputes and failure modes: resolution-definition fights, gaming, reversion to seats and credits, predictability, margins

### Takeaway
The main friction is the definition of a "resolution." Inactivity-based "assumed resolutions" bill customers when their users simply give up, or when a human quietly fixed the bot's wrong answer. Buyers complain about bill volatility, and Decagon explicitly markets against outcome pricing on these grounds. Salesforce is the clearest case of retreat: from $2/conversation to credits to seats, driven by the buyer's need for predictable budgets.

### Cited Findings
- **Intercom community thread "Fin's flawed assumed resolved & pricing design" (opened 2025-01-22, ~30 replies):**
  - Fin counts an "assumed resolution", and bills $0.99, even when a human agent steps in because Fin's answer was wrong, before the customer presses "Speak to Human". Quote: "stepping in because the AI is wrong, should be penalised."
  - An Intercom support engineer called the concern legitimate ("stepping in to provide the best experience for your customer should be seen as good judgment, not penalized") and passed it to product.
  - Otherwise staff pointed to knowledge-base or content fixes. The billing design remained unchanged. [confirmed, vendor forum] — [Intercom Community](https://community.intercom.com/ask-the-intercom-team-about-fin-54/fin-s-flawed-assumed-resolved-pricing-design-8929)
- A second community thread, "Fin's flawed resolution assumption", raises the same issue. [confirmed, vendor forum; not fetched] — [Intercom Community](https://community.intercom.com/fin-product-feedback-member-group-49/fin-s-flawed-resolution-assumption-10516)
- Reported: Intercom promised a fix at its "Built For You" event but later walked it back. A Reddit user reported a monthly bill jumping from **$4,000 to $9,000**. [aggregator, second-hand; unverified] — [aimdoc](https://aimdoc.ai/blog/intercom-resolution-pricing-explained); [kommunicate](https://www.kommunicate.io/blog/intercom-pricing-breakdown/)
- **Decagon's critique of outcome pricing:**
  - it causes "unpredictable invoices and the constant renegotiations"
  - it cannot tell whether a frustrated user who stops responding was really "resolved"
  - a trivial answer and a complex fix cost the same
  
  [confirmed] — [Decagon blog](https://decagon.ai/blog/pricing-ai-agents)
- **Abandonment problem.** Under 72-hour inactivity rules, customers who abandon look the same as satisfied ones (furniture-retailer example). Salesforce running three concurrent models suggests outcome pricing can't be a "standalone" solution. [reported, 2026-06-09] — [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice)
- **Salesforce reversion:**
  - the $2/conversation model faced backlash as pricey and unpredictable for SMBs and mid-market
  - the move to Flex Credits (May 2025) and then per-user AELA (late 2025) was driven by CFO predictability
  - early on, only 3,000 of 5,000 Agentforce deals were paid
  
  [reported] — [SaaStr](https://www.saastr.com/salesforce-now-has-3-pricing-models-for-agentforce-and-maybe-right-now-thats-the-way-to-do-it/); [aggregator] [getmacha](https://www.getmacha.com/blog/agentforce-pricing-explained)
- **Vendor-side complexity.** Sierra concedes outcome pricing "is more complex… operationally, contractually, accounting-wise". [confirmed] — [Sierra blog](https://sierra.ai/blog/outcomemaxxing). Deloitte issued accounting guidance (2026-06-04) on revenue recognition for outcome-based agentic AI pricing, which signals real ASC 606 complexity (variable consideration). [confirmed title; content not fetched] — [Deloitte DART](https://dart.deloitte.com/USDART/home/publications/deloitte/industry/technology/accounting-outcome-based-pricing-agentic-ai)
- General practitioner guidance on disputes: outcomes that need a human judgment call on every instance "create constant contract disputes". Contracts should define the outcome, attribution rules, billing cadence and a dispute process. Less mature vendors leave definitions vague, "drifting in the vendor's favor". [aggregator/opinion] — [Pickaxe](https://pickaxe.co/post/ai-agent-pricing-models); [institutepm](https://www.institutepm.com/knowledge-hub/outcome-based-pricing-ai-agents)

### Inferences
- **Margin dynamics.** Fixed per-resolution prices mean the vendor bears the LLM inference cost of failed and long conversations, which are not billed at Fin and Zendesk. That is the likely reason Intercom invested in its own post-trained "Apex" model, which the Salesforce PR calls purpose-built for support. Better resolution plus cheaper inference widens margin. This is an inference; no margin data was disclosed.
- The incentive problem runs both ways:
  - Outcome vendors are rewarded for loose "assumed resolution" definitions and for not escalating.
  - Per-conversation vendors are rewarded for volume.
  - Decagon's "we're not incentivized to push partial resolutions" is itself a marketing claim.
- Expanding billable outcomes (Fin's Procedure handoff) is a quiet form of renegotiating the definition in the vendor's favor.

### Gaps
- No public lawsuits or formal billing disputes were found, and no customer audits overturning resolution counts.
- No disclosed gross-margin data for Fin, Sierra or Decagon.
- The Reddit bill-shock claim could not be traced to the original post.

## Q4. How do vendors measure and verify an outcome?

### Takeaway
Verification is overwhelmingly **absence-of-return within a time window**, plus no-escalation, sometimes supplemented by explicit customer confirmation or an LLM judge on the transcript. No vendor found ties billing to CSAT or to an independent audit by default. Enterprise contracts (Sierra; Zendesk's 2026 custom deals) negotiate bespoke, "auditable" metrics.

### Cited Findings
- **Fin:**
  - confirmed resolution = the customer affirms
  - assumed resolution = 24h of disengagement after Fin's last *answer* (not a question)
  - the charge is reversed if the customer returns to the same conversation
  - no charge if a human is requested or on rule-based escalation
  - at most one charge per conversation
  - Procedure handoffs are billable if the procedure executes successfully
  
  [confirmed] — [Fin help](https://fin.ai/help/en/articles/13975800-fin-pricing-outcomes)
- Intercom reportedly doesn't bill an assumed resolution if the same customer reaches a human within 24h. [aggregator] — [aimdoc](https://aimdoc.ai/blog/intercom-resolution-pricing-explained)
- **Zendesk:**
  - inactivity window of up to 72h (shorter on messaging)
  - or explicit confirmation, or acceptance of a help-center article
  - plus "rule-based checks and AI assessment" of the transcript
  
  [reported/aggregator] — [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice); [eesel](https://www.eesel.ai/blog/zendesk-ai-dynamic-pricing-resolution)
  
  The 2026 custom contracts add "clear, auditable success metrics and pricing guardrails" and continuous QA via Quality Score. [reported] — [Futurum](https://futurumgroup.com/insights/zendesk-bets-on-autonomous-ai-agents-outcome-pricing-to-upend-service-models/)
- **HubSpot:** a resolved conversation with no human handoff, and up to 72h of inactivity. [reported] — [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice)
- **Salesforce:** no outcome verification. The billing unit is a 24-hour conversation session or a counted action. [reported] — [The Pricing Conundrum](https://thepricingconundrum.substack.com/p/outcome-based-pricing-in-practice)
- **Sierra:** definitions are negotiated per contract and could include business outcomes (a saved cancellation, a completed transaction). No verification method is published. [confirmed absence] — [Sierra blog](https://sierra.ai/blog/outcomemaxxing); [aggregator] [fin.ai/learn](https://fin.ai/learn/sierra-ai-pricing)

### Inferences
- Time windows range from 24h (Fin) to 72h (Zendesk, HubSpot). The shorter window favors the vendor, since it gives customers less time to come back and have the charge reversed.
- Business-outcome contracts (Sierra saves, conversions) presumably rely on transaction or CRM system events for verification. This is inferred and not documented publicly.

### Gaps
- No vendor was found that ties billable outcomes to post-conversation CSAT or offers third-party audit rights as standard.
- Decagon's per-resolution verification rules and Crescendo's resolution definition were not found in primary sources.

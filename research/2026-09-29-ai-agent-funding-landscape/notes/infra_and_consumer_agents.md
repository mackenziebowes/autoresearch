# AI Agent Infrastructure (Picks-and-Shovels) and General/Consumer Agents: Funding and Traction, Oct 2025 – Sept 2026

Research date: 2026-09-29. Source-quality note: many 2026 search hits came from SEO aggregators (aifunding.me, agentmarketcap.ai, presenc.ai, valueaddvc.com, getlatka, etc.). Where a primary source (company blog, press release, TechCrunch, CNBC, SiliconANGLE, Fortune, VentureBeat) was available it is preferred; aggregator-only figures are marked **[aggregator, unverified]**. Several aggregator figures conflicted with primary sources (flagged inline).

## (A) Who funded agent infrastructure, how much, at what valuation, by whom, and with what traction?

### Takeaway
From Oct 2025 to Sept 2026 the infra layers with the biggest checks were (1) compute/sandboxes (Modal at $4.65B), (2) web data/search for agents (Exa at $2.2B, Parallel at $2B, Tavily bought by Nebius), (3) identity/auth (WorkOS at $2B, Stytch bought by Twilio, Keycard $38M), and (4) evals/observability (Braintrust at a reported $800M, with consolidation: ClickHouse–Langfuse, Cisco–Galileo). Frameworks (LangChain at $1.25B) and voice infra (LiveKit at $1B) crossed unicorn status. Tool/MCP gateways and memory raised smaller rounds and are consolidating (Arcade bought Smithery).

### Cited Findings

**Agent frameworks and orchestration**
- LangChain: $125M Series B at a $1.25B valuation (announced Oct 21, 2025). IVP led; Sequoia, Benchmark and Amplify followed on; CapitalG and Sapphire were new investors. — [TechCrunch](https://techcrunch.com/2025/10/21/open-source-agentic-startup-langchain-hits-1-25b-valuation); [LangChain blog](https://www.langchain.com/blog/series-b)
- LangChain describes itself as the #1 downloaded agent framework with 100M+ monthly downloads. Products are LangGraph (orchestration of long-running agents with durable execution and human-in-the-loop) and LangSmith (observability/evals). — [LangChain blog](https://www.langchain.com/blog/series-b); [AI Agent News](https://ai-agent-news.com/posts/langchain-125m-agent-infrastructure-unicorn/)
- Background: LangChain raised a $10M seed (Benchmark) in Apr 2023 and a $25M Series A (Sequoia) at about $200M a week later. — [WebProNews](https://www.webpronews.com/langchain-raises-125m-in-series-b-hits-1-25b-unicorn-valuation/)
- Mastra (TypeScript agent framework, YC): $13M seed (background, 2025; per Mastra the "largest post-YC cap table" in years, 120+ investors including pg, Gradient, Amjad Masad, Guillermo Rauch, Balaji), then a $22M Series A led by Spark Capital, for $35M total. The Series A date was not confirmed in the snippet. — [Mastra blog, Series A](https://mastra.ai/blog/series-a); [Mastra blog, seed](https://mastra.ai/blog/seed-round)
- CrewAI: $18M total (Boldstart seed; Insight Partners Series A). Background, Oct 2024. No 2026 round found. — [SiliconANGLE](https://siliconangle.com/2024/10/22/agentic-ai-startup-crewai-closes-18m-funding-round/)
- Temporal (durable execution used under many agent stacks): $550M Series E at a reported $12.55B valuation in 2026. **[aggregator, unverified]** — [AI Funding Tracker](https://aifunding.me/ai-agent-funding)
- LlamaIndex: no 2025–26 round found in this pass (gap).

**Browser and computer-use infrastructure**
- Browserbase: $40M Series B (June 2025, background) led by Notable Capital with CRV and Kleiner Perkins, at a $300M valuation; $67.5M total raised. No Series C was found through Sept 2026. — [Browserbase blog](https://www.browserbase.com/blog/series-b-and-beyond); [Contrary Research](https://research.contrary.com/company/browserbase)
- Browser Use: $17M seed (Mar 2025, background) led by Felicis, with A Capital, Nexus, YC and Paul Graham. Browser Use says it is the biggest open-source web-agent project with 110k+ GitHub stars (star count from a 2026 aggregator). — [SiliconANGLE](https://siliconangle.com/2025/03/23/browser-use-raises-17m-help-steer-ai-agents-internet/); [Browser Use blog](https://browser-use.com/posts/seed-round)
- Anchor, Steel, Hyperbrowser: no 2025–26 funding data found in this pass (gap).

**Sandboxes and runtimes**
- Modal: $355M Series C (May 2026) at a $4.65B post-money valuation, led by General Catalyst with Redpoint, Accel and Menlo. This follows an $80M Series B at $1.1B about 8 months earlier. Named customers include Runway, Suno, Lovable and Quora. — [bex.co roundup (secondary)](https://bex.co/blog/2026/09/11/ai-sandbox-funding-modal-daytona-e2b)
- Daytona: $24M Series A (Feb 2026) led by FirstMark, with Pace, Upfront, E2VC, Darkmode and, per bex.co, Datadog and Figma Ventures. Daytona says it reached a $1M forward revenue run rate in under 3 months and doubled that 6 weeks later. Sandbox creation takes under 90 ms. In June 2026 Daytona moved its production code to closed source. — [PR Newswire](https://www.prnewswire.com/news-releases/daytona-raises-24m-series-a-to-give-every-agent-a-computer-302680740.html); [Northflank](https://northflank.com/blog/daytona-vs-e2b-ai-code-execution-sandboxes)
- E2B: last priced round was a $21M Series A (July 2025, background) led by Insight Partners with Decibel, Sunflower and Kaya, for about $32M total. E2B reports 88% of the Fortune 100 signed up (later marketing says 94%) and 1B+ sandboxes started. Named users include Perplexity. — [E2B blog](https://e2b.dev/blog/series-a); [bex.co](https://bex.co/blog/2026/09/11/ai-sandbox-funding-modal-daytona-e2b)
- Hyperscaler competition: AWS AgentCore, Vercel Sandbox (GA Jan 2026), Cloudflare and Fly.io Sprites. — [bex.co](https://bex.co/blog/2026/09/11/ai-sandbox-funding-modal-daytona-e2b)

**Memory**
- Mem0: $24M Series A (Oct 2025), about $24.5M total. About 55k GitHub stars as of May 2026. — [Medium comparison (secondary)](https://medium.com/@wasowski.jarek/i-compared-5-ai-agent-memory-systems-across-6-dimensions-none-wins-6a658335ed0a); [datapace](https://datapace.ai/blog/ai-agent-memory-tools-2026)
- Zep/Graphiti scores 71.2% on LongMemEval versus 49% for Mem0 (vendor-reported benchmark). Zep's Flex plan costs $125/month; Letta Pro costs $20/month. — [datapace](https://datapace.ai/blog/ai-agent-memory-tools-2026)
- Letta and Zep: no new 2025–26 priced rounds found (gap). Background: Letta had a 2024 seed.

**Evals and observability**
- Braintrust: $80M Series B (Feb 17, 2026) led by ICONIQ, with a16z, Greylock, Elad Gil and Basecase. The valuation was reported as about $800M in LinkedIn and trade coverage but was not confirmed in primary text. — [SiliconANGLE](https://siliconangle.com/2026/02/17/braintrust-lands-80m-series-b-funding-round-become-observability-layer-ai/); [LinkedIn post](https://www.linkedin.com/posts/devcuration_aiinfrastructure-mlops-aiagents-activity-7430651955531698176-YYjc)
- Consolidation, as reported: ClickHouse acquired Langfuse (Jan 2026), Cisco is acquiring Galileo, Mintlify bought Helicone, and OpenAI acquired Promptfoo. These come from vendor and aggregator blogs; confirm with primary sources. — [Latitude blog](https://latitude.so/blog/best-llm-observability-tools-agents-latitude-vs-langfuse-langsmith); [AgentMarketCap](https://agentmarketcap.ai/blog/2026/04/11/ai-agent-evaluation-startup-market-map-2026)
- Galileo: $68.1M raised across 3 rounds (Series B from Scale Venture Partners and Databricks Ventures). — [Latitude](https://latitude.so/blog/best-llm-observability-tools-agents-latitude-vs-langfuse-langsmith)
- Arize: no 2026 round found (gap). Background: $70M Series C in Feb 2025.

**Agent identity and auth**
- WorkOS: $100M Series C at a $2B valuation (about Mar 3–4, 2026), led by Meritech and Sapphire with Audacious, Craft, Abstract and Greenoaks. ARR is about $30M. **[ARR aggregator, unverified]** — [WorkOS blog](https://workos.com/blog/series-c); [ARR Club](https://www.arr.club/workos)
- Stytch: acquired by Twilio (closed Nov 14, 2025) for $104.1M in cash ($58.5M net of cash acquired), per Twilio's FY2025 10-K. Twilio positions Stytch as "an identity platform for AI agents." — [Twilio blog](https://www.twilio.com/en-us/blog/company/news/twilio-to-acquire-stytch); [Twilio 10-K](https://www.sec.gov/Archives/edgar/data/1447669/000144766926000021/twlo-20251231.htm)
- Keycard: emerged from stealth Oct 22, 2025 with $38M: an $8M seed co-led by a16z and Boldstart, plus a $30M Series A led by Acrew. It acquired Runebook (MCP connectivity) in Nov 2025. Keycard says it has the first production implementation of OAuth 2.1 Client ID Metadata Documents in MCP. — [Help Net Security](https://www.helpnetsecurity.com/2025/10/22/keycard-ai-agents-identity-access-platform/); [SiliconANGLE](https://siliconangle.com/2025/11/20/ai-agent-identity-startup-keycard-acquires-runebook-empower-mcp-connectivity/)
- Descope: no 2025–26 round found in this pass (gap).

**MCP tooling and gateways**
- Arcade.dev: $60M Series A (June 15, 2026) led by SYN Ventures, with strategic investment from Morgan Stanley and Wipro; $72M total including a $12M 2025 seed. — [BusinessWire](https://www.businesswire.com/news/home/20260615229631/en/Arcade-Raises-$60M-to-Become-the-Secure-Action-Layer-Behind-Every-Production-AI-Agent)
- Arcade acquired Smithery, the public MCP server registry (about Aug 5–10, 2026). Smithery had raised only about $400K. — [Forbes](https://www.forbes.com/sites/janakirammsv/2026/08/10/arcade-acquires-smithery-to-own-the-agent-tool-supply-chain/); [Tracxn](https://tracxn.com/d/companies/smithery/__0V9FsUTLIizQPCeUJpRQnoi_JjyhL9uoi0Bx6fwSF0k)
- Composio: $25M Series A led by Lightspeed (July 2025, background), about $29M total. No 2026 round found. — [Tracxn](https://tracxn.com/d/companies/composio/__S4CqdyIkWZd1BSTOwnjS82Hz0ppMkmDoAP_j4_oMBfk/funding-and-investors)

**Voice-agent infrastructure**
- LiveKit: $100M Series C at a $1B valuation (Jan 2026) led by Index Ventures, with Salesforce Ventures, Altimeter, Redpoint and Hanabi. Prior rounds: $45M Series B at $345M (Apr 2025) and about $110M valuation (Jun 2024). LiveKit sells voice tooling to OpenAI. — [LiveKit blog](https://livekit.com/blog/livekit-series-c); [FinSMEs](https://www.finsmes.com/2026/01/livekit-raises-100m-in-series-c-funding-at-a-1-billion-valuation.html); [Dina Bass/Bloomberg LinkedIn](https://www.linkedin.com/posts/dina-bass_livekit-seller-of-voice-tools-to-openai-activity-7420155871936081920-invK)
- Pipecat/Daily: no 2025–26 funding data found (gap).

**Web data for agents**
- Exa Labs: $250M Series C at a $2.2B post-money valuation (May 20, 2026), led by a16z. This more than tripled the $700M valuation of its $85M Series B (Sept 2025, with Nvidia and YC). Exa reports 400k+ developers; customers include Cursor, Cognition, HubSpot, OpenRouter and Monday.com. — [Exa blog](https://exa.ai/blog/announcing-series-c); [SiliconANGLE](https://siliconangle.com/2026/05/20/exa-labs-raises-250m-2-2b-valuation-ai-search-tools/). CONFLICT: aggregator aifunding.me says "$400M at $2.4B, Nvidia-backed". The primary source says $250M at $2.2B. [aifunding.me](https://aifunding.me/ai-agent-funding)
- Parallel Web Systems (Parag Agrawal): $100M Series B at about $2B (reported late Apr 2026), led by Sequoia. This follows a $100M Series A in 2025. Parallel says it supports 100k+ developers. — [The Source Code](https://www.the-sourcecode.com/startups/parag-agrawal-parallel-web-systems-100m-series-b-sequoia-2bn-valuation); [American Bazaar](https://americanbazaaronline.com/2026/04/30/parag-agrawals-parallel-web-systems-hits-2-billion-in-valuation-479920/)
- Tavily: acquired by Nebius (Feb 2026) for a reported $275M, rising to as much as $400M on milestones, after raising only about $25M. **[aggregator, verify]** — [AI Funding Tracker](https://aifunding.me/ai-agent-funding)
- Firecrawl: Tracxn lists about $16.2M from Nexus. A 2025 Series A may be missing from this figure (gap). — [Tracxn](https://tracxn.com/d/companies/firecrawl/__AUIyUTgBhP2fuJfSidYDmsLSkPNv7XqK1ZuBMuVi3yw)

**Agent payments and commerce**
- OpenAI and Stripe's Agentic Commerce Protocol (ACP) uses a Shared Payment Token bound to one merchant and amount, time-limited and single-use. Stripe shipped an Agentic Commerce Suite plus a Machine Payments Protocol. Onboarding brands include URBN, Etsy, Ashley Furniture, Coach, Kate Spade and Revolve. — [RisingWave](https://risingwave.com/blog/mastercard-agent-pay-vs-visa-vs-stripe-agentic-commerce/); [DeepLumen](https://www.deeplumen.com/blog/agentic-payment-infrastructure/)
- Google's AP2 (background, 2025) had 60+ partners, including Mastercard, Visa, Stripe, Adyen, PayPal and Coinbase. Google donated AP2 to the FIDO Alliance in Apr 2026. Visa has Intelligent Commerce and the Trusted Agent Protocol; Mastercard has Agent Pay with Agentic Tokens. In Apr 2026 Mastercard said its network accepts AP2 Mandates as Verifiable Intent artifacts. **[secondary blogs; verify with FIDO and Mastercard releases]** — [RisingWave](https://risingwave.com/blog/mastercard-agent-pay-vs-visa-vs-stripe-agentic-commerce/)
- Skyfire: $8.5M seed plus an a16z CSX grant, about $9.5M total (background, 2024). No 2026 round found. Crossmint supports stablecoin payments (x402/USDC) and Visa/Mastercard rails. Payman handles agent-to-human payouts. — [Finextra](https://www.finextra.com/newsarticle/44621/skyfire-raises-85m-to-bring-autonomous-payments-to-ai-agents); [FluxA](https://fluxapay.xyz/learning/6-crossmint-alternatives-for-ai-agent-payments-2026)

**Aggregate market signal**
- One aggregator claims more than $6B went into "AI agent infrastructure" in Q3 2026, including Lyzr's $100M Series B. Its category definition is unclear. **[aggregator, unverified]** — [The Agent Report](https://the-agent-report.com/2026/07/ai-agent-funding-q3-2026-runta-lyzr-gradium/)

### Inferences
- Valuations are increasing fastest where the infra layer is metered usage tied to agent volume: compute seconds (Modal went from $1.1B to $4.65B in about 8 months) and search calls (Exa went from $700M to $2.2B in about 8 months). Investors appear to be betting that agent traffic grows faster than per-seat software.
- Thin, easily copied layers are being absorbed by bigger platforms rather than funded as standalones: observability (Langfuse, Galileo, Helicone, Promptfoo), identity (Stytch), MCP registries (Smithery) and search APIs (Tavily). Acquirers are data infra (ClickHouse), networking/security (Cisco), comms (Twilio) and neoclouds (Nebius).
- Browser infra (Browserbase, Browser Use) had no disclosed up-round in the window despite heavy developer adoption. This may reflect the model labs building computer-use into their own products, and Atlas being folded back into ChatGPT (see B).

### Gaps
- No 2025–26 round data found for LlamaIndex, Anchor, Steel, Hyperbrowser, Letta, Zep, Arize, Descope, Pipecat/Daily, Payman or Crossmint. These may have raised; they were not searched individually because of the tool-call budget.
- Braintrust's $800M and Temporal's $12.55B valuations, the Nebius–Tavily price, the Cisco–Galileo deal and ClickHouse–Langfuse were not confirmed against primary releases.
- No investor memos (a16z, Menlo, Sequoia) were fetched, so theses are inferred from deal patterns rather than quoted.
- GitHub star counts for LangChain, CrewAI and Mastra were not independently checked.

## (B) General-purpose and consumer agents: funding, usage and retention versus hype

### Takeaway
2026 was a year of consolidation and retreat for standalone consumer agents. Meta's roughly $2B Manus deal was unwound by China. OpenAI shut down the standalone Atlas browser (August 9, 2026) and folded it into ChatGPT, and Google discontinued Project Mariner (May 2026). Anthropic folded Cowork into Claude chat (Sept 2026), and Cognition bought Poke. The clearest monetization winners were Genspark (about $250M ARR, $2.6B valuation) and Perplexity ($750M annualized revenue claimed, about $20–23B valuation). The biggest grassroots phenomenon was open-source OpenClaw, with 300k+ GitHub stars.

### Cited Findings

**Manus (Meta deal and unwind)**
- Meta agreed to acquire Manus for about $2B (announced late Dec 2025). Manus was founded in China in 2022 and later moved to Singapore. — [TechCrunch, Jan 6 2026](https://techcrunch.com/2026/01/06/metas-manus-news-is-getting-different-receptions-in-washington-and-beijing/)
- In Jan 2026, China's Ministry of Commerce opened a review covering export controls, technology import/export and outbound investment. — [CNBC, Jan 8 2026](https://www.cnbc.com/2026/01/08/china-investigate-meta-acquisition-manus-export.html); [SCMP](https://www.scmp.com/tech/big-tech/article/3339335/review-meta-manus-deal-underlines-chinas-tightening-grip-ai-exports)
- On Apr 27, 2026, China's NDRC ordered the deal unwound on national-security grounds. Meta cut ties on June 15, 2026. On Aug 11, 2026, Manus said it would operate as an independent company. — [O'Melveny](https://www.omm.com/insights/alerts-publications/china-unwinds-meta-s-acquisition-of-manus-implications-for-cross-border-ai-transactions/); [CNBC, Aug 11 2026](https://www.cnbc.com/2026/08/11/manus-china-meta-acquisition.html)
- Background: Benchmark's 2025 investment in Manus drew US scrutiny. US regulators later appeared comfortable with the Meta deal. — [TechCrunch](https://techcrunch.com/2026/01/06/metas-manus-news-is-getting-different-receptions-in-washington-and-beijing/)

**Genspark**
- Series B extended to $485M total at a $2.6B post-money valuation (June 11–17, 2026; $100M extension). This is up from $1.6B in March 2026. Investors include Sozo Ventures, Korea Mirae Asset and UpHonest. Genspark also appointed a CRO. — [BusinessWire](https://www.businesswire.com/news/home/20260617758937/en/Genspark.ai-Extends-Series-B-to-$485M-at-$2.6B-Post-Money-Valuation-Appoints-Jamison-Powell-as-Chief-Revenue-Officer); [Axios](https://www.axios.com/pro/enterprise-software-deals/2026/06/11/genspark-extension-agentic-workplace)
- Revenue path: about $100M ARR (Jan 2026, Axios), then a $200M run rate (Dealroom), then about $250M ARR by June 2026 (Latka, aggregator). — [Axios Jan 2026](https://www.axios.com/pro/enterprise-software-deals/2026/01/21/genspark-funding-100-million-arr-genai); [Dealroom](https://app.dealroom.co/news/note/genspark-hits-200m-run-rate-extends-series-b-to-385m-with-new-ai-employee); [Latka](https://getlatka.com/companies/genspark.ai)

**Perplexity / Comet**
- Raised about $200M at about a $20B valuation (June 2026). Latest valuation is about $23B (Sept 4, 2026), with about $1.72B raised in total. **[aggregators; verify]** — [TechTimes](https://www.techtimes.com/articles/318028/20260608/perplexity-raises-200-million-comet-ai-browser-agent-economy-front-door.htm); [Sacra](https://sacra.com/c/perplexity/)
- Annualized revenue was $750M in Aug 2026, up from $232M at end of 2025 **[Sacra/aggregator]**. Comet has about 3M MAU (Android launched Nov 2025). Comet Plus pays publishers an 80/20 revenue share. — [Sacra](https://sacra.com/c/perplexity/); [leadgen-economy](https://www.leadgen-economy.com/blog/perplexity-comet-200m-comet-plus-publisher-revenue-share/)

**OpenAI Operator / ChatGPT Agent / Atlas**
- OpenAI deprecated the standalone ChatGPT Atlas browser, which was scheduled to stop working Aug 9, 2026. Its browser-agent capabilities moved into ChatGPT and Codex. The help-center page returned 403 to direct fetch; the finding comes from search snippets plus secondary coverage. — [OpenAI Help Center](https://help.openai.com/en/articles/20001371-evolving-atlas-into-chatgpt-for-browser-based-agentic-work); [tobira.ai](https://blog.tobira.ai/agentic-browser-consolidation-atlas-chatgpt/)
- UNRELIABLE: presenc.ai claims Atlas had about 11M MAU in Q1 2026, "up from 1.6M in Q1 2025," plus 9M agent tasks/month at a 76% completion rate. Atlas launched Oct 2025, so a Q1 2025 figure is impossible. Treat all of these numbers as suspect. — [presenc.ai](https://presenc.ai/research/atlas-usage-statistics-2026)

**Anthropic Claude Cowork**
- Cowork launched Jan 12, 2026 (Max plan, macOS), expanded to Pro in Jan 2026, and added an enterprise release with connectors (Drive, Gmail, Docusign, FactSet) and plugins in Feb 2026. It went GA on Apr 9, 2026 alongside Managed Agents. — [Vellum](https://www.vellum.ai/blog/official-claude-cowork-breakdown); [pasqualepillitteri.it](https://pasqualepillitteri.it/en/news/755/anthropic-managed-agents-cowork-ga-april-9-2026)
- On Sept 16, 2026, Anthropic merged Cowork into ordinary Claude chat as part of a "superapp" push and launched Claude Docs and Slides. The Cowork brand will eventually disappear. — [Fortune](https://fortune.com/2026/09/16/anthropic-merges-its-claude-chat-and-agentic-cowork-products-into-a-single-ai-assistant-as-part-of-a-push-to-build-an-ai-superapp/); [VentureBeat](https://venturebeat.com/technology/anthropic-is-killing-off-cowork-and-folding-it-into-claude-launching-claude-docs-and-claude-slides)
- Cowork is being made available via the cloud. — [NBC News](https://www.nbcnews.com/tech/tech-news/anthropic-will-make-claude-cowork-available-users-cloud-rcna353218)

**Google Project Mariner / Opera Neon / Dia**
- Google discontinued Project Mariner (a browser-automation research prototype) on May 4, 2026. Its capabilities appear to live on as Gemini "Agent Mode." — [Wikipedia](https://en.wikipedia.org/wiki/Project_Mariner)
- Opera Neon dropped its waitlist and became generally available on Dec 11, 2025 at $19.90/month, with Chat/Do/Make agents and the ODRA deep-research agent. It became free to use "as an agent target" on Aug 14, 2026 (aggregator phrasing). — [MacRumors](https://www.macrumors.com/2025/12/11/opera-neon-ai-browser-ends-waitlist/); [Opera blog](https://blogs.opera.com/news/2025/12/opera-neon-becomes-available-in-public-early-access/)
- Atlassian agreed on Sept 4, 2025 (background) to buy The Browser Company (Arc, Dia) for $610M in cash, expected to close by Dec 2025. The Browser Company had previously raised at a $550M valuation. — [CNBC](https://www.cnbc.com/2025/09/04/atlassian-the-browser-company-deal.html); [BusinessWire](https://www.businesswire.com/news/home/20250904645125/en/Atlassian-Enters-Into-Definitive-Agreement-to-Acquire-The-Browser-Company-of-New-York)

**Personal-assistant agents**
- Poke (The Interaction Company of California) launched publicly in Mar 2026 as an agent used entirely over text messages. It was backed by Spark Capital and General Catalyst at a reported $300M valuation. Cognition acquired it (announced July 23, 2026). — [TechCrunch, Apr 8 2026](https://techcrunch.com/2026/04/08/poke-makes-ai-agents-as-easy-as-sending-a-text/); [usecarly (secondary)](https://www.usecarly.com/blog/cognition-acquires-poke/)
- Cognition itself raised $2B+ in a Series E at a reported $48B valuation. **[aggregator, unverified]** — [AI Funding Tracker](https://aifunding.me/ai-agent-funding)
- Lindy: about $50–53.6M total, with the last known round a 2023 Series A. No 2026 round found. — [Tracxn](https://tracxn.com/d/companies/lindy/__FJe0QVe6UcRHtdiPJpmyRG3livSd4eIGsIxMxz-kNPI)
- OpenClaw (open-source local personal agent by PSPDFKit founder Peter Steinberger, connected through chat apps):
  - Stars: 60k+ in 72 hours; about 216k by mid-Feb 2026; 250,829 on Mar 3, 2026, passing React; 310k+ stars and 58k forks by Apr 2026. These are secondary sources; check against the GitHub API.
  - Steinberger joined OpenAI in Feb 2026, and Sam Altman said OpenClaw would stay open source and independently governed.
  - Security coverage called it "2026's first major security crisis."
  — [Wikipedia](https://en.wikipedia.org/wiki/OpenClaw); [Medium](https://medium.com/@aftab001x/openclaw-just-beat-reacts-10-year-github-record-in-60-days-now-nobody-knows-what-to-do-with-it-937b8f370507); [Hive Security](https://hivesecurity.gitlab.io/blog/openclaw-ai-agent-security-crisis-2026/)
- Downstream "Claw" effect: Lyzr reportedly used its "SivaClaw" agent to run its own $100M Series B fundraise. **[aggregator]** — [The Agent Report](https://the-agent-report.com/2026/07/ai-agent-funding-q3-2026-runta-lyzr-gradium/)

### Inferences
- Real traction versus hype:
  - Revenue-proven: Genspark (revenue more than doubled in about 5 months) and Perplexity. Both sell subscriptions to prosumers and workplaces rather than pure consumer "do my errands" agents.
  - Weaker standalone traction: agentic browsers. Comet has only about 3M MAU against Perplexity's 100M+ claimed. Atlas and Mariner were shut down, and Dia was absorbed into Atlassian. The browser-agent use case is being pulled back into the main chat apps.
  - Incumbent consolidation: Anthropic (Cowork into Claude), OpenAI (Atlas into ChatGPT) and Google (Mariner into Gemini) are all merging their agent surfaces into one assistant. That leaves little independent room for general consumer agents; acqui-hires such as Poke to Cognition result.
- Manus is now a precedent for cross-border risk. Chinese-origin agent startups face exit constraints from Beijing even after moving to Singapore. This likely lowers the exit value of that cohort for US acquirers.
- OpenClaw shows demand for local, model-agnostic personal agents, but the company value flowed to OpenAI through the hire rather than to a funded startup.

### Gaps
- No verified retention data (D30 or cohort curves) was found for any consumer agent. App-store ranking coverage was not retrieved.
- Manus ARR and user figures for 2026 were not retrieved (CNBC returned 403). Manus reportedly passed about $100M ARR in late 2025, but this was not verified in this pass.
- Martin (personal assistant) was not researched.
- ChatGPT Agent usage numbers from OpenAI primary sources were not found. The Dia user count after the acquisition is unknown.

## Protocol and standards adoption as a signal

### Takeaway
MCP is the de facto agent-to-tool standard, now under neutral Linux Foundation governance. A2A is its agent-to-agent counterpart. Agent payments are converging on AP2 (governed by FIDO), with ACP (OpenAI/Stripe) as the main alternative.

### Cited Findings
- The Linux Foundation formed the Agentic AI Foundation (AAIF) in Dec 2025, anchored by contributions of MCP (Anthropic), goose (Block) and AGENTS.md (OpenAI). — [Linux Foundation](https://www.linuxfoundation.org/press/linux-foundation-announces-the-formation-of-the-agentic-ai-foundation); [MCP blog, Dec 9 2025](https://blog.modelcontextprotocol.io/posts/2025-12-09-mcp-joins-agentic-ai-foundation/)
- MCP has 110M+ monthly SDK downloads (Apr 2026), 10,000+ published servers, and 170+ AAIF members (Apr 2026). **[secondary]** — [IntuitionLabs](https://intuitionlabs.ai/articles/agentic-ai-foundation-open-standards); [dev.to](https://dev.to/pockit_tools/mcp-vs-a2a-the-complete-guide-to-ai-agent-protocols-in-2026-30li)
- A2A reached v1.0 and reportedly joined AAIF in Aug 2026. CAUTION: A2A was separately donated to the Linux Foundation in mid-2025, so the "joined AAIF Aug 2026" detail and the list of six co-founders from secondary sources need verification. — [dev.to](https://dev.to/pockit_tools/mcp-vs-a2a-the-complete-guide-to-ai-agent-protocols-in-2026-30li); [Wikipedia A2A](https://en.wikipedia.org/wiki/Agent2Agent)
- Payments: ACP (OpenAI/Stripe), AP2 (Google, moved to FIDO in Apr 2026, 60+ partners), Visa TAP and Mastercard Agent Pay, with x402 stablecoin payments via Crossmint and Coinbase. — [RisingWave](https://risingwave.com/blog/mastercard-agent-pay-vs-visa-vs-stripe-agentic-commerce/); [FluxA](https://fluxapay.xyz/learning/6-crossmint-alternatives-for-ai-agent-payments-2026)

### Inferences
- MCP's neutral governance helps explain why MCP-native infra won funding (Arcade $60M, Keycard's MCP OAuth work) while standalone registries were absorbed (Smithery).
- Card networks endorsing AP2 mandates reduces the room for independent agent-payment startups (Skyfire, Payman) on card rails. Their opening is stablecoin micropayments (x402).

### Gaps
- No primary transaction-volume data exists for agentic commerce (ACP, AP2 or Agent Pay). The Visa and Mastercard press releases were not fetched.

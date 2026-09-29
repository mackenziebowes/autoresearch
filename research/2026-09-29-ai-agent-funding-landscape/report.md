# Coding agents prove demand while VCs bet wider

As of late September 2026, **coding agents are the only category where venture money and proven user demand clearly agree**. They lead every revenue leaderboard, every token-usage ranking and every enterprise "scaling" survey, and they also hold the largest valuations and the largest exit. Outside coding, VCs are funding three things faster than users have proven them: **outcome-priced vertical agents** (customer support, legal, healthcare, finance), **metered agent infrastructure** (sandboxes, search, identity), and the **"services-as-software"** thesis. Investors pay 50–100x run-rate for these companies. Meanwhile, only about a quarter of enterprises say they are scaling any agent, and about a quarter have scaled deployments back on cost. The opposite gap shows up in consumer and general-purpose agents. Grassroots pull is real there (OpenClaw's 300k+ GitHub stars, a sudden token spike for the open-source Hermes Agent), but VCs say they avoid the category and the labs capture most of the value. The biggest structural risk in the most-funded category is that **the labs' own agents now out-earn the independents**: Claude Code passed a $2.5B run-rate by February 2026, when Cursor was at about $2B.

| Agent category | VC signal (rounds, valuations, theses) | User-pull signal (usage, revenue, retention) | Verdict |
|---|---|---|---|
| Professional coding agents (IDE, CLI, autonomous) | Largest valuations: Cursor $60B exit, Cognition $26B confirmed | Strongest: $1B–$4B ARR leaders, over 50% of OpenRouter tokens, lab products at $2.5B+ | **Aligned**, but the labs take a growing share |
| Vibe-coding app builders (non-developers) | Fastest step-ups: Lovable $13.3B, Replit $9B, Emergent $1.5B | High ARR, but 2025 traffic fell 27–64% from peak, "really high" churn, no disclosed retention | **Money ahead of retention evidence** |
| Customer support / CX and voice | About $2B+ in four rounds; Sierra above $15B at roughly 100x ARR | Real revenue: Sierra $150M ARR, Intercom Fin near $100M on per-resolution pricing | Pull is real; **valuations run far ahead of it** |
| Legal, healthcare, finance, GTM verticals | $1B–$12B valuations, many 3x step-ups within 6–9 months | $100M–$200M ARR at the leaders; broad enterprise scaling still only about 23% | Pull is real at the top; **breadth unproven** |
| Agent infrastructure | Metered layers re-rated 3–4x; thin layers acquired | Heavy developer adoption; MCP server count inflated about 3.2x | **Money follows metered usage, not adoption** |
| Standalone consumer / general agents | VCs say they avoid it; Manus deal unwound; Atlas, Mariner shut | Grassroots spikes (OpenClaw, Hermes); weak standalone retention | **Pull without investable companies** |

The funding figures below come from company announcements and tier-1 press unless marked otherwise. Several widely repeated numbers come only from small trackers and aggregators; the report flags each one, and the last analytical section explains how the five main data conflicts were handled.

## Record totals hide a narrow, top-heavy agent market

AI-wide venture numbers are enormous but say little about agents specifically. Crunchbase counts a record **$510B of global venture funding in H1 2026**, with more than 70% going to AI in Q2. But **OpenAI and Anthropic alone raised $217B, 43% of the half** ([Crunchbase News](https://news.crunchbase.com/venture/global-startup-exits-ipo-ma-soar-ai-q2-h1-2026/)). CB Insights finds 89% of Q2 AI dollars went into 142 mega-rounds of $100M or more ([CB Insights Q2'26](https://www.cbinsights.com/research/report/ai-trends-q2-2026/)), and Crunchbase notes deal counts "did not meaningfully grow" ([Crunchbase Q1](https://news.crunchbase.com/venture/record-breaking-funding-ai-global-q1-2026/)). The headline growth is money per deal, not more companies getting funded.

Agent-only totals are poorly measured, and the published trackers disagree by roughly 10x. The best anchor is PitchBook: **$24.2B across 1,311 agentic-AI deals in 2025**, concentrated in cybersecurity, developer tooling and enterprise productivity. PitchBook also finds vertical agents took **54.6% of 2026 year-to-date agent capital** ([PitchBook](https://pitchbook.com/news/reports/q2-2026-pitchbook-analyst-note-agentic-ai-the-evolution-to-autonomous-systems-part-i); paywalled, figures via search snippets). For 2026 the estimates split sharply:

| Tracker | Period | Figure | Definition |
|---|---|---|---|
| [CallSphere](https://callsphere.ai/blog/agentic-ai-startup-funding-12-billion-q1-2026-tripling-year-over-year) | Q1 2026 | $12B | Broad; methodology unclear (vendor blog) |
| [New Market Pitch](https://newmarketpitch.com/blogs/news/agentic-ai-funding-trends) | Jan–May 2026 | ~$1.1B across 29 deals | Narrow editorial list |
| [gravity.fast](https://gravity.fast/blog/ai-agent-funding-tracker-q3-2026/) | Q3 2026 | $5.25B across 56 rounds | Named lead required; excludes labs, infrastructure, robotics |

These numbers are not really in conflict. They measure different things, depending mostly on whether coding agents such as Cognition and Cursor and agent infrastructure are counted as "agents." This report does not pick one. The defensible reading is that **agent application and infrastructure companies raise in the low tens of billions per year**, which is a small fraction of AI-wide totals. The trackers do agree on the shape: in New Market Pitch's data the top 10 deals took about 78% of 2026 capital and the bottom half about 11.5%, and gravity.fast puts the median Q3 round at $30M.

Stated VC theses line up on three bets. The first is **"services-as-software"**: Sequoia argues "a copilot sells the tool, an autopilot sells the work" and targets outsourced, judgment-light work such as insurance brokerage, claims, accounting and IT services ([Sequoia](https://sequoiacap.com/article/services-the-new-software)). General Catalyst has committed **$1.5B to AI-enabled roll-ups** of service firms ([Capital and Clarity](https://capitalandclarity.substack.com/p/the-general-catalyst-behind-15-billion)). The second is **coding agents**, which a16z credits with over $1B of new revenue in 2025 ([a16z](https://a16z.com/notes-on-ai-apps-in-2026/)). The third is **picks-and-shovels infrastructure**: governance, security, observability, evals and payments. YC's Summer 2026 Requests for Startups follow the same pattern: "AI-Native Service Companies," "Software for Agents" and "Inference Chips for Agent Workflows" ([YC RFS](https://www.ycombinator.com/rfs)). Investors say they avoid thin wrappers and horizontal assistants the labs will absorb. Bessemer names the main ARR-quality worry: "supernovas" that reach $100M quickly, "often with fragile retention and thin margins" ([Bessemer](https://www.bvp.com/atlas/the-state-of-ai-2025)).

## Coding agents: the one place money and demand agree

### Revenue, tokens and surveys all point to code

Coding agents have the strongest demand evidence of any category. **Cursor** went from $1B annualized revenue in November 2025 ([BusinessWire](https://www.businesswire.com/news/home/20251113939996/en/Cursor-Secures-$2.3-Billion-Series-D-Financing-at-$29.3-Billion-Valuation-to-Redefine-How-Software-is-Written)) to **$2B ARR by February 2026** ([TheNextWeb](https://thenextweb.com/news/cursor-anysphere-2-billion-funding-50-billion-valuation-ai-coding)). **Cognition** went from about $37M to **$492M ARR** in twelve months, with enterprise usage up 50% month-over-month for six months ([TechCrunch](https://techcrunch.com/2026/05/27/ai-coding-startup-cognition-raises-1b-at-25b-pre-money-valuation/)).

The labs grew as fast or faster. **Claude Code passed a $2.5B run-rate by February 2026**, and business subscriptions quadrupled that year ([Simon Willison quoting Anthropic](https://x.com/simonw/status/2022044549733056861)). **OpenAI Codex** went from 1.6M users in March ([Fortune](https://fortune.com/2026/03/04/openai-codex-growth-enterprise-ai-agents/)) to **more than 5M weekly users in June** ([Constellation Research](https://www.constellationr.com/insights/news/openai-touts-broadening-codex-usage-5-million-weekly-active-users)) and about 8M after GPT-5.6 launched in July ([The New Stack](https://thenewstack.io/gpt-5-6-codex-user-surge/)).

Usage telemetry agrees. Programming grew from about 11% to **more than 50% of OpenRouter tokens** ([OpenRouter](https://openrouter.ai/apps/category/coding)). Anthropic's June 2026 Economic Index finds Claude Code sessions are measurably more autonomous than chat. About two-thirds of that gap comes from users delegating *the same tasks* more fully when the interface allows it ([Anthropic](https://www.anthropic.com/research/economic-index-june-2026-report)).

Enterprise spend data tells the same story. Menlo put coding tools at **$4B of 2025 enterprise spend, about 55% of departmental AI spend** ([Menlo Ventures](https://menlovc.com/perspective/2025-the-state-of-generative-ai-in-the-enterprise/)). McKinsey's 2026 survey names software coding as the most common agent use that has reached scale ([McKinsey](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai)).

### Capital concentrated in a handful of leaders

VC money followed. **Cursor raised $2.3B at $29.3B** in November 2025 ([CNBC](https://www.cnbc.com/2025/11/13/cursor-ai-startup-funding-round-valuation.html)). SpaceX then acquired it in an **all-stock deal valuing Anysphere at $60B**, widely described as the largest acquisition of a venture-backed startup ever ([CNBC](https://www.cnbc.com/2026/06/16/spacex-spcx-cursor-acquisition-ipo.html)). The deal closed in mid-August ([Yahoo Finance](https://finance.yahoo.com/technology/ai/articles/spacex-completes-record-60-billion-131311785.html)). **Cognition raised $1B at $26B post-money** in May, co-led by Lux, General Catalyst and 8VC ([TechCrunch](https://techcrunch.com/2026/05/27/ai-coding-startup-cognition-raises-1b-at-25b-pre-money-valuation/)). **Factory raised $150M at $1.5B** in April for model-agnostic enterprise "Droids," without disclosing revenue ([TechCrunch](https://techcrunch.com/2026/04/16/factory-hits-1-5b-valuation-to-build-ai-coding-for-enterprises/)).

Buyers also treat code review as strategic. Cursor bought Graphite ([Graphite](https://graphite.com/blog/graphite-joins-cursor)), and Cognition now says 89% of its own commits are written by Devin ([entrepreneurloop](https://entrepreneurloop.com/cognition-ai-funding-2026-1b-raise-26b-valuation/), secondary).

### Three figures in this category conflict

**Cognition: $26B or $48B?** The **$26B post-money valuation (May 27, 2026) is the last confirmed figure**. It is reported by TechCrunch and Bloomberg ([Bloomberg](https://www.bloomberg.com/news/articles/2026-05-27/ai-coding-startup-cognition-raises-1-billion-at-26-billion-value)). A "$2B+ Series E at $48B" led by a16z and Accel on September 8 appears only in two small trackers ([gravity.fast](https://gravity.fast/blog/ai-agent-funding-tracker-q3-2026/); [aifunding.me](https://aifunding.me/ai-agent-funding)). No primary announcement was found. The claim is plausible: a 1.8x step-up in about 3.5 months matches the category's pace. It should still be treated as **unverified**.

**Factory: $1.5B or $5B?** These two numbers are not necessarily contradictory. The **$150M at $1.5B (April 16) is confirmed** by TechCrunch. The **$200M at $5B dated September 15** appears only in the gravity.fast tracker. If it is real, it is a later round, not a restatement, and it would mean a 3.3x step-up in five months. It is **unverified**.

**Cursor revenue: $2B or $4B?** These are two dates, not two competing measurements. **$2B ARR in February 2026 is well sourced.** **About $4B annualized revenue around the June SpaceX deal** was reported by [Quartz](https://qz.com/spacex-buying-cursor-anysphere-60-billion-deal-061626) and aggregators, but no company statement confirming it was found. The path from $1B (November) to $2B (February) makes another doubling by June plausible. The report treats $2B as confirmed and ~$4B as a reported, unconfirmed estimate. As a result, Cursor's exit multiple lands somewhere between **15x and 30x run-rate**.

### Where the gap opens: the labs, and vibe-coding retention

Inside this aligned category, two gaps stand out.

The first is **who captures the demand**. By early 2026 Claude Code's run-rate already exceeded Cursor's. Codex has millions of weekly users, and TechCrunch notes that the lab products have "captured significant market share" even as VCs keep backing independents ([TechCrunch](https://techcrunch.com/2026/05/27/ai-coding-startup-cognition-raises-1b-at-25b-pre-money-valuation/)). Independents resell those same labs' models. In 2025, coding-startup gross margins were described as "either neutral or negative" ([TechCrunch](https://techcrunch.com/2025/08/07/the-high-costs-and-thin-margins-threatening-ai-coding-startups/)), and none has disclosed margins since. The biggest outcome so far is telling: Cursor was bought by a buyer that owns compute, not taken public. That suggests inference cost is the real constraint. User pull for coding agents is proven; pull for *independent* coding agents at 25–50x revenue is much less so.

The second gap is **vibe-coding app builders for non-developers**, which got the fastest valuation step-ups of anything in the dataset:

| Company | Valuation step | Source |
|---|---|---|
| Lovable | $6.6B to **$13.3B** (Dec 2025 to Aug 2026) | [TechCrunch](https://techcrunch.com/2026/08/12/lovable-confirms-new-13-3b-valuation-raises-another-400m/) |
| Replit | $3B to **$9B** in six months | [TechCrunch](https://techcrunch.com/2026/03/11/replit-snags-9b-valuation-6-months-after-hitting-3b) |
| Emergent | $300M to a reported **$1.5B** in six months | [TechCrunch](https://techcrunch.com/2026/01/20/indian-vibe-coding-startup-emergent-raises-70m-at-300m-valuation-from-softbank-khosla-ventures/); [Axios Pro](https://www.axios.com/pro/enterprise-software-deals/2026/07/15/vibe-coding-emergent-creaegis-claypond), headline only |

Revenue is real. Lovable reached **$500M ARR in June 2026** and was tracking toward $600M, with 900M monthly visits to apps built on it (TechCrunch, above). But the retention evidence runs the other way. Barclays traffic data through September 2025 showed Lovable down about 40% from peak, v0 down 64% and Bolt down 27%, and Bolt's CEO said "the churn rate for everyone is really high" ([TestSprite](https://www.testsprite.com/blog/beyond-the-hype-why-vibe-coding-leaders-are-facing-a-retention-crisis)). None of these companies discloses churn or gross margin ([valueaddvc](https://valueaddvc.com/blog/lovable-valuation-2026-13-2b-and-500m-arr-how-vibe-coding-actually-makes-money)).

ARR rising while traffic falls most likely means revenue is shifting toward fewer, heavier and more corporate users. That is a healthier story than tourist churn, but it has not been shown. This is Bessemer's "supernova" profile, and VCs are paying 25x+ for it.

## Vertical enterprise agents draw 50–100x multiples on thin adoption

Outside coding, **customer support and CX is the most heavily funded agent function**. Four rounds between January and May 2026 totaled about $2B:

| Company | Round | Valuation | Source |
|---|---|---|---|
| Sierra | $950M (May 2026) | Above $15B | [TechCrunch](https://techcrunch.com/2026/05/04/sierra-raises-950m-as-the-race-to-own-enterprise-ai-gets-serious/) |
| ElevenLabs | $500M (Feb 2026) | $11B | [TechCrunch](https://techcrunch.com/2026/02/04/elevenlabs-raises-500m-from-sequioia-at-a-11-billion-valuation/) |
| Parloa | $350M (Jan 2026) | $3B, tripled in about 8 months | [TechCrunch](https://techcrunch.com/2026/01/15/parloa-triples-its-valuation-in-8-months-to-3b-with-350m-raise/) |
| Decagon | $250M (Jan 2026) | $4.5B, tripled from $1.5B | [Bloomberg](https://www.bloomberg.com/news/articles/2026-01-28/ai-customer-support-startup-decagon-valued-at-4-5-billion) |

Demand here is real and well documented. Sierra reached **$150M ARR by February 2026**, and more than 40% of the Fortune 50 are customers. Intercom's Fin is nearing **$100M ARR at $0.99 per resolved conversation**, with resolution rates up from about 25% to 65–70% ([Enterprise DNA](https://enterprisedna.co/resources/ai-pulse/ai-pulse-2026-08-02-intercom-s-fin-ai-agent-is-nearing-100m-arr-roughly-half-of/); [Fin pricing](https://fin.ai/help/en/articles/13975800-fin-pricing-outcomes)). CB Insights counts six private CX-agent companies with $100M+ revenue ([CB Insights](https://www.cbinsights.com/research/ai-agent-predictions-2026/)).

What diverges is price. **Sierra's $15B+ on $150M ARR is roughly 100x run-rate**, the highest multiple among companies with confirmed revenue in this dataset.

**Legal** is the next-hottest vertical, and it has verified revenue. **Harvey raised $200M at $11B** in March 2026. It serves more than 100,000 lawyers at 1,300 organizations ([CNBC](https://www.cnbc.com/2026/03/25/legal-ai-startup-harvey-raises-200-million-at-11-billion-valuation.html); [TechCrunch](https://techcrunch.com/2026/04/30/legal-ai-startup-legora-hits-5-6-valuation-and-its-battle-with-harvey-just-got-hotter/)). **Legora raised $600M at $5.6B** with $100M+ ARR, per the same TechCrunch article. It reportedly doubled to $200M by September and is seeking $10B+ ([Lawyer Monthly](https://www.lawyer-monthly.com/2026/08/legora-10bn-valuation-harvey-15bn-ai-funding/), unconfirmed).

Other verticals:

- **Healthcare:** the largest valuation goes to OpenEvidence, **$12B in January**, reportedly $15B in September ([Sacra](https://sacra.com/c/openevidence/), aggregator). OpenEvidence is a clinical search product, not a workflow agent; the true agent plays are ambient scribes such as Abridge ([Fierce Healthcare](https://www.fiercehealthcare.com/ai-and-machine-learning/ambient-ai-startup-abridge-scores-300m-series-e-backed-a16z-and-khosla)).
- **Finance and accounting:** many $100M-class rounds with valuations near $1B. Rogo raised $160M ([FinTech Global](https://fintech.global/2026/04/29/rogo-raises-160m-series-d-to-scale-finance-ai-platform/)); Rillet reached a $1B valuation ([TechFundingNews](https://techfundingnews.com/rillet-becomes-unicorn-100m-series-c-iconiq/)).
- **Go-to-market:** the money went to data platforms with agents on top. Clay reached **$7.1B on $150M ARR** ([Runtimewire](https://runtimewire.com/article/kareem-amin-clay-115m-series-d-7-1b-valuation)). Autonomous AI-SDR plays are still marked by the 2025 11x allegations of inflated ARR and 70–80% churn ([Salesmotion](https://salesmotion.io/blog/turns-out-ai-sdrs-are-too-good-to-be-true-11x-might-face-legal-action)).
- **Security operations:** mid-sized rounds. Torq reached $1.2B ([Torq](https://torq.io/news/torq-seriesd/)).

The divergence in this category is between **the revenue of the leaders and the adoption of the market**. McKinsey's 2026 survey finds only **23% of organizations scaling an agentic system in any function**, rising to 40% at large firms ([McKinsey](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai)). The 23% figure matches McKinsey's November 2025 number, so its year should be confirmed.

Other surveys point the same way. KPMG's Q2 2026 pulse finds **about a quarter of organizations scaled back AI deployments and a similar share paused them** because costs outran value ([KPMG](https://kpmg.com/se/en/insights/ai/global-ai-pulse-Q2-2026.html)). Menlo classifies only **16% of enterprise "agent" deployments as true agents** ([Menlo Ventures](https://menlovc.com/perspective/2025-the-state-of-generative-ai-in-the-enterprise/)). Gartner still expects **more than 40% of agentic projects to be canceled by 2027** ([Gartner](https://www.gartner.com/en/newsroom/press-releases/2025-06-25-gartner-predicts-over-40-percent-of-agentic-ai-projects-will-be-canceled-by-end-of-2027)).

Valuations near 100x assume the category leaders will capture that slow-moving majority. Outcome pricing is what makes the bet coherent: it works where the output is a discrete, countable event, like a resolved ticket. It has not spread to legal or finance, which still sell seats.

## Infrastructure money follows metered usage, not developer popularity

Agent infrastructure shows its own divergence. **Valuations are re-rating fastest where revenue scales with agent traffic.** Modal (compute sandboxes) went from $1.1B to **$4.65B in about eight months** on a $355M Series C ([bex.co](https://bex.co/blog/2026/09/11/ai-sandbox-funding-modal-daytona-e2b), secondary). Exa (search for agents) raised **$250M at $2.2B**, more than triple its valuation eight months earlier. Its customers include Cursor and Cognition ([Exa](https://exa.ai/blog/announcing-series-c)). An aggregator's "$400M at $2.4B" figure for Exa conflicts with Exa's own announcement and should be ignored. Parallel Web Systems raised at about $2B ([American Bazaar](https://americanbazaaronline.com/2026/04/30/parag-agrawals-parallel-web-systems-hits-2-billion-in-valuation-479920/)). Identity also drew large checks: WorkOS raised **$100M at $2B** ([WorkOS](https://workos.com/blog/series-c)). LangChain ([TechCrunch](https://techcrunch.com/2025/10/21/open-source-agentic-startup-langchain-hits-1-25b-valuation)) and LiveKit ([LiveKit](https://livekit.com/blog/livekit-series-c)) both crossed $1B.

Developer popularity alone did not bring new money. Browser Use calls itself the biggest open-source web-agent project, and Browserbase has heavy adoption, but **neither disclosed a new round in the window** ([Browserbase](https://www.browserbase.com/blog/series-b-and-beyond)). The labs building computer use into their own products is the likely reason.

Thin, easy-to-copy layers were **bought rather than funded**:

- Twilio bought Stytch for identity ([Twilio](https://www.twilio.com/en-us/blog/company/news/twilio-to-acquire-stytch)).
- Arcade, after raising $60M ([BusinessWire](https://www.businesswire.com/news/home/20260615229631/en/Arcade-Raises-$60M-to-Become-the-Secure-Action-Layer-Behind-Every-Production-AI-Agent)), bought the Smithery MCP registry ([Forbes](https://www.forbes.com/sites/janakirammsv/2026/08/10/arcade-acquires-smithery-to-own-the-agent-tool-supply-chain/)).
- Observability tools Langfuse, Galileo and Helicone went to ClickHouse, Cisco and Mintlify ([AgentMarketCap](https://agentmarketcap.ai/blog/2026/04/11/ai-agent-evaluation-startup-market-map-2026), unconfirmed against primary releases).

MCP adoption is real, and the protocol now sits under neutral Linux Foundation governance ([Linux Foundation](https://www.linuxfoundation.org/press/linux-foundation-announces-the-formation-of-the-agentic-ai-foundation)). But a September census found **101,219 registry entries collapse to 31,309 unique servers**, and about 23% have no source repository ([DEV Community](https://dev.to/leroy_jenkins_951c84b2838/i-counted-the-mcp-registry-101219-entries-are-31309-servers-and-23-have-no-source-repo-33a6)). Server counts measure experimentation, not usage. Investors seem to know this: they are paying for metered compute and search calls, not for ecosystem size.

## Consumer agents show real pull that VCs cannot capture

General-purpose and consumer agents diverge in the opposite direction from everything above. Here **users show pull and VCs step back**. Standalone products struggled across the board:

- **Atlas:** OpenAI shut down its standalone Atlas browser and folded agentic browsing into ChatGPT and Codex ([OpenAI Help Center](https://help.openai.com/en/articles/20001371-evolving-atlas-into-chatgpt-for-browser-based-agentic-work)).
- **Project Mariner:** Google discontinued it in May ([Wikipedia](https://en.wikipedia.org/wiki/Project_Mariner)).
- **Cowork:** Anthropic merged it into ordinary Claude chat on September 16 ([Fortune](https://fortune.com/2026/09/16/anthropic-merges-its-claude-chat-and-agentic-cowork-products-into-a-single-ai-assistant-as-part-of-a-push-to-build-an-ai-superapp/)).
- **Manus:** Meta's roughly $2B acquisition was ordered unwound by China's NDRC, and Manus now operates independently ([CNBC](https://www.cnbc.com/2026/08/11/manus-china-meta-acquisition.html)).
- **Relay:** the agent-workflow Zapier alternative closed in August because the labs built similar features ([TechCrunch](https://techcrunch.com/2026/09/15/the-ai-graveyard-a-running-list-of-projects-and-startups-that-didnt-make-it/)).

**Atlas shutdown date: July 9 or August 9?** The sources split. TechCrunch's "AI graveyard" and OpenAI's help-center page (read through search snippets) give **August 9, 2026**. [PPC Land](https://ppc.land/openai-kills-atlas-browser-folds-it-into-new-chatgpt-work-agent/) gives **July 9**. July 9 is also the day GPT-5.6 launched ([The New Stack](https://thenewstack.io/gpt-5-6-codex-user-surge/)), and ChatGPT Work, the product that absorbed Atlas, runs on GPT-5.6. The likeliest explanation, which is an inference, is that the deprecation was **announced on July 9 and the browser stopped working on August 9**. Either way, Atlas lasted about nine to ten months.

Yet the demand signals in this category are among the most striking in the dataset:

- **OpenClaw**, an open-source local personal agent, reportedly passed **250k GitHub stars by March**, overtaking React, and 310k+ by April ([Wikipedia](https://en.wikipedia.org/wiki/OpenClaw)). Its creator joined OpenAI, so the value went to a lab through a hire, not to a funded startup.
- On OpenRouter, the open-source persistent **Hermes Agent** leads the coding-agent token rankings at about 48.5T tokens, ahead of Claude Code ([OpenRouter](https://openrouter.ai/apps/category/coding)). Blocmates reports that 85–90% of those tokens accrued in the last 30 days ([blocmates](https://www.blocmates.com/articles/top-apps-on-openrouter-september-2026-edition)). That is a spike, not yet proven retention.
- Codex has more than 1M people using it for work outside software ([PPC Land](https://ppc.land/openai-kills-atlas-browser-folds-it-into-new-chatgpt-work-agent/)).

The pattern is consistent: **consumer agent demand lives inside surfaces people already use every day**, and the labs own those surfaces.

The two funded exceptions sell subscriptions to prosumers and workplaces, not errand-running agents. **Genspark** extended its Series B to $485M at a **$2.6B valuation**, with revenue growing from about $100M ARR in January toward roughly $250M by June ([BusinessWire](https://www.businesswire.com/news/home/20260617758937/en/Genspark.ai-Extends-Series-B-to-$485M-at-$2.6B-Post-Money-Valuation-Appoints-Jamison-Powell-as-Chief-Revenue-Officer); [Axios](https://www.axios.com/pro/enterprise-software-deals/2026/01/21/genspark-funding-100-million-arr-genai)). **Perplexity** reportedly reached about $750M annualized revenue at a $20–23B valuation ([Sacra](https://sacra.com/c/perplexity/), aggregator). Its Comet agentic browser has only about 3M MAU, but it carried about 47% of measured agentic web traffic in May ([PPC Land](https://ppc.land/openai-kills-atlas-browser-folds-it-into-new-chatgpt-work-agent/)). At roughly **10x run-rate, Genspark is priced far below the enterprise-agent leaders**. Proven consumer revenue is cheaper per dollar than unproven enterprise revenue.

## Where the money and the demand part ways

Putting the valuations next to the best available revenue figures shows the divergence most clearly. The multiples below are rough: several revenue figures are aggregator-reported, and the dates of the valuation and the revenue figure don't always match.

| Company | Valuation | Revenue basis | Approx. multiple |
|---|---|---|---|
| Sierra (CX) | >$15B | $150M ARR (Feb 2026) | ~100x |
| Harvey (legal) | $11B | ~$190M ARR (aggregator) | ~58x |
| Cognition (coding) | $26B (confirmed) | $492M ARR | ~53x |
| Clay (GTM) | $7.1B | $150M ARR | ~47x |
| Perplexity (consumer) | ~$23B (aggregator) | ~$750M (aggregator) | ~31x |
| Lovable (vibe coding) | $13.3B | ~$500M ARR | ~27x |
| Replit (vibe coding) | $9B | ~$525M ARR (secondary) | ~17x |
| Cursor (coding, exit) | $60B | $2B confirmed / ~$4B reported | 15–30x |
| Genspark (consumer/prosumer) | $2.6B | ~$250M ARR (aggregator) | ~10x |

The ranking roughly inverts the strength of the demand evidence. VCs pay the **highest multiples for enterprise autonomy**: CX, legal and autonomous coding. In those categories the leaders' revenue is real, but broad adoption is stuck near a quarter of enterprises and pilot cancellations are forecast. They pay the **lowest multiples where user revenue is most proven**: Cursor at exit, Replit and Genspark. The market is betting on outcome pricing and labor budgets, which fits Sequoia's thesis. It is not betting on the usage data in hand.

Four specific divergences stand out:

1. **Coding agents:** demand is proven for the category, but the labs' own agents capture a growing share of it. Independents remain priced as if they will keep that demand.
2. **Vibe-coding builders:** valuations doubled and tripled while the only public retention data points the wrong way.
3. **Vertical enterprise agents:** pull is proven at the leaders but not across the market. KPMG, Menlo and Gartner all describe a large pilot-to-production gap, and Ramp finds the median US business spends just **$11.95 per employee per year** on AI. Spend is concentrated in a small tail of heavy adopters ([Ramp](https://ramp.com/data/ai-index-august-2026)).
4. **Consumer and general-purpose agents:** the gap runs the other way. Open-source and in-app agents show real pull, but it produces no fundable company because the labs absorb both the products and the people.

## Conclusion

The durable signal is not "agents are hot"; it is that **verifiability predicts demand**. Agents whose output can be checked cheaply, such as code that compiles and tests or a support ticket that gets resolved, are the ones with revenue, token share and enterprise scaling. They are also the only ones where outcome pricing works. VCs have extended that logic to legal, finance, healthcare and "AI-native services," where outputs are harder to verify and the adoption data is weaker. The 50–100x multiples in those categories are a bet that verification tooling and outcome contracts will catch up. That also explains why evals, observability and governance keep getting funded or acquired: they are the missing piece that would turn enterprise pilots into production.

The second implication is about who captures the value. In both coding and consumer agents, the strongest user pull increasingly goes to the model labs. Claude Code out-earns Cursor, Atlas and Cowork were folded into the core chat apps, and OpenClaw's creator joined OpenAI. The independents that thrive either own a scarce input (compute for Cursor via SpaceX, metered search for Exa, proprietary workflow data for Harvey and Clay) or sell into budgets the labs don't serve directly. The indicators to watch through Q4 2026 are primary confirmation of Cognition's $48B and Factory's $5B rounds, whether Hermes Agent's token spike holds for a second month, and whether any vibe-coding leader discloses churn or gross margin before its next raise.

## Sources

- [a16z, Notes on AI Apps in 2026](https://a16z.com/notes-on-ai-apps-in-2026/)
- [AgentMarketCap, AI agent evaluation market map 2026](https://agentmarketcap.ai/blog/2026/04/11/ai-agent-evaluation-startup-market-map-2026)
- [aifunding.me, AI agent funding tracker](https://aifunding.me/ai-agent-funding)
- [American Bazaar, Parallel Web Systems at $2B](https://americanbazaaronline.com/2026/04/30/parag-agrawals-parallel-web-systems-hits-2-billion-in-valuation-479920/)
- [Anthropic Economic Index, June 2026](https://www.anthropic.com/research/economic-index-june-2026-report)
- [Axios Pro, Emergent round](https://www.axios.com/pro/enterprise-software-deals/2026/07/15/vibe-coding-emergent-creaegis-claypond)
- [Axios Pro, Genspark $100M ARR](https://www.axios.com/pro/enterprise-software-deals/2026/01/21/genspark-funding-100-million-arr-genai)
- [Bessemer, State of AI 2025](https://www.bvp.com/atlas/the-state-of-ai-2025)
- [bex.co, AI sandbox funding](https://bex.co/blog/2026/09/11/ai-sandbox-funding-modal-daytona-e2b)
- [blocmates, Top apps on OpenRouter, September 2026](https://www.blocmates.com/articles/top-apps-on-openrouter-september-2026-edition)
- [Bloomberg, Cognition $26B](https://www.bloomberg.com/news/articles/2026-05-27/ai-coding-startup-cognition-raises-1-billion-at-26-billion-value)
- [Bloomberg, Decagon $4.5B](https://www.bloomberg.com/news/articles/2026-01-28/ai-customer-support-startup-decagon-valued-at-4-5-billion)
- [Browserbase, Series B](https://www.browserbase.com/blog/series-b-and-beyond)
- [BusinessWire, Arcade $60M](https://www.businesswire.com/news/home/20260615229631/en/Arcade-Raises-$60M-to-Become-the-Secure-Action-Layer-Behind-Every-Production-AI-Agent)
- [BusinessWire, Cursor Series D](https://www.businesswire.com/news/home/20251113939996/en/Cursor-Secures-$2.3-Billion-Series-D-Financing-at-$29.3-Billion-Valuation-to-Redefine-How-Software-is-Written)
- [BusinessWire, Genspark Series B extension](https://www.businesswire.com/news/home/20260617758937/en/Genspark.ai-Extends-Series-B-to-$485M-at-$2.6B-Post-Money-Valuation-Appoints-Jamison-Powell-as-Chief-Revenue-Officer)
- [CallSphere, agentic AI funding Q1 2026](https://callsphere.ai/blog/agentic-ai-startup-funding-12-billion-q1-2026-tripling-year-over-year)
- [Capital and Clarity, General Catalyst $1.5B](https://capitalandclarity.substack.com/p/the-general-catalyst-behind-15-billion)
- [CB Insights, AI agent predictions 2026](https://www.cbinsights.com/research/ai-agent-predictions-2026/)
- [CB Insights, State of AI Q2'26](https://www.cbinsights.com/research/report/ai-trends-q2-2026/)
- [CNBC, Cursor Series D](https://www.cnbc.com/2025/11/13/cursor-ai-startup-funding-round-valuation.html)
- [CNBC, Harvey $11B](https://www.cnbc.com/2026/03/25/legal-ai-startup-harvey-raises-200-million-at-11-billion-valuation.html)
- [CNBC, Manus independence](https://www.cnbc.com/2026/08/11/manus-china-meta-acquisition.html)
- [CNBC, SpaceX acquires Cursor](https://www.cnbc.com/2026/06/16/spacex-spcx-cursor-acquisition-ipo.html)
- [Constellation Research, Codex 5M weekly users](https://www.constellationr.com/insights/news/openai-touts-broadening-codex-usage-5-million-weekly-active-users)
- [Crunchbase News, H1 2026](https://news.crunchbase.com/venture/global-startup-exits-ipo-ma-soar-ai-q2-h1-2026/)
- [Crunchbase News, Q1 2026](https://news.crunchbase.com/venture/record-breaking-funding-ai-global-q1-2026/)
- [DEV Community, MCP registry census](https://dev.to/leroy_jenkins_951c84b2838/i-counted-the-mcp-registry-101219-entries-are-31309-servers-and-23-have-no-source-repo-33a6)
- [Enterprise DNA, Intercom Fin near $100M ARR](https://enterprisedna.co/resources/ai-pulse/ai-pulse-2026-08-02-intercom-s-fin-ai-agent-is-nearing-100m-arr-roughly-half-of/)
- [entrepreneurloop, Cognition 2026](https://entrepreneurloop.com/cognition-ai-funding-2026-1b-raise-26b-valuation/)
- [Exa, Series C](https://exa.ai/blog/announcing-series-c)
- [Fierce Healthcare, Abridge Series E](https://www.fiercehealthcare.com/ai-and-machine-learning/ambient-ai-startup-abridge-scores-300m-series-e-backed-a16z-and-khosla)
- [Fin, pricing](https://fin.ai/help/en/articles/13975800-fin-pricing-outcomes)
- [FinTech Global, Rogo Series D](https://fintech.global/2026/04/29/rogo-raises-160m-series-d-to-scale-finance-ai-platform/)
- [Forbes, Arcade acquires Smithery](https://www.forbes.com/sites/janakirammsv/2026/08/10/arcade-acquires-smithery-to-own-the-agent-tool-supply-chain/)
- [Fortune, Anthropic merges Cowork into Claude](https://fortune.com/2026/09/16/anthropic-merges-its-claude-chat-and-agentic-cowork-products-into-a-single-ai-assistant-as-part-of-a-push-to-build-an-ai-superapp/)
- [Fortune, Codex growth](https://fortune.com/2026/03/04/openai-codex-growth-enterprise-ai-agents/)
- [Gartner, 40% of agentic projects canceled by 2027](https://www.gartner.com/en/newsroom/press-releases/2025-06-25-gartner-predicts-over-40-percent-of-agentic-ai-projects-will-be-canceled-by-end-of-2027)
- [Graphite joins Cursor](https://graphite.com/blog/graphite-joins-cursor)
- [gravity.fast, AI agent funding tracker Q3 2026](https://gravity.fast/blog/ai-agent-funding-tracker-q3-2026/)
- [KPMG, Global AI Pulse Q2 2026](https://kpmg.com/se/en/insights/ai/global-ai-pulse-Q2-2026.html)
- [Lawyer Monthly, Legora and Harvey valuations](https://www.lawyer-monthly.com/2026/08/legora-10bn-valuation-harvey-15bn-ai-funding/)
- [Linux Foundation, Agentic AI Foundation](https://www.linuxfoundation.org/press/linux-foundation-announces-the-formation-of-the-agentic-ai-foundation)
- [LiveKit, Series C](https://livekit.com/blog/livekit-series-c)
- [McKinsey, State of AI](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai)
- [Menlo Ventures, 2025 State of GenAI in the Enterprise](https://menlovc.com/perspective/2025-the-state-of-generative-ai-in-the-enterprise/)
- [New Market Pitch, agentic AI funding trends](https://newmarketpitch.com/blogs/news/agentic-ai-funding-trends)
- [OpenAI Help Center, Atlas into ChatGPT](https://help.openai.com/en/articles/20001371-evolving-atlas-into-chatgpt-for-browser-based-agentic-work)
- [OpenRouter, coding agent rankings](https://openrouter.ai/apps/category/coding)
- [PitchBook, Agentic AI analyst note Part I](https://pitchbook.com/news/reports/q2-2026-pitchbook-analyst-note-agentic-ai-the-evolution-to-autonomous-systems-part-i)
- [PPC Land, OpenAI kills Atlas](https://ppc.land/openai-kills-atlas-browser-folds-it-into-new-chatgpt-work-agent/)
- [Quartz, SpaceX buying Cursor](https://qz.com/spacex-buying-cursor-anysphere-60-billion-deal-061626)
- [Ramp AI Index, August 2026](https://ramp.com/data/ai-index-august-2026)
- [Runtimewire, Clay Series D](https://runtimewire.com/article/kareem-amin-clay-115m-series-d-7-1b-valuation)
- [Sacra, OpenEvidence](https://sacra.com/c/openevidence/)
- [Sacra, Perplexity](https://sacra.com/c/perplexity/)
- [Salesmotion, 11x allegations](https://salesmotion.io/blog/turns-out-ai-sdrs-are-too-good-to-be-true-11x-might-face-legal-action)
- [Sequoia, Services: The New Software](https://sequoiacap.com/article/services-the-new-software)
- [Simon Willison quoting Anthropic on Claude Code](https://x.com/simonw/status/2022044549733056861)
- [TechCrunch, AI graveyard](https://techcrunch.com/2026/09/15/the-ai-graveyard-a-running-list-of-projects-and-startups-that-didnt-make-it/)
- [TechCrunch, Cognition $1B at $25B pre](https://techcrunch.com/2026/05/27/ai-coding-startup-cognition-raises-1b-at-25b-pre-money-valuation/)
- [TechCrunch, coding startup margins](https://techcrunch.com/2025/08/07/the-high-costs-and-thin-margins-threatening-ai-coding-startups/)
- [TechCrunch, ElevenLabs $11B](https://techcrunch.com/2026/02/04/elevenlabs-raises-500m-from-sequioia-at-a-11-billion-valuation/)
- [TechCrunch, Emergent $300M](https://techcrunch.com/2026/01/20/indian-vibe-coding-startup-emergent-raises-70m-at-300m-valuation-from-softbank-khosla-ventures/)
- [TechCrunch, Factory $1.5B](https://techcrunch.com/2026/04/16/factory-hits-1-5b-valuation-to-build-ai-coding-for-enterprises/)
- [TechCrunch, LangChain $1.25B](https://techcrunch.com/2025/10/21/open-source-agentic-startup-langchain-hits-1-25b-valuation)
- [TechCrunch, Legora $5.6B](https://techcrunch.com/2026/04/30/legal-ai-startup-legora-hits-5-6-valuation-and-its-battle-with-harvey-just-got-hotter/)
- [TechCrunch, Lovable $13.3B](https://techcrunch.com/2026/08/12/lovable-confirms-new-13-3b-valuation-raises-another-400m/)
- [TechCrunch, Parloa $3B](https://techcrunch.com/2026/01/15/parloa-triples-its-valuation-in-8-months-to-3b-with-350m-raise/)
- [TechCrunch, Replit $9B](https://techcrunch.com/2026/03/11/replit-snags-9b-valuation-6-months-after-hitting-3b)
- [TechCrunch, Sierra $950M](https://techcrunch.com/2026/05/04/sierra-raises-950m-as-the-race-to-own-enterprise-ai-gets-serious/)
- [TechFundingNews, Rillet unicorn](https://techfundingnews.com/rillet-becomes-unicorn-100m-series-c-iconiq/)
- [TestSprite, vibe-coding retention](https://www.testsprite.com/blog/beyond-the-hype-why-vibe-coding-leaders-are-facing-a-retention-crisis)
- [The New Stack, GPT-5.6 Codex surge](https://thenewstack.io/gpt-5-6-codex-user-surge/)
- [TheNextWeb, Cursor $2B ARR](https://thenextweb.com/news/cursor-anysphere-2-billion-funding-50-billion-valuation-ai-coding)
- [Torq, Series D](https://torq.io/news/torq-seriesd/)
- [Twilio, acquiring Stytch](https://www.twilio.com/en-us/blog/company/news/twilio-to-acquire-stytch)
- [valueaddvc, Lovable valuation](https://valueaddvc.com/blog/lovable-valuation-2026-13-2b-and-500m-arr-how-vibe-coding-actually-makes-money)
- [Wikipedia, OpenClaw](https://en.wikipedia.org/wiki/OpenClaw)
- [Wikipedia, Project Mariner](https://en.wikipedia.org/wiki/Project_Mariner)
- [WorkOS, Series C](https://workos.com/blog/series-c)
- [Yahoo Finance, SpaceX completes Cursor acquisition](https://finance.yahoo.com/technology/ai/articles/spacex-completes-record-60-billion-131311785.html)
- [YC, Requests for Startups](https://www.ycombinator.com/rfs)

# Demand-Side and User-Adoption Signals for AI Agents (as of 2026-09-29)

Scope: what agents people and companies actually use and pay for, apart from VC funding. Every figure is dated. The research budget was limited to about 25 tool calls, so several items are marked as unverified or as gaps. Evidence strength ranking (strongest first): (1) paid spend and revenue from transaction data or company disclosures (Ramp, lab run-rates); (2) usage telemetry (OpenRouter tokens, Anthropic Economic Index, OpenAI usage counts); (3) large surveys (McKinsey, KPMG, Stack Overflow); (4) vendor or aggregator surveys and statistic roundups (weakest; often recycled or mis-dated).

## 1. Usage-share data (OpenRouter, Anthropic Economic Index, ChatGPT usage, traffic)

### Takeaway
Across every telemetry source, coding and software agents take the largest share of real token consumption, and usage is moving toward more delegated, autonomous work (Claude Code, API). General consumer chat is still mostly augmentation or non-work use. The one strong non-coding agentic signal is persistent general-purpose agents (Hermes Agent) on OpenRouter. This is a developer-heavy, opt-in sample.

### Cited Findings
- **OpenRouter, coding share:** programming rose from about 11% of OpenRouter tokens in early 2025 to more than 50%, per the OpenRouter/a16z "State of AI" usage report (2025). This figure comes through search-result summaries, and I did not read the primary report in this session. — [OpenRouter coding rankings](https://openrouter.ai/apps/category/coding); [OpenRouter app rankings](https://openrouter.ai/apps)
- **OpenRouter coding-agent leaderboard (snapshot surfaced September 2026):** Hermes Agent (Nous Research, an open-source persistent agent with memory and 40+ tools) 48.5T tokens; Claude Code 23.2T; Kilo Code 15.2T; Cline 14.3T. The time window, lifetime or trailing, was not clear from the summary. — [OpenRouter Coding Agents Rankings](https://openrouter.ai/apps/category/coding)
- **Blocmates, September 2026 edition:** Hermes had about 50T lifetime tokens, and 85–90% of them accrued in the last 30 days. That is a very recent spike, so it is not yet proven retention. Top apps in other categories were Descript (creative; about two-thirds of its selections were Claude Sonnet 4.6/5), HelloMinds (productivity; switched to MiniMax M3 and reported 90–95% inference savings), and ISEKAI ZERO (entertainment/roleplay). Rising apps: Kilo Code, Mira, DeepSeek Agent Harness, and Freebuff (a free, ad-supported coding tool). Caveat: the rankings include only apps that opt in to public tracking. — [blocmates](https://www.blocmates.com/articles/top-apps-on-openrouter-september-2026-edition)
- **Anthropic Economic Index, January 2026 report (November 2025 data):** on Claude.ai, augmentation was 52% and automation 45%. Automation had briefly overtaken augmentation in August 2025. First-party API traffic stays mostly automated. There are more than 3,000 distinct work tasks, but the top 10 make up 24% of sampled conversations, and most of those top tasks are coding. — [Anthropic, Jan 2026 report](https://www.anthropic.com/research/anthropic-economic-index-january-2026-report)
- **Anthropic Economic Index, March 2026 report:** API workflows for business sales and automated trading roughly doubled between November 2025 and February 2026 (summarised through search; not read in full). — [Anthropic, March 2026 report](https://www.anthropic.com/research/economic-index-march-2026-report)
- **Anthropic Economic Index, "Cadences" (June 2026; data from April 10 to June 10, 2026):**
  - Claude Code sessions score 0.37 points higher on a 1–5 autonomy scale than chat/Cowork.
  - About 66% of that autonomy gap comes from the *same tasks* being done with more delegation.
  - For blog posts, the median is 13 turns in chat versus 1 prompt in Claude Code.
  - 54% of Claude Code sessions use Opus, versus 10% in chat.
  - Personal (non-work) use is about 35% of chat/Cowork conversations on weekdays and about 50% on weekends.
  - Survey (n≈9,700): 35%+ expect AI to handle "most or nearly all" of their work tasks within 12 months.
  - Caveat: respondents are heavily over-represented in computer/math occupations (30% of respondents vs. 4% of employment).
  — [Anthropic, June 2026 report](https://www.anthropic.com/research/economic-index-june-2026-report)
- **ChatGPT scale:** weekly active users grew from about 400M (February 2025) to about 800M (October 2025) and about 900M (February 2026). According to Apptopia, ChatGPT's share of the chatbot app market fell from 69.1% (January 2025) to 45.3% (early 2026). These figures come from aggregators and were not verified against the primary sources. — [Wikipedia: ChatGPT](https://en.wikipedia.org/wiki/ChatGPT); [sqmagazine](https://sqmagazine.co.uk/chatgpt-statistics/)
- **Codex:** more than 5M weekly active users, and more than 1M people use it for work outside software development (as reported in July 2026). — [PPC Land](https://ppc.land/openai-kills-atlas-browser-folds-it-into-new-chatgpt-work-agent/)

### Inferences
- The token-weighted "killer app" for agents is coding, and increasingly *headless or persistent* agents (Hermes, Claude Code CLI) rather than IDE autocomplete. Tokens per user are far higher for agents, so token share overstates how many *people* use them compared with how many use chat.
- Anthropic's data suggests that when the interface allows delegation (Claude Code), users take it. So demand for autonomy is real where outputs can be verified (code), but consumer chat stays mostly collaborative.
- OpenRouter's non-coding top apps (roleplay, creative, cheap productivity) show strong price sensitivity: apps move to cheaper open or Chinese models (MiniMax, DeepSeek, GLM).

### Gaps
- I did not get the actual current OpenRouter share percentages for Cline, Roo, Kilo, or Claude Code (the pages are JS-rendered). The Hermes 30-day spike needs a check against the next month to separate retention from a tourist spike.
- OpenAI's "How People Use ChatGPT" (NBER, September 2025) was not re-fetched. From memory, not verified this session: most usage is non-work, "Doing" (task execution) is about 40% of messages, and coding is only a few percent. Treat as unverified. — [OpenAI](https://openai.com/index/how-people-are-using-chatgpt/)
- A newer OpenAI-affiliated paper, "How Organizations Use AI: Evidence from ChatGPT" ([arXiv 2608.12236](https://arxiv.org/pdf/2608.12236)), appeared in search but was not read.
- I found no current Similarweb traffic data for agent products.

## 2. Revenue leaderboards and lab revenue share from agentic/coding products

### Takeaway
Paid revenue is the strongest demand signal, and it is concentrated in coding agents: Cursor, Claude Code, Codex, and Lovable. Anthropic's run-rate went from about $9B to about $65B in seven months, driven mainly by enterprise, API, and coding use. The exact share from Claude Code in 2026 is not public.

### Cited Findings
- **Anthropic run-rate:** about $9B at the end of 2025, $30B (reported earlier in 2026), $47B in May 2026, and more than $65B at the end of July 2026 (Bloomberg, via TechCrunch, August 17, 2026). Investors expect it to finish 2026 at $100–120B (per the FT). OpenAI was at about $40B annualized (Bloomberg, August 2026). Caveat: the two companies define revenue differently. — [TechCrunch](https://techcrunch.com/2026/08/17/anthropics-annualized-revenue-surges-to-65b/); [Axios](https://www.axios.com/2026/08/17/anthropic-revenue-run-rate-ipo-openai); [VentureBeat, $30B](https://venturebeat.com/technology/anthropic-says-it-hit-a-30-billion-revenue-run-rate-after-crazy-80x-growth); [Simon Willison, $47B](https://simonwillison.net/2026/May/29/anthropic/)
- **Claude Code:** about $1B annualized within about 6 months of launch, and more than $2.5B run-rate by February 2026 (secondary summaries). An aggregator claims "~$14B ARR" in mid-2026. That figure is **unverified and treat as suspect**. — [Axis Intelligence](https://axis-intelligence.com/anthropic-statistics/); [OpusClip blog](https://www.opus.pro/blog/hottest-ai-startups-2026)
- **Cursor (Anysphere):**
  - Growth path: $100M ARR (January 2025), $500M (June 2025), $1B (November 2025), $2B (February 2026). Cursor is described as the fastest B2B SaaS company to reach $1B ARR. — [TheNextWeb](https://thenextweb.com/news/cursor-anysphere-2-billion-funding-50-billion-valuation-ai-coding); [MLQ](https://mlq.ai/news/ai-coding-startup-cursor-reaches-2-billion-arr/)
  - Aggregators also claim $4B ARR (June 2026) and a $60B all-stock acquisition by SpaceX (announced June 16, 2026). **Not verified against a primary source; the report writer should verify before use.** — [gradually.ai](https://www.gradually.ai/en/cursor-statistics/); [aibusinessweekly](https://aibusinessweekly.net/p/cursor-ai-statistics)
- **Lovable:** $100M ARR in about 8 months, $200M (late 2025), $300M (January 2026), and about $400M (March 2026). Lovable does **not** disclose churn or its monthly/annual subscriber mix. — [Aakash Gupta on X](https://x.com/aakashgupta/status/2031770080481296683?lang=en); [TestSprite](https://www.testsprite.com/blog/beyond-the-hype-why-vibe-coding-leaders-are-facing-a-retention-crisis)
- **Ramp AI Index (card and bill-pay transaction data from US businesses; the strongest paid-adoption source):**
  - May 2026: Anthropic passed OpenAI in paid business adoption for the first time. About 79% of Anthropic's business customers also pay OpenAI, so buyers are adding a second provider rather than switching.
  - August 2026: Anthropic 43.5% of US businesses (+1.1pp month over month); OpenAI 39.7% (+0.23pp); xAI 4% (+0.94pp); model-serving platforms for open-source and Chinese models 6.1%.
  - Spend per employee is heavily skewed: the top 1% of firms spend a median $7,400 per employee per year, the top 10% spend $650, and the median firm spends $11.95.
  - Ramp's "cracks" thesis: the priciest frontier model (Fable 5, about $10 per 1M tokens) took only 6% of Anthropic tokens but 11.4% of Anthropic spend, which suggests a ceiling on willingness to pay.
  - Caveat: the sample skews toward tech companies.
  — [Ramp, Aug 2026](https://ramp.com/data/ai-index-august-2026); [Ramp, May 2026](https://ramp.com/data/ai-index-may-2026)
- **Menlo Ventures (December 2025):**
  - Enterprise generative AI spend was $37B in 2025 ($19B applications, $18B infrastructure).
  - Coding tools reached $4B, up from about $550M, which is about 55% of departmental AI spend.
  - Only about 16% of enterprise "agent" deployments qualify as true agents; the rest are mostly fixed-sequence workflows.
  - I found no 2026 edition.
  — [Menlo 2025 report](https://menlovc.com/perspective/2025-the-state-of-generative-ai-in-the-enterprise/)

### Inferences
- Money follows *verifiable-output* agents: code first, then document and office agents (ChatGPT Work, Cowork, Codex used outside coding). Coding leads every revenue leaderboard found.
- Ramp shows that business adoption breadth is still rising but concentrated. At the median firm, AI spend per employee is trivial, so the heavy spend (likely coding and agent tokens) sits in a small tail of AI-heavy firms.

### Gaps
- No official disclosure of the Claude Code or Codex share of lab revenue in 2026.
- Stripe's AI-startup revenue-growth data and the a16z top-100 consumer AI list (2026 editions) were not retrieved.
- Cursor figures after February 2026 and the SpaceX deal are unverified.

## 3. Enterprise surveys (2025–2026): agents in production and use cases

### Takeaway
Credible surveys agree that most enterprises *experiment* with agents, but only about 20–25% are *scaling* them in any function. Large enterprises are further ahead (about 40%). Headline numbers of "65–80% use agents" come from vendor surveys or loose definitions. KPMG reports meaningful pullback on cost grounds.

### Cited Findings
- **McKinsey, State of AI 2026** ("On the road to ROI", August 2026; n=1,719, 97 countries, fielded May 4 to June 8, 2026):
  - 23% of respondents say their organization is scaling an agentic AI system in at least one function.
  - 40% of large organizations (revenue above $1B) are scaling agents, up from 27% the previous year; smaller organizations are flat at 22%.
  - About two in ten report reaching scaling with software-coding agents.
  - Caveat: these came through search summaries because the PDF returned 503. The 23% figure also matches McKinsey's November 2025 survey, so the report writer should confirm which year it belongs to.
  — [McKinsey](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai); [McKinsey 2026 PDF](https://www.mckinsey.com/~/media/mckinsey/business%20functions/quantumblack/our%20insights/the%20state%20of%20ai/the-state-of-ai-in-2026-on-the-road-to-roi.pdf)
- **McKinsey via Forbes (March 2026):** only about 10% of enterprise functions use AI agents. — [Forbes](https://www.forbes.com/sites/josipamajic/2026/03/22/10-of-enterprise-functions-use-ai-agents-mckinsey-finds/)
- **KPMG Global AI Pulse Q2 2026:**
  - Organizations in the "driving adoption" phase rose from 13% to 22% in one quarter.
  - About one-quarter have *scaled back* AI deployments, and a similar share have *delayed or paused* them when costs outweighed expected value.
  - Few organizations report established ROI.
  — [KPMG Q2 2026](https://kpmg.com/se/en/insights/ai/global-ai-pulse-Q2-2026.html)
- **KPMG Q3 2026** (n>2,100, 20 countries, July 23 to August 26, 2026): 86% of organizations with established ROI run a formal cross-functional or enterprise-wide AI management layer, versus 31% of experimenting organizations. — [KPMG AI Pulse](https://kpmg.com/xx/en/our-insights/ai-and-technology/ai-pulse.html)
- **Gartner:**
  - Predicts more than 40% of agentic AI projects will be canceled by the end of 2027 (June 2025 prediction).
  - A January 2025 poll (n=3,412 webinar attendees) found 19% had made significant investment in agentic AI.
  - A 2026 figure attributed to Gartner in secondary coverage: only 17% have deployed agents, while more than 60% expect to within two years.
  - An aggregator cites Gartner at "80% of enterprises have at least one production app embedding an agent (Q1 2026)". **Unverified, and it conflicts with the 17% figure.**
  — [Gartner press release](https://www.gartner.com/en/newsroom/press-releases/2025-06-25-gartner-predicts-over-40-percent-of-agentic-ai-projects-will-be-canceled-by-end-of-2027); [Forbes, Jul 2026](https://www.forbes.com/sites/robertszczerba/2026/07/07/why-40-of-agentic-ai-projects-may-be-canceled-by-2027/); [paul-okhrem.com aggregator](https://paul-okhrem.com/enterprise-ai-agents-statistics-2026/)
- **Vendor surveys, inflated by selection bias:**
  - LangChain State of Agent Engineering (late 2025): 57.3% of surveyed teams run agents in production. Respondents are LangChain users.
  - CrewAI 2026 report (February 2026): 65% say their enterprises already use AI agents.
  — [Digital Commerce 360 on CrewAI](https://www.digitalcommerce360.com/2026/02/11/survey-enterprises-ai-agents-crewai-report/); [prefactor aggregator](https://prefactor.tech/learn/ai-agent-adoption-statistics)

### Inferences
- A realistic picture for mid-2026: about 20–25% of enterprises scale agents somewhere (about 40% of large firms). Coding agents are the most common scaled use. Customer service and IT operations are commonly cited next, but I did not get a primary source ranking use cases in this session.
- KPMG's finding that about a quarter scaled back on cost grounds, together with Ramp's willingness-to-pay ceiling, suggests *inference economics* is now the binding constraint, more than capability alone.

### Gaps
- The PDFs for the KPMG Q2 2026 detail (agent counts, use cases) and the McKinsey 2026 report could not be parsed or fetched. The following were not retrieved: Deloitte, PwC agent survey 2026, and LangChain's 2026 update.

## 4. Consumer signals (agentic browsers and features)

### Takeaway
Standalone consumer agent products are the weakest category. OpenAI shut down its Atlas browser after about 9 months and folded agentic browsing into ChatGPT/Codex. Consumer agent demand seems to live *inside* existing high-retention surfaces (ChatGPT, Codex) rather than in new standalone apps.

### Cited Findings
- **Atlas:** launched October 21, 2025, and discontinued July 9, 2026. Its agentic browsing moved into "ChatGPT Work", a GPT-5.6-powered agent for multi-step office tasks such as spreadsheets, slide decks, and documents. — [PPC Land](https://ppc.land/openai-kills-atlas-browser-folds-it-into-new-chatgpt-work-agent/)
- **Atlas vs. Comet (May 2026, HUMAN Security data):** Perplexity Comet had about 47% of *measured agentic web traffic*, and Atlas had 20.3%. — [PPC Land](https://ppc.land/openai-kills-atlas-browser-folds-it-into-new-chatgpt-work-agent/)
- **Atlas MAU:** about 11M in Q1 2026 (from a lower-quality statistics site; a date conflict also exists, with "August 9" given there versus "July 9" at PPC Land). — [Presenc AI](https://presenc.ai/research/atlas-usage-statistics-2026)

### Inferences
- The "agent as a separate app" consumer thesis shows hype without retention. The "agent as a mode in an app people already use daily" thesis has distribution.

### Gaps
- I found no reliable 2026 weekly active user or retention data for the ChatGPT agent mode, Manus, Genspark, or Comet. Sensor Tower and Appfigures rankings and the a16z consumer top-100 (2026) were not retrieved.

## 5. Developer signals (Stack Overflow, GitHub, MCP ecosystem)

### Takeaway
Developers use AI coding tools almost universally, but trust is falling. Agent use is growing yet still a minority practice as of the 2025 survey. The MCP ecosystem is large but inflated: about 31K unique servers against claims of more than 100K entries.

### Cited Findings
- **Stack Overflow 2025 survey (published July 2025).** Many 2026 articles *mislabel these as "2026" results.*
  - 84% use or plan to use AI tools.
  - Trust in AI accuracy fell to 29% (from 40%), and only about 3% "highly trust" it. 46% actively distrust it.
  - 66% cite "almost right, but not quite" as the top frustration.
  - About 52% don't use agents or stick to simpler tools, and 38% have no plans to adopt agents.
  - About 23% use agents regularly (The New Stack).
  — [Stack Overflow 2025 AI section](https://survey.stackoverflow.co/2025/ai); [The New Stack](https://thenewstack.io/23-of-devs-regularly-use-ai-agents-per-stack-overflow-survey/); [byteiota, mislabelled 2026](https://byteiota.com/stack-overflow-dev-survey-2026-ai-at-84-trust-at-3/)
- **Stack Overflow 2026 survey:** opened June 23, 2026, and Stack Overflow's framing is that agent usage has roughly doubled. I found no published 2026 results. — [Stack Overflow blog](https://stackoverflow.blog/2026/06/23/the-2026-developer-survey-is-now-open-for-human-developers-only/); [Stack Overflow trust-gap post, Feb 2026](https://stackoverflow.blog/2026/02/18/closing-the-developer-ai-trust-gap/)
- **MCP ecosystem:**
  - A deduplicated census of the public registry (September 14, 2026) found 101,219 entries but only 31,309 unique servers. Entry counts overstate the ecosystem about 3.2x.
  - About 23% (roughly one in five) of servers have no source repository, and three hosts front 14.5% of the registry.
  - The official MCP Registry had 9,652 distinct servers in July 2026.
  — [DEV Community census](https://dev.to/leroy_jenkins_951c84b2838/i-counted-the-mcp-registry-101219-entries-are-31309-servers-and-23-have-no-source-repo-33a6); [bex.co](https://bex.co/blog/2026/07/10/mcp-registry-discoverability-trust)

### Inferences
- Supply of MCP servers is huge, but server count measures developer experimentation, not usage. Many servers are likely thin wrappers or duplicates.
- Adoption is high while trust is low. This fits "augmentation with human review" as the dominant developer mode, with fully autonomous agents limited to verifiable tasks.

### Gaps
- GitHub Octoverse 2025/2026 figures on agent repositories and star growth were not retrieved.

## 6. Counter-evidence: reliability gaps, tourist usage, and failed deployments

### Takeaway
Hype without retention shows most clearly in consumer vibe-coding and app-builder tools (traffic falls and high churn despite ARR headlines), standalone agent browsers (Atlas shut down), and enterprise pilots (a quarter scaled back; Gartner expects 40%+ cancellations). Coding agents for professional developers show the strongest retention-type signals: repeat token use, rising paid adoption, and enterprise spend.

### Cited Findings
- **Vibe-coding traffic:** Barclays traffic data through September 2025 showed Lovable down about 40% from its June 2025 peak, v0 down 64% since May, and Bolt.new down 27%. Bolt's CEO said, "The churn rate for everyone is really high." — [TestSprite](https://www.testsprite.com/blog/beyond-the-hype-why-vibe-coding-leaders-are-facing-a-retention-crisis); [Final Round AI](https://www.finalroundai.com/blog/vibe-coding-bubble)
- **Fixing AI code:** in a 2025 Fastly survey of about 800 developers, about 95% said they spend extra time fixing AI-generated code. — [TestSprite, citing Fastly](https://www.testsprite.com/blog/beyond-the-hype-why-vibe-coding-leaders-are-facing-a-retention-crisis)
- **Cost pullback and cancellations:** KPMG Q2 2026 found about 25% scaled back and about 25% delayed or paused deployments. Gartner predicts more than 40% of agentic projects will be canceled by 2027. — [KPMG](https://kpmg.com/se/en/insights/ai/global-ai-pulse-Q2-2026.html); [Gartner](https://www.gartner.com/en/newsroom/press-releases/2025-06-25-gartner-predicts-over-40-percent-of-agentic-ai-projects-will-be-canceled-by-end-of-2027)
- **Willingness-to-pay ceiling:** Ramp, August 2026. — [Ramp](https://ramp.com/data/ai-index-august-2026)
- **Benchmark reliability:** these are the original papers, not re-read this session.
  - TheAgentCompany (December 2024): the best agent autonomously completed only about a quarter of simulated workplace tasks. — [arXiv 2412.14161](https://arxiv.org/abs/2412.14161)
  - τ-bench (June 2024): pass^k consistency drops sharply over repeated trials in retail and airline customer-service tasks. — [arXiv 2406.12045](https://arxiv.org/abs/2406.12045)

### Inferences
- Rough ranking of genuine pull, strongest first:
  1. Professional coding agents (Claude Code, Cursor, Codex, Cline/Kilo). Supported by revenue, tokens, Ramp, and McKinsey.
  2. Persistent general-purpose agents on open models (Hermes). Strong but very new token spike.
  3. Office and document agents inside incumbent apps (ChatGPT Work, Codex outside coding with 1M+ users, Claude Cowork). Early.
  4. Enterprise workflow agents (customer service, IT, sales automation). Real but patchy, and cost-constrained.
  5. Consumer vibe-coding app builders. High ARR but high churn and tourist usage.
  6. Standalone consumer agent browsers. Weakest; Atlas shut down.
- Lovable ARR growing to about $400M while traffic fell suggests revenue is shifting toward fewer, heavier and more corporate users. Without churn data, its ARR quality cannot be judged.

### Gaps
- No 2026 benchmark updates were retrieved, such as TheAgentCompany or τ²-bench scores for current frontier models, so the capability gap may be smaller now than the 2024 papers show.
- No hard 2026 retention cohorts were found for any agent product.
- Traffic data for vibe-coding apps after 2025 was not found.

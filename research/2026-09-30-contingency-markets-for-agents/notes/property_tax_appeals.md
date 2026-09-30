# Property Tax Appeals (US residential + commercial) as a market for outcome-priced AI agents

Research date: 2026-09-30. Labels: **[confirmed]** = primary/government source or company's own filing/press release; **[reported]** = press/trade reporting; **[aggregator]** = competitor blog, SEO comparison site, or search-snippet summary not verified against primary. Session note: about 26 tool calls. Several primary PDFs (NYC Tax Commission 2025 Annual Report, CCAO Berry report press release) could not be parsed in this environment. Figures from them come only from search snippets and are flagged.

## Q1. Market size: tax levied, appeal volumes, success rates, share appealing, reduction sizes

### Takeaway
The underlying pool is large: about $797B in US state and local property tax in 2024. Appeal volume is concentrated in a few mass-appeal jurisdictions. Harris County, TX had 391k protests in 2025, Travis County, TX 205k, Cook County, IL BOR 290k, and NYC about 57k applications covering 259k lots. Harris shows the key dynamic: filings shifted from DIY to agents in 2025, while agent success rates collapsed.

### Cited Findings
**US levy**
- US state and local property tax revenue was **$797B in 2024**, up 8.2% year over year and about 38% of state/local tax revenue (Census Quarterly Summary of State & Local Taxes, via NAHB, March 2025) **[reported, citing Census]** — [NAHB Eye on Housing / NAHB blog, 2025-03](https://www.nahb.org/blog/2025/03/state-local-property-tax-collection-2024)

**Texas: Harris County (HCAD)**, 2017–2025 per HCAD data compiled by Square Deal (a competing tax-protest firm; page last updated 2026-04-13) **[aggregator, HCAD-sourced]** — [Square Deal Harris protest data](https://blog.squaredeal.tax/texas/harris-county-protest-data/)
- Total protests: 300,212 (2021), 307,661 (2022), 354,514 (2023), 348,052 (2024), **391,455 (2025)**.
- Agent-filed protests: 81,450 (2023), 132,813 (2024), **312,358 (2025)**. The DIY share fell from 75% (2023) to **22% (2025)**.
- Overall success rate: 84% (2021), 74% (2022), then **29% (2023), 25% (2024), 29% (2025)**.
- Success by filer type in 2025: **DIY 71% vs agent 16%**. Median reduction in 2025: DIY $20,640 vs agent $13,202 of market value. In 2024: DIY 23% / agent 27%.
- Protests as a share of single-family homes rose from 24% (2017) to **35% (2025)**.

**Texas: Travis County (TCAD)**
- **About 205,000 protests in 2025**, up from 181,509 in 2024. **Agents filed about 90%**, and about 72% were filed electronically. Per Chief Appraiser Leana Mann to the TCAD board, 2025 **[reported]** — [Austin Bulldog, 2025](https://theaustinbulldog.org/205000-property-value-protests-filed/)

**Texas: statewide**
- I found no statewide 2025 protest total. The Comptroller publishes ARB survey reports but no headline count surfaced — [Texas Comptroller ARB page](https://comptroller.texas.gov/taxes/property-tax/arb/)

**Cook County, IL**
- The Board of Review processed a **record 290,533 appeals for tax year 2025** **[aggregator: search-snippet summary of BOR/WTTW coverage; primary BOR table not fetched]** — [Cook County BOR news](https://www.cookcountyboardofreview.com/news-0)
- About **one-third of the county files appeals annually**. Appeal rates run 53–92% in some northeast-side communities vs about 5% elsewhere. Just over **56% of appeals get a decreased assessment and 43.4% see no change** (WTTW, 2025-11-25) **[reported]** — [WTTW 2025-11-25](https://news.wttw.com/2025/11/25/cook-county-board-review-reopen-2025-property-tax-appeals-window)
- Conflicting figure: **39.4% of 2024 BOR appeals won a reduction, vs 28.4% in 2022** **[aggregator; law-firm blog, unverified]** — [cookcountytaxappeal.com](https://www.cookcountytaxappeal.com/property-tax-appeal-blogs/property-tax-appeal-results-savings-reduction-rates-2025). This conflicts with the 56% reported by WTTW. The difference is probably one of denominator or year.
- Assessor Kaegi: in the Chicago reassessment the BOR **cut commercial values by nearly 20% vs about 1% for residential**. The median homeowner's bill rose a record 16.7% as commercial value fell by about $500M, shifting burden to homes (WTTW, 2025-11-25) **[reported]** — [WTTW](https://news.wttw.com/2025/11/25/cook-county-board-review-reopen-2025-property-tax-appeals-window)
- The BOR reopened the 2025 appeal window, citing "unprecedented circumstances" (WTTW, 2025-11-25) **[reported]** — same source.

**New York City**
- The Tax Commission received **57,302 applications covering about 259,000 tax lots in 2024** **[aggregator: search snippet of Tax Commission annual report; PDF not parseable here]** — [NYC Tax Commission 2025 Annual Report](https://www.nyc.gov/assets/taxcommission/downloads/pdf/2025%20Tax%20Commission%20Annual%20Report.pdf)
- For 2025/26 the Tax Commission raised its "hear-first" threshold for Class 4 lots from $55M to $250M AV. That covers 42 Manhattan lots (41 offices and 1 hotel) **[reported; law firm]** — [Rosenberg & Estis, 2025-01](https://www.rosenbergestis.com/blog/2025/01/comprehensive-2025-nyc-tax-commission-updates-and-filing-requirements/)

**California**
- LA County had **over 27,000 open assessment-appeal applications as of 2025-04-25**. It had closed about 80% of the backlog dating to 2022, and cut time-to-schedule from about 10 to about 5 months. From a BOE working-group meeting **[aggregator: Citizen Portal AI summary of BOE meeting]** — [Citizen Portal](https://citizenportal.ai/articles/6251653/California/Executive/Other-State-Agencies/Board-of-Equalization/Los-Angeles-County-says-tech-training-cut-appeals-backlog-and-boosted-online-filings); primary: [BOE meeting docs 2025](https://boe.ca.gov/meetings/pdf/2025/202504-BWG-AAB-CATA.pdf)

**Florida**
- Orange County VAB 2025 final report: **3,416 petitions** **[aggregator: Citizen Portal summary]** — [Citizen Portal](https://citizenportal.ai/articles/10035578/Florida/Orange-County/VAB-accepts-final-2025-report-petitions-total-revised-to-3416-and-fees-updated). I found no statewide figure.

**Share of homeowners who appeal**
- An Ownwell survey of 2,500 homeowners (2025) found that 74% worry about rising property taxes and **22% have ever appealed** **[confirmed as company survey; vendor-sponsored]** — [Ownwell PR Newswire, 2026-02](https://www.prnewswire.com/news-releases/ownwell-raises-50m-launches-national-service-to-streamline-property-tax-appeals-and-make-home-ownership-more-affordable-302692103.html)

**Typical reduction sizes**
- Ownwell reports **average annual savings of $774 per customer** and an 86% success rate across more than 1M appeals (2026-02) **[confirmed as company claim; unaudited]** — [PR Newswire](https://www.prnewswire.com/news-releases/ownwell-raises-50m-launches-national-service-to-streamline-property-tax-appeals-and-make-home-ownership-more-affordable-302692103.html)
- In Harris County, median market-value reductions ran about $13k (agent) to about $21k (DIY) in 2025. At about a 2% effective rate, this implies roughly $260–$410 per year in tax savings. This is my inference, not a source figure. [Square Deal](https://blog.squaredeal.tax/texas/harris-county-protest-data/) **[aggregator]**

### Inferences
- The addressable fee pool is a small slice of the $797B levy. If about 1% of the levy is contested and reduced by about 5–10%, total annual savings would be in the low billions. At 25–50% contingency, that is a fee pool in the high hundreds of millions to about $1–2B. This is not sourced; treat it as an order-of-magnitude estimate only. Commercial likely dominates the dollars: Ryan alone had more than $700M in revenue across all tax lines (see Q3).
- Harris 2025 is the canary for AI-scale filing. Agent filings roughly 2.4x'd while agent success fell to 16%, against 71% for DIY filers. The mechanism looks like this: volume filers (plausibly including Ownwell's 200k+ "never protested before" Texas properties) file indiscriminately, and ARBs deny weak cases or settle them lower. Blanket filing dilutes win rates. The agent's edge must come from case selection and evidence quality, not from filing throughput.
- Appeals are also highly concentrated geographically: from 5% to 92% participation within Cook County. Penetration headroom exists outside Texas, Chicago and NYC.

### Gaps
- Statewide Texas protest counts and ARB outcome data for 2025 (the Comptroller publishes ARB reports, but I did not locate totals).
- An NYC 2025 application count and offer rate. The PDF was unreadable here.
- A clean Cook County BOR residential vs commercial success-rate split, and the Assessor-stage appeal count.
- Statewide appeal volumes for Florida, California and New Jersey.
- The share of commercial owners who appeal. I found no national figure.

## Q2. Fees: residential contingency vs commercial consultants vs DIY flat fee; multi-year billing

### Takeaway
Residential contingency pricing now runs from **25% (Ownwell) to 50% (O'Connor) of first-year savings**, with NJ attorneys at 25–40%. AI "packet" tools have already pushed DIY prices to **$45–$149 flat**. Incumbents bill **per protested year only**; they do not bill for multi-year savings. The exception is where statute locks a value in: NJ's Freeze Act and California Prop 8 restorations.

### Cited Findings
- **Ownwell charges 25% of savings**, which it calls "industry-low." It also launched a "National Appeals Packet": AI-generated, ready-to-file documents for jurisdictions it does not serve directly. The price was not disclosed (2026-02) **[confirmed; company PR]** — [PR Newswire](https://www.prnewswire.com/news-releases/ownwell-raises-50m-launches-national-service-to-streamline-property-tax-appeals-and-make-home-ownership-more-affordable-302692103.html)
- **O'Connor charges a 50% contingency** on the taxes saved for that tax year. There are no fees without a reduction. It claims it saved clients more than $213M in 2025 **[confirmed for the fee term, from the company site; the savings figure is aggregator/search]** — [O'Connor FAQ](https://www.poconnor.com/frequently-asked-questions); [O'Connor terms PDF](https://www.poconnor.com/wp-content/uploads/2023/06/Our-Terms.pdf)
- O'Connor bills annually: "If we reduce your taxes, the fee is applicable for the protested year only." The contract auto-continues until cancelled, with a protest each year **[confirmed; company site, accessed 2026-09-30]** — [O'Connor: fees current and future years](https://www.poconnor.com/question/fees-billed-current-future-years/)
- NJ residential attorneys typically charge **25–40% of one year's savings**, sometimes with a minimum of about $250 **[aggregator; SEO guide]** — [AppealDesk NJ guide](https://www.appealdesk.com/blog/new-jersey-property-tax-appeal)
- Flat-fee AI DIY tools (2025–2026) **[confirmed as vendor-listed prices; small, unverified traction]**:
  - AppealDesk: $49 nationwide packet (Nashville; founder Travis Bunn) — [AppealDesk](https://www.appealdesk.com/compare/appealdesk-vs-diy)
  - ProtestMax.ai: $45 (GA, Cook County IL, Philadelphia, TX) — [ProtestMax.ai](https://protestmax.ai/)
  - TaxFight.ai: $49 (TX) — [TaxFight.ai](https://www.taxfight.ai/)
  - taxappeal.app: $149 flat (IL) — [taxappeal.app](https://taxappeal.app/)
  - TaxAppealKit: $79.99 — [TaxAppealKit](https://www.taxappealkit.com/)
- Texas consultants may use "contingency, flat, minimum, portfolio or blended" pricing **[aggregator summary of TDLR materials]** — [TDLR PTC FAQ](https://www.tdlr.texas.gov/ptc/ptcfaq.htm)
- **NJ Freeze Act:** a county board or Tax Court judgment binds the assessor for the appeal year **plus the two succeeding years**, barring revaluation or a value-changing event. This makes 3-year savings a structural feature of NJ **[confirmed; statute and regulation]** — [N.J.A.C. 18:12A-1.13 (LII)](https://www.law.cornell.edu/regulations/new-jersey/N-J-A-C-18-12A-1-13); [NJ Courts Freeze Act form](https://www.njcourts.gov/sites/default/files/forms/11016_freezeact.pdf)

### Inferences
- Fee compression is already under way. The top of the residential market fell from 50% (O'Connor) to 25% (Ownwell), and a $45–$149 flat-fee floor has appeared. An AI-native agent that prices below 25%, or as "flat fee plus small success fee," is plausible. But the median residential saving is only a few hundred dollars per year, so per-case gross margin is thin: 25% of $774 is about $194. CAC and ARB-hearing labor are the binding constraints.
- Billing only the protested year keeps revenue recurring, because the customer re-enrolls every year. It also means a single win does not capture durable value. Where statutes create multi-year effects (NJ Freeze Act, Texas homestead 10% cap interactions, California Prop 13 base-year restoration), an agent could justify multi-year pricing, but incumbents do not appear to do so.

### Gaps
- Commercial consultant fee levels: typical commercial contingency (anecdotally 20–35%) and portfolio pricing. I found no sourced 2024–2026 figure.
- Ownwell's National Appeals Packet price.
- Hometown Tax / other mid-size Texas players' pricing (not researched within budget).

## Q3. Incumbents and entrants

### Takeaway
Ryan LLC dominates commercial after a roll-up: Marvin F. Poer in 2022, RETC in 2023, Popp Hutcheson in 2024. Residential has consolidated around O'Connor, a Texas volume incumbent, and Ownwell, a VC-backed tech player with about $74M raised. A long tail of $45–$149 AI packet tools appeared in 2025–2026.

### Cited Findings
- **Ryan LLC:**
  - Acquired **Marvin F. Poer & Co., "the second-largest property tax consulting firm in the US,"** adding 186 staff and 12 locations (2022-02) **[confirmed; press release]** — [Ryan press release](https://ryan.com/about-ryan/press-room/marvin-f-poer-and-company-acquisition/); [D Magazine 2022](https://www.dmagazine.com/business-economy/2022/02/property-tax-consulting-firm-ryan-acquires-biggest-competitor/)
  - Acquired the consulting business of **Popp Hutcheson** (founded 1983; represented more than $70B in Texas commercial value) (2024-03-06) **[confirmed; BusinessWire]** — [BusinessWire](https://www.businesswire.com/news/home/20240306035672/en/Ryan-Strengthens-Property-Tax-Practice-with-Acquisition-of-Consulting-Business-of-Popp-Hutcheson-PLLC)
  - Acquired **RETC** (2023-04) **[confirmed]** — [BusinessWire](https://www.businesswire.com/news/home/20230411005178/en/Ryan-Expands-Property-Tax-Team-with-Acquisition-of-RETC)
  - Firm-wide revenue was about $715M in 2021 and over $700M in 2022, across all tax lines, not property-only **[aggregator; Wikipedia]** — [Wikipedia: Ryan LLC](https://en.wikipedia.org/wiki/Ryan_LLC)
- **Ownwell** (Austin; founded 2020; CEO Colton Pace):
  - **$50M Series B (2026-02-25): $30M equity** co-led by Alpha Edison and Mercato Partners, with Intuit Ventures, Left Lane, First Round and others participating, plus **$20M debt** from Western Alliance Bank. Total raised is about $74M; about $54M of it is equity.
  - Full service in 7 states: CA, FL, GA, IL, NY, TX, WA.
  - Metrics: more than $400M saved, more than 1M appeals, 86% success rate, $774 average annual savings, and more than 200k Texas properties in 2024 that had never protested before.
  - **[confirmed as company claims]** — [PR Newswire](https://www.prnewswire.com/news-releases/ownwell-raises-50m-launches-national-service-to-streamline-property-tax-appeals-and-make-home-ownership-more-affordable-302692103.html); [Inman 2026-02-25](https://www.inman.com/2026/02/25/ownwell-raises-50m-to-grow-its-property-tax-appeal-fintech/); [HousingWire](https://www.housingwire.com/articles/ownwell-property-tax-appeal-funding/)
  - Prior $30M raise **[reported]** — [Crunchbase News](https://news.crunchbase.com/venture/ownwell-raise-lower-homeowner-property-tax/)
- **O'Connor** (Houston): 50% contingency; claims more than $213M saved for clients in 2025 **[aggregator for the savings number]** — [O'Connor](https://www.poconnor.com/)
- **AI-native entrants, 2025–2026:** AppealDesk, ProtestMax.ai, TaxFight.ai, taxappeal.app ("Property Tax Appeal AI") and TaxAppealKit are all flat-fee packet generators. None showed disclosed funding in search results **[confirmed as existing sites; traction unknown]** — see links in Q2.

### Inferences
- There are two distinct games:
  - **Commercial** is a relationship-driven, attorney-heavy, roll-up market (Ryan). Complexity per case is high (income approach, litigation). AI there is more likely a copilot sold to or built by incumbents than a displacing contingency agent.
  - **Residential** is a CAC plus volume game. Ownwell's 25% rate and the $49 packets show AI is already compressing price.
- Ownwell's own "National Appeals Packet" signals that the incumbent tech player is itself moving to AI-generated DIY in its long-tail states. That weakens the whitespace for a pure AI-native entrant.

### Gaps
- Hometown Tax Group, Texas Tax Protest, Five Stone, Tax Ease and other Texas players (not covered).
- Revenue for Ryan's property-tax segment and for Ownwell.
- Whether any 2025–2026 AI-native entrant raised venture funding. None surfaced.
- County-built DIY tools: for example, the Cook County Assessor/BOR online filing and HCAD iFile. I did not research their uptake.

## Q4. Process, verification and billing mechanics

### Takeaway
Deadlines are short and statutory; Texas is typically May 15. Decisions come from a quasi-neutral government body (ARB, BOR, Tax Commission, AAB, VAB, CBT) and are written into the tax roll, and savings show up on the tax bill. That gives the market an unusually clean, third-party outcome record for contingency billing. Latency ranges from weeks (Texas informal) to years (LA AAB, NYC Tax Commission, NJ Tax Court).

### Cited Findings
- **Texas:**
  - The protest deadline is generally **May 15**, or 30 days after the notice (Harris 2025 coverage) **[reported]** — [Houston Public Media, 2025-05-08](https://www.houstonpublicmedia.org/articles/news/harris-county/2025/05/08/520874/harris-county-property-owners-have-until-may-15-to-protest-2025-appraisal-values/)
  - The process runs from an informal settlement with the appraisal district to a formal ARB hearing, then to binding arbitration, SOAH or district court — [Texas Comptroller: Protests & Appeals](https://comptroller.texas.gov/taxes/property-tax/protests/) **[confirmed]**
  - Agents need a statutory Appointment of Agent form (Form 50-162) **[confirmed; Comptroller/CAD procedures]** — [Dallas CAD protest process](https://www.dallascad.org/forms/protest_process.pdf)
  - 2025 legislation:
    - **SB 1163:** a guaranteed postponement for owner or agent, and relief for overlapping hearings.
    - **SB 2063:** bars market-value rebuttal in unequal-appraisal-only cases.
    - **SB 2452:** bans compensation of chief appraisers tied to rising values.
    - **[aggregator; policy-group summaries]** — [Texas Policy Research SB 2452](https://www.texaspolicyresearch.com/bills/89th-legislature-sb-2452/); [TX 89th Leg. effective dates](https://lrl.texas.gov/sessions/effDates/billsEffective89.cfm)
- **Cook County:**
  - A two-stage appeal: the Assessor, then the Board of Review, with further appeal to PTAB or circuit court. The BOR opens township-by-township windows (2026 season under way) **[confirmed]** — [BOR dates & deadlines](https://www.cookcountyboardofreview.com/dates-and-deadlines); [BOR 2026 second group](https://www.cookcountyboardofreview.com/news/cook-county-board-review-opens-second-group-townships-2026-property-tax-appeal-season)
  - **Corporations must be represented by counsel before the BOR** for commercial appeals **[confirmed; BOR site]** — [BOR Commercial](https://www.cookcountyboardofreview.com/assessment-appeals/commercial)
- **NYC:** Tax Commission applications are due in the tentative-roll window (Jan to Mar 1/15). Small Claims Assessment Review (SCAR) is available for eligible properties **[confirmed; NYC forms]** — [TC600 How to appeal (2026)](https://www.nyc.gov/assets/taxcommission/downloads/pdf/tc600-2026.pdf); [TC708 SCAR](https://www.nyc.gov/assets/taxcommission/downloads/pdf/tc708-2026.pdf)
- **New Jersey:**
  - Form A-1 petition to the County Board of Taxation, **due April 1** (May 1 in revaluation counties). The filing fee is $5–$25.
  - A CBT generally cannot raise a residential homestead assessment on appeal **[aggregator]** — [AppealDesk NJ](https://www.appealdesk.com/blog/new-jersey-property-tax-appeal); form: [NJ Form A-1](https://nj.gov/treasury/taxation/pdf/other_forms/lpt/petappl.pdf)
- **California (LA):** average time-to-schedule is about 5 months, down from about 10. More than 27k applications were open as of 2025-04 **[aggregator]** — [Citizen Portal/BOE](https://citizenportal.ai/articles/6251653/California/Executive/Other-State-Agencies/Board-of-Equalization/Los-Angeles-County-says-tech-training-cut-appeals-backlog-and-boosted-online-filings)
- **Florida:** VAB petition (DR-486) to the county Value Adjustment Board **[confirmed; FL DOR form]** — [FL DOR DR-486](https://floridarevenue.com/property/documents/dr486.pdf)

### Inferences
- **Verification:** the final decision (ARB order, BOR result, stipulation) is a public record, and the reduction appears on the certified roll and the tax bill. That lets billing be computed mechanically: (original value − final value) × tax rate. This is exactly how O'Connor and Ownwell bill, and it is close to ideal for outcome pricing. Caveats:
  - Tax rates are set after hearings, in Texas in the fall. The dollar saving is therefore known only when bills issue (October–January in Texas).
  - Informal settlements are negotiated with the appraiser, who is not neutral. The ARB is nominally independent but is funded through the CAD.
- **Latency:** Texas runs from May filing to a summer decision and an autumn bill, so under 6 months, which gives fast feedback. Cook County runs about 6–12 months. NYC, LA and NJ Tax Court can take 1–3+ years for commercial cases.

### Gaps
- Official median time from filing to decision for each jurisdiction.
- Evidence rules for AI-generated comps or reports. I found no rule addressing AI-generated evidence specifically.

## Q5. Regulatory risk

### Takeaway
There are three distinct constraints:
- **Texas** requires TDLR registration for paid consultants, which a software-only agent would need to route through registered humans.
- **New Jersey** and Cook County commercial effectively **require attorneys**, because non-attorney contingency appeals in NJ are unauthorized practice of law.
- Mass-filing is drawing structural pushback: collapsing agent win rates in Harris, and assessors defending commercial values at the BOR.

### Cited Findings
- **Texas TDLR:**
  - Anyone performing property tax consulting for compensation must register. Requirements: 40 hours of education, sponsorship by a Senior PTC, a $50 fee and a passing exam score of at least 70%.
  - Renewal requires 24 hours of continuing education.
  - Those who assist or testify for others are covered if more than 50% of their time or income comes from it.
  - **[confirmed; TDLR]** — [TDLR PTC FAQ](https://www.tdlr.texas.gov/ptc/ptcfaq.htm); [TDLR PTC Laws & Rules](https://www.tdlr.texas.gov/ptc/laws-rules.htm)
- **New Jersey:** the NJ Supreme Court held that a non-attorney who contracts to procure a tax reduction requiring a county-board appeal is engaged in the **unauthorized practice of law** **[confirmed via NJ ACPE opinion summarizing precedent]** — [NJ ACPE 1992 opinion (Justia)](https://law.justia.com/cases/new-jersey/advisory-committee-on-professional-ethics/1992/cua25-1.html)
- **Cook County:** corporations must be represented by counsel at the BOR **[confirmed]** — [BOR Commercial](https://www.cookcountyboardofreview.com/assessment-appeals/commercial)
- **Assessor pushback:**
  - For 2025, the Cook County Assessor created a team to defend assessments at BOR hearings. It targeted 14 large commercial (mostly data-center) properties; 4 received no reduction **[confirmed; CCAO press]** — [CCAO: defends data center assessments](https://www.cookcountyassessoril.gov/news/assessors-office-defends-higher-data-center-assessments-appeal-hearings)
  - Academic and political critique holds that appeals worsen regressivity. Berry's work found about $2.2B shifted from high- to low-value properties in Chicago. A 2025-09-08 University of Chicago study credited reforms with saving homeowners $1.9B **[reported / confirmed press release]** — [ProPublica](https://www.propublica.org/article/cook-county-property-tax-shift-regressive-assessments); [CCAO release on UChicago study, 2025-09](https://www.cookcountyassessoril.gov/news/cook-county-homeowners-saved-19-billion-under-fritz-kaegis-reforms-university-chicago-study); [UChicago Property Tax Project](https://propertytaxproject.uchicago.edu/related-research-2-3)
  - Robert Ross (2017) found appeals exacerbate regressivity in Cook County **[reported via search]** — [UChicago Property Tax Project](https://propertytaxproject.uchicago.edu/related-research-2-3)
- **Texas 2025 legislation:** SB 1163 expanded agent postponement rights, which is agent-friendly. I found no 2025 Texas bill restricting bulk agent filings **[aggregator]** — [TX effective dates](https://lrl.texas.gov/sessions/effDates/billsEffective89.cfm)

### Inferences
- An AI agent cannot legally "appear" on its own. In Texas it needs registered PTCs (or the owner self-files with an AI-prepared packet, which is the $49 model). In NJ and Cook County commercial it needs licensed attorneys. The realistic structures are:
  - an AI-leveraged licensed firm (Ownwell's model), or
  - an unlicensed DIY-packet tool that avoids "representation," the route every flat-fee entrant takes.
- The Harris 2025 data (agent success 16% vs DIY 71%) suggests ARBs may be triaging agent bulk filings harder. This is a risk to any high-volume AI filer's unit economics and reputation, and a likely trigger for future legislative or ARB procedural restrictions.
- Regressivity politics cut both ways. Agents democratizing residential appeals (Ownwell's "200k first-time protesters") can be framed as pro-equity. But assessors facing record volume (Harris 391k, Cook 290k) have strong incentives to push back.

### Gaps
- Explicit contingency-fee caps or disclosure rules by state (Texas Occupations Code ch. 1152 specifics, California, Florida). I found no sourced caps.
- Whether California or Florida require agent licensing. Not verified here.
- Any rule on AI-generated evidence at ARB, BOR or AAB. None found.
- Any 2025–2026 legislative proposals targeting bulk filers. None found.

## Q6. Frequency: reassessment cycles and recurring revenue

### Takeaway
- **Texas:** annual reappraisal and annual protests, the most recurring market. Harris protests rose every year to 391k in 2025.
- **Cook County:** a triennial rotation by region, but appeals are allowed every year.
- **NJ:** the Freeze Act locks wins for 3 years.
- **California:** Prop 13 means appeals are mainly Prop 8 decline-in-value cases, cyclical with the market.

Annual-appeal jurisdictions produce subscription-like contingency revenue. Multi-year-lock states produce lumpier, larger per-win revenue.

### Cited Findings
- Cook County has **a rolling triennial reassessment**. BOR appeals open by township group each year, including non-reassessment townships (2025 set a record even outside reassessment townships) **[confirmed on schedules; aggregator on the "record" framing]** — [BOR dates](https://www.cookcountyboardofreview.com/dates-and-deadlines); [BOR news](https://www.cookcountyboardofreview.com/news-0)
- The NJ Freeze Act binds the appeal year plus 2 years **[confirmed]** — [LII](https://www.law.cornell.edu/regulations/new-jersey/N-J-A-C-18-12A-1-13)
- O'Connor's annual auto-renewing protest contract, billed per protested year **[confirmed]** — [O'Connor](https://www.poconnor.com/question/fees-billed-current-future-years/)
- Harris protest volumes grew almost every year, from 251,852 in 2017 to 391,455 in 2025 **[aggregator, HCAD data]** — [Square Deal](https://blog.squaredeal.tax/texas/harris-county-protest-data/)

### Inferences
- Texas is the best-fit jurisdiction for recurring outcome pricing: annual notices, a May 15 deadline, fast decisions, and an existing culture of auto-renewing contingency contracts. Its downside is saturation: 90% agent share in Travis, and 80% of Harris filings made by agents in 2025.
- Florida (annual TRIM notices and VAB), Georgia (annual notices) and Washington state are similar annual markets. Ownwell already operates in all three.
- California (outside Prop 8 downturns) and NJ (Freeze Act) give lower frequency but higher-value wins. They suit commercial and attorney-partnered models more than a consumer subscription.

### Gaps
- An authoritative list of reassessment frequency by state. The Lincoln Institute's Significant Features of the Property Tax was not fetched.

## Q7. Verification profile scorecard for outcome-priced AI agents

### Takeaway
Property tax appeals score **high** on neutral verification, auditability and frequency (Texas), **medium** on latency, **low** on clawback exposure, and **medium-low** on attribution. AI is already collapsing fees on the residential DIY end. Durable margin sits in hearing representation, which is licensed or attorney-gated, and in commercial complexity.

### Scorecard
These are inferences from the findings above; scores run 1 (poor for outcome pricing) to 5 (ideal).

| Feature | Score | Rationale / evidence |
|---|---|---|
| **Neutral verifier** | 4 | The government body decides and the roll or bill records it. Caveat: informal settlements are negotiated with the counterparty appraiser, and ARBs and BORs have political and funding ties. Cook BOR commercial cuts of about 20% vs 1% residential show that the verifier is not value-neutral ([WTTW](https://news.wttw.com/2025/11/25/cook-county-board-review-reopen-2025-property-tax-appeals-window)). |
| **Feedback latency** | 3 (TX 4; NYC/NJ/LA commercial 2) | Texas decides in weeks to months. LA took about 5 months to schedule ([Citizen Portal](https://citizenportal.ai/articles/6251653/California/Executive/Other-State-Agencies/Board-of-Equalization/Los-Angeles-County-says-tech-training-cut-appeals-backlog-and-boosted-online-filings)). Tax Court or litigation takes years. |
| **Clawback exposure** | 4–5 (low exposure) | A reduction is final for the year. NJ CBTs generally cannot raise residential homestead assessments ([AppealDesk NJ](https://www.appealdesk.com/blog/new-jersey-property-tax-appeal)). The residual risk is refunds reversed on appeal by the taxing unit (rare). |
| **Buyer auditability** | 5 | Notice value, final value and tax rate are all public, so the customer can recompute the fee. Billing is formulaic (O'Connor: "protested year only"). |
| **Attribution** (would value have dropped anyway?) | 2–3 | This is the weakest point. DIY filers in Harris won 71% of the time in 2025 vs 16% for agents, and got larger reductions ([Square Deal](https://blog.squaredeal.tax/texas/harris-county-protest-data/)). Many reductions come from routine informal settlements the owner could get alone. There are no counterfactuals for market-wide value declines. Cook's 2025 reopening was driven by a systemic shift, not by case merit. |
| **Frequency** | 4 (TX, FL, GA) / 2 (NJ, CA) | Annual in Texas; the NJ Freeze Act locks 3 years; California is mostly downturn-driven. |
| **Stakes / liability** | 2 (low stakes, low liability) residential; 4 commercial | Residential savings average about $774 per year (Ownwell). The worst case is usually "no change," though some jurisdictions allow increases. Commercial involves millions, requires counsel (Cook County, NJ), and carries malpractice exposure. |
| **Will AI collapse the fee?** | Yes for residential packets; partially for representation | Contingency ran 50% (O'Connor), then 25% (Ownwell), then $45–$149 flat (AppealDesk, ProtestMax, TaxFight, taxappeal.app). Ownwell itself ships an AI packet. Hearing appearance is gated by TDLR registration (TX), attorney requirements (NJ, Cook commercial), and time, so representation fees persist longer. |

### Inferences: where AI agents can win
- **Best segment and jurisdiction:** outcome-priced residential representation in **Texas**, plus Georgia, Florida and Washington. Annual cycle, fast decisions, public auditable outcomes, and established contingency norms. To exploit the attribution problem, an agent should win on **selectivity and evidence quality**: file only where comps or condition evidence are strong, to escape the 16% agent win rate. Price it around 15–25% contingency, or low flat plus success fee, and route hearings through TDLR-registered staff. Saturation (90% agent share in Travis) and Ownwell's head start are the main risks.
- **Second-best:** small and mid commercial (under about $10M) via an AI-leveraged attorney or PTC firm, where Ryan-style firms under-serve. Higher fees per case, but attorney gating in NJ and Cook County and longer latency.
- **Weakest:** NJ and California residential (attorney requirement or low frequency), and pure DIY packets. Those have already collapsed to $45–$149, leaving little outcome-priced revenue.

### Gaps
- No independent audit of Ownwell's 86% success rate or of any AI entrant's outcomes.
- No data isolating AI-prepared vs human-prepared appeal outcomes.
- The 2025 Harris agent success rate of 16% comes from a competitor's analysis. It needs verification against raw HCAD ARB data.

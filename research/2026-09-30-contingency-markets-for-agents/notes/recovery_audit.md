# Recovery Audit / Profit Recovery as a Market for Outcome-Priced AI Agents

Research date: 2026-09-30. Evidence labels: **[confirmed]** = primary source (filing, government page, company press release); **[reported]** = trade press / reputable news; **[aggregator]** = vendor blog, review site, or data aggregator (Sacra, Tracxn, SEO blogs). About 22 tool calls were made. Several sub-questions stay open and are listed under Gaps.

---

## 1. Market size: overpayment rates, recovery audit industry size, freight audit volumes

### Takeaway
The leakage is real but thin. Benchmarks put duplicate or erroneous disbursements at about 0.8–2.0% of payments (APQC, by count of disbursements) and duplicates at about 0.1–0.5% of invoices (IOFM, via aggregators). Large pools of spend flow through audit firms: Cass processed about $36.4B of transportation spend in 2025, and PRGX says it has analyzed $2.3T of client spend. I found no reliable, sourced figure for the size of the whole recovery-audit industry.

### Cited Findings
- APQC Open Standards Benchmarking measures "percentage of total annual number of disbursements processed which are duplicate or erroneous payments" — [APQC measure page](https://www.apqc.org/resources/benchmarking/open-standards-benchmarking/measures/percentage-total-annual-number) **[confirmed that the measure exists]**. An aggregator summary of APQC 2024 data gives top performers 0.8%, median 1.5% and bottom performers 2.0% — [Transparent blog, n.d.](https://transparentglobal.com/blog/what-percentage-of-ap-spend-is-lost-to-duplicate-payments-industry-benchmarks/) **[aggregator; I did not see the APQC figures directly]**. Note that this metric counts disbursements, not dollars.
- IOFM is cited as saying companies "may lose up to 1.5% of outgoing cash flow to duplicate payments," and elsewhere that duplicates affect 0.1–0.5% of invoices — [ExpensePoint blog](https://www.expensepoint.com/blog/duplicate-payments/); [Billed AP stats 2026](https://billed.app/hub/statistics/accounts-payable-statistics/) **[aggregator; the two figures conflict and I could not verify either at IOFM]**.
- apexanalytix (a vendor) says "$2 million per billion in duplicates" (0.2% of spend) slips past normal ERP controls, and claims "$9B+ recovered annually" for clients — [apexanalytix overpayment prevention](https://www.apexanalytix.com/solutions/audit-recovery/overpayment-prevention/); [apexanalytix AP recovery audit](https://www.apexanalytix.com/solutions/audit-recovery/accounts-payable-recovery-audit/) **[aggregator/vendor claim, undated, accessed 2026-09-30]**.
- PRGX says it has "analyzed $2.3 trillion in client spend" and found "$2.3 billion in client revenue", with "10x average ROI" — [PRGX AP recovery audit guide](https://www.prgx.com/guides/ap-recovery-audit-services-guide/) **[vendor claim, undated]**. If both numbers cover the same base, recovery is about 0.1% of audited spend (my inference).
- Cass Information Systems FY2025 10-K: transportation dollar volume processed was $36.45B in 2025, up 0.9% from $36.11B in 2024 and down 5.7% from $38.29B in 2023. Cass attributes the decline to "the on-going freight recession and the impact of tariffs" — [Cass 10-K FY2025 (SEC)](https://www.sec.gov/Archives/edgar/data/708781/000070878126000010/cass-20251231.htm) **[confirmed, via search extraction of the 10-K]**. Cass also markets "~35 million commercial invoices and $37B in spend annually" — same source/[Cass site](https://www.cassinfo.com/freight-audit-payment/services/freight-invoice-management/) **[confirmed/vendor]**.
- Competitors' freight spend figures: Trax Technologies about $25B of transportation spend under management; Intelligent Audit serves "20% of Fortune 50" — [Sacra Loop profile, ~Mar 2026](https://sacra.com/c/loop/) **[aggregator]**.
- Healthcare: Medicare FFS RAC-identified overpayments were about $352.5M in FY2023 per the CMS Program Integrity Report to Congress — [CMS FY2023 Medicare & Medicaid Program Integrity Report](https://www.cms.gov/files/document/fy2023-medicare-and-medicaid-report-congress.pdf) **[confirmed source, figure from a search snippet; I did not verify the page number]**. Alaffia frames the target as "up to $570 billion" of annual US healthcare administrative waste — [BusinessWire, 2026-02-03](https://www.businesswire.com/news/home/20260203113559/en/Alaffia-Health-Raises-$55M-to-Tackle-Healthcares-$570-Billion-Waste-Problem) **[vendor framing, not an addressable market]**.
- The Medicare RAC Region 3/4/5 awards total $100.27M in contract value, with 4 bidders (awarded 2025-04-28) — [OrangeSlices AI](https://orangeslices.ai/cotiviti-beats-out-3-to-win-100m-cms-recovery-audit-contractor-rac-regions-3-4-5-contract/) **[reported, gov-contracting trade press]**.

### Inferences
- A rough, unsourced model: 0.1–0.2% of audited spend recovered × 20–30% contingency gives the firm about 2–6 bps of client spend. That fits PRGX being valued at only $195M in 2021 despite auditing trillions of spend.
- Medicare RAC is now a small pool. About $350M/yr of identified overpayments at a 9–12.5% fee is roughly $30–45M/yr of fee revenue across all regions (inference). It is a reputational wedge, not a big revenue pool.
- Freight audit is volume-rich, since Cass alone handles $36B+/yr. But much of the market is bundled "audit + payment" (float/payments revenue), not pure contingency.

### Gaps
- No credible, dated total-industry figure for recovery audit (AP/profit recovery). Market-research-report numbers were not found or checked. I found no reliable source.
- No sourced aggregate US figures for freight audit market size, telecom expense audit, or retail deduction volume (e.g. deductions as % of CPG gross sales). The Glimpse coverage did not quantify it.
- I did not directly read the APQC dollar-based duplicate-payment data.

---

## 2. Fees by sub-segment: contingency percentages and trends

### Takeaway
Contingency rates split into tiers by sub-segment: Medicare RAC about 9–12.5% (set in the bid), commercial AP recovery 20–40%, Amazon seller reimbursement about 25%, and freight audit mostly per-invoice or platform fees rather than contingency. AI-native entrants (Loop, Glimpse) mostly do not disclose pricing and appear to be moving toward platform/SaaS pricing, not contingency.

### Cited Findings
- **Commercial AP recovery audit:** contingency fees "typically range between 20 to 30 percent," with ranges of "20% and 40%". Broader scope and longer lookback push the percentage down, and narrow or hard scope pushes it up. Freight and VAT specialists quote different rates — [apexanalytix recovery-audit cost blog](https://www.apexanalytix.com/resources/blog/recovery-audit-cost/) (full page failed to load; claim from search extraction) and [invoicedataextraction.com](https://invoicedataextraction.com/blog/accounts-payable-recovery-audit) **[aggregator/vendor, 2025–2026]**.
- **Medicare RAC:** RAC contingency fees are "typically 9 to 12.5 percent" of recoveries, set in the bid. The RAC must return the fee if the overpayment is overturned at any level of appeal — [FAH RAC issue page](https://fah.org/issues-advocacy/medicare/medicare-recovery-audit-contractor-program/); fee-return rule also in the [CMS FY2023 Program Integrity Report](https://www.cms.gov/files/document/fy2023-medicare-and-medicaid-report-congress.pdf) **[reported/confirmed]**. The current CMS program page does not publish fee rates — [CMS RAC page, modified 2026-09-28](https://www.cms.gov/data-research/monitoring-programs/medicare-fee-service-compliance-programs/medicare-fee-service-recovery-audit-program) **[confirmed absence]**.
- **Medicaid RACs (states):** Virginia publishes an annual report, "Contingency Fee-Based Recovery Audit Contractors (RACs) – FY 2025" (Nov 2025) — [Virginia RD646](https://rga.lis.virginia.gov/Published/2025/RD646) **[confirmed that the doc exists; contents not read]**.
- **Amazon FBA reimbursement:** Carbon6 (Seller Investigators, now SPS Revenue Recovery) charges 25% of recovered amounts. GETIDA charges "starting at 25%," with no fee on the first $400 — [Hack'celeration Carbon6 review 2026](https://hackceleration.com/labs/review/carbon6); [AMZFinder GETIDA review 2026](https://www.amzfinder.com/blog/getida-review/) **[aggregator]**.
- **Freight audit (AI):** Loop's pricing "is not publicly disclosed." It appears to be platform fees plus volume-based pricing (invoice count, shipment volume, modules) — [Sacra, ~Mar 2026](https://sacra.com/c/loop/) **[aggregator]**.
- **Retail deductions (AI):** Glimpse did not disclose pricing in its March 2026 Series A coverage — [TechCrunch, 2026-03-25](https://techcrunch.com/2026/03/25/a16z-backed-glimpse-raises-new-funds-accelerates-dispute-tracking-automation-for-cpg-brands/) **[reported]**.
- **Telecom expense audit:** sold on contingency ("a percentage of all savings realized") — [Wikipedia: Audit (telecommunication)](https://en.wikipedia.org/wiki/Audit_(telecommunication)) **[aggregator; no rate found]**.

### Inferences
- Structural fee compression. (a) Amazon now auto-reimburses most FBA losses (see §5), which removes the easy 25% cases. (b) In government health the price is set by competitive bid, landing at about 10%. (c) AI-native entrants price as software, which caps the contingency take. The market seems to be splitting: contingency survives on hard, ambiguous claims, and flat or software pricing wins on the detectable, high-volume ones.
- An AI agent charging, say, 10–15% of recoveries on commercial AP would undercut the 20–40% incumbents. Whether that is a durable moat is doubtful given ERP-native prevention (see §8).

### Gaps
- No dated source for freight audit contingency/overcharge-recovery rates, telecom/SaaS audit rates, or retailer-deduction outsourced recovery rates (commonly said to be about 20–35%, but I found no source).
- No source on fee trends over time, e.g. a decline in PRGX's average rate.

---

## 3. Incumbents

### Takeaway
The incumbents are PE-owned consolidators. PRGX was taken private by Ardian for $195M (2021). Cotiviti was recapitalized at about $11B by KKR + Veritas (2024) and holds three of the five Medicare RAC regions after the 2025 awards. Performant was bought by AI-native Machinify for about $670M (closed 2025-10-21), which is the clearest sign that "AI payment integrity" is absorbing legacy contingency auditors. Cass is the public freight audit/payment benchmark at $36B+ volume.

### Cited Findings
**PRGX (AP/retail recovery audit)**
- Ardian closed its acquisition of PRGX on 2021-03-04 for about $195M. PRGX then divested non-core businesses and refocused on profit recovery and contract management. Ardian says EBITDA "more than doubled" since — [Ardian press release](https://www.ardian.com/press-releases/ardian-and-prgx-announce-acquisition-close); [Sheppard Mullin](https://www.sheppard.com/news-and-events/press-releases/sheppard-advises-ardian-in-acquistion-of-prgx-for-195-million); [Ardian focus article](https://www.ardian.com/news-insights/article/focus-prgx-ardian-helps-data-analytics-specialist-refocus-and-grow) **[confirmed]**.
- Historical positioning (last public filings): PRGX served "80% of the top 15 global retailers and over 25% of the top 50" Fortune 500. Recovery audit was the "vast majority" of revenue and retail was a "mature service offering" — [PRGX 10-K FY2019 (SEC)](https://www.sec.gov/Archives/edgar/data/1007330/000100733020000003/a201910kprgx.htm) **[confirmed, dated FY2019]**.
- I found no 2024–2026 news of a PRGX sale or change of ownership. Ardian still appears to own it as of 2026-09-30 **[negative finding]**.

**Healthcare payment integrity**
- Cotiviti: recapitalization with KKR and Veritas announced 2024-02-14, giving co-equal ownership at about $11B valuation, with a $5B term loan. Closed around May 2024 — [Cotiviti press release](https://www.cotiviti.com/press-release/cotiviti-announces-recapitalization-with-kkr-and-long-standing-owner-veritas); [Healthcare Dive](https://www.healthcaredive.com/news/kkr-talks-buy-stake-cotiviti-veritas-capital/702397/); [PitchBook](https://pitchbook.com/news/articles/cotiviti-eyes-5b-term-loan-facility-to-back-kkr-stake-purchase); [Cotiviti close release](https://www.cotiviti.com/press-release/cotiviti-completes-recapitalization-with-kkr-and-longstanding-owner-veritas) **[confirmed]**.
- Medicare RAC awards: on 2025-04-28 CMS awarded Cotiviti GOV Services RAC Regions 3, 4 and 5 (Region 5 = nationwide DMEPOS/HH/Hospice, previously Performant). Active reviews were expected from Summer 2025. Performant keeps an admin/appeals role for Region 5 — [Cotiviti press release](https://www.cotiviti.com/press-release/cotiviti-gov-services-awarded-contracts-for-cms-recovery-audit-contractor-rac-region-3-region-4-and-region-5); [CMS RAC page](https://www.cms.gov/data-research/monitoring-programs/medicare-fee-service-compliance-programs/medicare-fee-service-recovery-audit-program) **[confirmed]**.
- As of CMS page modification on 2026-09-28: Regions 1 and 2 = "Performant, a Machinify company"; Regions 3, 4 and 5 = Cotiviti GOV Services — [CMS RAC page](https://www.cms.gov/data-research/monitoring-programs/medicare-fee-service-compliance-programs/medicare-fee-service-recovery-audit-program) **[confirmed]**.
- Machinify (New Mountain Capital-backed) agreed on 2025-08-01 to acquire Performant Healthcare (Nasdaq: PHLT) for about $670M. Shareholders approved on 2025-10-17 and the deal closed on 2025-10-21. It was Machinify's second major acquisition of 2025 — [BusinessWire, 2025-08-01](https://www.businesswire.com/news/home/20250801154303/en/Performant-Healthcare-Inc.-to-Be-Acquired-by-Machinify); [BusinessWire close, 2025-10-21](https://www.businesswire.com/news/home/20251021569768/en/Machinify-Completes-Acquisition-of-Performant-Healthcare-Accelerating-Intelligent-Healthcare-Payments); [Fierce Healthcare](https://www.fiercehealthcare.com/health-tech/machinify-closes-670m-acquisition-performant-healthcare-broaden-its-reach); [Ropes & Gray](https://www.ropesgray.com/en/news-and-events/news/2025/10/machinify-to-acquire-performant-healthcare) **[confirmed]**.
- HMS Holdings: acquired by Gainwell (Veritas) in 2021 — [Wikipedia: HMS Holdings](https://en.wikipedia.org/wiki/HMS_Holdings) **[aggregator; not re-verified this session]**. So Veritas has exposure to both Cotiviti and Gainwell/HMS (inference).

**Freight audit**
- Cass Information Systems: see §1. Transportation dollar volume was $36.45B (2025), $36.11B (2024) and $38.29B (2023) — [Cass 10-K FY2025](https://www.sec.gov/Archives/edgar/data/708781/000070878126000010/cass-20251231.htm) **[confirmed]**. Cass reported "record annual net income and EPS" for FY2025 — [Cass IR release](https://ir.cassinfo.com/news-releases/news-release-details/cass-information-systems-reports-record-annual-net-income-and) **[confirmed headline; figures not extracted]**.
- Trax Technologies (about $25B of spend) and Intelligent Audit are named as main competitors — [Sacra](https://sacra.com/c/loop/) **[aggregator]**.

**AP recovery / prevention**
- apexanalytix positions itself on both recovery audit and "overpayment prevention" — [apexanalytix](https://www.apexanalytix.com/solutions/audit-recovery/) **[vendor]**.

### Inferences
- The healthcare incumbent layer is already being bought by AI-positioned platforms (Machinify–Performant). The "AI beats incumbents" thesis there is being played as roll-up, not displacement.
- PRGX's $195M take-private price against trillions of audited spend suggests commercial AP recovery is a low-growth, cash-flow business. That makes it an acquirable distribution channel for an AI entrant, or a slow-moving target.

### Gaps
- No revenue data for PRGX after 2019, CTSI, nVision Global, or telecom/expense audit firms (e.g. Tangoe, Cass telecom), and no current Cotiviti revenue. CTSI and nVision were not covered by any search result this session.
- The RAC contingency rates in the 2025 Cotiviti awards were not found.

---

## 4. AI entrants: funding, traction, pricing

### Takeaway
Venture money is going into three places: healthcare payment integrity (Alaffia $55M Series B in Feb 2026, Codoxo $35M Series C in Dec 2025, Machinify's roll-up), freight/logistics invoice audit (Loop $95M Series C in Apr 2026, about $44M estimated ARR), and CPG retail deductions (Glimpse $35M Series A led by a16z in Mar 2026, 200+ brands). Amazon reimbursement tooling is consolidating into suite vendors (Carbon6 into SPS Commerce, 2025). Almost none publicly price on pure contingency.

### Cited Findings
**Retail deductions**
- Glimpse raised a $35M Series A led by Andreessen Horowitz, with 8VC and YC participating (announced 2026-03-25). It had earlier raised $10M in 2024 (8VC). Total is about $52M. It serves 200+ brands (e.g. Suave, Chapstick). The company pivoted before this. Competitors named: Revya and Confido. It keeps human oversight on disputes — [TechCrunch, 2026-03-25](https://techcrunch.com/2026/03/25/a16z-backed-glimpse-raises-new-funds-accelerates-dispute-tracking-automation-for-cpg-brands/); [Glimpse blog](https://www.tryglimpse.com/post/seriesa); [BevNET, 2026-03-25](https://www.bevnet.com/pr/2026/03/25/glimpse-raises-35m-to-bring-ainative-infrastructure-to-cpg-and-retail) **[confirmed/reported]**.
- Glimpse claims that for a $1B CPG company, an agent reviewed 17,000 deductions in under 24 hours (about 2 years of manual work) and found "millions" recoverable — [Let's Data Science summary](https://letsdatascience.com/news/glimpse-raises-35m-to-automate-retail-deductions-cf3235c8) **[vendor claim via aggregator]**.
- Cloverleaf, Vividly and other 2025–2026 deduction startups: no funding or traction results came up in searches **[gap]**.

**Amazon seller reimbursement**
- Carbon6 was acquired by SPS Commerce (Feb 2025, per review site). Seller Investigators was rebranded "SPS Revenue Recovery" at a 25% fee — [Hack'celeration 2026 review](https://hackceleration.com/labs/review/carbon6); [SPS Commerce reimbursement article](https://www.spscommerce.com/community/articles/amazon-reimbursement-policy) **[aggregator; deal price not verified]**.
- GETIDA uses a hybrid of AI and case management, starting at 25% — [AMZFinder GETIDA review 2026](https://www.amzfinder.com/blog/getida-review/); [DrStock GETIDA review 2026](https://drstock.ai/compare/getida-review/) **[aggregator]**.

**Freight audit AI**
- Loop (founded 2021) raised a $95M Series C led by Valor Equity Partners and the Valor Atreides AI Fund (2026-04-17). It claims freight audits take 2 hours instead of several weeks. Customers include Dell and Estée Lauder — [SiliconANGLE, 2026-04-17](https://siliconangle.com/2026/04/17/supply-chain-ai-startup-loop-secures-95m-investment/) **[reported]**. Prior rounds: $30M Series A (2022) and $35M Series B (2023), total about $160M. Investors include Founders Fund, 8VC, Index, JPM Growth. About $44M annualized revenue (Mar 2026 estimate). 20 Fortune 100 customers — [Sacra](https://sacra.com/c/loop/) **[aggregator estimate]**. Loop launched a "Logistics Data Platform" on 2026-05-04 — [BusinessWire](https://www.businesswire.com/news/home/20260504771246/en/Loop-Launches-the-Logistics-Data-Platform-Powered-by-New-AI-Capabilities) **[confirmed]**.

**Healthcare payment integrity AI**
- Alaffia Health raised a $55M Series B led by Transformation Capital, with FirstMark, Tau and Twine (2026-02-03). Total is $73M+. It describes itself as "agentic AI" for claims review — [BusinessWire](https://www.businesswire.com/news/home/20260203113559/en/Alaffia-Health-Raises-$55M-to-Tackle-Healthcares-$570-Billion-Waste-Problem); [HLTH](https://hlth.com/insights/news/alaffia-health-raises-55m-series-b-to-expand-ai-driven-claims-operations-2026-02-04); [MedCity News](https://medcitynews.com/2026/02/alaffia-health-insurance-funding-capital-claims/) **[confirmed/reported]**.
- Codoxo raised a $35M Series C led by CVS Health Ventures, with Echo Health Ventures new and Sands, QED and Wipro following on (Dec 2025). Total is $75M+. Its footprint is "more than 80 million" covered lives, focused on pre-claim payment integrity — [PR Newswire](https://www.prnewswire.com/news-releases/codoxo-caps-breakout-year-with-oversubscribed-35m-series-c-cementing-significant-market-momentum--leadership-in-pre-claim-payment-integrity-302689012.html); [MedCity News, 2025-12](https://medcitynews.com/2025/12/codoxo-payment-integrity/) **[confirmed/reported]**.
- Machinify: see §3 (acquired Performant for about $670M, closed 2025-10-21, New Mountain-backed) **[confirmed]**.

**AP audit AI**
- Discover Dollar markets AI duplicate-payment recovery. apexanalytix and PRGX both market AI/analytics — [Discover Dollar blog](https://www.discoverdollar.com/blog/duplicate-payment-recovery-7-hidden-error-patterns-ap-teams-miss) **[vendor]**. Trustmi (B2B payment protection) positions on prevention of duplicates and fraud — [Trustmi](https://trustmi.ai/resource/the-duplicate-payment-dilemma/) **[vendor]**. No funding data was gathered this session.

### Inferences
- The best-funded AI entrants (Loop, Glimpse, Alaffia, Codoxo) sell workflow/platform software to the claimant or payer. They are not pure contingency agents. That suggests VCs expect contingency pricing to compress once detection is cheap.
- The healthcare trend is moving from post-pay recovery (RAC-style contingency) to pre-pay/pre-claim prevention (Codoxo), which destroys the contingency base by design.

### Gaps
- No revenue or pricing for Glimpse, Alaffia or Codoxo. No data on Cloverleaf, Vividly, CTSI, nVision, or AP-audit AI startup funding.

---

## 5. How recovery is verified and paid; attribution and data access

### Takeaway
Payment mechanics differ sharply. Amazon reimbursements settle on a platform ledger with very short claim windows (60 days since Oct 2024) and are now mostly automatic. Medicare RAC recoveries are offset against provider payments, and the fee is clawed back if overturned at any appeal level. Commercial AP recoveries come through supplier credit memos or checks and depend on supplier cooperation. Freight audit mostly prevents overpayment before payment (pre-pay audit), so the "recovery" is avoided spend.

### Cited Findings
- **Amazon:** from 2024-11-01, Amazon proactively reimburses most FBA warehouse lost/damaged and customer-return cases. It cut the claim window from 18 months to 60 days on 2024-10-23 (returns: 60–120 days; removals: 15–75 days). From 2025-03-10 reimbursements are valued at manufacturing cost, not sale price — [Forceget, 2025](https://forceget.com/blog/amazon-reimbursement-policy-update-2025/); [Amazon Seller Central forum announcement](https://sellercentral.amazon.com/seller-forums/discussions/t/81c3235d-4c44-47ba-96c5-883cecab3244); [eComEngine](https://www.ecomengine.com/blog/fba-reimbursement-policy); [SPS Commerce](https://www.spscommerce.com/community/articles/amazon-reimbursement-policy) **[reported; the Amazon forum post is primary but I did not read it directly]**.
- **Medicare RAC:** the RAC returns its contingency fee if the determination is overturned at any appeal level. Net recoveries subtract underpayments and overturned amounts — [CMS FY2023 PI Report](https://www.cms.gov/files/document/fy2023-medicare-and-medicaid-report-congress.pdf) **[confirmed]**. RAC performance requirements cited: overturn rate below 10% at first-level appeal and accuracy of at least 95% — [Cotiviti RAC provider overview (PDF)](https://www.cotiviti.com/hubfs/Cotiviti%20-%20CMS%20RAC%20PDFs/CMS%20RAC%20Provider%20Overview.pdf) / search extraction **[reported]**.
- **Commercial AP:** recovery audits rely on remote ERP data access with SAP-certified and Oracle integrations — [apexanalytix](https://www.apexanalytix.com/solutions/audit-recovery/accounts-payable-recovery-audit/) **[vendor]**.
- **Freight:** Loop ingests invoices and bills of lading, normalizes them, and uses agents to flag discrepancies — [SiliconANGLE](https://siliconangle.com/2026/04/17/supply-chain-ai-startup-loop-secures-95m-investment/) **[reported]**.

### Inferences
- Attribution ("who found it first") matters most in commercial AP and deductions. There the client's own AP team, the ERP duplicate checker, a prevention tool and a post-audit firm can all "find" the same item. Contracts need an explicit baseline and exclusion list. I found no public dispute cases.
- The Amazon ledger is the cleanest neutral verifier. But Amazon's automation and 60-day window have taken most of the contingency value (inference from the policy change).

### Gaps
- No public disputes or litigation found on attribution between recovery vendors and clients or ERP tools. No primary Amazon policy text was read directly.

---

## 6. Regulatory and relationship risk

### Takeaway
Healthcare carries the heaviest political risk. Hospital groups (AHA, FAH) keep lobbying against RAC burden, and CMS has redesigned contracts around accuracy and fee clawback. Commercial AP and deductions carry relationship risk, since clawing money from suppliers or retailers strains partnerships. Healthcare data (HIPAA/PHI) and ERP access restrict who can run agents.

### Cited Findings
- AHA published "The Real Cost of the Inefficient Medicare RAC Program" (hospital survey, July 2025) — [AHA PDF](https://www.aha.org/system/files/media/file/2025/07/hospsurveyreport.pdf) **[confirmed existence; the PDF could not be parsed this session, so no stats extracted]**. FAH maintains an advocacy page on the RAC program — [FAH](https://fah.org/issues-advocacy/medicare/medicare-recovery-audit-contractor-program/) **[confirmed]**.
- CMS frames the 2025 awards around "quality, accuracy, and transparency of reviews ... and minimizing provider burden" — [Cotiviti release, 2025-04-28](https://www.cotiviti.com/press-release/cotiviti-gov-services-awarded-contracts-for-cms-recovery-audit-contractor-rac-region-3-region-4-and-region-5) **[confirmed]**.
- GAO (2023) found that CMS oversight and guidance could improve Medicaid RAC programs — [GAO-23-106025](https://www.gao.gov/products/gao-23-106025) **[confirmed title; details not read]**.
- A new DMEPOS RAC (Cotiviti Region 5) launched five audit projects, reported by DME trade press — [Medtrade](https://medtrade.com/news/legislative-advocacy/new-medicare-dmepos-rac-contractor-launches-five-audit-projects/); [VGM](https://www.vgm.com/services/government-relations/out-with-the-old-rac-in-with-the-new-/) **[reported, 2025]**.
- Payment errors "strain critical supplier relationships and create unnecessary friction" — [apexanalytix](https://www.apexanalytix.com/resources/blog/what-is-a-recovery-audit/) **[vendor claim]**.

### Inferences
- An AI agent that raises the volume of RAC-style findings will draw more appeals and provider lobbying. Because the fee is clawed back on overturn, precision matters more than recall.
- In CPG deductions, the AI agent acts for the supplier against the retailer. Retailers tolerate that because it runs through their own dispute portals, but aggressive auto-disputing could get throttled.

### Gaps
- No current RAC appeal-overturn statistics (FY2023–FY2025) could be extracted, because the AHA PDF and CMS report were not parsed. There is no numeric evidence of supplier churn caused by recovery audits.
- Data privacy: I found no sources on constraints specific to AP/retail recovery (e.g. GDPR limits on vendor master data).

---

## 7. Verification profile scorecard

### Takeaway
The best fit for outcome-priced AI agents is **CPG retail deductions** (clear per-item verification, high frequency, a retailer portal as semi-neutral verifier, modest stakes) and, in second place, **freight/parcel audit** (contract-rate verification, very high volume). Amazon reimbursement verifies best but has been largely automated away by Amazon. Medicare RAC verifies worst: long appeal latency, full fee clawback, political risk, and a bid-set fee around 10%.

### Scorecard (1 = poor for agents, 5 = excellent). All scores are my inferences from the cited findings above.

| Feature | Commercial AP (duplicates/overpayments) | Freight / parcel audit | Telecom/utility/SaaS expense | Retail deductions (CPG) | Amazon FBA reimbursement | Medicare RAC / health PI |
|---|---|---|---|---|---|---|
| Neutral verifier | 3: supplier credit memo; supplier must agree | 4: contracted rate tables; carrier credit | 3: carrier/vendor credit; contracts opaque | 4: retailer dispute portal decides | 5: Amazon ledger | 2: the provider appeal chain is the verifier, and it is adversarial |
| Feedback latency | 2: weeks to months for credits | 4: pre-pay audit means immediate | 3: billing cycles | 3: weeks to months per dispute | 5: days; auto-reimbursed | 1: appeals take months to years (the 5-level Medicare appeal process) |
| Clawback exposure | 3: low once credit posted | 4: low | 4 | 3: retailer can re-deduct | 4 | 1: fee returned if overturned at any level ([CMS](https://www.cms.gov/files/document/fy2023-medicare-and-medicaid-report-congress.pdf)) |
| Buyer auditability | 4: client sees ERP evidence | 5: line-item | 4 | 4 | 5 | 3: CMS oversight metrics (≥95% accuracy) |
| Attribution | 2: ERP, AP team, prevention tools and auditor overlap | 3: pre-pay audit makes the counterfactual fuzzy | 3 | 4: each deduction is a discrete dispute | 3: Amazon auto-reimburses, so "found" is contested | 4: RAC claim-level |
| Frequency/volume | 3: large backfile, episodic | 5: millions of invoices ([Cass](https://www.sec.gov/Archives/edgar/data/708781/000070878126000010/cass-20251231.htm)) | 3 | 5: thousands per brand ([Glimpse](https://letsdatascience.com/news/glimpse-raises-35m-to-automate-retail-deductions-cf3235c8)) | 4 | 4 |
| Stakes/liability | 4: moderate | 4 | 4 | 4 | 5: low | 1: provider harm and political scrutiny ([AHA](https://www.aha.org/system/files/media/file/2025/07/hospsurveyreport.pdf)) |
| Fee-collapse risk from AI/ERP | 1: ERP-native duplicate detection and prevention tools cut the base ([apexanalytix](https://www.apexanalytix.com/solutions/audit-recovery/overpayment-prevention/)) | 2: platforms price as SaaS (Loop) | 3 | 3: AI entrants are software-priced, but contested items keep value | 1: Amazon auto-reimbursement since Nov 2024 | 2: fee already about 10% via bid; shift to pre-pay |
| **Overall fit** | **Medium-low** | **Medium-high** | **Medium** (under-researched) | **High** | **Low (automated away)** | **Low for startups; roll-up play** |

### Inferences
- Collapse mechanism: whenever the paying party (Amazon, ERP vendors, payers moving pre-pay) can detect the error itself, the contingency pool shrinks. The durable wedge is where the agent acts for the claimant against a counterparty who will not self-correct. Retail deductions (brand vs. retailer) and freight (shipper vs. carrier) fit that. So does telecom expense, though it was under-researched here.
- Because the fee is capped by bid (about 9–12.5%) and clawed back on overturn, Medicare RAC rewards precision engineering, not scale. The Machinify–Performant deal shows the winning move is buying the contract holder.

### Gaps
- The scores are qualitative. There is no measured data on dispute win rates by segment (e.g. retailer deduction win rates) or on freight overcharge rates as % of spend.

---

## 8. Would AI collapse the fee? (ERP / platform self-detection)

### Takeaway
AI is already collapsing fees in the most detectable segments. Amazon automated FBA reimbursements (Nov 2024), and AP prevention tools sit in front of ERPs. Fees hold up only where recovery needs adversarial evidence and negotiation with a counterparty.

### Cited Findings
- Amazon auto-reimbursement since 2024-11-01, a 60-day window since 2024-10-23, and a switch to manufacturing-cost valuation from 2025-03-10 — [Forceget](https://forceget.com/blog/amazon-reimbursement-policy-update-2025/); [eComEngine](https://www.ecomengine.com/blog/fba-reimbursement-policy) **[reported]**.
- Vendors sell "overpayment prevention" to stop duplicates before payment. They claim about $2M per $1B in duplicates slips past ERP controls — [apexanalytix](https://www.apexanalytix.com/solutions/audit-recovery/overpayment-prevention/) **[vendor]**. SAP-embedded AI duplicate detection add-ons exist — [SAVI AI blog](https://www.savic.ai/blog-fraud-detection.html) **[vendor]**.
- Codoxo positions on pre-claim payment integrity — [PR Newswire, Dec 2025](https://www.prnewswire.com/news-releases/codoxo-caps-breakout-year-with-oversubscribed-35m-series-c-cementing-significant-market-momentum--leadership-in-pre-claim-payment-integrity-302689012.html) **[confirmed]**.

### Inferences
- For a founder, a pure-contingency AI agent in AP duplicate recovery is a shrinking, commoditizing niche. Retail deductions and freight/parcel audit have better durability. Healthcare PI is attractive only for well-capitalized players selling to payers, not on RAC contingency.

### Gaps
- I found no source confirming that SAP, Oracle or NetSuite have shipped native AI duplicate-payment detection (2024–2026) that measurably reduced recovery audit yields.

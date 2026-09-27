use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::components::{A, Form, Route, Router, Routes};
use leptos_router::hooks::{use_params_map, use_query_map};
use leptos_router::path;
use serde::{Deserialize, Serialize};
use thaw::*;

// ---- Shared types (serialised between server and browser) ----

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunSummary {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub status: String,
    pub summary: String,
    /// Search-hit excerpt as escaped HTML with `<b>` highlights.
    pub snippet: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunList {
    pub runs: Vec<RunSummary>,
    /// Every tag with its run count, most used first.
    pub tags: Vec<(String, usize)>,
    pub total: usize,
    /// Directories that failed to load: (slug, error).
    pub broken: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunDetail {
    pub run: RunSummary,
    pub report_html: Option<String>,
    pub report_name: Option<String>,
    pub files: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarkdownDoc {
    pub run_title: String,
    pub path: String,
    pub html: String,
}

// ---- Server functions ----

#[server]
pub async fn list_runs(q: String, tag: String, status: String) -> Result<RunList, ServerFnError> {
    crate::server::list_runs(&q, &tag, &status).map_err(|e| ServerFnError::new(format!("{e:#}")))
}

#[server]
pub async fn get_run(slug: String) -> Result<Option<RunDetail>, ServerFnError> {
    Ok(crate::server::get_run(&slug))
}

#[server]
pub async fn get_markdown(slug: String, path: String) -> Result<Option<MarkdownDoc>, ServerFnError> {
    crate::server::get_markdown(&slug, &path).map_err(|e| ServerFnError::new(format!("{e:#}")))
}

#[server]
pub async fn reload_runs() -> Result<(), ServerFnError> {
    crate::server::reload().await.map_err(|e| ServerFnError::new(format!("{e:#}")))
}

// ---- Shell and app ----

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos::hydration::{AutoReload, HydrationScripts};
    use leptos_meta::MetaTags;
    view! {
        <thaw::ssr::SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                </head>
                <body>
                    <App />
                </body>
            </html>
        </thaw::ssr::SSRMountStyleProvider>
    }
}

/// Bumped after a reload so every page refetches.
#[derive(Clone, Copy)]
struct ReloadTick(RwSignal<u32>);

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = RwSignal::new(Theme::light());
    provide_context(ReloadTick(RwSignal::new(0)));

    view! {
        <Stylesheet id="leptos" href="/pkg/viewer.css" />
        <Title text="Autoresearch" />
        <ConfigProvider theme>
            <Router>
                <Layout class="app" content_style="width: 100%;">
                    <Header theme />
                    <main class="content">
                        <Routes fallback=|| view! { <Notice text="Page not found." /> }>
                            <Route path=path!("") view=RunsPage />
                            <Route path=path!("/runs/:slug") view=RunPage />
                            <Route path=path!("/runs/:slug/md/*path") view=MarkdownPage />
                        </Routes>
                    </main>
                </Layout>
            </Router>
        </ConfigProvider>
    }
}

#[component]
fn Header(theme: RwSignal<Theme>) -> impl IntoView {
    let query = use_query_map();
    let q = RwSignal::new(String::new());
    Effect::new(move || q.set(query.read().get("q").unwrap_or_default()));

    let tick = expect_context::<ReloadTick>().0;
    let reload = ServerAction::<ReloadRuns>::new();
    Effect::new(move || {
        if let Some(Ok(())) = reload.value().get() {
            tick.update(|n| *n += 1);
        }
    });

    let dark = move || theme.with(|t| t.name == "dark");

    view! {
        <LayoutHeader class="header">
            <A href="/" attr:class="brand">"autoresearch"</A>
            <Form method="GET" action="/" attr:class="search">
                <Input name="q" value=q placeholder="Search runs…" class="search-input" />
                <Button button_type=ButtonType::Submit appearance=ButtonAppearance::Primary>
                    "Search"
                </Button>
            </Form>
            <Flex gap=FlexGap::Small>
                <Button
                    appearance=ButtonAppearance::Subtle
                    loading=reload.pending()
                    on_click=move |_| {
                        reload.dispatch(ReloadRuns {});
                    }
                >
                    "Reload"
                </Button>
                <Button
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| {
                        theme.set(if dark() { Theme::light() } else { Theme::dark() })
                    }
                >
                    {move || if dark() { "Light" } else { "Dark" }}
                </Button>
            </Flex>
        </LayoutHeader>
    }
}

// ---- Pages ----

/// Build a `/?…` URL with the given filters, omitting empty ones.
fn list_href(q: &str, tag: &str, status: &str) -> String {
    let params: Vec<String> = [("q", q), ("tag", tag), ("status", status)]
        .into_iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| format!("{k}={}", urlencoding::encode(v)))
        .collect();
    if params.is_empty() { "/".into() } else { format!("/?{}", params.join("&")) }
}

const STATUSES: [&str; 4] = ["draft", "running", "complete", "abandoned"];

fn status_color(status: &str) -> BadgeColor {
    match status {
        "complete" => BadgeColor::Success,
        "running" => BadgeColor::Brand,
        "abandoned" => BadgeColor::Danger,
        _ => BadgeColor::Informative,
    }
}

#[component]
fn StatusBadge(status: String) -> impl IntoView {
    view! {
        <Badge appearance=BadgeAppearance::Tint color=status_color(&status) size=BadgeSize::Small>
            {status}
        </Badge>
    }
}

#[component]
fn Notice(#[prop(into)] text: String, #[prop(optional)] intent: MessageBarIntent) -> impl IntoView {
    view! {
        <MessageBar intent layout=MessageBarLayout::Multiline>
            <MessageBarBody>{text}</MessageBarBody>
        </MessageBar>
    }
}

#[component]
fn Loading() -> impl IntoView {
    view! { <Spinner label="Loading…" /> }
}

#[component]
fn RunsPage() -> impl IntoView {
    let query = use_query_map();
    let param = move |k: &str| query.read().get(k).unwrap_or_default();
    let tick = expect_context::<ReloadTick>().0;
    let data = Resource::new(
        move || (param("q"), param("tag"), param("status"), tick.get()),
        |(q, tag, status, _)| list_runs(q, tag, status),
    );

    view! {
        <Title text="Runs · Autoresearch" />
        <Suspense fallback=Loading>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(list) => view! {
                        <RunListView list q=param("q") tag=param("tag") status=param("status") />
                    }
                        .into_any(),
                    Err(e) => view! { <Notice text=e.to_string() intent=MessageBarIntent::Error /> }
                        .into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn RunListView(list: RunList, q: String, tag: String, status: String) -> impl IntoView {
    let n = list.runs.len();
    let heading = if q.is_empty() {
        format!("{n} of {} runs", list.total)
    } else {
        format!("{n} {} for “{q}”", if n == 1 { "result" } else { "results" })
    };
    let has_filters = !(q.is_empty() && tag.is_empty() && status.is_empty());

    let status_filters = STATUSES
        .iter()
        .map(|s| {
            let active = status == *s;
            let href = list_href(&q, &tag, if active { "" } else { s });
            view! {
                <A href=href>
                    <Badge
                        appearance=if active { BadgeAppearance::Filled } else { BadgeAppearance::Outline }
                        color=status_color(s)
                    >
                        {s.to_string()}
                    </Badge>
                </A>
            }
        })
        .collect_view();

    let tag_filters = list
        .tags
        .into_iter()
        .map(|(t, n)| {
            let active = tag == t;
            let href = list_href(&q, if active { "" } else { &t }, &status);
            view! {
                <A href=href>
                    <Badge
                        appearance=if active { BadgeAppearance::Filled } else { BadgeAppearance::Outline }
                        color=BadgeColor::Informative
                    >
                        {format!("{t} · {n}")}
                    </Badge>
                </A>
            }
        })
        .collect_view();

    let broken = list
        .broken
        .into_iter()
        .map(|(slug, err)| {
            view! {
                <MessageBar intent=MessageBarIntent::Warning layout=MessageBarLayout::Multiline>
                    <MessageBarBody>
                        <MessageBarTitle>{slug}</MessageBarTitle>
                        {err}
                    </MessageBarBody>
                </MessageBar>
            }
        })
        .collect_view();

    let cards = list.runs.into_iter().map(|run| view! { <RunCard run /> }).collect_view();

    view! {
        <Flex vertical=true gap=FlexGap::Large>
            {broken}
            <Flex vertical=true gap=FlexGap::Small>
                <Flex gap=FlexGap::Small align=FlexAlign::Center class="filters">
                    <Caption1Strong>"Status"</Caption1Strong>
                    {status_filters}
                </Flex>
                <Flex gap=FlexGap::Small align=FlexAlign::Center class="filters">
                    <Caption1Strong>"Tags"</Caption1Strong>
                    {tag_filters}
                </Flex>
            </Flex>
            <Flex justify=FlexJustify::SpaceBetween align=FlexAlign::Center>
                <h2 class="count">{heading}</h2>
                {has_filters.then(|| view! { <A href="/">"Clear filters"</A> })}
            </Flex>
            <div class="cards">{cards}</div>
        </Flex>
    }
}

#[component]
fn RunCard(run: RunSummary) -> impl IntoView {
    let href = format!("/runs/{}", run.slug);
    let tags = run
        .tags
        .into_iter()
        .map(|t| {
            let href = list_href("", &t, "");
            view! {
                <A href=href>
                    <Tag size=TagSize::ExtraSmall>{t}</Tag>
                </A>
            }
        })
        .collect_view();

    view! {
        <Card class="run-card">
            <CardHeader>
                <A href=href attr:class="run-title">{run.title}</A>
                <CardHeaderDescription slot>
                    <Flex gap=FlexGap::Small align=FlexAlign::Center>
                        <Caption1>{run.date}</Caption1>
                        <StatusBadge status=run.status />
                    </Flex>
                </CardHeaderDescription>
            </CardHeader>
            {(!run.summary.is_empty()).then(|| view! { <Body1>{run.summary}</Body1> })}
            {run.snippet.map(|html| view! { <div class="snippet" inner_html=html></div> })}
            <Flex gap=FlexGap::Small class="tags">{tags}</Flex>
        </Card>
    }
}

#[component]
fn RunPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();
    let tick = expect_context::<ReloadTick>().0;
    let data = Resource::new(move || (slug(), tick.get()), |(slug, _)| get_run(slug));

    view! {
        <Suspense fallback=Loading>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(Some(detail)) => view! { <RunDetailView detail /> }.into_any(),
                    Ok(None) => view! { <Notice text="No run with that name." /> }.into_any(),
                    Err(e) => view! { <Notice text=e.to_string() intent=MessageBarIntent::Error /> }
                        .into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn RunDetailView(detail: RunDetail) -> impl IntoView {
    let RunDetail { run, report_html, report_name, files } = detail;
    let slug = run.slug.clone();
    let tags = run
        .tags
        .into_iter()
        .map(|t| {
            let href = list_href("", &t, "");
            view! {
                <A href=href>
                    <Tag size=TagSize::Small>{t}</Tag>
                </A>
            }
        })
        .collect_view();
    let files = files
        .into_iter()
        .map(|f| {
            if f.ends_with(".md") {
                let href = format!("/runs/{slug}/md/{f}");
                view! { <li><A href=href>{f}</A></li> }.into_any()
            } else {
                let href = format!("/files/{slug}/{f}");
                view! { <li><a href=href target="_blank" rel="external noopener">{f}</a></li> }.into_any()
            }
        })
        .collect_view();

    view! {
        <Title text=format!("{} · Autoresearch", run.title) />
        <Flex vertical=true gap=FlexGap::Large>
            <A href="/">"← All runs"</A>
            <header class="run-header">
                <h1>{run.title}</h1>
                <Flex gap=FlexGap::Small align=FlexAlign::Center>
                    <Caption1>{run.date}</Caption1>
                    <StatusBadge status=run.status />
                    <Caption1 class="slug">{run.slug.clone()}</Caption1>
                </Flex>
                {(!run.summary.is_empty()).then(|| view! { <Body1 class="summary">{run.summary}</Body1> })}
                <Flex gap=FlexGap::Small class="tags">{tags}</Flex>
            </header>
            <div class="run-body">
                <Card class="report">
                    {match (report_html, report_name) {
                        (Some(html), Some(name)) => view! {
                            <Caption1 class="report-name">{name}</Caption1>
                            <article class="prose" inner_html=html></article>
                        }
                            .into_any(),
                        _ => view! {
                            <Notice text="This run has no report.md, README.md or index.md." />
                        }
                            .into_any(),
                    }}
                </Card>
                <Card class="files">
                    <Caption1Strong>"Files"</Caption1Strong>
                    <ul class="file-list">{files}</ul>
                </Card>
            </div>
        </Flex>
    }
}

#[component]
fn MarkdownPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();
    let path = move || params.read().get("path").unwrap_or_default();
    let tick = expect_context::<ReloadTick>().0;
    let data = Resource::new(
        move || (slug(), path(), tick.get()),
        |(slug, path, _)| get_markdown(slug, path),
    );

    view! {
        <Suspense fallback=Loading>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(Some(doc)) => view! {
                        <Title text=format!("{} · Autoresearch", doc.path) />
                        <Flex vertical=true gap=FlexGap::Large>
                            <A href=format!("/runs/{}", slug())>{format!("← {}", doc.run_title)}</A>
                            <Card class="report">
                                <Caption1 class="report-name">{doc.path}</Caption1>
                                <article class="prose" inner_html=doc.html></article>
                            </Card>
                        </Flex>
                    }
                        .into_any(),
                    Ok(None) => view! { <Notice text="File not found." /> }.into_any(),
                    Err(e) => view! { <Notice text=e.to_string() intent=MessageBarIntent::Error /> }
                        .into_any(),
                }
            })}
        </Suspense>
    }
}

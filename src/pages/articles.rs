use leptos::*;
use leptos_router::*;

#[derive(Debug, Clone)]
pub struct Article {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub thumbnail: Option<String>,
    pub gallery: Vec<String>,
}

impl Article {
    pub fn new(id: u32, title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            content: content.into(),
            thumbnail: Some("https://gaming-cdn.com/images/products/64/616x353/deus-ex-human-revolution-director-s-cut-director-s-cut-pc-mac-jeu-steam-europe-cover.jpg".to_string()),
            gallery: vec![],
        }
    }
}

#[component]
pub fn Articles() -> impl IntoView {
    let articles = vec![
        Article::new(1, "Test 1", "lorem lorem lorem"),
        Article::new(2, "Test 2", "lorem lorem lorem"),
        Article::new(3, "Test 3", "lorem lorem lorem"),
        Article::new(4, "Test 4", "lorem lorem lorem"),
    ];

    view! {
        <div class="container">
            <h1>Pages Articles</h1>
            <ul class="grid md:grid-cols-3 gap-4">
                {articles
                    .into_iter()
                    .enumerate()
                    .map(|(index, article)| {
                        view! {
                            <li style="animation-delay: {index * 0.5}s">
                                <ArticleCard article/>
                                {index}
                            </li>
                        }
                    })
                    .collect_view()}

            </ul>
        </div>
    }
}

#[component]
pub fn ArticleCard(#[prop()] article: Article) -> impl IntoView {
    view! {
        <article>
            <img src=article.thumbnail.unwrap() alt="" class="h-40 w-full object-cover"/>
            <h3>{article.title}</h3>
            <p>{article.content}</p>
            <A href=article.id.to_string()>Lire la suite</A>
        </article>
    }
}

#[component]
pub fn ArticleSingle() -> impl IntoView {
    let articles = vec![
        Article::new(1, "Deus Ex", "lorem lorem lorem"),
        Article::new(2, "Test 2", "lorem lorem lorem"),
        Article::new(3, "Test 3", "lorem lorem lorem"),
        Article::new(4, "Test 4", "lorem lorem lorem"),
    ];

    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let article = articles
        .get(id().parse::<usize>().unwrap_or_default())
        .unwrap()
        .to_owned();

    view! {
        <A href="/articles">Retour</A>
        <article>
            <img src=article.thumbnail.clone().unwrap() alt="" class="h-40 w-full object-cover"/>
            <h3>{article.title}</h3>
            <p>{article.content.clone()}</p>
            <A href=article.id.to_string()>Lire la suite</A>
        </article>
    }
}


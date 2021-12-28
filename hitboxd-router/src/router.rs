struct CacheKey {
    path: String,
    query: Vec<String>,
    headers: Vec<String>,
}

struct Matcher {
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
}

struct Endpoint {
    cache_key: CacheKey,
    matcher: Matcher,
}

struct Router {
    endpoints: Vec<Endpoint>,
}

CREATE TABLE IF NOT EXISTS public.games
(
    id TEXT NOT NULL,
    store TEXT NOT NULL,
    created_at DATE NOT NULL,
    title TEXT NOT NULL,
    identifier TEXT NOT NULL,
    url TEXT NOT NULL,
    original_price TEXT NOT NULL,
    offer_until DATE NOT NULL,
    game_type TEXT NOT NULL,
    UNIQUE (id, store)
);

CREATE TABLE IF NOT EXISTS public.platform_posts
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    game_id TEXT,
    game_store TEXT,
    platform TEXT,
    posted_at TIMESTAMP
);

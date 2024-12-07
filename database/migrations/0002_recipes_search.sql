-- Full-text search with tsvector, generated column from title and description.
ALTER TABLE recipes
ADD COLUMN search_tsvector TSVECTOR
GENERATED ALWAYS AS (
	setweight(to_tsvector('english', title), 'A') || ' ' || 
	setweight(to_tsvector('english', description), 'B') :: tsvector
) stored; 
--Full-text search index
CREATE INDEX idx_recipe_tsvector ON recipes USING GIN(search_tsvector);

CREATE EXTENSION pg_trgm;
-- Trigram indices 
CREATE INDEX idx_trgm_recipe_title ON recipes USING GIN(title gin_trgm_ops);
CREATE INDEX idx_trgm_recipe_description ON recipes USING GIN(description gin_trgm_ops);

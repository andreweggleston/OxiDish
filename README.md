# OxiDish
Working repo for an app to organize and search recipes.
The database is a cargo library, so it can be imported by different web framework "backends" as I experiment.

## Current tasks:
- more database tests (using #[sqlx::test])


## Database schema:
recipes:
| id      | title        | description    | search      |
|---------|--------------|----------------|-------------|
| integer | varchar(255) | varchar(65535) | tsvector    |

-- the 'search' column is generated from the title and description columns (see [0002_recipes_search.sql](database/migrations/0002_recipes_search.sql)).

recipe_ingredients:
| id_recipe | id_ingredient_name | ingredient_amount |
|-----------|--------------------|-------------------|
| integer   | integer            | varchar(255)      |

ingredient_names:
| id      | name         |
|---------|--------------|
| integer | varchar(255) |


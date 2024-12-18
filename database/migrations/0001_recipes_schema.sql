CREATE TABLE IF NOT EXISTS public.ingredient_names
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY,
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT ingredient_name_pkey PRIMARY KEY (id),
    UNIQUE(name)
    );

CREATE TABLE IF NOT EXISTS public.recipe_ingredients
(
    id_recipe integer NOT NULL,
    id_ingredient_name integer NOT NULL,
    ingredient_amount character varying(255) COLLATE pg_catalog."default" NOT NULL
    );

CREATE TABLE IF NOT EXISTS public.recipes
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    title character varying(255) COLLATE pg_catalog."default" NOT NULL,
    description character varying(65535) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT pk_constraint PRIMARY KEY (id)
    );


ALTER TABLE IF EXISTS public.recipe_ingredients
    ADD CONSTRAINT ingredient_name_id FOREIGN KEY (id_ingredient_name)
    REFERENCES public.ingredient_names (id) MATCH SIMPLE
    ON UPDATE NO ACTION
       ON DELETE NO ACTION
        NOT VALID;

ALTER TABLE IF EXISTS public.recipe_ingredients
    ADD CONSTRAINT recipe_id FOREIGN KEY (id_recipe)
    REFERENCES public.recipes (id) MATCH SIMPLE
    ON UPDATE NO ACTION
       ON DELETE NO ACTION
        NOT VALID;

CREATE VIEW view_recipe_ingredients AS
SELECT recipes.id, ingredient_names.name, recipe_ingredients.ingredient_amount
FROM ((recipe_ingredients
    left join recipes on recipe_ingredients.id_recipe = recipes.id)
    left join ingredient_names on recipe_ingredients.id_ingredient_name = ingredient_names.id);


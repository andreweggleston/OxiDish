WITH recipe_id AS (
  INSERT INTO RECIPES (title, description)
  VALUES ('Pasta Bolognese', 'A simple pasta sauce')
  RETURNING id
)
INSERT INTO recipe_ingredients (id_recipe, id_ingredient_name, ingredient_amount)
VALUES
  (
    (SELECT id FROM recipe_id),
    (SELECT id FROM ingredient_names WHERE name = 'tomato sauce'),
    '2 tbsp'
  ),
  (
    (SELECT id FROM recipe_id),
    (SELECT id FROM ingredient_names WHERE name = 'ground beef'),
    '200 g'
  );

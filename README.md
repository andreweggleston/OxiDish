# OxiDish
A Rust CRUD app for storing recipes, with Axum and Sqlx. Heavily borrowed from https://github.com/davidpdrsn/realworld-axum-sqlx. 

## API Reference

### Recipes
#### Listing recipes
<details>
 <summary><code>GET</code> <code>/api/recipes</code> </summary>

##### Parameters

> | name                                |  type       | data type         | description                                                                      |
> |-----------------------------|------------|------------------|--------------------------------------------------------------|
> | ingredient_name_ids      |  optional | Integer Array  | list of ids of ingredient names to filter recipes by. If not provided, all recipes will be returned       |

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{"recipes": [{id, title, description}..]}`                         |

</details>

#### Creating a recipe
<details>
 <summary><code>POST</code> <code>/api/recipes</code> </summary>

##### Parameters

> | name              |  type        | data type  | description                                                                      |
> |-----------------|------------|-------------|--------------------------------------------------------------|
> | title                 |  required | String                       | Title for new recipe       |
> | description     |  required | String                       | description for new recipe       |
> | ingredients     |  optional | Array of {int,int,int} | array of `RecipeIngredientDto` objects with format: `{id_ingredient_name: integer, id_ingredient_quantity: integer, id_ingredient_unit: integer}`    |

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{id, title, description, ingredients: [{id_ingredient_name, id_ingredient_quantity, id_ingredient_unit}..]}`                         |

</details>

------------------------------------------------------------------------------------------

### Ingredient Names
#### Listing ingredient names
<details>
 <summary><code>GET</code> <code>/api/ingredients/names</code> </summary>

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{"names": [{id, name}..]}`                         |

</details>

#### Creating an ingredient name
<details>
 <summary><code>POST</code> <code>/api/ingredients/names</code> </summary>

##### Parameters

> | name              |  type        | data type  | description                                                                      |
> |-----------------|------------|-------------|--------------------------------------------------------------|
> | name               |  required | String                       | new ingredient name     |

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{"id": integer, "name": string}`                         |

</details>

------------------------------------------------------------------------------------------

### Ingredient Quantities
#### Listing ingredient quantities
<details>
 <summary><code>GET</code> <code>/api/ingredients/quantities</code> </summary>

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{"quantities": [{id, quantity}..]}`                         |

</details>

#### Creating an ingredient quantity
<details>
 <summary><code>POST</code> <code>/api/ingredients/quantities</code> </summary>

##### Parameters

> | name              |  type        | data type  | description                                                                      |
> |-----------------|------------|-------------|--------------------------------------------------------------|
> | quantity               |  required | String                       | new ingredient quantity     |

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{"id": integer, "quantity": string}`                         |

</details>

------------------------------------------------------------------------------------------

### Ingredient Units
#### Listing ingredient units
<details>
 <summary><code>GET</code> <code>/api/ingredients/units</code> </summary>

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{"units": [{id, unit}..]}`                         |

</details>

#### Creating an ingredient unit
<details>
 <summary><code>POST</code> <code>/api/ingredients/units</code> </summary>

##### Parameters

> | name              |  type        | data type  | description                                                                      |
> |-----------------|------------|-------------|--------------------------------------------------------------|
> | unit               |  required | String                       | new ingredient unit     |
> | truncation    |  required | String                       | new ingredient unit truncation     |

##### Responses

> | http code   | content-type                             | response                                                            |
> |--------------|------------------------------------|-------------------------------------------------------- |
> | `200`         | `application/json`                | `{"id": integer, "unit": string, "truncation": string}`                         |

</details>

------------------------------------------------------------------------------------------

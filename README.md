# burger-api
## Usage
### Database connection
Install MongoDB server : `https://www.mongodb.com/docs/manual/tutorial/install-mongodb-on-windows/#std-label-install-mdb-community-windows`
Make sure you have connected to your local MongoDB server with `mongodb://localhost:27017/?directConnection=true` connection URI.
***This application currently works only with local instance of MongoDB database with that URI.***
### Endpoints
```
POST /api/create_burger - creates a document of Burger model in a database 
POST /api/create_ingredient - creates a document of Ingredient model in a database 
GET /api/burger/search/{name} - searches Burgers by given {name}
GET /api/burger/searchletter/{letter} - searches Burgers by first {letter} in name
GET /api/burger/lookup/{id} - lookup full Burger details by {id}(ObjectId)
GET /api/burger/random - lookup a random Burger
GET /api/burger/randomselection - lookup 10 random Burgers
GET /api/burger/latest - lists most latest Burgers
GET /api/burger/filteri/{ingr} - searches Burgers by given ingredient {ingr}
GET /api/burger/filterv/{diet} - filters vegan or non-vegan Burgers in the database(currently supported {diet} values are "Vegan" and "Non_Vegan")
GET /api/burger/filterc/{category} - filters Burgers by {category}
GET /api/ingredient/lookup/{id} - lookup Ingredient by {id}
GET /api/ingredient/search/{name} - searches Ingredient(s) by given {name}
GET /api/ingredient/list - lists all Ingredients in the database
```

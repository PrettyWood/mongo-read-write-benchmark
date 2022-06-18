package main

import (
	"context"
	"fmt"
	"math/rand"
	"time"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

const TOTAL_DOCS = 1_000_000

func main() {
	client_uri := "mongodb://localhost:27017"
	client, _ := mongo.Connect(context.Background(), options.Client().ApplyURI(client_uri))

	collection := client.Database("test").Collection("go")

	collection.Drop(context.Background())

	// INSERT DATA

	authors := [3]string{"Eric", "Chiara", "Stew"}
	docs := []interface{}{}

	for i := 1; i <= TOTAL_DOCS; i++ {
		docs = append(docs, bson.D{{"title", fmt.Sprintf("my doc %d", i)}, {"author", authors[rand.Intn(3)]}})
	}

	start := time.Now()

	collection.InsertMany(context.Background(), docs)

	elapsed := time.Since(start)
	fmt.Printf("Insert time for %d docs: %.2fs\n", TOTAL_DOCS, elapsed.Seconds())

	// RETRIEVE DATA

	start2 := time.Now()

	cursor, _ := collection.Find(context.Background(), bson.M{"author": "Chiara"})

	var books []bson.D
	cursor.All(context.Background(), &books)
	found_books := len(books)

	elapsed2 := time.Since(start2)
	fmt.Printf("Read time for %d docs: %.2fs\n", TOTAL_DOCS, elapsed2.Seconds())
	fmt.Printf("Amount of found books: %d\n", found_books)
}

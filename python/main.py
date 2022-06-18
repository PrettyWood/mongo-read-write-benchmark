import asyncio
import random
import time

import motor.motor_asyncio
import motor.core


TOTAL_DOCS = 1_000_000


async def main():
    client_uri = "mongodb://localhost:27017"
    client = motor.motor_asyncio.AsyncIOMotorClient(client_uri)
    collection: motor.core.AgnosticCollection = client["test"]["python"]

    collection.drop()

    # INSERT DATA

    authors = ["Eric", "Chiara", "Stew"]
    docs = [
        {"title": f"my doc {i}", "author": random.choice(authors)}
        for i in range(1, TOTAL_DOCS + 1)
    ]
    start = time.time()
    await collection.insert_many(docs)
    print(f"Insert time for {TOTAL_DOCS} docs: {(time.time() - start):.2f}s")

    # RETRIEVE DATA

    start = time.time()

    cursor: motor.core.AgnosticCursor = collection.find({"author": "Chiara"}).sort(
        "title", 1
    )
    books = await cursor.to_list(None)
    found_books = len(books)

    print(f"Read time for {TOTAL_DOCS} docs: {(time.time() - start):.2f}s")
    print(f"Amount of found books: {found_books}")


asyncio.run(main())

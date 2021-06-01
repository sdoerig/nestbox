# nestbox

## Idea

Some birdwatcher are also taking care of nestboxes. So these nestboxes are regularly checked during wintertime. To give you an idea, we're taking care of nearly 400 boxes. 
If a bird has been using the nestbox one can distinguish the specific bird by the nest built. Then the birdwatcher cleans the box, which means the nest is removed and the box is ready for the next breeding. Normaly there is also a list keept, which nestboxes where used by which birds and which year. So my intention is to give the nestbox cleaner a tool as today nearly everybody has a smartphone available. So the key thoughts are:

- A QR code tag on each nestbox.
- A authenticaated cleaner now can scan the QR code and perform the follwing action.
  - Set a new geolocation for the box, so it can be found later.
  - Set a detected breeding (type of bird and year)
- Somebody just walking by can also scan the QR code an gets the following informations:
  - Association responsible for the box and how to contact them.
  - History of the box, breedings in the past.
- Getting the associations taking care of those boxes "out of the dark", since they do valuable protection work and many people do not know the even exist.
- Personally I find it fun developing this kind of application beside work. Any backend like software is written in Rust - not because I'm exceptionally good at Rust, but I like it and I like learning too.

## Repo
Currently it is a multipart repo containing the parts

- dateabase_bouncycastle
  - This is uses to stock the database (mongodb) with data, since it can be bothersome to develop something on an empty database.
  
- nestboxd
  - Contains the backend server part.

## database_bouncycastle
As written above, this is only to fill up the database with some example data, 
```
Usage: target/release/database_bouncycastle -d mongodb://127.0.2.15:27017/?w=majority -n 123

Options:
    -d, --database_uri MONGO_DB_URI
                        URI to mongodb e.g mongodb://<db_host>:<db_port>/
    -n, --number of records to insert NUMBER
                        The number of records to insert
```
E.g. if executed with -5 12 the follwing number of records are inserted:

- 2 records in the collestion mandants
- 12 records in the collection nestboxes
- 12 records in the collection users
- 72 records in the collection geolocations
- 72 records in the collection breeds
- 300 records in the collestion birds

As written above, database_bouncycastle is only to stock the database with example data. A reasonable nomber of nestboxes are 10 millions, so one can see at a glance where indices are missing. Each 100 nextboxes a new mandant will be generated holding 100 nestboxes. Each mandant owns 150 birds, each nestbox has 6 geolocations and 6 breeds.

## Database model

### General

Allthough mongodb is a schemaless database, the concept of the model can described easy. Note it is just a skeleton, which means I just included the attributs I really consider to tbe vital. 

### uuid

In each record one finds an attribut `uuid`. It acts as public visible primary key, since I think on should not unveil to much about the database and mongodb object ids are guessable. So the uuid is randomly generated. Any field within a collection named "uuid" is a public accessable key and can be used in URLs an so on. They all are randomly generated type 4 uuids.

### Model

Before going into details of each collection here is a quick overview ot the relations of the collections.

```

mandant 1 ----- n users
|
+----- 1 ------ n nestboxes 1 ----- n breeds
                  |
                  + n ------------- n geolocation

```

### mandants

A mandant is an association of birdwatchers if you want so. They are taking care of a cuple of birdboxes in a defined geografical area. One record holds now the attributes

- _id: ObjectId - not used in any context now.
- uuid: Public accessable key e.g. in URLs. Example e7620353-b6f6-47e9-b543-66af20769145
- name: Name of the association, E.g. BirdLife 
- website: Most of these associations do have a website e.g. https://www.birdwatcher.ch
- email: Obvious e.g. bird@iseeyou.ch 


### users

A user belongs to a mandant and can mutate any birdbox belonging tho this mandant. Currently a simple password based authentication is implemented. The attributs:

- _id: ObjectId - not used in any context now. 
- mandant_uuid: user belongs to this mandant. 
- username: login name of the user  
- uuid: Public accessable key 
- lastname
- firstname
- email
- password_hash: Salted SHA3 hash of the password
- salt: Type 4 uuid

### nestboxes

A nestbox represents by concept a QR code referencable item. In our case it would be a wooden nestbox e.g. hanging in a tree.

- _id: ObjectId 
- public: Is the nestbox data public - true or false 
- uuid: Public accessable key
- mandant_uuid: Nestbox belongs to this mandant 
- created_at: ISODate Zulu time

### geolocations 

Geolocation indicating where the nestbox was placed over the time.

- _id: ObjectId
- uuid: Public key of the geolocation 
- nestbox_uuid: nestbox the location is attached to
- from_date: Geolocation valid from, timestamp Zulu time
- until_date: Geolocation valid until, timestamp Zulu time
- position: Geospatial type point 

### breeds

Holds track of all the breeds.

- _id: ObjectId
- uuid: Public key 
- nestbox_uuid: Breed discovered in this nestbox
- user_uuid: Breed discoverd by this user 
- discovery_date: Breed discovered at timestamp Zulu time
- bird_uuid: Estimated bird according to the nest found in the box

### birds

This collection stores all the birds of one mandant. Each mandant must create its own birds. The reasons for this redundancy are

- different location different birds
- different clima, different birds
- different altitude, different birds
- different language, different birds

The attributes are

- _id: ObjectId
- uuid: Public key of the bird
- bird: Name of the bird
- mandant_uuid: Mandant by which the bird was created










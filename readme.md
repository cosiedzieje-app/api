# Endpoints:

## [GET]/markers:

Returns all inserted markers in the form of a JSON array:

```
[{"id":2,
"latitude":50.21212,
"longtitude":1.12121,
"title":"pomoc dzieciom",
"type":"NeighborHelp"},
{"id":3,
"latitude":50.21212,
"longtitude":1.12121,
"title":"pomoc dzieciom", "type":"NeighborHelp"}]
```
###### Structure of a marker returned by this endpoint:

- id: number {the id of the marker}
- latitude: number (double) {coordinate}
- longtitude: number (double) {coordinate}
- title: string {title of the marker}
- type: Enum (NeighborHelp/Happening/Charity) {type of the event}

## [GET]/markers/\<id>:

Returns full information about Marker with ID \<id>:

```
{"id":1,
"latitude":50.21212,
"longtitude":1.12121,
"title":"pomoc dzieciom",
"description":"sasas",
"type":"NeighborHelp",
"addTime":1665747948,
"endTime":1666784387,
"address":{
	"street":"Jagiellonska",
	"postalCode": "41-207",
	"country": "Poland",
	"number":13
},
"contactInfo":"5151",
"userID":1}
```

if the ID is not present in the DB, it responds with [SomsiadStatus::error](#somsiadError)

```
{"status":"error",
"errors":["Invalid ID"]}
```

<a name="markerStructure"></a>Structure:
- id: number {the id of the marker}
- latitude: number (double) {coordinate}
- longtitude: number (double) {coordinate}
- title: string {title of the marker}
- description: string {description of a given event}
- type: Enum ("NeighborHelp"/"Happening"/"Charity") {type of the event}
- addTime: Unix milis {Time when the marker got added}
- endTime: Unix milis/Nothing {If present - time when the marker expires}
- address: [address](#addressStructure)
- contactInfo: not sure yet, up to you to decide {probably either phone number or email}
- userID: number {ID of the user that added the given marker}

## [POST]/add_marker:
Adds marker which will be supplied by frontend in a JSON format
e.g:
```
{
	latitude:50.21212,
	longtitude:1.12121,title:"pomoc dzieciom",
	description:"sasas",
	type:"NeighborHelp",
	addTime:1665747948,
	endTime:1666784387,
	address: {
		postalCode: "41-207",
		street: "Jagiellonska",
		number: 13,
		country: "Poland"
	},
	contactInfo:"5151",
	userID:1
}
```
[Structure](#markerStructure)

Responds with [SomsiadStatus](#somsiadStatus)

## [GET]/rm_marker/\<marker_id>
Removes the marker with ID \<marker_id>, checking if it is being removed by the user who added it (checks if user_ID in private cookie and the given database row is equal)

Responds with the Marker that has been removed or [SomsiadStatus::error](#somsiadError)

## [POST]/register
Tries to register an user, given their credentials
e.g:
```
  {
    login: {
	email: 'example@sasad.com',
	password: "toor"
    }, 
    username: "lol",
    name: "Paweł",
    surname: "Kowalski",
    sex: 'Male',
    reputation: 1337,
    address: {
      postalCode: "42-230",
      street: "Jagiellonska",
      number: 13,
      country: "Poland"
    }
  }
```
<a name="userStructure"></a>Structure:
- login: JSON array:
    -email: string, validated server-side, unique {email of the user being registred}
    -password: string {unhashed password of the user being registered}
- username: string, unique {username of a given user}
- name: string {real world name}
- surname: string {self-explanatory}
- sex: Enum ('Male','Female','Other') {self-explanatory}
- reputation number {Reputation points of the user, should probably be set to 0 during registration}
- address: [address](#addressStructure)

Responds with [SomsiadStatus](#somsiadStatus)

## [POST]/login
Tries to login with given credentials
e.g.:
```
	{
		email: 'example@gmail.com',
		password: "toor"})
	}
```
Structure:
- email: string, validated server-side, unique {email of the user being registred}
- password: string {unhashed password of the user being registered}

Responds with [SomsiadStatus](#somsiadStatus)
If user logs in correctly, it sets a private cookie which represents their id

## [GET]/logout:
Removes the private cookie that indicates that a user is logged in

## [GET]/user_data:
If the user has the private cookie, which indicates that they are logged in set, it displays data about them:
e.g:
```
{"login_name":"root",
"name":"Paweł",
"surname":"Kowalski",
"email":"example@gmail.com",
"sex":"Male",
"address":{
	"street":"Jagiellonska",
	"postalCode": "41-207",
	"country": "Poland",
	"number":13
},
"reputation":1337}
```
Structure:
[user structure](#userStructure) but without the password

If the user doesn't have that cookie set it responds with SomsiadStatus error
Responds with [SomsiadStatus](#somsiadStatus)

# Types/Structures: 

## <a name="somsiadStatus"></a>SomsiadStatus type:
If everything goes correctly it is:
```
{
    "status": "ok",
    "errors": []
}
```
<a name="somsiadError"></a>If it does not, it is:
```
{
    "status": "error",
    "errors": [
        "Podany e-mail jest zajęty"
    ]
}
```
where the errors array entry is the error that has occured

## <a name="addressStructure"></a>Structure of the address field:

- postalCode: string {self-explanatory}
- street: string {only street of the address}
- number: number {number of the house}
- country: string {self-explanatory}

e.g:
```
    address: {
      postalCode: "42-230",
      street: "Jagiellonska",
      number: 13,
      country: "Poland"
    }
  }
```

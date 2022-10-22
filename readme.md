# Endpoints:

## <a name="markers"></a> [GET]/markers:

Returns all inserted markers in the form of a JSON array:

```
[
	{
		id:2, 
		latitude:50.21212,
		longitude:1.12121,
		title:"pomoc dzieciom",
		type:"NeighborHelp"
	},
	{
		id:3,
		latitude:50.21212,
		longitude:1.12121,
		title:"pomoc dzieciom", 
		type:"NeighborHelp",
		userID:1
	}
]
```
###### Structure of a marker returned by this endpoint:

- id: number {the id of the marker}
- latitude: number (double) {coordinate}
- longitude: number (double) {coordinate}
- title: string {title of the marker}
- type: Enum (NeighborHelp/Happening/Charity) {type of the event}
- userID: number {ID of the user who added given marker}

## [GET]/user_markers:

Returns all markers added by the currently logged in user:

```
[
	{
		id:1,
		latitude:50.21212,
		longitude:1.12121,
		title:"pomoc dzieciom",
		description:"sasas",
		type:"NeighborHelp",
		addTime:1665747948,
		endTime:1666784387,
		address:{
			street:"Jagiellonska",
			city: "Sosnowiec",
			number:"13"
		},
		contactInfo:{
			name: "Paweł",
			surname: "Kowalksi",
			address: {
				postalCode: "41-207",
				street: "Jagiellonska",
				number: "13",
			},
			method:{
				type: "PhoneNumber",
				val: "123456789"
			}
	}
]
```

## [GET]/markers/\<id>:

Returns full information about Marker with ID \<id>:

<a name="fullMarkerStructure"></a>
```
{
	id:1,
	latitude:50.21212,
	longitude:1.12121,
	title:"pomoc dzieciom",
	description:"sasas",
	type:"NeighborHelp",
	addTime:1665747948,
	endTime:1666784387,
	address:{
		street:"Jagiellonska",
		city: "Sosnowiec",
		number:"13"
	},
	contactInfo:{
		name: "Paweł",
		surname: "Kowalksi",
		address: {
			street: "Jagiellonska",
			city: "Sosnowiec",
			number: "13",
		},
		method:{
			type: "PhoneNumber",
			val: "123456789"
		},
	userID: 1
}
```

if the ID is not present in the DB, it responds with [SomsiadStatus::error](#somsiadError)

<a name="markerStructure"></a>Structure:
- id: number {the id of the marker}
- latitude: number (double) {coordinate}
- longitude: number (double) {coordinate}
- title: string {title of the marker}
- description: string {description of a given event}
- type: Enum ("NeighborHelp"/"Happening"/"Charity") {type of the event}
- addTime: Unix milis {Time when the marker got added, optional when adding}
- endTime: Unix milis/Nothing {If present - time when the marker expires}
- address: [address](#addressStructure)
- contactInfo: [contactInfo](#contactInfoStructure)
- userID: number {ID of the user who added given marker}

## [GET]/markers/\<city>
Gets all markers where the city is \<city>, in the same form as the [/markers](#markers)

## [GET]/markers?\<lat>&\<long>&\<dist>
Gets all markers within distance \<dist> of given coordinates \<lat> (latitude) and \<long> (longitude)
returns them in the same form as [/markers](#markers)

## [PUT]/marker:
Adds marker which will be supplied by frontend in a JSON format
e.g:
```
{
	latitude:50.21212,
	longitude:1.12121,title:"pomoc dzieciom",
	description:"sasas",
	type:"NeighborHelp",
	addTime:1665747948,
	endTime:1666784387,
	address: {
		city: "Sosnowiec",
		street: "Jagiellonska",
		number: "13",
	},
	contactInfo:{
		name: "Paweł",
		surname: "Kowalksi",
		address: {
			city: "Sosnowiec",
			street: "Jagiellonska",
			number: "13",
		},
		method:{
			type: "PhoneNumber",
			val: "123456789"
		}
	}
```
[Structure](#markerStructure)

Responds with [SomsiadStatus](#somsiadStatus)

## [DELETE]/markers/\<marker_id>
Removes the marker with ID \<marker_id>, checking if it is being removed by the user who added it (checks if user_ID in private cookie and the given database row is equal)

Responds with the [FullMarker](#fullMarkerStructure) that has been removed or [SomsiadStatus::error](#somsiadError)

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
      city: "Sosnowiec",
      street: "Jagiellonska",
      number: "13",
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
- reputation number {Reputation points of the user, optional}
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

## [GET]/is_logged:
Return SomsiadStatus::ok if the user is logged in, if not it returns SomsiadStatus::error
[SomsiadStatus](#somsiadStatus)

## [GET]/user/\<id>
Gets public info about the user with id \<id>

###### Structure of PublicInfo:
```
{
	username:"root",
	name:"Paweł",
	surname:"Kowalski",
	sex:"Other",
	reputation:0
}
```

## [GET]/user_data:
If the user has the private cookie, which indicates that they are logged in set, it displays data about them:
e.g:
###### Structure of PrivateInfo:
```
{
	username:"root",
	name:"Paweł",
	surname:"Kowalski",
	email:"example@gmail.com",
	sex:"Male",
	address:{
		street:"Jagiellonska",
		city: "Sosnowiec",
		number: 13
	},
	reputation:1337
}
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
    status: "ok",
    res: null 
}
```
<a name="somsiadError"></a>If it does not, it is:
```
{
    status: "error",
    res: [
        "Podany e-mail jest zajęty"
    ]
}
```
where status is either "ok" or "error", if it is "ok" then res is null and if it is erros then res is a array containing all of the errors that have occured

## <a name="addressStructure"></a>Structure of the address field:

- postalCode: string {self-explanatory}, this is optional, does not need to be in structure or can be just null
- street: string {only street of the address}
- number: string (since houseNumbers can be 55a) {number of the house}
- country: string {self-explanatory}

e.g:
```
    address: {
      street: "Jagiellonska",
      number: "13",
      city: "Sosnowiec"
    }
  }
```

## <a name="contactInfoStructure"></a>Structure of the ContactInfo field:

- name: string {self-explanatory}
- surname: string {self-explanatory}
- address: [addres](#addressStructure)
- method:
	- type: Enum ("PhoneNumber"/Email) {type of the contact method} 
	- val: string {Either phone number or email with which the user adding the marker can be contacted}

e.g:
```
contactInfo:{
	name: "Paweł",
	surname: "Kowalksi",
	address: {
		postalCode: "41-207",
		city: "Sosnowiec",
		number: 13,
	},
	method:{
		type: "PhoneNumber",
		val: "123456789"
	}
}

```

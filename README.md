# SmartHomeWeb
There is simple project based on Rust which created while studying Rust programming language on Otus platform. 

### Target:

Learn how to use Rust web frameworks.
The result is: Smart home web server.

Description / Step-by-step instructions for completing homework:

Implement using the web framework an HTTP server that implements the "Smart Home" functionality:
Library "Smart Home" provides a dynamic structure of the house in the rooms of which devices are located.

 - The house has a name and contains several rooms.
 - The library allows you to query the list of rooms in the house, as well as add and remove rooms.
 - The room has a unique name and contains the names of several devices.
 - The device has a unique name within the room.
 - The library allows you to get a list of devices in the room, as well as add and remove devices.
 - The library has a function that returns a textual report about the state of the house.
 - This function takes as an argument a generic type that allows you to get text information device status to include in the report. This information must be provided for each device based on the location of the device in the house: room name and device name. If the device is not found in the information source, then return an error message instead of a status text.

An arbitrary mock object can be used as a source of device state data.
Write a client with requests to the smart home HTTP API.
Write an example of communicating with a smart home via an HTTP client.
Additional task: store the structure of the house in the database.

### Criteria for evaluation:

The "Accepted" status is set if:

 - There is all the functionality from the description.
 - Functional tests from the description are performed.
 - The cargo clippy utility does not issue warnings.
 - The cargo fmt --check command produces no warnings.

### Competencies:

Knowledge of the Rust language
 - creating a web server
 - 

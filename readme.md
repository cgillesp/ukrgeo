# ukrgeo

A collaborative geolocation server for the Ukraine conflict written in Vue + Typescript and Rust on the backend. Currently you can add provisionally add geolocations to the database, but the server uses a cookie to only show unverified geolocations to the person who submitted them.

TODO:

- Moderation queue
- Nicer location pop-ups
- Fix Vite optimization bug that causes the pin not to show up when adding a new location

![Demo image](ukrgeo.png)

[Demo instance](https://ukrgeo.x.charliegillespie.com)
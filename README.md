# HTMX: A Demonstration

This simple site serves up a server rendered web page that displays the current day and time. The date and time are grabbed once on page load. If you hit the refresh button of your browser, the values will update.

Eventually, I hope to make this site interactive so that you can hit a button and update the clock. I will eventually do this using [htmx](https://htmx.org).

This site is not something you would use in production. It's a learning ground for me to cover:

- [Actix-Web framework](https://actix.rs)
- [Tera](https://keats.github.io/tera/) a templating engine for Rust
- the actix-files crate, for serving static files with Actix-Web
  - CSS and the site favicon are served this way
- chrono crate, for working with the date and time
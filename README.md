STOODLY
=======

WARNING

This project is under construction and is subject to my chaos that includes rebasing commit history and major refactoring. Once the MVP is done and deployed later this year, expect order and proper development behavior to ensue with the chaos coming to a complete stop.

TODO
----

- [ ] Setup IDP, authentication, and read JWT into the service.

- [x] Break out project into a multi-crate project.

        Project:
        * Stoodly        
        Crates:
        * client
        * server 
        * status 
        * account 
        * organization 

- [ ] Update [README.md](README.md)

SUMMARY
-------

Escape the tiresome rituals of Stand-Up without losing the benefits!

Asynchronously provide your daily stand-up by answering the following questions:

1. What did I do yesterday?
2. What did I do today?
3. Any blockers or impediments?

INSTALLATION
------------

[Install](INSTALL.md)

DEVELOPER SETUP
---------------

##### You have a working [Rust environment].

```
git clone https://github.com/stoodly/stoodly
cd stoodly
cargo run
```

##### You have a working [Docker environment].

```
git clone https://github.com/stoodly/stoodly
cd stoodly
docker-compose up
```

CHANGELOG
---------

[Changelog](CHANGELOG.md)

LICENSE
-------

* [MIT License](LICENSE-MIT)
* [Apache License 2.0](LICENSE-APACHE)

CONTRIBUTING
------------

[Contributing](CONTRIBUTING.md)

[Docker environment]: https://docs.docker.com/engine
[Rust environment]: https://www.rust-lang.org/tools/install
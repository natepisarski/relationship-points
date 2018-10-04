# Research Outcomes
* Rust makes a particularly fantastic web backend. Rust's ability to gaurauntee safety in a highly asynchronous workload does wonders for a web server, which is highly asynchronous.
* Statically typed frontends are, if not a necessity, then damn close to it. So much safety is gained, and so much time is saved, by using technologies like Flow. Contracts become concrete in the form of types. Tools get better autocompletion (which, personally, has been an incredible time saver).
* **REST** may by its very nature cause a proliferation in the codebase that you just don't see in other technologies (i.e GraphQL)

### Origin
Relationship-Points arose out of a particularly interesting hypothetical that a friend and I were discussing. What if there were a system, similar to Google Reviews or Glassdoor, where the thing under review were romantic relationships?

I found the concept amusing, but rather than developing it into a full-fledged app that did just that, I decided that it would
be most valuable for me to use the concept as a test-bed for a R-R-R (Rust / React / Redux) application. The same stack is
currently being used in a private repository, which is much better off because of the research done as part of `relationship-points`.

### Technical Features
Since this is simply here for research, there are a lot of either half-baked or improvable things. With that said, the goal
of the repository was to reach a point where:

* A custom database component library (rust) handled **database**->**service layer**
* A custom hierarchical serialization framework (rust) handled **rust web server** -> **redux state store**
* The frontend adhered strongly to domain-driven principles. Static type checking stuff (like Typescript and Flow) would be responsible for this part.

# Where it is today
Inside there are in-progress (but, for now, working) versions of this CRUD component library, serialization framework, and web server.
There are numerous performance and ergonomic optimizations that would need to be made if any of this were to be used in a serious project. Since the day this project ended I took the principles I learned here
to build a much more realistic backend for another long running project.

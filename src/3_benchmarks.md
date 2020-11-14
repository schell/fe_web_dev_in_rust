# Benchmarks

Using the canonical TodoMVC for most of these libraries (and more) you can bench
their performance with regard to DOM updates.
[Try it yourself here](http://schell.github.io/mogwai/todomvc/perf/). Here are
the results I got:

![rust todomvc bench](./img/perf.png)

I believe `mogwai` is fastest here specifically because of the lack of vdom. In
my opinion it hits a good power-to-weight ratio and achieves its goals. I'm
biased though because I wrote it, lol 😊.

I hope you enjoyed this guide. You can find others like it at my web log -
[zyghost.com][z]. If you would like to contribute please sponsor me [at github][g],
a coffee a day goes a long way <3.

[z]: https://zyghost.com
[g]: https://github.com/schell

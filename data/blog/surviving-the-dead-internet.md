Nearly [half](https://www.independent.co.uk/tech/internet-bots-web-traffic-imperva-b2339153.html) of today’s internet is made up of bot traffic, and if you’ve touched pretty much any search engine or social media in the past 5 years, you’ve probably felt the ramifications of this paradigm shift. It’s just going to get worse, too, following the recent rise of ostensibly informative but meaningfully vacant GPT-generated blog posts, tutorials, and StackOverflow answers, feeding into an already-festering pool of SEO junk. In short, we live in a post-information age: Instead of the world on a silver platter, we’re handed a clump of soil processed into a brick, baked into a sphere of molten plastic and painted so we can’t tell the difference.

So what do we do?

---
## regarding search engines
Search engines, for me, seem to be the prime suspect in the internet's murder. Especially when searching for instructions on anything remotely technical, I've found it's often necessary to sift through a mountain of SEO-optimized garbage that explains every irrelevant detail of the problem *except* for the actual solution. Here's how i've started trying to deal with the problem:

### not using google (except maybe google scholar)
Hosting a search index is hard <sup>_[citation needed]_</sup>. Pretty much all that exists for people who aren't complete goobers is Google, Bing, and *maybe* Brave (DuckDuckGo just uses Bing's). Of these three, I'd say I've had the best experience with Bing: Google seems to have more junk than the others, and while Brave shows you actual human discussions, it's by far the worst at giving accurate results for more difficult queries.

If you are a goober, though, you might want to check out metasearch engines like [SearX](https://metasearx.com/), which amalgamate results from a customizable array of sources. SearX in particular avoids sharing your IP address with search engines, provides direct links rather than tracked redirect links, and even allows hosting your own instance, allowing for a degree of customizability and objectivity you really can't find anywhere else.

### blocking search results
If you're sticking with a normal engine, though, there are still ways to filter through the noise. Recently, I've had some luck with [uBlacklist](https://github.com/iorate/ublacklist), a plugin that allows you to blacklist domains directly from the list of search results. Though there may seem to be an endless list of non-information sites for any given search, you can make genuine headway by just blocking the domain of each bad result you find. I'd also recommend subscribing to one of uBlacklist's [spam rulesets](https://iorate.github.io/ublacklist/subscriptions) for a bit of a head start.

### literally just appending "reddit" after any search
If you need a silver bullet, this is the one. Enough said.

---
## regarding social media
It’s no secret that social media site updates are made pretty much for the express purpose of optimally exploiting your attention, usually by substituting meaningful information for empty calories of algorithmically-selected “content”. Ostensibly innocent UI updates maintain these goals as well, making it as easy as possible to drift from the meaningful to the meaningless. Naturally, then, it seems that the best way to combat this is reverting to the old.

### twitter
This manifests most clearly in the case of Twitter, where newer iterations try their hardest to drag you away from your following feed and toward the algorithmically-curated “For You.” I’d recommend dimden’s plugin [Old Twitter Layout (2023)](https://addons.mozilla.org/en-US/firefox/addon/old-twitter-layout-2022/); I’ve found it to be a much better experience psychologically and ergonomically, deobfuscating the information you’re really there for.

### reddit
You can revert to an old Reddit UI by prepending replacing Reddit.com with old.reddit.com, but that’s stupidly inconvenient, so I’d again recommend a plugin, namely tomjwatson's [Old Reddit Redirect](https://addons.mozilla.org/en-US/firefox/addon/old-reddit-redirect/).

### on mobile
It’s obviously a lot harder to filter through the internet's dump heap when you're locked to a carefully curated set of apps. This is less of an issue with Android's generous Play store, but iOS makes it a real pain. My trusty workaround has been [AltStore](https://altstore.io/), a pretty reliable and easy way to sideload external apps. With it, you can install apps like [UYouPlus](https://github.com/qnblackcat/uYouPlus), a modified YouTube client which allows you to customize the content you see (including ads and sponsors!), and [BHTwitter](https://github.com/BandarHL/BHTwitter), a similarly-customizable Twitter client.

---
## where else to look
With all of that said, search engines and social media are *bad* places to look for information. Here are some much better alternatives:
- Community-curated [Awesome lists](https://github.com/sindresorhus/awesome), providing resources for everything from [computer science](https://github.com/prakhar1989/awesome-courses#computer-graphics) to [music](https://github.com/noteflakes/awesome-music#readme) and even [fantasy literature](https://github.com/RichardLitt/awesome-fantasy#readme)
- Internet libraries such as [libgen.is](https://libgen.is/) or [halcyon.ooo](https://www.halcyon.ooo/)
- Your local library! Don't take it for granted!
- Wikipedia (despite what your english teacher tells you)

---

With that, I've exhausted pretty much every tool I have for surviving the dead internet. I hope you found this at least a little helpful. Now stop wasting your time, get out there and learn some cool shit.

Peace ✌️
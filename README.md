I tested someone else's code for vector search and I've never found anything that does vector search faster.

I worked on a vector search application for about 3 years in my spare time and at the time of writing, the author of the usearch library has been working on the same problem for 10 years.

I have decided to move on to other fruitful pursuits but I learned so much about computing by getting into the granular details of this one task.

If you need vector search, without metadata filter, this is the library to use. My library went through various iterations of reading (RAM, disk) and parsing (slurping, all at once), low-level languages (C, ASM, Rust) and features (geo-search included). I will look back at that period of my learning fondly but it is now behind me.

Some specific lessons from this project were benchmarking different hardware, benchmarking different low-level languages, caring about CPU cycles, lessons in developer experience (don't make me think but let me know what it does), the lesson that non-technical collaborators sometimes contribute technical feedback, etc.

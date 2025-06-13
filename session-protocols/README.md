# The Mandel.rs session protocols

This directory contains protocols of the sessions I had with AI helper programs while implementing the program. AI help has been an important cornerstone of this whole project and for me it is important to document how AI was involved.

The Claude Code protocols have been generated using an infinite console log and storing that after ending the session. I use `Konsole` and storing the content unfortunately looses the ANSI codes for color or font shape, so the records are a bit less expressive than the original dialog. Anyhow, three markers at the line beginning show what's going on:

* `>` are inputs written by the human (me).
* `●` are outputs written by Claude targeting me.
* `✻ Thinking…` is the start of an "internal dialog" of Claude with itself showing what the program destilles out of the input and how its plan to answer comes together. This is always followed by a `●` output.

Generally, there is much more output than input. This is somehow intended behaviour…

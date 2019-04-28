# free-speech
The idea is to try to build a free, cross-platform voice coding tool backed by Mozilla's DeepSpeech api. 

# Nothing to see here, yet
This is in the pre-protoype phase, just cobbling together crates and tools to see what's possible.

# Motivation
I was recently injured and unable to type. As a developer, and currently a remote one at that, it completely prevented me from working. Naturally, voice coding solutions came to mind. After spending a lot of time investigating what's out there, I realized that there is no good out of the box solution, or even good starting point. [Dragon Naturally speaking](https://www.nuance.com/dragon.html) tends to be the preferred recognition engine, but that is not free, and OS X support for Dictate has been discontinued. Moreover, as it is pre-baked and closed source, the voice coding community has/had very little control over the actual recognition piece of the puzzle.

As someone who was injured, the idea of buying some (old, unsupported) software and putting in months of coding to get something workable, was not even possible. The only "out of the box" solution that seemed workable was [VoiceCode](voicecode.io), but that is also built on Dragon, is not itself free, and does not seem to offer a trial. It looks like a promising project, though I'm not sure how actively maintained it is. 

So I decided to give this a shot with DeepSpeech in Rust.

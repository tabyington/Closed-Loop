# Closed-Loop
## WHAT: 
An artificial life simulator developed in Rust to enable speed and scale for large simulations. Closed Loop Requirements Development (CLRD) is the development process I am experimenting with. Human–AI collaboration is intrinsic to the architecture of this project.

### What is CLRD
Based on the description in Matt Shumer's article below I was tempted to call this spec driven development, SDD, it's similar to what some places are using. But it actually seems to go a step further.
* requirements are expressed to the AI system
* use cases, architectural decisions, and UI/UX are made by the AI
* code is generated and structured by the AI
* code is built and tested by the AI
* failures are sent back to be fixed by the AI
* when all requirements are met, and application meets stability and performance needs the completed release candidate is handed off to a human for review
* a human gives the final go or no-go for the software

## WHY:
1. I love ALife games but nothing has been released in a while with the depth I've been looking for. I'd like to create something with the biological depth and emergent behavior I feel is missing from other recent releases.
2. I've worked in software my whole life, but was laid off with my whole team a bit back. And I've still not been able to find a new position, and I'm not the only one. The accelerating capability of AI systems is reshaping how software is produced, and when I read this article by Matt Shumer https://shumer.dev/something-big-is-happening I realized, even in my optimistic views of technology and its capabilities, my understanding of the scope was behind the times. This paragraph was the particular motivator for trying this project the way I'm doing it:

> "I am no longer needed for the actual technical work of my job. I describe what I want built, in plain English, and it just... appears. Not a rough draft I need to fix. The finished thing. I tell the AI what I want, walk away from my computer for four hours, and come back to find the work done. Done well, done better than I would have done it myself, with no corrections needed. A couple of months ago, I was going back and forth with the AI, guiding it, making edits. Now I just describe the outcome and leave."

## WANT TRY IT OUT?
### Requirements:
Compile&linking Rust - https://forge.rust-lang.org/infra/other-installation-methods.html

### Running the command line simulation from repo root
cargo run -p cli

You will see something similar to this if all goes well:
```Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
Running `target/debug/cli`
Final state: EngineState { seed: 42, current_tick: 10000, births: 20, deaths: 16 }
Run artifacts written to runs/2026-02-18T22-44-47Z```

A new directory will be created to house a manifest and a metrics file which will display the state of the simulation every x ticks.

### Arguments (cargo run -- <arg>)
"--seed" set the starting seed to make simulations easier to test
"--ticks" set the number of ticks to run the simulation for
"--snapshot-interval" set how often in ticks the metrics file is written to with the current state
"--out-dir" sets the directory you want to put artifact runs to so your ci can be set up easier
"--run-id" sets the sub-directory so you can overrun existing old runs with new runs

cargo run -p cli -- \
  --seed 42 \
  --ticks 10000 \
  --snapshot-interval 1000 \
  --run-id test-run

will output
runs/test-run/
  manifest.json
  metrics.jsonl


## Roadmap

Engine behavioral complexity

Real resource modeling

Mutation and selection systems

Automated regression gates

CI-based experiment validation

Visualization tooling

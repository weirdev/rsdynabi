## Setup
1. Clone repo
2. Install buck https://buck2.build/docs/about/getting_started/
3. `git submodule update --init --recursive` (pulls down prelude)
4. Try build. If prelude issue may need to pull latest version `cd prelude & git pull origin main` f

## Commands

### Rust
  buck run rs:main

### C++
  buck run cpp:main

### ABI Generation
  buck run abigen:main
  buck build abigen:generate_cpp


## In Progress
- No usage of dyn args yet
- Passing child adapter to C++ call accepting parent class
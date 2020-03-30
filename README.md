# MSM
The Meat Session Manager
## Usage
### Configure
Set XSESSIONS_PATH in src/main.rs to the path containing your .desktop files. Modifications to the appearance of the session list can be done in the fmt function in the Display impl in src/session.rs or at the beginning of process_input in src/main.rs.
### Build
`cargo build --release`
### Install
`mv target/release/msm ~/.local/bin/` or any directory in your PATH

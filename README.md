# Class Schedule Helper

## Why?

[This reddit post](https://www.reddit.com/r/NoStupidQuestions/comments/6c3xcs/is_there_a_program_or_a_way_to_write_a_program/).

## How to use

### Prerequisites 

[Install Git](https://git-scm.com/)
[Install Rust](https://www.rust-lang.org/)

[Clone](https://help.github.com/articles/cloning-a-repository/) this repository into the folder of your choice:

```
git clone https://github.com/marcusball/class-scheduler.git
```

`cd` into the cloned repository, then try to build it:

```
cd class-scheduler

cargo build
```

If all goes well, it should build successfully. 

## Usage

Edit the `Classes.toml` to list all of the possible classes and class sections,
as well as all of the available periods during a day. 

Sample:

```
periods = [1,2,3,4,5,6,7]

[[classes]]
name = "calc"
sections = [
    ["MWF3"],
    ["MWF4-5"]
]

[[classes]]
name = "phys"
sections = [
    ["TR3", "W4"],
    ["TR4", "M3"]
]
```

For example, this defines a schedule that contains 7 periods per day. 
It also contains two possible classes, `calc` and `phys`, each with
two available sections.

The calculus class has a section on Monday, Wednesday and Friday during
period 3 (`MWF3`), as well as a section on the same days spanning both periods 
4 and 5 (`MWF4-5`). 

The physics class has also has *two* sections, but each section spans 
different periods during different days of the week. The first section 
meets on Tuesday (`T`) and Thursday(`R`) during period 3, then meets 
on Wednesday on period 4. The section section meets Tuesday/Thurday 
period 4 (`TR4`) and Monday period 3. 

To add new classes, simply just copy the format blocks adding the class name
and the periods of each of the available sections for the class.

*Note: class names are currently restricted to 9 character max*.

### Day notation

* `M` - Monday
* `T` - Tuesday
* `W` - Wednesday
* `R` - Thursday
* `F` - Friday

### Session notation

The sessions are just the letters for each day the session meets,
then the period, or period range for that session. 

If sessions are not continuous, they must be listed as separate sessions.
So, if a class meets on Monday at period 1, then meets again for period 3,
it must be listed as two sessions: `M1` and `M3`. 

*Periods must be numbers*.

#### Examples

* `MTWRF2` - meeting everyday at period 2. 
* `TR4-5` - meeting Tuesday and Thursday during periods 4 and 5. 
* `M9-10` - Monday from 9 to 10. 


## Running

After modifying the `Classes.toml`, run the program from the terminal 
using the command:

```
cargo run
```

This should dump all of the possible schedules based on the class sections
that do not conflict.

```
┌───┬───────────┬───────────┬───────────┬───────────┬───────────┐
│ # │  Monday   │  Tuesday  │ Wednesday │ Thursday  │  Friday   │
├───┼───────────┼───────────┼───────────┼───────────┼───────────┤
│ 1 │           │           │           │           │           │
│ 2 │           │           │           │           │           │
│ 3 │   calc    │   phys    │   calc    │   phys    │   calc    │
│ 4 │           │           │   phys    │           │           │
│ 5 │           │           │           │           │           │
│ 6 │           │           │           │           │           │
│ 7 │           │           │           │           │           │
└───┴───────────┴───────────┴───────────┴───────────┴───────────┘

┌───┬───────────┬───────────┬───────────┬───────────┬───────────┐
│ # │  Monday   │  Tuesday  │ Wednesday │ Thursday  │  Friday   │
├───┼───────────┼───────────┼───────────┼───────────┼───────────┤
│ 1 │           │           │           │           │           │
│ 2 │           │           │           │           │           │
│ 3 │   phys    │           │           │           │           │
│ 4 │   calc    │   phys    │   calc    │   phys    │   calc    │
│ 5 │   calc    │           │   calc    │           │   calc    │
│ 6 │           │           │           │           │           │
│ 7 │           │           │           │           │           │
└───┴───────────┴───────────┴───────────┴───────────┴───────────┘
```

Because it is very likely that there will be a large number of possible combinations,
I recommend sending the output to a text file and then reading the combinations from there:

```
cargo run > possible-schedules.txt
```

This will create a new file named `possible-schedules.txt` containing 
all of the generated schedules. 
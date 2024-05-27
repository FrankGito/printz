# Contracts

This is the Contracts Folder.

## Tech 
The Contract uses ink! version 5.
The Contract uses test macros for testing.

## Files
We are using interface to define the outcome,.  
`./item/lib.rs` - contains reimplentation for PSP34 in ink version 5.
`./item_sale/interfaces/lib.rs` - contains the Item Sale Interface

## Story of Creation

```
$ git log --oneline --reverse

---Initialise ink! contract---
3f9a5f4 :zap: (contracts) initialise item
---Initialise write interfaces---
ea997ea :zap: (contracts) write psp34 interface
66d6470 :zap: (contracts) write psp34_metadata interface
7272e40 :zap: (contracts) write psp34_mintable interface
79c1e52 :zap: (contracts) modularize interfaces
a409c0a :zap: (contracts) add variables
2127b3d :zap: (contracts) gocha, interface compiles and gets inluded, next implementation
3e951ba :zap: (contracts) add empty impl for PSP34
---First try---
9e829d6 :zap: (contracts) bare simple implenation, no approval thingies
52135f8 :zap: (contracts) impl mint & transfer & add tests
18946b4 :zap: (contracts) modularize export via macro
eaccf83 :zap: (contracts) adjust test_suite
5d5dffd :zap: (contracts) add approve
---Add Test Suite---
35d3c42 :cop: (contracts) add tests
62820dd :abc: (contracts) write tests.md for braindump
748a5c6 :zap: (contracts) gotcha, next up allowance test
3e950c9 :cop: (contracts) add allowance + test
63d5ef1 :zap: (contracts) initalise events
0a26e1b :zap: (contracts) add Events, add Psp34Metadata, add set_attribute
---Add Id---
8461531 :zap: (contracts) add id Enum, adjust till compile
9c133d1 :zap: (contracts) make Event AttributeSet work
14e326a :zap: (contracts) repair tests
f2768e1 :zap: (contracts) add enumerable & burnable to interfaces, for completion sake, rename to upercase to match standard
fb79a8c :zap: (contracts) adjust, hardcode atribute, use .into()
---Markify---
774b095 Update contracts build and nixify it
d049088 Clean up warnings and some implementation details
574c851 Calrify transfer method
a8f4516 Minor updates on counting
4fc2cfa :zap: (contracts) cargo fmt
9e57830 Fix allowance mechanism
---Clean up---
1f012f6 :zap: (contracts) adjust set_attribute, discovered incorrect check :see_no_evil:
47fa875 :zap: (contracts) allow only once attribute set for a given key
727477b Fill in contracts READMEs
5318ff8 :zap: (./README.md) modify README.md
```

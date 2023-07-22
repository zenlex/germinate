// scaffolding engine
/*
The basic idea for a given stack:
- ask for project name
- create a directory with slugified project name
- init package (npm init, cargo init, etc)
- install dependencies
- install dev dependencies - testing, linting, formatting, etc
- create a docker image from template file
- copy/patch templates for that stack
- git init
- commit initial files
- option to create github repo and push
 */

/* strategy
    - put stack name and description in stack_template file in subfolder
    - read subfolders to populate stack options in dialog? not sure if this is a good idea - if I do this, I need a mechanism to put questions/answers in the stack_template file I think and that seems like a possible rabbithole. Maybe just extract the stack definitions as much as possible so
    - once stack and options are selected (db, linters, etc), create a config struct
    - use config struct to create a builder struct
    - builder struct will have methods for each step of the scaffolding process (see above)
*/

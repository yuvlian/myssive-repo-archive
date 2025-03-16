@echo off
REM Store the current directory
set "originalDir=%cd%"

REM Check if the current directory is "source"
for %%I in ("%cd%") do set "currentDirName=%%~nI"

if /i "%currentDirName%" NEQ "source" (
    cd source
    REM Run the command in the "source" directory
    opam exec -- dune exec sdk_server/sdk_server.exe
    REM Return to the original directory
    cd "%originalDir%"
) else (
    REM Run the command without changing directories
    opam exec -- dune exec sdk_server/sdk_server.exe
)

REM put this file somewhere in your PATH
@echo off

for /f "tokens=*" %%I in ('d:\path\to\mktmp\target\debug\mktmp.exe  %*') do (
  %%I
)

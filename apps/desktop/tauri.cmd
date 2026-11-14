@echo off
setlocal
rem Map the repository first: CMD hooks cannot keep a UNC working directory.
pushd "%~dp0..\.." || exit /b 1
cd apps\desktop
call cargo tauri %*
set "tauri_exit_code=%ERRORLEVEL%"
popd
exit /b %tauri_exit_code%

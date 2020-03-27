set RELEASE_PATH="..\\..\\Nature-Release"

if not exist %RELEASE_PATH% md %RELEASE_PATH%

@REM copy /Y ..\.env %RELEASE_PATH%
copy /Y ..\target\debug\nature.exe %RELEASE_PATH%
copy /Y ..\target\debug\retry.exe %RELEASE_PATH%
copy /Y ..\target\debug\nature_demo_executor.dll %RELEASE_PATH%
copy /Y ..\target\debug\restful_executor.exe %RELEASE_PATH%
@REM copy /Y ..\..\Nature-DB\nature.sqlite %RELEASE_PATH%

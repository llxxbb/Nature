set RELEASE_PATH="..\\..\\Nature-Release"

if not exist %RELEASE_PATH% md %RELEASE_PATH%

copy /Y ..\.env %RELEASE_PATH%
copy /Y ..\target\debug\nature.exe %RELEASE_PATH%
copy /Y ..\target\debug\retry.exe %RELEASE_PATH%
copy /Y ..\target\debug\nature_demo.dll %RELEASE_PATH%
copy /Y ..\target\debug\restful_executor.exe %RELEASE_PATH%
copy /Y schema.sql %RELEASE_PATH%

del %RELEASE_PATH%\release.zip
7z a -tzip %RELEASE_PATH%\release.zip %RELEASE_PATH%\nature.exe %RELEASE_PATH%\retry.exe %RELEASE_PATH%\.env %RELEASE_PATH%\schema.sql
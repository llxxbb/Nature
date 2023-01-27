cargo build --manifest-path=../Cargo.toml -r

set RELEASE_PATH="..\\..\\Nature-Release"

if not exist %RELEASE_PATH% md %RELEASE_PATH%

copy /Y ..\NATURE\.env %RELEASE_PATH%
copy /Y ..\target\release\nature.exe %RELEASE_PATH%
copy /Y ..\target\release\retry.exe %RELEASE_PATH%
copy /Y ..\target\release\nature_demo.dll %RELEASE_PATH%
copy /Y ..\target\release\nature_demo_restful.exe %RELEASE_PATH%
copy /Y schema.sql %RELEASE_PATH%

del %RELEASE_PATH%\release.zip
7z a -tzip %RELEASE_PATH%\release.zip %RELEASE_PATH%\nature.exe %RELEASE_PATH%\retry.exe %RELEASE_PATH%\.env %RELEASE_PATH%\schema.sql %RELEASE_PATH%\nature_demo.dll %RELEASE_PATH%\nature_demo_restful.exe
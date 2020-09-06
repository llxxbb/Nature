git -C .. -c credential.helper= -c core.quotepath=false -c log.showSignature=false checkout -B master origin/master --
git -C ..\..\Nature-Demo -c credential.helper= -c core.quotepath=false -c log.showSignature=false checkout -B master origin/master --
git -C ..\..\Nature-Integrate-Test-Executor -c credential.helper= -c core.quotepath=false -c log.showSignature=false checkout -B master origin/master --

git -C .. -c credential.helper= -c core.quotepath=false -c log.showSignature=false merge origin/dev
git -C ..\..\Nature-Demo -c credential.helper= -c core.quotepath=false -c log.showSignature=false merge origin/dev
git -C ..\..\Nature-Integrate-Test-Executor -c credential.helper= -c core.quotepath=false -c log.showSignature=false merge origin/dev


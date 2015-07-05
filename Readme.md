jobo
====
jobo is a small program for managing [Windows Job Objects][msdn-job-objects].
It will run the process in a job (so any processes started by that process will)
be also joined to the same job by default.

[msdn-job-objects]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms684161(v=vs.85).aspx

Usage
-----

    $ jobo cmd "/c sleep 15 && echo 15 && cmd /c sleep 5 && echo 5"

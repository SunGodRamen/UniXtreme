<div class="markdown prose w-full break-words dark:prose-invert light">
  <h1>UniXtreme</h1>
  <p>UniXtreme is a platform that controls and monitors various elements of Unix-based operating systems, with a focus
    on satisfying the needs of Unix power users in terms of Security, Customizability, Performance, and Compatibility.
    The platform is designed to run on macOS and Arch/Manjaro Linux.</p>
  <h2>Directory Structure</h2>
  <pre><div class="bg-black mb-4 rounded-md"><div class="flex items-center relative text-gray-200 bg-gray-800 px-4 py-2 text-xs font-sans"><span class="">python</span><button class="flex ml-auto gap-2"><svg stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path><rect x="8" y="2" width="8" height="4" rx="1" ry="1"></rect></svg>Copy code</button></div><div class="p-4 overflow-y-auto"><code class="!whitespace-pre hljs language-python">UniXtreme/
  ├── README.md
  ├── <span class="hljs-built_in">bin</span>/                    <span class="hljs-comment"># Contains executable files for the various services and the control app.</span>
  ├── etc/                    <span class="hljs-comment"># Contains configuration files for the platform and the control app.</span>
  ├── lib/                    <span class="hljs-comment"># Contains shared libraries for the various services.</span>
  ├── log/                    <span class="hljs-comment"># Contains log files for the various services.</span>
  ├── run/                    <span class="hljs-comment"># Contains Unix sockets for inter-process communication between the services and the control app.</span>
  ├── scripts/                <span class="hljs-comment"># Contains user-defined scripts that can be launched by the control app.</span>
  ├── src/
  │   ├── app/                <span class="hljs-comment"># Contains files for monitoring and controlling applications.</span>
  │   ├── control/            <span class="hljs-comment"># Contains files for the control app.</span>
  │   ├── hid/                <span class="hljs-comment"># Contains files for monitoring and controlling HID devices.</span>
  │   ├── keyboard/           <span class="hljs-comment"># Contains files for monitoring and controlling keyboard input.</span>
  │   ├── media/              <span class="hljs-comment"># Contains files for monitoring and controlling media playback.</span>
  │   └── window/             <span class="hljs-comment"># Contains files for monitoring and controlling windows.</span>
  └── usr/
      └── share/
          └── man/
              ├── man1/       <span class="hljs-comment"># Contains manual pages for the various services and the control app.</span>
              └── man5/       <span class="hljs-comment"># Contains manual pages for the platform configuration file.</span>
  </code></div></div></pre>
  <h2>Technologies Used</h2>
  <p>UniXtreme is built primarily using Rust, a systems programming language that provides memory safety guarantees and
    helps prevent common programming errors that can lead to security vulnerabilities. The platform also uses Unix
    sockets for inter-process communication between the various services and the control app. Additionally, the platform
    uses a modular approach that allows users to customize and tailor the behavior of the system to their specific
    needs.</p>
  <h2>Role of Each File</h2>
  <p>Here is a brief description of the role of each file in the project:</p>
  <ul>
    <li><code>bin/</code>: Contains executable files for the various services and the control app.</li>
    <li><code>etc/</code>: Contains configuration files for the platform and the control app.</li>
    <li><code>lib/</code>: Contains shared libraries for the various services.</li>
    <li><code>log/</code>: Contains log files for the various services.</li>
    <li><code>run/</code>: Contains Unix sockets for inter-process communication between the services and the control
      app.</li>
    <li><code>scripts/</code>: Contains user-defined scripts that can be launched by the control app.</li>
    <li><code>src/app/</code>: Contains files for monitoring and controlling applications.</li>
    <li><code>src/control/</code>: Contains files for the control app.</li>
    <li><code>src/hid/</code>: Contains files for monitoring and controlling HID devices.</li>
    <li><code>src/keyboard/</code>: Contains files for monitoring and controlling keyboard input.</li>
    <li><code>src/media/</code>: Contains files for monitoring and controlling media playback.</li>
    <li><code>src/window/</code>: Contains files for monitoring and controlling windows.</li>
    <li><code>usr/share/man/man1/</code>: Contains manual pages for the various services and the control app.</li>
    <li><code>usr/share/man/man5/</code>: Contains manual pages for the platform configuration file.</li>
  </ul>
  <p>This directory structure and the files within it are organized according to the Filesystem Hierarchy Standard
    (FHS), a widely used standard for organizing files and directories in Unix-based operating systems.</p>
</div>
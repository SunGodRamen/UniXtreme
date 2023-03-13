<div class="markdown prose w-full break-words dark:prose-invert light">
  <h1>Unix System Control Platform</h1>
  <p>The Unix System Control Platform is a project aimed at providing Unix power users with a secure, customizable,
    performant, and compatible system that controls and monitors various elements of Unix-based operating systems. The
    platform specifically targets macOS and Arch/Manjaro Linux and focuses on satisfying the needs of Unix power users
    in terms of Security, Customizability, Performance, and Compatibility.</p>
  <h2>Services</h2>
  <p>The platform consists of the following services, each responsible for monitoring and controlling a specific system
    element:</p>
  <ul>
    <li>HID monitoring and control service</li>
    <li>Keyboard monitoring and control service</li>
    <li>Application monitoring and control service</li>
    <li>Window monitoring and control service</li>
    <li>Media monitoring and control service</li>
  </ul>
  <p>Additionally, the platform includes a control app that provides a graphical user interface (GUI) for users to
    interact with and control the various services.</p>
  <h2>Directory Structure</h2>
  <p>The directory structure of the platform is organized as follows:</p>
  <ul>
    <li><code>/bin</code>: Contains executable files for the various services and the control app.</li>
    <li><code>/etc</code>: Contains configuration files for the platform and the control app.</li>
    <li><code>/lib</code>: Contains shared libraries for the various services.</li>
    <li><code>/log</code>: Contains log files for the various services.</li>
    <li><code>/run</code>: Contains Unix sockets for inter-process communication between the services and the control
      app.</li>
    <li><code>/scripts</code>: Contains user-defined scripts that can be launched by the control app.</li>
    <li><code>/usr/share/man/man1</code>: Contains manual pages for the various services and the control app.</li>
    <li><code>/usr/share/man/man5</code>: Contains manual pages for the platform configuration file.</li>
  </ul>
  <h2>Technologies Used</h2>
  <p>The platform is implemented primarily in Rust, a systems programming language that provides memory safety
    guarantees and helps prevent common programming errors that can lead to security vulnerabilities. Authentication
    mechanisms are implemented to ensure that only authorized users can access control and monitor services, and
    encryption is used for messages exchanged over Unix sockets.</p>
  <p>The platform also uses Unix sockets for event queues, which are designed for inter-process communication and can
    handle a high volume of messages with low overhead. Separate control and monitor services are optimized for their
    specific tasks, ensuring that the system remains responsive and uninvasive. The use of a modular approach and a
    configuration file allows for greater flexibility and customization of the system.</p>
  <h2>Getting Started</h2>
  <p>To get started with the platform, follow these steps:</p>
  <ol>
    <li>Clone the repository.</li>
    <li>Build the services and the control app using the provided build scripts.</li>
    <li>Configure the platform and the control app using the provided configuration files.</li>
    <li>Run the services and the control app.</li>
  </ol>
  <p>For more detailed instructions on building and running the platform, see the platform manual pages located in
    <code>/usr/share/man/man5</code>.</p>
</div>
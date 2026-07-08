# Portcullis
## Protocol translation and socket mapping

### Goals
The goal of this project is for me to learn Rust, while also learning socket programming, advanced networking, and packet capturing.

The primary vehicle of this deep study is a program called Portcullis, which will be a protocol translator. The user will be able to point Portcullis at a specific data stream, socket, or other source and choose a destination to receive the output. Portcullis will use Rust, leaning heavily on UDPSocket from the std library, and convert the data stream/socket output to the desired format at the desired destination. A camera outputing video in UDP over a VPN, converted to RTSP and sent to a TAK server, would be one example of an application.

#### Phases

##### Fact-finding and research

This phase will serve to teach me (the architect) about the needed languages, libraries, protocols, standards, best practices, and conventions. I am an IT helpdesk tech and Systems/BI Analyst-turned-Data Engineer who is now training in Robotics Engineering and embedded systems. I know Python, SQL, and a good bit of bash/shell but have very little experience with Rust or low-level languages. I want to learn the basics and build my way up.

##### Test programs to learn syntax, conventions, and the Rust standard library

This is the phase we are currently on as of this document creation. I have built a very simple UDP listener, and I plan to build a UDP transmitter and then finally a UDP relay. 

After I have learned the basics of UDP and sockets, I will transition to TCP experiments.

##### Expansion and continued education

Next will be learning the particulars of a few types of commonly used protocols, particularly in the UAS space, as that will be the primary initial use of the tools. We will work with TCP and UDP, add/remove wrappers for other protocols for various network OSI layers, and create some sample messages for training a model in the next phases.

##### Model input and protocol translation

This phase will serve as the prototype of the final Portcullis tool. Goal of the end result will entail grabbing data from a particular socket connection, translating/cleaning it into a different message format or protocol wrapper, and then pushing that to the destination socket.

##### Refinement and polish

This will be the testing and CI/CD phase. Other developers will begin using the "beta" version of the tool and make suggestions for optimizations or extra features.

### Further efforts

The next phase of development, depending on the success of the tool, will be to implement it as one piece of a larger toolset that will allow a developer to capture traffic on a network, filter and identify certain info streams via a built-in, specifically trained AI agent, and then send them through Portcullis for translation to a specified destination.

For example, a Trillium HD40 broadcasting video on a network via an unknown protocol (and possibly with an unknown port or IP). This device would be difficult to capture manually and would require much trial and error. Using the toolset, the developer can have the agent auto-recognize and filter based on headers added via network encapsulation, hex values to identify encoding, and so forth. 

### Languages

- Rust (primary tool language for protocol and socket manipulation)
- Python (for any AI/agentic pieces)
- Linux/Bash shell (for OS access and protocol/stream recognition)
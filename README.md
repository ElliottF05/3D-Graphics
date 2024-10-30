# Web-Based 3D Graphics Renderer

## Demo
Check out the live demo of the project:  
[**3D Graphics Renderer - Interactive Demo**](https://elliottf05.github.io/3D-Graphics/)

## Overview
This project is a **fully custom-built web-based 3D graphics renderer** developed from the ground up in C++ and compiled to WebAssembly. It combines **rasterization for real-time performance** with an optional **ray-tracing mode** for high-quality, physically-based renders. Users can interact with the scene in real-time by adding or removing objects, with immediate updates to lighting and rendering.

## Features
- **Hybrid Rendering Pipeline:** Utilizes rasterization for fast real-time rendering and ray-tracing for higher-quality, physically-based visuals.
- **Advanced Lighting:** Implements diffuse and specular reflections, real-time shadow mapping, and a custom z-buffer for accurate depth calculations and realistic lighting effects.
- **Multi-Threading and Performance:** Built with a custom thread pool for high-performance computing and CPU parallelization, significantly improving rendering speed and responsiveness.
- **Real-Time Interactivity:** Users can dynamically add or remove objects in the scene, with real-time updates to lighting and rendering, running at 50fps.
- **Cross-Platform Compatibility:** The renderer is compiled to WebAssembly, enabling seamless execution in web browsers on various devices.
- **User Authentication and Data Persistence:** Integrates **Supabase** with **PostgreSQL** for user authentication and secure storage of user-generated scenes and images.

## Technologies Used
- **C++**: Core development language for the renderer.
- **WebAssembly**: Compiled output for cross-platform browser execution.
- **TypeScript**: Front-end for handling interactions and web components.
- **Supabase + PostgreSQL**: Used for user authentication and data storage.
- **Git/GitHub**: Version control and project management.
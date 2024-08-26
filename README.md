<a id="readme-top"></a>

> [!CAUTION]
> <ins><strong>THIS IS AN EXPERIMENTAL, UNCOMPLETED, PRE-ALPHA VERSION </ins></strong> <br />
> Please be aware, that this is not anywhere near to be released. Eventually we are planning to replace the upstream python version with TagStudioRusted, but that is still far away. Until an actually useable alpha release, TagStudioRusted is recommended for Contributors only.

[Product Name Screenshot][product-screenshot]
<p align="right"><i>Current Dashboard of TagStudioRusted running on (Arch) Linux</i></p>

TagStudio is a photo & file organization application with an underlying system that focuses on giving freedom and flexibility to the user. No proprietary programs or formats, no sea of sidecar files, and no complete upheaval of your filesystem structure.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* [![Tauri][tauri]][tauri-url]
* [![Rust][rust]][rust-url]
* [![SvelteKit][sveltekit]][sveltekit-url]
* [![Typescript][typescript]][typescript-url]
* [![TailwindCSS][tailwind]][tailwind-url]
* [![DaisyUI][daisy]][daisy-url]
* [![Paraglide][paraglide]][paraglide-url]
* [![SQLite][sqlite]][sqlite-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Philosophy

TagStudio is not a product. It is an *idea*. Here are the boundaries that define it.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

#### Goals

> [!NOTE]
> **TLDR:** Portable. Private. Open. Extensible. Feature-rich. Powerful. Fast. Reliable. Lightweight. Scalable. Cross-platform. Beautiful. Tags. Lots of em.
>
>  Neat, right?

- To achieve a portable, privacy-oriented, open, extensible, and feature-rich system of organizing and rediscovering files.
- To provide powerful methods for organization, notably the concept of tag composition, or “taggable tags”.
- To create an implementation of such a system that is resilient against a user’s actions outside the program (modifying, moving, or renaming files) while also not burdening the user with mandatory sidecar files or otherwise requiring them to change their existing file structures and workflows.
- To support a wide range of users spanning across different platforms, multi-user setups, and those with large (several terabyte) libraries.
- To make the darn thing look like nice, too. It’s 2024, not 1994.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

#### Priorities

<p align="right">(<a href="#readme-top">back to top</a>)</p>


> [!NOTE]
> This list is being expanded as we move forward with the project. Not all planned features are listed.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Top Priority

- [x] Have a working window
- [x] Build the app i18n ready (continous)
- [ ] Expand Building Instructions
- [ ] UI Layout and design
- [ ] Have a mostly complete frontend
  - [ ] Dashboard
    - [x] Have file type Buttons
    - [ ] Show stats for each filetype
    - [ ] Most recent list at the bottom
- [ ] Expanding this list

- [ ] LATER: Replace TagStudio upstream with TagStudioRusted

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Low Priority

- [ ] Start working on backend (only after having a somewhat useable frontend)
- [ ] Translations
  - [x] English (native)
  - [ ] German
  - [ ] French
  - [ ] Spanish
  - [ ] Make a template for community translations
- [ ] Themes
  - [ ] Additional Themes
  - [ ] Easy Theme Hue Adjustion
  - [ ] Fully Custom Themes

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Request a Feature

We currently accept no feature requests. Our top priority is to match the featureset of the upstream python version first.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Installation

This section will guide you through the requirements and installation of TagStudio.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Prerequisites

> [!CAUTION]
> There are currently no official builds for the project yet. We do, however develop TagStudioRusted with Cross-platform support in mind. You can build then project for yourself on any platform, and open a Bug Report issue in case you find something unusual. After we get to a point, where this build is *somewhat* usable, we will have prebuilt binaries available to download.

Binaries System Support:

|        | [![Windows][win]][win-url] | [![Mac][macos]][macos-url]| [![Linux][linux]][linux-url] |
| :----: | :-------------: | :-----------: | :-------------: |
| x86_64 |  ❌ | ❌ | ❌ |
| ARM | ❌ | ❌ | ❌ |
| Portable | ❌ | ❌ | ❌ |

Feel free to build the project for the architecture and OS you need.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Building from Source

> [!NOTE]
> This is currently the only installation method.

Thanks Tauri's documentation, the OS specific installations are documented there.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Step 1.

- Installing Tauri and Rust according to **[Tauri's documentation](https://tauri.app/v1/guides/getting-started/prerequisites)**
- We also intend to use a JavaScript frontend framework, so **we need Node.js installed** as well (also in the docs above).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Step 2.

- Cloning the project

```
git clone https://github.com/TagStudioDev/TagStudioRusted.git
cd TagStudioRusted
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Step 3.

- Installing packages
- We recommend using [pnpm](https://pnpm.io/) as package manager. It's fast, space efficient. It's just better.

```
pnpm install
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Step 4.

- You can now run a dev server, or build the project.
  > [!WARNING]
  > You may need the `WEBKIT_DISABLE_COMPOSITING_MODE=1` AND/OR `WEBKIT_DISABLE_DMABUF_RENDERER=1`enviroment variable while running the command below if you get a blank screen.

```
pnpm tauri dev
```

- Building and then running the project.
  > [!CAUTION]
  > Linux: It requires `NO_STRIP=true` as enviroment variable due to a bug in linuxdeploy, which is used to build .Appimage file.

```
pnpm tauri build
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Step 5.

> [!WARNING]
> You may need the `WEBKIT_DISABLE_COMPOSITING_MODE=1` AND/OR `WEBKIT_DISABLE_DMABUF_RENDERER=1` enviroment variable while running the command below if you get a blank screen.

- Locate the binary in `/src-tauri/target/release/bundle`, pray and then run it.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

Please feel free to contribute according to our Roadmap and Bug Reports. Thank you for helping enchancing TagStudioRusted further!

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Top contributors
<h3 align="center">They are the ❤️ of our Project</h3> <br />
<a href="https://github.com/TagStudioDev/TagStudioRusted/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=TagStudioDev/TagStudioRusted" alt="TagStudio Top Contributors" />
</a>

## License

Distributed under the GPL-3.0 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Social & Contact

* [![Discord][discord]][discord-url]
* [![CyanVoxelYT][cyanyt]][cyanyt-url]

Project Link: [https://github.com/TagStudioDev/TagStudioRusted](https://github.com/TagStudioDev/TagStudioRusted)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments

* [othneildrew's Readme Template](https://github.com/othneildrew/best-readme-template/)
* [CyanVoxel's absolute banger video](https://youtu.be/wTQeMkYRMcw?si=NEooVj0zsfHoSQJt)
* Mario 😼

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[win]: https://img.shields.io/badge/Windows-blue?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48IS0tIFVwbG9hZGVkIHRvOiBTVkcgUmVwbywgd3d3LnN2Z3JlcG8uY29tLCBHZW5lcmF0b3I6IFNWRyBSZXBvIE1peGVyIFRvb2xzIC0tPgo8c3ZnIHdpZHRoPSI4MDBweCIgaGVpZ2h0PSI4MDBweCIgdmlld0JveD0iMCAwIDM2IDM2IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiBhcmlhLWhpZGRlbj0idHJ1ZSIgcm9sZT0iaW1nIiBjbGFzcz0iaWNvbmlmeSBpY29uaWZ5LS10d2Vtb2ppIiBwcmVzZXJ2ZUFzcGVjdFJhdGlvPSJ4TWlkWU1pZCBtZWV0Ij48cGF0aCBmaWxsPSIjQkY2OTUyIiBkPSJNMzMuNTQxIDIzLjE5OGMuMzY0LTEuNTc4LjI0My0zLjI2Ni0uNDU4LTQuOTQ2YTguMDE4IDguMDE4IDAgMCAwLTMuMjcxLTMuNzczYy4zMTgtMS4xOTIuMjM0LTIuNDc1LS4zMjQtMy43NWMtLjg0MS0xLjkyLTIuNjYtMy4yMDEtNC43MTItMy41NjJjLjI0OS0uNTcyLjMyOS0xLjI4OS4wMzYtMi4xNjdjLTEtMy01LTEtOC00Ljk5OWMtMi40NCAxLjQ2NC0yLjk3IDMuNjQtMi44NzggNS40ODdjLTIuNDIxLjQxMi0zLjguOTM2LTMuOC45MzZ2LjAwMmEzLjcxMyAzLjcxMyAwIDAgMC0yLjMyMiAzLjQ0MmMwIC44NzkuMzE4IDEuNjc2LjgyOCAyLjMxMmwtLjY5Mi4yNThsLjAwMS4wMDNjLTIuMzMuODcxLTMuOTc1IDIuOTc2LTMuOTc1IDUuNDM5YzAgMS4wNDcuMyAyLjAyNy44MiAyLjg3OEMxLjk3MSAyMi4wMjcgMCAyNC43ODEgMCAyOGMwIDQuNDE4IDMuNjkxIDggOC4yNDQgOGMzLjI2OSAwIDYuNTU5LS43MDMgOS41MzEtMS42NjVDMjAuMDE4IDM1LjM3NSAyMy40NyAzNiAyOC42NjcgMzZBNy4zMzMgNy4zMzMgMCAwIDAgMzYgMjguNjY3YTcuMzEgNy4zMSAwIDAgMC0yLjQ1OS01LjQ2OXoiPjwvcGF0aD48ZWxsaXBzZSBmaWxsPSIjRjVGOEZBIiBjeD0iMTMuNSIgY3k9IjE1LjUiIHJ4PSIzLjUiIHJ5PSI0LjUiPjwvZWxsaXBzZT48ZWxsaXBzZSBmaWxsPSIjRjVGOEZBIiBjeD0iMjMuNSIgY3k9IjE1LjUiIHJ4PSIzLjUiIHJ5PSI0LjUiPjwvZWxsaXBzZT48ZWxsaXBzZSBmaWxsPSIjMjkyRjMzIiBjeD0iMTQiIGN5PSIxNS41IiByeD0iMiIgcnk9IjIuNSI+PC9lbGxpcHNlPjxlbGxpcHNlIGZpbGw9IiMyOTJGMzMiIGN4PSIyMyIgY3k9IjE1LjUiIHJ4PSIyIiByeT0iMi41Ij48L2VsbGlwc2U+PHBhdGggZmlsbD0iIzI5MkYzMyIgZD0iTTkuNDQ3IDI0Ljg5NUM5LjIwMSAyNC40MDIgOS40NSAyNCAxMCAyNGgxOGMuNTUgMCAuNzk5LjQwMi41NTMuODk1QzI4LjU1MyAyNC44OTUgMjYgMzAgMTkgMzBzLTkuNTUzLTUuMTA1LTkuNTUzLTUuMTA1eiI+PC9wYXRoPjxwYXRoIGZpbGw9IiNGMkFCQkEiIGQ9Ik0xOSAyNmMtMi43NzEgMC01LjE1Ny45MjItNi4yOTIgMi4yNTZDMTQuMiAyOS4yMTEgMTYuMjUzIDMwIDE5IDMwczQuODAxLS43ODkgNi4yOTItMS43NDRDMjQuMTU3IDI2LjkyMiAyMS43NzEgMjYgMTkgMjZ6Ij48L3BhdGg+PC9zdmc+
[win-url]: https://youtu.be/toTtunvlqE4?si=5yJDWt9QkzAIbbYG
[macos]: https://img.shields.io/badge/MacOS-000000?style=for-the-badge&logo=apple&logoColor=white
[macos-url]: https://youtu.be/aE9_olxc-cA?si=K2pIs7CfkLD71qVv
[linux]: https://img.shields.io/badge/Linux-yellow?style=for-the-badge&logo=linux&logoColor=000000
[linux-url]: https://en.wikipedia.org/wiki/Linux

[discord]: https://img.shields.io/badge/Discord_Server-5865F2?style=for-the-badge&logo=discord&logoColor=white
[discord-url]: https://discord.gg/hRNnVKhF2G
[cyanyt]: https://img.shields.io/badge/CyanVoxel's_YouTube_Channel-red?style=for-the-badge&logo=youtube&logoColor=white
[cyanyt-url]: https://youtube.com/@cyanvoxel


[contributors-shield]: https://img.shields.io/github/contributors/TagStudioDev/TagStudioRusted.svg?style=for-the-badge
[contributors-url]: https://github.com/TagStudioDev/TagStudioRusted/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/TagStudioDev/TagStudioRusted.svg?style=for-the-badge
[forks-url]: https://github.com/TagStudioDev/TagStudioRusted/network/members
[stars-shield]: https://img.shields.io/github/stars/TagStudioDev/TagStudioRusted.svg?style=for-the-badge
[stars-url]: https://github.com/TagStudioDev/TagStudioRusted/stargazers
[issues-shield]: https://img.shields.io/github/issues/TagStudioDev/TagStudioRusted.svg?style=for-the-badge
[issues-url]: https://github.com/TagStudioDev/TagStudioRusted/issues
[license-shield]: https://img.shields.io/github/license/TagStudioDev/TagStudioRusted.svg?style=for-the-badge
[license-url]: https://github.com/TagStudioDev/TagStudioRusted/blob/master/LICENSE.txt
[product-screenshot]: .github/images/tauri-tagstudio-current.png
[qt]: https://img.shields.io/badge/Qt_For_Python-000000?style=for-the-badge&logo=qt&logoColor=white
[qt-url]: https://doc.qt.io/qtforpython-6/
[python-url]: https://python.org/
[python]: https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=Python&logoColor=ffc331
[sqlite]: https://img.shields.io/badge/Sqlite-003B57?style=for-the-badge&logo=sqlite&logoColor=f2f2f2
[sqlite-url]: https://sqlite.org/
[tauri]: https://img.shields.io/badge/Tauri-0d798f?style=for-the-badge&logo=tauri&logoColor=d6ae0d
[tauri-url]: https://tauri.app/
[rust]: https://img.shields.io/badge/Rust-bd4606?style=for-the-badge&logo=rust&logoColor=black
[rust-url]: https://rustlang.org/
[sveltekit]: https://img.shields.io/badge/Sveltekit-gray?style=for-the-badge&logo=svelte&logoColor=FF3E00
[sveltekit-url]: https://kit.svelte.dev/
[typescript]: https://img.shields.io/badge/Typescript-000000?style=for-the-badge&logo=typescript&logoColor=3178C6
[typescript-url]: https://typescript.org/
[tailwind]: https://img.shields.io/badge/TailwindCss-gray?style=for-the-badge&logo=tailwindcss&logoColor=06B6D4
[tailwind-url]: https://tailwindcss.com/
[daisy]: https://img.shields.io/badge/DaisyUI-5A0EF8?style=for-the-badge&logo=daisyui&logoColor=white
[daisy-url]: https://daisyui.com/
[paraglide]: https://img.shields.io/badge/Paraglide-f2f2f2?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iaXNvLTg4NTktMSI/Pg0KPCEtLSBVcGxvYWRlZCB0bzogU1ZHIFJlcG8sIHd3dy5zdmdyZXBvLmNvbSwgR2VuZXJhdG9yOiBTVkcgUmVwbyBNaXhlciBUb29scyAtLT4NCjxzdmcgZmlsbD0iIzAwMDAwMCIgaGVpZ2h0PSI4MDBweCIgd2lkdGg9IjgwMHB4IiB2ZXJzaW9uPSIxLjEiIGlkPSJDYXBhXzEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIA0KCSB2aWV3Qm94PSIwIDAgNjEyIDYxMiIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSI+DQo8Zz4NCgk8cGF0aCBkPSJNNTM4LjIwMywyMTEuMzUzQzUzNi41NDcsOTQuNTU2LDQzMy4wMDMsMCwzMDUuODkzLDBDMTc3Ljc1LDAsNzMuNSw5Ni4wODEsNzMuNSwyMTQuMThjMCwwLjEwOCwwLjAyOCwwLjIwNCwwLjAzMSwwLjMxMg0KCQljMC4wMTEsMC41MywwLjA4OCwxLjA1NSwwLjE1OSwxLjU4NWMwLjA1NCwwLjQwNSwwLjA4NSwwLjgxNywwLjE3MywxLjIxMWMwLjEwMiwwLjQ1NCwwLjI2NywwLjg5LDAuNDE0LDEuMzM2DQoJCWMwLjE0NywwLjQ0NSwwLjI3NSwwLjg5MywwLjQ2NSwxLjMxOWMwLjE2NCwwLjM2OSwwLjM4NiwwLjcxNywwLjU4NCwxLjA3N2MwLjI1OCwwLjQ2NSwwLjUwOCwwLjkzLDAuODE3LDEuMzYxDQoJCWMwLjA2MiwwLjA4OCwwLjA5NiwwLjE4NCwwLjE2MiwwLjI2OWwxODQuNTMyLDI0Ny44MTJsLTIyLjMzOCwxMy4wNDNjLTYuNzYzLDMuOTUtOS4wNDIsMTIuNjI5LTUuMDk1LDE5LjM5Mg0KCQljMi42MzcsNC41MTcsNy4zODEsNy4wMzIsMTIuMjU4LDcuMDMyYzIuNDI3LDAsNC44ODgtMC42MjcsNy4xMzQtMS45MzdsMTQuNzA1LTguNTg2djQwLjMxMnY1OC4xMDINCgkJYzAsNy44MzIsNi4zNDksMTQuMTc4LDE0LjE3OCwxNC4xNzhzMTQuMTc4LTYuMzQ2LDE0LjE3OC0xNC4xNzh2LTQzLjkyNWgyMC4zMjh2NDMuOTI1YzAsNy44MzIsNi4zNDksMTQuMTc4LDE0LjE3OCwxNC4xNzgNCgkJczE0LjE3OC02LjM0NiwxNC4xNzgtMTQuMTc4VjUzOS43MnYtMzguNDY0bDExLjU0Myw2Ljc0YzIuMjQ5LDEuMzEsNC43MDcsMS45MzcsNy4xMzQsMS45MzdjNC44NzcsMCw5LjYyNC0yLjUxOCwxMi4yNTgtNy4wMzINCgkJYzMuOTUtNi43NjMsMS42NjctMTUuNDQyLTUuMDk1LTE5LjM5MmwtMjAuMTM1LTExLjc1NmwxODUuNDkxLTI0OS4xQzUzOC4yNDksMjE5LjI3OSw1MzguOTgzLDIxNS4xNyw1MzguMjAzLDIxMS4zNTN6DQoJCSBNMzM0Ljk2LDM5Ny40NzFjLTUuNjIzLTEwLjIwOC0xNi40ODMtMTcuMTQxLTI4LjkzNi0xNy4xNDFjLTEyLjQ1NCwwLTIzLjMxNCw2LjkzNi0yOC45MzYsMTcuMTQxbC00Mi4wNDgtMTY5LjExM2gxNDEuOTY4DQoJCUwzMzQuOTYsMzk3LjQ3MXogTTExNS45MTEsMjI4LjM1OGg4OS45MTFsNDUuMDY4LDE4MS4yNjZMMTE1LjkxMSwyMjguMzU4eiBNNDA2LjIyNCwyMjguMzU4aDg5LjkwOEwzNjEuMTU3LDQwOS42MjENCgkJTDQwNi4yMjQsMjI4LjM1OHoiLz4NCjwvZz4NCjwvc3ZnPg==
[paraglide-url]: https://inlang.com/m/gerre34r/library-inlang-paraglideJs

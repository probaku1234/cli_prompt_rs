<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <!-- <a href="https://github.com/probaku1234/cli_prompt_rs">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a> -->

<h3 align="center">cli_prompt_rs</h3>

  <p align="center">
    Easily build beautiful command-line apps
    <br />
    <a href="https://github.com/probaku1234/cli_prompt_rs"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/probaku1234/cli_prompt_rs/blob/main/examples/example1.rs">View Demo</a>
    ·
    <a href="https://github.com/probaku1234/cli_prompt_rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/probaku1234/cli_prompt_rs/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <!-- <li><a href="#prerequisites">Prerequisites</a></li> -->
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <!-- <li><a href="#contact">Contact</a></li> -->
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

Inspired by [@clack/prompts](https://github.com/natemoo-re/clack),
This library provides easy-to-use CLI prompt functions to help you build beautiful command-line-apps easily.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

<!-- * [![Next][Next.js]][Next-url]
* [![React][React.js]][React-url]
* [![Vue][Vue.js]][Vue-url]
* [![Angular][Angular.io]][Angular-url]
* [![Svelte][Svelte.dev]][Svelte-url]
* [![Laravel][Laravel.com]][Laravel-url]
* [![Bootstrap][Bootstrap.com]][Bootstrap-url]
* [![JQuery][JQuery.com]][JQuery-url] -->
* [![Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started



<!-- ### Prerequisites

This is an example of how to list things you need to use the software and how to install them.
* npm
  ```sh
  npm install npm@latest -g
  ``` -->

### Installation

Run the following Cargo command in your project directory:
   ```sh
   cargo add cli_prompts_rs
   ```
Or add the following line to your Cargo.toml:
  ```sh
  cli_prompts_rs = "0.1"
  ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

```rust
 use cli_prompts_rs::{CliPrompt, LogType, PromptSelectOption};
 use std::process::exit;

 fn main() {
     let mut cli_prompt = CliPrompt::new();
     cli_prompt.intro("example app").unwrap();

     cli_prompt.prompt_text("Enter your name").unwrap();

     let answer = cli_prompt.prompt_confirm("Are you sure?").unwrap();

     if !answer {
         cli_prompt.cancel("Operation cancelled").unwrap();
         exit(0);
     }

     let options = vec![
         PromptSelectOption::new("option1", "Pikachu"),
         PromptSelectOption::new("option2", "Charmander"),
         PromptSelectOption::new("option3", "Squirtle"),
     ];
     let selected_option = cli_prompt
         .prompt_select("Which one do you prefer?", options)
         .unwrap();

     cli_prompt
         .log(&format!("{}", selected_option), LogType::Info)
         .unwrap();
     cli_prompt.outro("Good Bye").unwrap();
 }
```

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Multiple Choice
- [ ] Print memo
- [ ] Color / style utils


See the [open issues](https://github.com/probaku1234/cli_prompt_rs/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
<!-- ## Contact -->

<!-- Your Name - [@twitter_handle](https://twitter.com/twitter_handle) - fhzotxldj@gmail.com

Project Link: [https://github.com/probaku1234/cli_prompt_rs](https://github.com/probaku1234/cli_prompt_rs)

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* []()
* []()
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/probaku1234/cli_prompt_rs.svg?style=for-the-badge
[contributors-url]: https://github.com/probaku1234/cli_prompt_rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/probaku1234/cli_prompt_rs.svg?style=for-the-badge
[forks-url]: https://github.com/probaku1234/cli_prompt_rs/network/members
[stars-shield]: https://img.shields.io/github/stars/probaku1234/cli_prompt_rs.svg?style=for-the-badge
[stars-url]: https://github.com/probaku1234/cli_prompt_rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/probaku1234/cli_prompt_rs.svg?style=for-the-badge
[issues-url]: https://github.com/probaku1234/cli_prompt_rs/issues
[license-shield]: https://img.shields.io/github/license/probaku1234/cli_prompt_rs.svg?style=for-the-badge
[license-url]: https://github.com/probaku1234/cli_prompt_rs/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/demo.gif
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 
[Rust]: https://img.shields.io/badge/Rust-4A4A55?style=for-the-badge&logo=Rust&logoColor=black
[Rust-url]: https://www.rust-lang.org/
# sd-wildcard-editor

## To Do
### Main Plans
- [ ] Wildcard Subject structure<br>
A structure that allows sorting wildcards into a structure similar to directories. All wildcards under one subject will be merged into a "Compository Wildcard" or a "Combination Wildcard" which takes the name of the subject itself.

- [ ] Subject templates<br>
A system allowing users to set up semi-automated templates. Useful for character definitions.

- [ ] Rich and intuitive interface<br>
To help the user streamline their process with max efficiency.

- [ ] Tag Catagorization<br>
The ability to catagorize tags. Will improve automation.

### Extras
- [ ] RNG Wildcards<br>
Allow users to adjust the odds of each entry in a wildcard appearing by adjusting sliders in the interface.

- [ ] A1111 & Fooocus Integrations<br>
Not sure if it's needed for A1111, but Fooocus could benefit from having its files refreshed via sd-wildcard-editor.



## Structure
**sd-wildcard-editor** works on a "project basis". Every project represents a collection of different wildcards, and each project also serves as a combination of all that project's wildcards.

![image](https://github.com/Cruxial0/sd-wildcard-editor/assets/25249091/e3cf4b0b-56e2-4604-b97d-9b7a3a94ec79)

A project can also contain other projects, in which case, the project's combination wildcard will be used.
![image](https://github.com/Cruxial0/sd-wildcard-editor/assets/25249091/c822236b-4d35-4f6b-ac35-1ab77b4752cf)

### Grouped wildcards 
>Terms for these are (probably) not final

**Compository wildcard:** creates a list of underlying wildcards and adds them to the subject wildcard.
```
📁 CHARACTERS 
 ∟ CHARACTER-1
 ∟ CHARACTER-2
 ∟ CHARACTER-3
 ∟ CHARACTER-4
```
```
__CHARACTER-1__
__CHARACTER-2__
__CHARACTER-3__
__CHARACTER-4__
```
**Combination wildcard:** merges all underlying wildcards into a single definition
```
📁 CHARACTER 
 ∟ CHARACTER-BASE
 📁 CHARACTER-CLOTHES 
  ∟ CHARACTER-CLOTHES-UPPER
  ∟ CHARACTER-CLOTHES-BOTTOM
```
```
__CHARACTER-BASE__, __CHARACTER-CLOTHES__
```

## Contributing
Contributions are always welcome, and encouraged.

### Setting up
1. Install Rust & Cargo
2. Install pnpm
3. Set up tauri ([guide](https://tauri.app/v1/guides/getting-started/prerequisites))
4. Install dependencies `pnpm install`
5. Start the tauri development server with `cargo tauri dev`
